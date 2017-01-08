/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
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
