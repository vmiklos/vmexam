/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */
package hu.vmiklos.addr_osmify

import java.io.OutputStream
import java.io.StringReader
import java.io.StringWriter
import java.net.HttpURLConnection
import java.net.URL
import org.apache.commons.io.IOUtils

/**
 * Default urlopen(), using HttpURLConnection.
 */
class DefaultUrlopener : Urlopener {
    override fun urlopen(urlString: String, data: String): String {
        val url = URL(urlString)
        val connection = url.openConnection() as HttpURLConnection
        if (data.isEmpty()) {
            val writer = StringWriter()
            IOUtils.copy(connection.inputStream, writer)
            return writer.toString()
        }
        val reader = StringReader(data)
        var outputStream: OutputStream? = null
        return try {
            connection.requestMethod = "POST"
            connection.doOutput = true
            outputStream = connection.outputStream
            IOUtils.copy(reader, outputStream)
            val writer = StringWriter()
            IOUtils.copy(connection.inputStream, writer)
            writer.toString()
        } catch (e: Exception) {
            throw e
        } finally {
            reader.close()
            outputStream?.close()
        }
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
