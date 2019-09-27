/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

package hu.vmiklos.addr_osmify;

/**
 * Sets and restores App.urlopener for testing.
 */
public class MockUrlopen
{
    Urlopener old;
    MockUrlopen(String suffix)
    {
        old = App.urlopener;
        App.urlopener = new MockUrlopener(suffix);
    }

    void destruct() { App.urlopener = old; }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
