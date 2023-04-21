#!/bin/bash -e
#
# Copyright 2020 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#

#
# This script starts up a simple HTTP server, runs the tests then shuts the HTTP server down.
#

python3 -m http.server &

echo "Waiting for the HTTP server to start up..."
PID=$!
trap 'kill $PID' EXIT
npx wait-on http://0.0.0.0:8000/

echo "Starting the tests..."
npx cypress run "$@"

# vim:set shiftwidth=4 softtabstop=4 expandtab:
