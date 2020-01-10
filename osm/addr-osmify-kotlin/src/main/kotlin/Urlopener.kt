/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */
package hu.vmiklos.addr_osmify

/**
 * urlopen() interface, to be implemented using HTTP or mocking.
 */
interface Urlopener {
    fun urlopen(urlString: String, data: String): String
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
