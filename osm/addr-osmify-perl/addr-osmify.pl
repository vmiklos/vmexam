#!/usr/bin/env perl
#
# Copyright 2020 Miklos Vajna
#
# SPDX-License-Identifier: MIT

use utf8;
use strict;
use warnings;

use JSON qw(decode_json);
use LWP::Simple qw(get);
use LWP::UserAgent;
use URI::Escape qw(uri_escape);

# Send query to overpass turbo.
sub query_turbo($)
{
    my ($query) = @_;
    my $url = "http://overpass-api.de/api/interpreter";
    my $ua = LWP::UserAgent->new();
    my $req = HTTP::Request->new(POST => $url);
    $req->content($query);
    my $res = $ua->request($req);
    return $res->decoded_content;
}

# Send query to nominatim.
sub query_nominatim($)
{
    my ($query) = @_;
    my $url = "http://nominatim.openstreetmap.org/search.php?format=json&q=" . uri_escape($query);
    return get($url);
}

# Turn query into a coordinate + address string.
sub osmify($)
{
    my ($query) = @_;
    # Use nominatim to get the coordinates and the osm type/id.
    # decode_json() returns an arrayref, dereference it to get the list:
    my @elements = @{decode_json(query_nominatim($query))};
    if (scalar(@elements) == 0) {
        print("No results from nominatim\n");
        return;
    }

    if (scalar(@elements) > 1) {
        # There are multiple elements, prefer buildings if possible.
        # Example where this is useful: 'Karinthy Frigyes út 18, Budapest'.
        my @buildings = ();
        foreach (@elements) {
            my %element = %{$_};
            if (exists($element{"class"})) {
                my $class = $element{"class"};
                if ($class eq "building") {
                    push @buildings, $_;
                }
            }
        }

        if (scalar(@buildings) > 0) {
            @elements = @buildings;
        }
    }

    # Dereference again:
    my %element = %{$elements[0]};
    my $lat = $element{"lat"};
    my $lon = $element{"lon"};
    my $object_type = $element{"osm_type"};
    my $object_id = $element{"osm_id"};

    # Use overpass to get the properties of the object.
    my $overpass_query = "[out:json];";
    $overpass_query .= "(";
    $overpass_query .= $object_type . "(" . $object_id . ");";
    $overpass_query .= ");";
    $overpass_query .= "out body;";
    my %j = %{decode_json(query_turbo($overpass_query))};
    @elements = @{$j{"elements"}};
    if (scalar(@elements) == 0) {
        print("No results from overpass\n");
        return;
    }

    %element = %{$elements[0]};
    my %tags = %{$element{"tags"}};
    my $city = $tags{"addr:city"};
    my $housenumber = $tags{"addr:housenumber"};
    my $postcode = $tags{"addr:postcode"};
    my $street = $tags{"addr:street"};
    my $addr = $postcode . " " . $city . ", " . $street . " " . $housenumber;

    # Print the result.
    printf("%s,%s (%s)\n", $lat, $lon, $addr);
}

# Commandline interface to this module.
sub main()
{
    binmode(STDOUT, ":utf8");

    if ($#ARGV + 1 > 0) {
        osmify($ARGV[0]);
    } else {
        printf("usage: addr-osmify <query>\n");
        printf("\n");
        printf("e.g. addr-osmify 'Mészáros utca 58/a, Budapest'\n");
    }
}

main();

# vim:set shiftwidth=4 softtabstop=4 expandtab:
