#!/usr/bin/env python3
#
# Copyright 2019 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#

import configparser
import os
import sys
import yaml
sys.path.append(os.path.dirname(__file__))
import overpass_query

def getWorkdir():
    config = configparser.ConfigParser()
    configPath = os.path.join(os.path.dirname(__file__), "wsgi.ini")
    config.read(configPath)
    return config.get('wsgi', 'workdir').strip()


# Returns a name -> relation ID dictionary.
# Expected YAML format (without quotes):
#
# "ormezo: 2713749
# terezvaros: 3229919"
def getRelations(workdir):
    return yaml.load(open(os.path.join(workdir, "relations.yaml")))


# Expected requestUri: e.g. /osm/streets/ormezo/view-query
def handleStreets(requestUri, workdir):
    output = ""

    tokens = requestUri.split("/")
    relation = tokens[-2]
    action = tokens[-1]

    if action == "view-query":
        output += "<pre>"
        with open(os.path.join(workdir, "streets-%s.txt" % relation)) as sock:
            output += sock.read()
        output += "</pre>"
    elif action == "view-result":
        output += "<pre>"
        with open(os.path.join(workdir, "streets-%s.csv" % relation)) as sock:
            output += sock.read()
        output += "</pre>"
    elif action == "update-result":
        with open(os.path.join(workdir, "streets-%s.txt" % relation)) as sock:
            query = sock.read()
        result = overpass_query.overpassQuery(query)
        with open(os.path.join(workdir, "streets-%s.csv" % relation), mode="w") as sock:
            sock.write(result)
            output += "update finished. <a href=\"/osm/streets/" + relation + "/view-result\">view</a>"

    return output


def handleMain(relations):
    output = ""

    output += "<h1>osm scripts</h1>"

    output += "<h2>streets</h2>"

    output += "<ul>"
    for k, v in relations.items():
        output += "<li>"
        output += "<a href=\"https://www.openstreetmap.org/relation/" + str(v) + "\">" + k + "</a>:"
        output += "<a href=\"/osm/streets/" + k + "/view-query\">view query</a>, "
        output += "<a href=\"/osm/streets/" + k + "/view-result\">view result</a>, "
        output += "<a href=\"/osm/streets/" + k + "/update-result\">update result</a>"
        output += "</li>"
    output += "</ul>"

    output += "<h2>street-housenumbers</h2>"

    output += "<ul>"
    for k, v in relations.items():
        output += "<li>"
        output += "<a href=\"https://www.openstreetmap.org/relation/" + str(v) + "\">" + k + "</a>:"
        output += "<a href=\"/osm/street-housenumbers/" + k + "/view-query\">view query</a>, "
        output += "<a href=\"/osm/street-housenumbers/" + k + "/view-result\">view result</a>, "
        output += "<a href=\"/osm/street-housenumbers/" + k + "/update-result\">update result</a>"
        output += "</li>"
    output += "</ul>"

    output += "<h2>suspicious-streets</h2>"

    output += "<ul>"
    for k, v in relations.items():
        output += "<li>"
        output += "<a href=\"https://www.openstreetmap.org/relation/" + str(v) + "\">" + k + "</a>:"
        output += "<a href=\"/osm/suspicious-streets/" + k + "/view-result\">view result</a>"
        output += "</li>"
    output += "</ul>"

    output = "<html><body>" + output + "</body></html>"

    return output


def application(environ, start_response):
    status = '200 OK'

    requestUri = environ.get("REQUEST_URI")

    workdir = getWorkdir()

    relations = getRelations(workdir)

    if requestUri.startswith("/osm/streets/"):
        output = handleStreets(requestUri, workdir)
    else:
        output = handleMain(relations)

    outputBytes = output.encode('utf-8')
    response_headers = [('Content-type', 'text/html; charset=utf-8'),
                        ('Content-Length', str(len(outputBytes)))]
    start_response(status, response_headers)
    return [outputBytes]

# vim:set shiftwidth=4 softtabstop=4 expandtab:
