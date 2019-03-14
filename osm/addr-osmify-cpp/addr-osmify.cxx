/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#include <iostream>
#include <memory>
#include <sstream>
#include <string>

#include <Poco/Dynamic/Var.h>
#include <Poco/Exception.h>
#include <Poco/JSON/Array.h>
#include <Poco/JSON/Object.h>
#include <Poco/JSON/Parser.h>
#include <Poco/Net/Context.h>
#include <Poco/Net/HTTPMessage.h>
#include <Poco/Net/HTTPRequest.h>
#include <Poco/Net/HTTPResponse.h>
#include <Poco/Net/HTTPSClientSession.h>
#include <Poco/Net/HTTPSStreamFactory.h>
#include <Poco/Net/NetSSL.h>
#include <Poco/Net/RejectCertificateHandler.h>
#include <Poco/Net/SSLManager.h>
#include <Poco/SharedPtr.h>
#include <Poco/StreamCopier.h>
#include <Poco/URI.h>
#include <Poco/URIStreamOpener.h>

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

SslContext::~SslContext() { Poco::Net::uninitializeSSL(); }

/// Gets the properties of an OSM object from overpass.
std::string queryTurbo(const std::string& query)
{
    Poco::URI uri("https://overpass-api.de/api/interpreter");
    Poco::Net::HTTPSClientSession session(uri.getHost(), uri.getPort());
    Poco::Net::HTTPRequest request(Poco::Net::HTTPRequest::HTTP_POST,
                                   uri.getPath(),
                                   Poco::Net::HTTPMessage::HTTP_1_1);
    request.setContentLength(query.length());
    std::ostream& requestStream = session.sendRequest(request);
    requestStream << query;
    Poco::Net::HTTPResponse response;
    std::istream& responseStream = session.receiveResponse(response);

    std::stringstream stringStream;
    Poco::StreamCopier::copyStream(responseStream, stringStream);

    return stringStream.str();
}

/// Gets the OSM object from nominatim.
std::string queryNominatim(const std::string& query)
{
    Poco::URI uri("https://nominatim.openstreetmap.org/search.php");
    uri.addQueryParameter("q", query);
    uri.addQueryParameter("format", "json");

    std::unique_ptr<std::istream> responseStream(
        Poco::URIStreamOpener::defaultOpener().open(uri));

    std::stringstream stringStream;
    Poco::StreamCopier::copyStream(*responseStream, stringStream);

    return stringStream.str();
}

/// Turns an address into a coodinate + normalized address combo.
void osmify(const std::string& query)
{
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
        std::cerr << "Failed to parse JSON from nominatim: "
                  << exception.message() << std::endl;
        return;
    }
    auto elementsArray = elements.extract<Poco::JSON::Array::Ptr>();
    if (elementsArray->size() == 0)
    {
        std::cerr << "No results from nominatim" << std::endl;
        return;
    }

    if (elementsArray->size() > 1)
    {
        // There are multiple elements, prefer buildings if possible.
        // TODO
    }

    Poco::Dynamic::Var element = elementsArray->get(0);
    auto elementObject = element.extract<Poco::JSON::Object::Ptr>();
    std::string lat = elementObject->get("lat").toString();
    std::string lon = elementObject->get("lon").toString();
    std::string objectType = elementObject->get("osm_type").toString();
    std::string objectId = elementObject->get("osm_id").toString();

    // Use overpass to get the properties of the object.
    auto overpassQuery = "[out:json];\n"
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
        std::cerr << "Failed to parse JSON from overpass: "
                  << exception.message() << std::endl;
        return;
    }
    auto jObject = j.extract<Poco::JSON::Object::Ptr>();
    elements = jObject->get("elements");
    elementsArray = elements.extract<Poco::JSON::Array::Ptr>();
    if (elementsArray->size() == 0)
    {
        std::cerr << "No results from overpass" << std::endl;
        return;
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
    std::cerr << "geo:" << lat << "," << lon << " (" << addr << ")"
              << std::endl;
}

int main(int argc, char** argv)
{
    if (argc > 1)
    {
        try
        {
            // TODO spinner
            osmify(argv[1]);
        }
        catch (const Poco::Exception& exception)
        {
            std::cerr << "Failed to osmify: " << exception.message()
                      << std::endl;
        }
    }
    else
    {
        std::cerr << "usage: addr-osmify <query>" << std::endl;
        std::cerr << std::endl;
        std::cerr << "e.g. addr-osmify 'Mészáros utca 58/a, Budapest'"
                  << std::endl;
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
