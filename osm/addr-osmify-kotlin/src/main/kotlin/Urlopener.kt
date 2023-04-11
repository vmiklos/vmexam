/*
 * Copyright 2020 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */
package hu.vmiklos.addr_osmify

/**
 * urlopen() interface, to be implemented using HTTP or mocking.
 */
interface Urlopener {
    fun urlopen(urlString: String, data: String): String
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
