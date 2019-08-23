/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

package hu.vmiklos.addr_osmify;

import com.google.gson.annotations.SerializedName;

/**
 * NominatimResult represents one element in the result array from
 * Nominatim.
 */
class NominatimResult
{
    @SerializedName("class") public String clazz;
    public String lat;
    public String lon;
    @SerializedName("osm_type") public String osmType;
    @SerializedName("osm_id") public String osmId;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
