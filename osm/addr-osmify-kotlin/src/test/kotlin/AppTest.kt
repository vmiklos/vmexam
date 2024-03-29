/*
 * Copyright 2020 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

package hu.vmiklos.addr_osmify

import org.junit.Test
import java.io.ByteArrayOutputStream
import java.nio.charset.StandardCharsets
import kotlin.test.assertEquals

class AppTest {
    @Test
    fun testHappy() {
        val mu = MockUrlopen("-happy")
        try {
            val args = arrayOf("Mészáros utca 58/a, Budapest")
            val baos = ByteArrayOutputStream()
            App(args, baos)
            val out = String(baos.toByteArray(), StandardCharsets.UTF_8)
            val expected = "47.490592,19.030662 (1016 Budapest, Mészáros utca 58/a)\n"
            assertEquals(expected, out)
        } catch (e: Exception) {
            throw e
        } finally {
            mu.destruct()
        }
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
