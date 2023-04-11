/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
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
