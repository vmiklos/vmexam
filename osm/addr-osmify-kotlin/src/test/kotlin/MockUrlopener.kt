/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

package hu.vmiklos.addr_osmify

import java.net.URLEncoder
import java.nio.charset.Charset
import java.nio.charset.StandardCharsets
import java.nio.file.Files
import java.nio.file.Paths

/**
 * Test urlopen(), using mocking.
 */
class MockUrlopener internal constructor(var urlopenSuffix: String) : Urlopener {
    override fun urlopen(urlString: String, data: String): String {
        if (!data.isEmpty()) {
            var path = URLEncoder.encode(urlString, "UTF-8")
            path = "mock/$path$urlopenSuffix.expected-data"
            val content = readFile(path, StandardCharsets.UTF_8)
            require(data == content) {
                "data vs content mismatch: data is '" + data +
                    "', content is '" + content + "'"
            }
        }
        var path = URLEncoder.encode(urlString, "UTF-8")
        path = "mock/$path$urlopenSuffix"
        return readFile(path, StandardCharsets.UTF_8)
    }

    companion object {
        fun readFile(path: String, encoding: Charset): String {
            val encoded = Files.readAllBytes(Paths.get(path))
            return String(encoded, encoding)
        }
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
