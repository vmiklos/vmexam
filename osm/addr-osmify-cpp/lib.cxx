/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#include "lib.hxx"

#include <chrono>
#include <cxxabi.h>
#include <future>
#include <iostream>
#include <sstream>
#include <string>
#include <system_error>

#include <Poco/Dynamic/Var.h>
#include <Poco/Exception.h>
#include <Poco/JSON/Array.h>
#include <Poco/JSON/Object.h>
#include <Poco/JSON/Parser.h>
#include <Poco/Net/Context.h>
#include <Poco/Net/HTTPSStreamFactory.h>
#include <Poco/Net/NetSSL.h>
#include <Poco/Net/RejectCertificateHandler.h>
#include <Poco/Net/SSLManager.h>
#include <Poco/SharedPtr.h>
#include <Poco/URI.h>

#include "urllib.hxx"

namespace osmify
{

/// Handles SSL state lifecycle.
class SslContext
{
  public:
    /// Sets up SSL state.
    SslContext();
    /// Shuts down SSL state.
    ~SslContext();
};

SslContext::SslContext()
{
    Poco::Net::initializeSSL();
    Poco::Net::HTTPSStreamFactory::registerFactory();
    // Just reject untrusted certificates.
    Poco::SharedPtr certHandler(new Poco::Net::RejectCertificateHandler(false));
    // Trust system certificates.
    const int verificationDepth = 9;
    Poco::AutoPtr netContext(new Poco::Net::Context(
        Poco::Net::Context::CLIENT_USE, "", Poco::Net::Context::VERIFY_RELAXED,
        verificationDepth, /*loadDefaultCAs=*/true));
    Poco::Net::SSLManager::instance().initializeClient(nullptr, certHandler,
                                                       netContext);
}

SslContext::~SslContext()
{
    Poco::Net::HTTPSStreamFactory::unregisterFactory();
    Poco::Net::uninitializeSSL();
}

/// Gets the properties of an OSM object from overpass.
std::string queryTurbo(const std::string& query)
{
    std::string url("https://overpass-api.de/api/interpreter");
    return urllib::request::urlopen(url, query);
}

/// Gets the OSM object from nominatim.
std::string queryNominatim(const std::string& query)
{
    Poco::URI uri("https://nominatim.openstreetmap.org/search.php");
    uri.addQueryParameter("q", query);
    uri.addQueryParameter("format", "json");

    return urllib::request::urlopen(uri.toString(), "");
}

/// Turns an address into a coodinate + normalized address combo.
std::string osmify(const std::string& query)
{
    std::stringstream ret;
    SslContext sslContext;

    // Use nominatim to get the coordinates and the osm type/id.
    Poco::JSON::Parser parser;
    Poco::Dynamic::Var elements;
    try
    {
        elements = parser.parse(queryNominatim(query));
    }
    catch (const Poco::Exception& exception)
    {
        ret << "Failed to parse JSON from nominatim: " << exception.message();
        return ret.str();
    }
    auto elementsArray = elements.extract<Poco::JSON::Array::Ptr>();
    if (elementsArray->size() == 0)
    {
        ret << "No results from nominatim";
        return ret.str();
    }

    if (elementsArray->size() > 1)
    {
        // There are multiple elements, prefer buildings if possible.
        // Example where this is useful: 'Karinthy Frigyes út 18, Budapest'.
        Poco::SharedPtr buildings(new Poco::JSON::Array());
        for (const auto& element : *elementsArray)
        {
            auto elementObject = element.extract<Poco::JSON::Object::Ptr>();
            if (!elementObject->has("class"))
            {
                continue;
            }

            if (elementObject->getValue<std::string>("class") != "building")
            {
                continue;
            }

            buildings->add(element);
        }

        if (buildings->size() > 0)
        {
            elementsArray = buildings;
        }
    }

    Poco::Dynamic::Var element = elementsArray->get(0);
    auto elementObject = element.extract<Poco::JSON::Object::Ptr>();
    std::string lat = elementObject->get("lat").toString();
    std::string lon = elementObject->get("lon").toString();
    std::string objectType = elementObject->get("osm_type").toString();
    std::string objectId = elementObject->get("osm_id").toString();

    // Use overpass to get the properties of the object.
    std::string overpassQuery = "[out:json];\n"
                                "(\n" +
                                objectType + "(" + objectId + ");" +
                                ");\n"
                                "out body;";
    Poco::Dynamic::Var j;
    try
    {
        j = parser.parse(queryTurbo(overpassQuery));
    }
    catch (const Poco::Exception& exception)
    {
        ret << "Failed to parse JSON from overpass: " << exception.message();
        return ret.str();
    }
    auto jObject = j.extract<Poco::JSON::Object::Ptr>();
    elements = jObject->get("elements");
    elementsArray = elements.extract<Poco::JSON::Array::Ptr>();
    if (elementsArray->size() == 0)
    {
        ret << "No results from overpass";
        return ret.str();
    }

    element = elementsArray->get(0);
    elementObject = element.extract<Poco::JSON::Object::Ptr>();
    Poco::Dynamic::Var tags = elementObject->get("tags");
    auto tagsObject = tags.extract<Poco::JSON::Object::Ptr>();
    std::string city = tagsObject->get("addr:city").toString();
    std::string housenumber = tagsObject->get("addr:housenumber").toString();
    std::string postcode = tagsObject->get("addr:postcode").toString();
    std::string street = tagsObject->get("addr:street").toString();
    std::string addr =
        postcode + " " + city + ", " + street + " " + housenumber;

    // Print the result.
    ret << "" << lat << "," << lon << " (" << addr << ")";
    return ret.str();
}

void spinner(std::future<std::string>& future, std::ostream& ostream)
{
    std::vector<char> spinCharacters = {'\\', '|', '/', '-'};
    std::size_t spinIndex = 0;
    while (true)
    {
        const int sleep = 100;
        std::future_status status =
            future.wait_for(std::chrono::milliseconds(sleep));
        if (status == std::future_status::ready)
        {
            std::cerr << "\r";
            std::cerr.flush();
            ostream << future.get() << std::endl;
            return;
        }

        std::cerr << "\r [" << spinCharacters[spinIndex] << "] ";
        std::cerr.flush();
        spinIndex = (spinIndex + 1) % spinCharacters.size();
    }
}

int main(const std::vector<const char*>& args, std::ostream& ostream)
{
    if (args.size() > 1)
    {
        std::string query = args[1];
        std::future<std::string> future =
            std::async(std::launch::async, [&query] { return osmify(query); });

        spinner(future, ostream);
    }
    else
    {
        ostream << "usage: addr-osmify <query>" << std::endl;
        ostream << std::endl;
        ostream << "e.g. addr-osmify 'Mészáros utca 58/a, Budapest'"
                << std::endl;
    }
    return 0;
}
} // namespace osmify

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
