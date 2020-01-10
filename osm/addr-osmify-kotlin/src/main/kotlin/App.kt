/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */
package hu.vmiklos.addr_osmify

import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import java.io.OutputStream
import java.net.URLEncoder
import java.nio.charset.Charset
import java.util.ArrayList

class App(args: Array<String>, out: OutputStream) {
    /**
     * Shows a spinner while osmify() is in progress.
     */
    private fun spinner(context: Context, thread: Thread, out: OutputStream) {
        val spinCharacters = "\\|/-".toCharArray()
        var spinIndex = 0
        while (true) {
            thread.join(100, 0)
            if (!thread.isAlive) {
                System.err.print("\r")
                System.err.flush()
                out.write(
                        (context.out + "\n").toByteArray(Charset.forName("UTF-8")))
                break
            }
            System.err.print("\r [" + spinCharacters[spinIndex] + "] ")
            System.err.flush()
            spinIndex = (spinIndex + 1) % spinCharacters.size
        }
    }

    companion object {
        var urlopener: Urlopener = DefaultUrlopener()
        /**
         * Send query to overpass turbo.
         */
        private fun queryTurbo(query: String): String {
            return urlopener.urlopen("http://overpass-api.de/api/interpreter",
                    query)
        }

        /**
         * Send query to nominatim.
         */
        private fun queryNominatim(query: String): String {
            var urlString = "https://nominatim.openstreetmap.org/search.php"
            urlString += "?q=" + URLEncoder.encode(query, "UTF-8")
            urlString += "&format=json"
            return urlopener.urlopen(urlString, "")
        }

        /**
         * Turn query into a coordinate + address string.
         */
        fun osmify(query: String): String {
            // Use nominatim to get the coordinates and the osm type/id.
            val nominatim = queryNominatim(query)
            val gson = Gson()
            val collectionType = object : TypeToken<Collection<NominatimResult>>() {}.type
            var elements = gson.fromJson<Collection<NominatimResult>>(nominatim, collectionType)
            if (elements.isEmpty()) {
                return "No results from nominatim"
            }
            if (elements.size > 1) {
                // There are multiple elements, prefer buildings if possible.
                // Example where this is useful: 'Karinthy Frigyes út 18, Budapest'.
                val buildings = ArrayList<NominatimResult>()
                for (element in elements) {
                    if ("building" == element.clazz) {
                        buildings.add(element)
                    }
                }
                if (!buildings.isEmpty()) {
                    elements = buildings
                }
            }
            val element = elements.iterator().next()
            val lat = element.lat
            val lon = element.lon
            val objectType = element.osmType
            val objectId = element.osmId
            // Use overpass to get the properties of the object.
            var overpassQuery = "[out:json];\n"
            overpassQuery += "("
            overpassQuery += "$objectType($objectId);"
            overpassQuery += ");"
            overpassQuery += "out body;"
            val turbo = queryTurbo(overpassQuery)
            val turboResult = gson.fromJson(turbo, TurboResult::class.java)
            val turboElements = turboResult.elements
            if (turboElements.isEmpty()) {
                return "No results from overpass"
            }
            val turboElement = turboElements[0]
            val city = turboElement.tags.city
            val houseNumber = turboElement.tags.houseNumber
            val postCode = turboElement.tags.postCode
            val street = turboElement.tags.street
            val addr = "$postCode $city, $street $houseNumber"
            // Print the result.
            return "geo:$lat,$lon ($addr)"
        }
    }

    init {
        if (args.size > 0) {
            val context = Context()
            context.input = args[0]
            val thread = Thread(Worker(context))
            thread.start()
            spinner(context, thread, out)
        } else {
            println("usage: addr-osmify <query>")
            println()
            println("e.g. addr-osmify 'Mészáros utca 58/a, Budapest'")
        }
    }
}

fun main(args: Array<String>) {
    try {
        App(args, System.out)
    } catch (e: Exception) {
        e.printStackTrace()
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
