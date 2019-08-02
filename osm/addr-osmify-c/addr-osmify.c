/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#define _GNU_SOURCE
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <curl/curl.h>
#include <json_object.h>
#include <json_tokener.h>

struct MemoryStruct
{
    char* data;
    size_t size;
};

static size_t writeMemoryCallback(void* contents, size_t size, size_t nmemb,
                                  void* user)
{
    size_t realsize = size * nmemb;
    struct MemoryStruct* mem = (struct MemoryStruct*)user;

    char* ptr = realloc(mem->data, mem->size + realsize + 1);
    if (!ptr)
    {
        fprintf(stderr, "writeMemoryCallback: out of memory");
        return 0;
    }

    mem->data = ptr;
    memcpy(&(mem->data[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->data[mem->size] = 0;

    return realsize;
}

/// Gets the properties of an OSM object from overpass.
char* queryTurbo(const char* query, char** err)
{
    char* ret = NULL;
    CURL* curl = NULL;
    struct MemoryStruct mem;
    mem.data = malloc(1);
    mem.size = 0;

    curl = curl_easy_init();
    if (!curl)
    {
        goto cleanup;
    }

    const char* url = "http://overpass-api.de/api/interpreter";

    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, query);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, -1L);
    curl_easy_setopt(curl, CURLOPT_USERAGENT, "addr-osmify/1.0");
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, writeMemoryCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &mem);
    int curlRes = curl_easy_perform(curl);
    if (curlRes != CURLE_OK)
    {
        char* str = NULL;
        asprintf(&str, "overpass failed: %s", curl_easy_strerror(curlRes));
        if (err)
        {
            *err = str;
        }
        goto cleanup;
    }

    ret = mem.data;
    mem.data = NULL;

cleanup:
    if (mem.data)
    {
        free(mem.data);
        mem.data = NULL;
    }

    if (curl)
    {
        curl_easy_cleanup(curl);
        curl = NULL;
    }

    return ret;
}

/// Gets the OSM object from nominatim.
char* queryNominatim(const char* query, char** err)
{
    char* ret = NULL;
    CURL* curl = NULL;
    char* url = NULL;
    struct MemoryStruct mem;
    mem.data = malloc(1);
    mem.size = 0;
    char* escapedQuery = NULL;

    const char* prefix = "https://nominatim.openstreetmap.org/search.php";
    curl = curl_easy_init();
    if (!curl)
    {
        goto cleanup;
    }

    escapedQuery = curl_easy_escape(curl, query, strlen(query));
    if (asprintf(&url, "%s?q=%s&format=json", prefix, escapedQuery) < 0)
    {
        goto cleanup;
    }

    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_USERAGENT, "addr-osmify/1.0");
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, writeMemoryCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &mem);
    int curlRes = curl_easy_perform(curl);
    if (curlRes != CURLE_OK)
    {
        char* str = NULL;
        asprintf(&str, "nominatim failed: %s", curl_easy_strerror(curlRes));
        if (err)
        {
            *err = str;
        }
        goto cleanup;
    }

    ret = mem.data;
    mem.data = NULL;

cleanup:
    if (escapedQuery)
    {
        curl_free(escapedQuery);
    }

    if (mem.data)
    {
        free(mem.data);
        mem.data = NULL;
    }

    if (url)
    {
        free(url);
        url = NULL;
    }

    if (curl)
    {
        curl_easy_cleanup(curl);
        curl = NULL;
    }

    return ret;
}

/// Turns an address into a coodinate + normalized address combo.
int osmify(const char* query, char** err)
{
    int ret = 0;
    char* nominatimRet = NULL;
    char* overpassQuery = NULL;
    char* overpassRet = NULL;
    char* addr = NULL;
    char* result = NULL;
    json_object* nominatimJson = NULL;
    json_object* overpassJson = NULL;

    curl_global_init(CURL_GLOBAL_ALL);

    // Use nominatim to get the coordinates and the osm type/id.
    char* nominatimErr = NULL;
    nominatimRet = queryNominatim(query, &nominatimErr);
    if (!nominatimRet)
    {
        char* str = NULL;
        asprintf(&str, "failed to query nominatim: %s", nominatimErr);
        free(nominatimErr);
        if (err)
        {
            *err = str;
        }
        ret = -1;
        goto cleanup;
    }

    nominatimJson = json_tokener_parse(nominatimRet);
    if (json_object_get_type(nominatimJson) != json_type_array)
    {
        if (err)
        {
            *err = strdup("result from nominatim is not a json array");
        }
        ret = -1;
        goto cleanup;
    }

    int nominatimLen = json_object_array_length(nominatimJson);
    if (nominatimLen == 0)
    {
        if (err)
        {
            *err = strdup("no results from nominatim");
        }
        ret = -1;
        goto cleanup;
    }

    if (nominatimLen > 1)
    {
        // There are multiple elements, prefer buildings if possible.
        // Example where this is useful: 'Karinthy Frigyes út 18, Budapest'.
        // TODO
    }

    json_object* element = json_object_array_get_idx(nominatimJson, 0);
    if (json_object_get_type(element) != json_type_object)
    {
        ret = -1;
        goto cleanup;
    }

    const char* lat =
        json_object_get_string(json_object_object_get(element, "lat"));
    const char* lon =
        json_object_get_string(json_object_object_get(element, "lon"));
    const char* objectType =
        json_object_get_string(json_object_object_get(element, "osm_type"));
    int64_t objectId =
        json_object_get_int64(json_object_object_get(element, "osm_id"));

    // Use overpass to get the properties of the object.
    if (asprintf(&overpassQuery,
                 "[out:json];\n"
                 "(\n"
                 "%s(%ld);\n"
                 ");\n"
                 "out body;",
                 objectType, objectId) < 0)
    {
        ret = -1;
        goto cleanup;
    }

    char* overpassErr = NULL;
    overpassRet = queryTurbo(overpassQuery, &overpassErr);
    if (!overpassRet)
    {
        char* str = NULL;
        asprintf(&str, "failed to query overpass: %s", overpassErr);
        free(overpassErr);
        if (err)
        {
            *err = str;
        }
        ret = -1;
        goto cleanup;
    }
    overpassJson = json_tokener_parse(overpassRet);
    if (json_object_get_type(overpassJson) != json_type_object)
    {
        if (err)
        {
            *err = strdup("result from overpass is not a json object");
        }
        ret = -1;
        goto cleanup;
    }

    json_object* elements = json_object_object_get(overpassJson, "elements");
    if (json_object_get_type(elements) != json_type_array)
    {
        ret = -1;
        goto cleanup;
    }

    int overpassLen = json_object_array_length(elements);
    if (overpassLen == 0)
    {
        if (err)
        {
            *err = strdup("no results from overpass");
        }
        ret = -1;
        goto cleanup;
    }

    json_object* overpassElement = json_object_array_get_idx(elements, 0);
    if (json_object_get_type(overpassElement) != json_type_object)
    {
        ret = -1;
        goto cleanup;
    }

    json_object* tags = json_object_object_get(overpassElement, "tags");
    if (json_object_get_type(tags) != json_type_object)
    {
        ret = -1;
        goto cleanup;
    }

    const char* city =
        json_object_get_string(json_object_object_get(tags, "addr:city"));
    const char* housenumber = json_object_get_string(
        json_object_object_get(tags, "addr:housenumber"));
    const char* postcode =
        json_object_get_string(json_object_object_get(tags, "addr:postcode"));
    const char* street =
        json_object_get_string(json_object_object_get(tags, "addr:street"));

    if (asprintf(&addr, "%s %s, %s %s", postcode, city, street, housenumber) <
        0)
    {
        ret = -1;
        goto cleanup;
    }

    printf("geo:%s,%s (%s)\n", lat, lon, addr);

cleanup:
    if (result)
    {
        free(result);
        result = NULL;
    }

    if (addr)
    {
        free(addr);
        addr = NULL;
    }

    if (overpassJson)
    {
        json_object_put(overpassJson);
        overpassJson = NULL;
    }
    if (overpassRet)
    {
        free(overpassRet);
        overpassRet = NULL;
    }

    if (overpassQuery)
    {
        free(overpassQuery);
        overpassQuery = NULL;
    }

    if (nominatimJson)
    {
        json_object_put(nominatimJson);
        nominatimJson = NULL;
    }

    if (nominatimRet)
    {
        free(nominatimRet);
        nominatimRet = NULL;
    }

    return ret;
}

int main(int argc, char** argv)
{
    if (argc > 1)
    {
        char* err = NULL;
        // TODO show a spinner while this is running.
        int ret = osmify(argv[1], &err);
        if (ret < 0)
        {
            fprintf(stderr, "failed to osmify: %s\n", err);
            free(err);
            return 1;
        }
    }
    else
    {
        fprintf(stderr, "usage: addr-osmify <query>\n\n");
        fprintf(stderr, "e.g. addr-osmify 'Mészáros utca 58/a, Budapest'\n");
    }
    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
