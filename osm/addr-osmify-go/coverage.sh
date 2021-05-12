#!/bin/bash -e
# Copyright 2021 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

go test -coverprofile=coverage.out

# These are intentionally mocked.
sed -i '/main.go/d' coverage.out
sed -i '/urllib.go/d' coverage.out

go tool cover -func=coverage.out 2>&1 |tee coverage.percent
echo "Coverage report is now available: go tool cover -html=coverage.out"

if !  grep -q "(statements)	100.0%" coverage.percent; then
    echo "Coverage failure: total is less than 100.0%"
    exit 1
fi

# vim:set shiftwidth=4 softtabstop=4 expandtab:
