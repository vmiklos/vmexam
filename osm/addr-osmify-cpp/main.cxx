/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#include <iostream>
#include <vector>

#include "lib.hxx"

int main(int argc, char** argv)
{
    std::vector<const char*> args(argv, argv + argc);
    return osmify::main(args, std::cerr);
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
