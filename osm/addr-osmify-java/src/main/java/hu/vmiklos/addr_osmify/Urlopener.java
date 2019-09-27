/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

package hu.vmiklos.addr_osmify;

/**
 * urlopen() interface, to be implemented using HTTP or mocking.
 */
public interface Urlopener {
    String urlopen(String urlString, String data) throws Exception;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
