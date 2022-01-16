/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#include <cassert>
#include <chrono>
#include <fstream>
#include <iterator>
#include <sstream>
#include <string>
#include <thread>
#include <vector>

#include <Poco/URI.h>
#include <gtest/gtest-message.h>
#include <gtest/gtest-test-part.h>
#include <gtest/gtest.h>

#include "lib.hxx"
#include "urllib.hxx"

namespace
{

struct URLRoute
{
    /// The request URL
    std::string url;
    /// Path of expected POST data, empty for GET
    std::string dataPath;
    /// Path of expected result data
    std::string resultPath;
};

class MockUrlopenFunction
{
    std::vector<URLRoute> _routes;

  public:
    MockUrlopenFunction(const std::vector<URLRoute>& routes);

    std::string operator()(const std::string& url, const std::string& data);
};

MockUrlopenFunction::MockUrlopenFunction(const std::vector<URLRoute>& routes)
    : _routes(routes)
{
}

std::string MockUrlopenFunction::operator()(const std::string& url,
                                            const std::string& data)
{
    for (const auto& route : _routes)
    {
        if (url != route.url)
        {
            continue;
        }

        if (!route.dataPath.empty())
        {
            std::ifstream stream(route.dataPath);
            if (!stream.is_open())
            {
                std::cerr << "failed to open '" << route.dataPath << "'"
                          << std::endl;
            }
            assert(stream.is_open());
            std::string content((std::istreambuf_iterator<char>(stream)),
                                std::istreambuf_iterator<char>());
            assert(data == content);
        }

        std::ifstream stream(route.resultPath);
        if (!stream.is_open())
        {
            std::cerr << "failed to open '" << route.resultPath << "'"
                      << std::endl;
        }
        assert(stream.is_open());

        std::string content((std::istreambuf_iterator<char>(stream)),
                            std::istreambuf_iterator<char>());

        // Make sure that the 100ms progressbar spins at least once.
        const int doubleSpin = 200;
        std::this_thread::sleep_for(std::chrono::milliseconds(doubleSpin));

        return content;
    }

    std::cerr << "unexpected url='" << url << "' and data='" << data << "'"
              << std::endl;
    assert(false);
    return std::string();
}

class MockUrlopen
{
    urllib::request::urlopenType _old;

  public:
    MockUrlopen(const std::vector<URLRoute>& routes)
    {
        _old = urllib::request::urlopen;
        urllib::request::urlopen = MockUrlopenFunction(routes);
    }

    ~MockUrlopen() { urllib::request::urlopen = _old; }
};

} // namespace

TEST(TestMain, testHappy)
{
    std::vector<URLRoute> routes = {
        URLRoute{"https://nominatim.openstreetmap.org/"
                 "search.php?q=M%C3%A9sz%C3%A1ros%20utca%2058%2Fa%2C%"
                 "20Budapest&format=json",
                 "", "mock/nominatim-happy.json"},
        URLRoute{"https://overpass-api.de/api/interpreter",
                 "mock/overpass-happy.expected-data",
                 "mock/overpass-happy.json"},
    };
    MockUrlopen urlopen(routes);
    std::vector<const char*> args{"", "Mészáros utca 58/a, Budapest"};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected =
        "47.490592,19.030662 (1016 Budapest, Mészáros utca 58/a)\n";
    ASSERT_EQ(expected, out.str());
}

TEST(TestMain, testPreferBuildings)
{
    std::vector<URLRoute> routes = {
        URLRoute{"https://nominatim.openstreetmap.org/"
                 "search.php?q=Karinthy%20Frigyes%20%C3%BAt%2018%2C%20Budapest&"
                 "format=json",
                 "", "mock/nominatim-prefer-buildings.json"},
        URLRoute{"https://overpass-api.de/api/interpreter",
                 "mock/overpass-prefer-buildings.expected-data",
                 "mock/overpass-prefer-buildings.json"},
    };
    MockUrlopen urlopen(routes);
    std::vector<const char*> args{"", "Karinthy Frigyes út 18, Budapest"};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected = "47.47690895,19.0512550758533 (1111 Budapest, "
                           "Karinthy Frigyes út 18)\n";
    ASSERT_EQ(expected, out.str());
}

TEST(TestMain, testNoBuildings)
{
    std::vector<URLRoute> routes = {
        URLRoute{"https://nominatim.openstreetmap.org/"
                 "search.php?q=Karinthy%20Frigyes%20%C3%BAt%2018%2C%20Budapest&"
                 "format=json",
                 "", "mock/nominatim-no-buildings.json"},
        URLRoute{"https://overpass-api.de/api/interpreter",
                 "mock/overpass-no-buildings.expected-data",
                 "mock/overpass-no-buildings.json"},
    };
    MockUrlopen urlopen(routes);
    std::vector<const char*> args{"", "Karinthy Frigyes út 18, Budapest"};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected = "47.47690895,19.0512550758533 (1111 Budapest, "
                           "Karinthy Frigyes út 18)\n";
    ASSERT_EQ(expected, out.str());
}

TEST(TestMain, testNoResult)
{
    std::vector<URLRoute> routes = {
        URLRoute{"https://nominatim.openstreetmap.org/"
                 "search.php?q=M%C3%A9sz%C3%A1ros%20utca%2058%2Fa%2C%"
                 "20Budapestt&format=json",
                 "", "mock/nominatim-no-result.json"},
    };
    MockUrlopen urlopen(routes);
    std::vector<const char*> args{"", "Mészáros utca 58/a, Budapestt"};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected = "No results from nominatim\n";
    ASSERT_EQ(expected, out.str());
}

TEST(TestMain, testOverpassNoResult)
{
    std::vector<URLRoute> routes = {
        URLRoute{"https://nominatim.openstreetmap.org/"
                 "search.php?q=M%C3%A9sz%C3%A1ros%20utca%2058%2Fa%2C%"
                 "20Budapest&format=json",
                 "", "mock/nominatim-happy.json"},
        URLRoute{"https://overpass-api.de/api/interpreter",
                 "mock/overpass-no-result.expected-data",
                 "mock/overpass-no-result.json"},
    };
    MockUrlopen urlopen(routes);
    std::vector<const char*> args{"", "Mészáros utca 58/a, Budapest"};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected = "No results from overpass\n";
    ASSERT_EQ(expected, out.str());
}

TEST(TestMain, testNominatimBadJson)
{
    std::vector<URLRoute> routes = {
        URLRoute{"https://nominatim.openstreetmap.org/"
                 "search.php?q=M%C3%A9sz%C3%A1ros%20utca%2058%2Fa%2C%"
                 "20Budapest&format=json",
                 "", "mock/nominatim-bad.json"},
    };
    MockUrlopen urlopen(routes);
    std::vector<const char*> args{"", "Mészáros utca 58/a, Budapest"};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected =
        "Failed to parse JSON from nominatim: JSON parser error.\n";
    ASSERT_EQ(expected, out.str());
}

TEST(TestMain, testOverpassBadJson)
{
    std::vector<URLRoute> routes = {
        URLRoute{"https://nominatim.openstreetmap.org/"
                 "search.php?q=M%C3%A9sz%C3%A1ros%20utca%2058%2Fa%2C%"
                 "20Budapest&format=json",
                 "", "mock/nominatim-happy.json"},
        URLRoute{"https://overpass-api.de/api/interpreter",
                 "mock/overpass-happy.expected-data", "mock/overpass-bad.json"},
    };
    MockUrlopen urlopen(routes);
    std::vector<const char*> args{"", "Mészáros utca 58/a, Budapest"};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected =
        "Failed to parse JSON from overpass: JSON parser error.\n";
    ASSERT_EQ(expected, out.str());
}

TEST(TestMain, testNoArgs)
{
    std::vector<const char*> args{""};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected = "usage:";
    ASSERT_EQ(0, out.str().find(expected));
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
