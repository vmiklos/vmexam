/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
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
