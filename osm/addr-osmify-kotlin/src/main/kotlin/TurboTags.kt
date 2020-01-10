/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */
package hu.vmiklos.addr_osmify

import com.google.gson.annotations.SerializedName

/**
 * TurboTags contains various tags about one Overpass element.
 */
class TurboTags {
    @SerializedName("addr:city")
    var city: String = String()
    @SerializedName("addr:housenumber")
    var houseNumber: String = String()
    @SerializedName("addr:postcode")
    var postCode: String = String()
    @SerializedName("addr:street")
    var street: String = String()
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
