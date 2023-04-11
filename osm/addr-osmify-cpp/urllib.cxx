/*
 * Copyright 2021 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#include "urllib.hxx"

#include <memory>
#include <sstream>
#include <string>

#include <Poco/Net/HTTPMessage.h>
#include <Poco/Net/HTTPRequest.h>
#include <Poco/Net/HTTPResponse.h>
#include <Poco/Net/HTTPSClientSession.h>
#include <Poco/StreamCopier.h>
#include <Poco/URI.h>
#include <Poco/URIStreamOpener.h>

namespace urllib::request
{

/// Default urlopen(), using Poco::Net.
std::string pocoUrlopen(const std::string& url, const std::string& data)
{
    Poco::URI uri(url);
    if (data.empty())
    {
        std::unique_ptr<std::istream> responseStream(
            Poco::URIStreamOpener::defaultOpener().open(uri));

        std::stringstream stringStream;
        Poco::StreamCopier::copyStream(*responseStream, stringStream);

        return stringStream.str();
    }

    Poco::Net::HTTPSClientSession session(uri.getHost(), uri.getPort());
    Poco::Net::HTTPRequest request(Poco::Net::HTTPRequest::HTTP_POST,
                                   uri.getPath(),
                                   Poco::Net::HTTPMessage::HTTP_1_1);
    request.setContentLength(static_cast<std::streamsize>(data.length()));
    std::ostream& requestStream = session.sendRequest(request);
    requestStream << data;
    Poco::Net::HTTPResponse response;
    std::istream& responseStream = session.receiveResponse(response);

    std::stringstream stringStream;
    Poco::StreamCopier::copyStream(responseStream, stringStream);

    return stringStream.str();
}

urlopenType urlopen = pocoUrlopen;

} // namespace urllib::request

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
