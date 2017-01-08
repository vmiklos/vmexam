/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#include "lib.hxx"

int main(int /*argc*/, char** /*argv*/)
{
    if (tpconv::convert(1, tpconv::ConversionUnit::Inch,
                        tpconv::ConversionUnit::Point) != 72)
        return 1;

    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
