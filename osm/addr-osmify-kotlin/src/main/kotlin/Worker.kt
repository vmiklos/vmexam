/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */
package hu.vmiklos.addr_osmify

/**
 * Invokes osmify() on a thread.
 */
class Worker(private val context: Context) : Runnable {
    override fun run() {
        try {
            context.out = App.osmify(context.input)
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
