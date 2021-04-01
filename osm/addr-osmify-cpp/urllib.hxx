#pragma once
/*
 * Copyright 2021 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#include <string>

namespace urllib::request
{
/// Function type for urlopen() customization.
using urlopenType = std::string (*)(const std::string& url,
                                    const std::string& data);
extern urlopenType urlopen;
} // namespace urllib::request

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
