#!/bin/bash -e
# Copyright 2021 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

go test -coverprofile=coverage.out

# These are intentionally mocked.
sed -i '/main.go/d' coverage.out
sed -i '/urllib.go/d' coverage.out

go tool cover -func=coverage.out
echo "Coverage report is now available: go tool cover -html=coverage.out"

# vim:set shiftwidth=4 softtabstop=4 expandtab:
