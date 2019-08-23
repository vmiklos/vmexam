/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

package hu.vmiklos.addr_osmify;

import com.google.gson.annotations.SerializedName;

/**
 * TurboTags contains various tags about one Overpass element.
 */
class TurboTags
{
    @SerializedName("addr:city") public String city;
    @SerializedName("addr:housenumber") public String houseNumber;
    @SerializedName("addr:postcode") public String postCode;
    @SerializedName("addr:street") public String street;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
