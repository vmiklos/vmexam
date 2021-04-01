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
std::string urlopenSuffix;

std::string mockUrlopen(const std::string& url, const std::string& data)
{
    if (!data.empty())
    {
        std::string path;
        Poco::URI::encode(url, "/", path);
        path = "mock/" + path + urlopenSuffix + ".expected-data";
        std::ifstream stream(path);
        assert(stream.is_open());
        std::string content((std::istreambuf_iterator<char>(stream)),
                            std::istreambuf_iterator<char>());
        assert(data == content);
    }

    std::string path;
    Poco::URI::encode(url, "/", path);
    path = "mock/" + path + urlopenSuffix;
    std::ifstream stream(path);
    assert(stream.is_open());

    std::string content((std::istreambuf_iterator<char>(stream)),
                        std::istreambuf_iterator<char>());

    // Make sure that the 100ms progressbar spins at least once.
    const int doubleSpin = 200;
    std::this_thread::sleep_for(std::chrono::milliseconds(doubleSpin));

    return content;
}

class MockUrlopen
{
    urllib::request::urlopenType _old = nullptr;
    std::string _oldSuffix;

  public:
    MockUrlopen(urllib::request::urlopenType custom, const std::string& suffix)
    {
        _old = urllib::request::urlopen;
        urllib::request::urlopen = custom;
        _oldSuffix = urlopenSuffix;
        urlopenSuffix = suffix;
    }

    ~MockUrlopen()
    {
        urlopenSuffix = _oldSuffix;
        urllib::request::urlopen = _old;
    }
};

} // namespace

TEST(TestMain, testHappy)
{
    MockUrlopen mu(mockUrlopen, "-happy");
    std::vector<const char*> args{"", "Mészáros utca 58/a, Budapest"};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected =
        "geo:47.490592,19.030662 (1016 Budapest, Mészáros utca 58/a)\n";
    ASSERT_EQ(expected, out.str());
}

TEST(TestMain, testPreferBuildings)
{
    MockUrlopen mu(mockUrlopen, "-prefer-buildings");
    std::vector<const char*> args{"", "Karinthy Frigyes út 18, Budapest"};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected = "geo:47.47690895,19.0512550758533 (1111 Budapest, "
                           "Karinthy Frigyes út 18)\n";
    ASSERT_EQ(expected, out.str());
}

TEST(TestMain, testNoBuildings)
{
    MockUrlopen mu(mockUrlopen, "-no-buildings");
    std::vector<const char*> args{"", "Karinthy Frigyes út 18, Budapest"};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected = "geo:47.47690895,19.0512550758533 (1111 Budapest, "
                           "Karinthy Frigyes út 18)\n";
    ASSERT_EQ(expected, out.str());
}

TEST(TestMain, testNoResult)
{
    MockUrlopen mu(mockUrlopen, "-no-result");
    std::vector<const char*> args{"", "Mészáros utca 58/a, Budapestt"};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected = "No results from nominatim\n";
    ASSERT_EQ(expected, out.str());
}

TEST(TestMain, testOverpassNoResult)
{
    MockUrlopen mu(mockUrlopen, "-overpass-noresult");
    std::vector<const char*> args{"", "Mészáros utca 58/a, Budapest"};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected = "No results from overpass\n";
    ASSERT_EQ(expected, out.str());
}

TEST(TestMain, testNominatimBadJson)
{
    MockUrlopen mu(mockUrlopen, "-nominatim-badjson");
    std::vector<const char*> args{"", "Mészáros utca 58/a, Budapest"};
    std::stringstream out;
    ASSERT_EQ(0, osmify::main(args, out));
    std::string expected =
        "Failed to parse JSON from nominatim: JSON parser error.\n";
    ASSERT_EQ(expected, out.str());
}

TEST(TestMain, testOverpassBadJson)
{
    MockUrlopen mu(mockUrlopen, "-overpass-badjson");
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
