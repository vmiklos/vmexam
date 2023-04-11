/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

package hu.vmiklos.addr_osmify;

/**
 * urlopen() interface, to be implemented using HTTP or mocking.
 */
public interface Urlopener
{
    String urlopen(String urlString, String data) throws Exception;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
