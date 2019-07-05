/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#include <initializer_list>
#include <string>

namespace tpconv
{

/// List of unit types we handle.
enum class ConversionUnit
{
    Inch,
    Point,
    Twip,

    M,
    Cm,
    Mm,
    Mm100,

    Emu
};

/// List of string representation of ConversionUnit elements.
std::initializer_list<std::string> getUnitNames();

/// Do the actual conversion between units.
double convert(double amount, ConversionUnit from, ConversionUnit to);

} // namespace tpconv

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
