/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#include "lib.hxx"

#include <map>

namespace tpconv
{

std::initializer_list<std::string> getUnitNames()
{
    static std::initializer_list<std::string> units = {"inch",  "point", "twip",

                                                       "m",     "cm",    "mm",
                                                       "mm100",

                                                       "emu"};
    return units;
}

double convert(double amount, ConversionUnit from, ConversionUnit to)
{
    static std::map<ConversionUnit, double> units;
    if (units.empty())
    {
        units[ConversionUnit::Inch] = 914400.0;
        units[ConversionUnit::Point] = 914400.0 / 72;
        units[ConversionUnit::Twip] = 914400.0 / 72 / 20;

        units[ConversionUnit::M] = 360 * 100000;
        units[ConversionUnit::Cm] = 360 * 1000;
        units[ConversionUnit::Mm] = 360 * 100;
        units[ConversionUnit::Mm100] = 360;

        units[ConversionUnit::Emu] = 1;
    }

    // Convert to EMU.
    double emu = amount * units[from];
    return emu / units[to];
}

} // namespace tpconv

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
