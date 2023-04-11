/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#include <cppunit/extensions/HelperMacros.h>

#include "lib.hxx"

class TpconvTest : public CPPUNIT_NS::TestFixture
{
    CPPUNIT_TEST_SUITE(TpconvTest);
    CPPUNIT_TEST(testConvert);
    CPPUNIT_TEST_SUITE_END();

    void testConvert();
};

void TpconvTest::testConvert()
{
    CPPUNIT_ASSERT_DOUBLES_EQUAL(72,
                                 tpconv::convert(1,
                                                 tpconv::ConversionUnit::Inch,
                                                 tpconv::ConversionUnit::Point),
                                 10e-4);
}

CPPUNIT_TEST_SUITE_REGISTRATION(TpconvTest);

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
