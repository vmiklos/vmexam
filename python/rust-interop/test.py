#!/usr/bin/env python3

import ranges

r = ranges.Range(2, 4)
assert not r.contains(1)
assert r.contains(3)
