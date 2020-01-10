/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */
package hu.vmiklos.addr_osmify

import com.google.gson.annotations.SerializedName

/**
 * NominatimResult represents one element in the result array from
 * Nominatim.
 */
class NominatimResult {
    @SerializedName("class")
    var clazz: String = String()
    var lat: String = String()
    var lon: String = String()
    @SerializedName("osm_type")
    var osmType: String = String()
    @SerializedName("osm_id")
    var osmId: String = String()
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
