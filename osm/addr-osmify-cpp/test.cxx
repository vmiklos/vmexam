/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#include <fstream>

#include <Poco/URI.h>
#include <gtest/gtest-message.h>
#include <gtest/gtest-test-part.h>
#include <gtest/gtest.h>

#include "lib.hxx"

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

    return content;
}

class MockUrlopen
{
    osmify::urlopenType _old = nullptr;
    std::string _oldSuffix;

  public:
    MockUrlopen(osmify::urlopenType custom, const std::string& suffix)
    {
        _old = osmify::getUrlopen();
        osmify::setUrlopen(custom);
        _oldSuffix = urlopenSuffix;
        urlopenSuffix = suffix;
    }

    ~MockUrlopen()
    {
        urlopenSuffix = _oldSuffix;
        osmify::setUrlopen(_old);
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

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
