#!/usr/bin/php
<?php declare(strict_types=1);

/*
 * Copyright 2020 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 *
 * Takes an OSM way ID and turns it into a string that is readable and
 * e.g. OsmAnd can parse it as well.
 */

// Send query to overpass turbo.
function query_turbo(string $query): string
{
    $url = 'http://overpass-api.de/api/interpreter';

    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, $url);
    curl_setopt($ch, CURLOPT_POST, true);
    curl_setopt($ch, CURLOPT_POSTFIELDS, $query);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);

    return curl_exec($ch);
}

// Send query to nominatim.
function query_nominatim(string $query): string
{
    $url = 'https://nominatim.openstreetmap.org/search.php?';
    $params = [
        'q' => $query,
        'format' => 'json',
    ];
    $url .= http_build_query($params);

    return file_get_contents($url);
}

// Turn query into a coordinate + address string.
function osmify(string $query): void
{
    ini_set('user_agent', 'Mozilla/5.0 (X11; Linux x86_64; rv:68.0) Gecko/20100101 Firefox/68.0');

    // Use nominatim to get the coordinates and the osm type/id.
    $elements = json_decode(query_nominatim($query));

    if (!count($elements)) {
        echo "No results from nominatim\n";

        return;
    }

    if (count($elements) > 1) {
        $buildings = [];

        foreach ($elements as $i) {
            if (!property_exists($i, 'class') || $i->class !== 'building') {
                break;
            }

            array_push($buildings, $i);
        }

        if (count($buildings)) {
            $elements = $buildings;
        }
    }

    $element = $elements[0];
    $lat = $element->lat;
    $lon = $element->lon;
    $object_type = $element->osm_type;
    $object_id = $element->osm_id;

    // Use overpass to get the properties of the object.
    $overpass_query = "[out:json];\n";
    $overpass_query .= '(';
    $overpass_query .= $object_type . '(' . $object_id . ');';
    $overpass_query .= ');';
    $overpass_query .= 'out body;';
    $j = json_decode(query_turbo($overpass_query));
    $elements = $j->elements;

    if (!count($elements)) {
        echo "No results from overpass\n";

        return;
    }

    $element = $elements[0];
    $city = $element->tags->{'addr:city'};
    $housenumber = $element->tags->{'addr:housenumber'};
    $postcode = $element->tags->{'addr:postcode'};
    $street = $element->tags->{'addr:street'};
    $addr = $postcode . ' ' . $city . ', ' . $street . ' ' . $housenumber;

    // Print the result.
    echo 'geo:' . $lat . ',' . $lon . ' (' . $addr . ")\n";
}

// Commandline interface to this module.
function main(): void
{
    if (count($_SERVER['argv']) > 1) {
        osmify($_SERVER['argv'][1]);
    } else {
        echo "usage: addr-osmify <query>\n";
        echo "\n";
        echo "e.g. addr-osmify 'Mészáros utca 58/a, Budapest'\n";
    }
}

main();

// vim:set shiftwidth=4 softtabstop=4 expandtab:
?>
