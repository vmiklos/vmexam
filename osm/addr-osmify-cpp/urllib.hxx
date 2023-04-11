#pragma once
/*
 * Copyright 2021 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#include <functional>
#include <string>

namespace urllib::request
{
/// Function type for urlopen() customization.
using urlopenType =
    std::function<std::string(const std::string& url, const std::string& data)>;
extern urlopenType urlopen;
} // namespace urllib::request

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
