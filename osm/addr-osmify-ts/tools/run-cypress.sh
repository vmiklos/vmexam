#!/bin/bash -e
#
# Copyright 2020 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#

#
# This script starts up a simple HTTP server, runs the tests then shuts the HTTP server down.
#

python3 -m http.server &

echo "Waiting for the HTTP server to start up..."
PID=$!
trap 'kill $PID' EXIT
node_modules/.bin/wait-on http://0.0.0.0:8000/

echo "Starting the tests..."
node_modules/.bin/cypress run "$@"

# vim:set shiftwidth=4 softtabstop=4 expandtab:
