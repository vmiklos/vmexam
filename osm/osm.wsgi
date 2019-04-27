#!/usr/bin/env python3
#
# Copyright 2019 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#

import configparser
import os
import sys


def getWorkdir():
    config = configparser.ConfigParser()
    configPath = os.path.join(os.path.dirname(__file__), "wsgi.ini")
    config.read(configPath)
    return config.get('wsgi', 'workdir').strip()


def application(environ, start_response):
    status = '200 OK'

    workdir = getWorkdir()

    output = ""

    output += "<h1>osm scripts</h1>"

    output += "<h2>streets</h2>"

    output += "<h2>street-housenumbers</h2>"

    output += "<h2>suspicious-streets</h2>"

    output = "<html><body>" + output + "</body></html>"

    outputBytes = output.encode('utf-8')
    response_headers = [('Content-type', 'text/html; charset=utf-8'),
                        ('Content-Length', str(len(outputBytes)))]
    start_response(status, response_headers)
    return [outputBytes]

# vim:set shiftwidth=4 softtabstop=4 expandtab:
