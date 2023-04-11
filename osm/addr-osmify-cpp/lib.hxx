#pragma once
/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#include <ostream>
#include <vector>

namespace osmify
{
/// CLI wrapper around the C++ API.
int main(const std::vector<const char*>& args, std::ostream& ostream);
} // namespace osmify

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
