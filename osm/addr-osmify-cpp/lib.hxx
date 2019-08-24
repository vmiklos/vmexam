#pragma once
/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#include <ostream>
#include <string>
#include <vector>

namespace osmify
{
/// Function type for urlopen() customization.
using urlopenType = std::string (*)(const std::string& url,
                                    const std::string& data);
/// Returns the current urlopen().
urlopenType getUrlopen();
/// Sets the current urlopen().
void setUrlopen(urlopenType custom);

/// CLI wrapper around the C++ API.
int main(const std::vector<const char*>& args, std::ostream& ostream);
} // namespace osmify

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
