/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

package hu.vmiklos.addr_osmify

import hu.vmiklos.addr_osmify.App.Companion.urlopener

/**
 * Sets and restores App.urlopener for testing.
 */
class MockUrlopen internal constructor(suffix: String) {
    var old: Urlopener
    fun destruct() {
        urlopener = old
    }

    init {
        old = urlopener
        urlopener = MockUrlopener(suffix)
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
