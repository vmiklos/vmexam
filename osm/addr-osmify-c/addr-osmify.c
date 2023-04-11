/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#define _GNU_SOURCE
#include <assert.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/time.h>
#include <time.h>

#include <curl/curl.h>
#include <json_object.h>
#include <json_tokener.h>

/// Used by curl to dynamically allocate download result.
struct MemoryStruct
{
    char* data;
    size_t size;
};

/// Used for communication between osmify() and spinner().
struct SpinnerContext
{
    char* query;
    pthread_mutex_t mutex;
    pthread_cond_t conditionVariable;
    char* result;
    char* error;
    bool processed;
};

/// SpinnerContext constructor.
void spinnerContextInit(struct SpinnerContext* context)
{
    context->query = NULL;
    pthread_mutex_init(&context->mutex, NULL);
    pthread_cond_init(&context->conditionVariable, NULL);
    context->result = NULL;
    context->error = NULL;
    context->processed = false;
}

/// SpinnerContext destructor.
void spinnerContextDestroy(struct SpinnerContext* context)
{
    if (context->error)
    {
        free(context->error);
        context->error = NULL;
    }

    if (context->result)
    {
        free(context->result);
        context->result = NULL;
    }

    pthread_cond_destroy(&context->conditionVariable);
    pthread_mutex_destroy(&context->mutex);

    if (context->query)
    {
        free(context->query);
        context->query = NULL;
    }
}

/// curl's data write callback.
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

    escapedQuery = curl_easy_escape(curl, query, (int)strlen(query));
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
void osmify(struct SpinnerContext* spinnerContext)
{
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
    nominatimRet = queryNominatim(spinnerContext->query, &nominatimErr);
    if (!nominatimRet)
    {
        char* str = NULL;
        asprintf(&str, "failed to query nominatim: %s", nominatimErr);
        free(nominatimErr);
        spinnerContext->error = str;
        goto cleanup;
    }

    nominatimJson = json_tokener_parse(nominatimRet);
    if (json_object_get_type(nominatimJson) != json_type_array)
    {
        spinnerContext->error =
            strdup("result from nominatim is not a json array");
        goto cleanup;
    }

    int nominatimLen = json_object_array_length(nominatimJson);
    if (nominatimLen == 0)
    {
        spinnerContext->error = strdup("no results from nominatim");
        goto cleanup;
    }

    if (nominatimLen > 1)
    {
        // There are multiple elements, prefer buildings if possible.
        // Example where this is useful: 'Karinthy Frigyes út 18, Budapest'.
        json_object* buildings = json_object_new_array();
        for (int i = 0; i < nominatimLen; i++)
        {
            json_object* building = json_object_array_get_idx(nominatimJson, i);
            if (json_object_get_type(building) != json_type_object)
            {
                continue;
            }

            const char* class = json_object_get_string(
                json_object_object_get(building, "class"));
            if (!class)
            {
                continue;
            }

            if (strcmp(class, "building") != 0)
            {
                continue;
            }

            json_object_get(building);
            json_object_array_add(buildings, building);
        }

        if (json_object_array_length(buildings) > 0)
        {
            json_object_put(nominatimJson);
            nominatimJson = buildings;
        }
        else
        {
            json_object_put(buildings);
        }
    }

    json_object* element = json_object_array_get_idx(nominatimJson, 0);
    if (json_object_get_type(element) != json_type_object)
    {
        spinnerContext->error = strdup("element type is not object");
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
        spinnerContext->error = strdup("failed to build overpass query");
        goto cleanup;
    }

    char* overpassErr = NULL;
    overpassRet = queryTurbo(overpassQuery, &overpassErr);
    if (!overpassRet)
    {
        char* str = NULL;
        asprintf(&str, "failed to query overpass: %s", overpassErr);
        free(overpassErr);
        spinnerContext->error = str;
        goto cleanup;
    }
    overpassJson = json_tokener_parse(overpassRet);
    if (json_object_get_type(overpassJson) != json_type_object)
    {
        spinnerContext->error =
            strdup("result from overpass is not a json object");
        goto cleanup;
    }

    json_object* elements = json_object_object_get(overpassJson, "elements");
    if (json_object_get_type(elements) != json_type_array)
    {
        spinnerContext->error = strdup("elements type is not array");
        goto cleanup;
    }

    int overpassLen = json_object_array_length(elements);
    if (overpassLen == 0)
    {
        spinnerContext->error = strdup("no results from overpass");
        goto cleanup;
    }

    json_object* overpassElement = json_object_array_get_idx(elements, 0);
    if (json_object_get_type(overpassElement) != json_type_object)
    {
        spinnerContext->error =
            strdup("overpassElement type type is not object");
        goto cleanup;
    }

    json_object* tags = json_object_object_get(overpassElement, "tags");
    if (json_object_get_type(tags) != json_type_object)
    {
        spinnerContext->error = strdup("tags type type is not object");
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
        spinnerContext->error = strdup("failed to build the address string");
        goto cleanup;
    }

    if (asprintf(&spinnerContext->result, "%s,%s (%s)", lat, lon, addr) < 0)
    {
        spinnerContext->error = strdup("failed to build the result string");
        goto cleanup;
    }

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
}

/// Runs on a thread, invokes osmify().
void* worker(void* context)
{
    struct SpinnerContext* spinnerContext = (struct SpinnerContext*)context;
    osmify(spinnerContext);

    pthread_mutex_lock(&spinnerContext->mutex);
    spinnerContext->processed = true;
    pthread_mutex_unlock(&spinnerContext->mutex);
    pthread_cond_signal(&spinnerContext->conditionVariable);

    return NULL;
}

/// Moral equivalent of C++'s std::condition_variable::wait_for().
void wait_for(struct SpinnerContext* spinnerContext, int sleep)
{
    struct timespec abstime;
    clock_gettime(CLOCK_REALTIME, &abstime);
    // Larger values would require a while loop during normalize.
    assert(sleep < 1000);
    const int milliToNano = 1000000;
    abstime.tv_nsec += sleep * milliToNano;
    // Normalize.
    const int milliToSec = 1000000000;
    if (abstime.tv_nsec >= milliToSec)
    {
        abstime.tv_sec++;
        abstime.tv_nsec -= milliToSec;
    }
    pthread_cond_timedwait(&spinnerContext->conditionVariable,
                           &spinnerContext->mutex, &abstime);
}

/// Spinner that waits till osmify() completes.
int spinner(struct SpinnerContext* spinnerContext)
{
    const char spinCharacters[] = "\\|/-";
    size_t spinIndex = 0;

    while (true)
    {
        pthread_mutex_lock(&spinnerContext->mutex);

        const int sleep = 100;
        wait_for(spinnerContext, sleep);

        if (spinnerContext->processed)
        {
            printf("\r");
            fflush(stdout);
            if (spinnerContext->error != NULL)
            {
                fprintf(stderr, "failed to osmify: %s\n",
                        spinnerContext->error);
                pthread_mutex_unlock(&spinnerContext->mutex);
                return -1;
            }

            printf("%s\n", spinnerContext->result);
            pthread_mutex_unlock(&spinnerContext->mutex);
            return 0;
        }

        printf("\r [%c] ", spinCharacters[spinIndex]);
        fflush(stdout);
        spinIndex = (spinIndex + 1) % strlen(spinCharacters);

        pthread_mutex_unlock(&spinnerContext->mutex);
    }
}

int main(int argc, char** argv)
{
    int ret = 0;
    struct SpinnerContext spinnerContext;
    spinnerContextInit(&spinnerContext);

    if (argc > 1)
    {
        spinnerContext.query = strdup(argv[1]);
        pthread_t thread;
        if (pthread_create(&thread, NULL, worker, &spinnerContext))
        {
            fprintf(stderr, "pthread_create() failed\n");
            goto cleanup;
        }

        if (spinner(&spinnerContext) < 0)
        {
            ret = 1;
        }
        pthread_join(thread, NULL);
    }
    else
    {
        fprintf(stderr, "usage: addr-osmify <query>\n\n");
        fprintf(stderr, "e.g. addr-osmify 'Mészáros utca 58/a, Budapest'\n");
    }

cleanup:
    spinnerContextDestroy(&spinnerContext);
    return ret;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
