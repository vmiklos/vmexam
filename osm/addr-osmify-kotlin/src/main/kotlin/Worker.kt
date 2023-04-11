/*
 * Copyright 2020 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
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
