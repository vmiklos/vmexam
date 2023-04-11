#!/usr/bin/env python3
#
# Copyright 2019 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#

"""This module contains functionality to invoke plain wsgi scripts on pythonanywhere."""

import sys

path = '/home/vmiklos/git/osm-gimmisn'
if path not in sys.path:
    sys.path.append(path)

from wsgi import application
