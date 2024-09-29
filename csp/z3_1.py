#!/usr/bin/env python3
#
# Copyright 2024 Miklos Vajna
#
# SPDX-License-Identifier: MIT

from z3 import Int
from z3 import Or
from z3 import Solver

"""
https://www.oktatas.hu/pub_bin/dload/kozoktatas/beiskolazas/feladatsorok/2022/M4_2022_1_fl.pdf 7)

In an addition specified by letters, the same letters represent the same digits.
The following letters can only be the following numbers:
Z = 1, 2, 3, 4; R = 1, 2, 3; B = 3, 5
Calculate all options so that the result of the operations is correct!

 ZZR
+RRZ
 BBB
"""

z = Int('z')
r = Int('r')
b = Int('b')

s = Solver()
s.add(Or(z == 1, z == 2, z == 3, z == 4))
s.add(Or(r == 1, r == 2, r == 3))
s.add(Or(b == 3, b == 5))
s.add((100 * z + 10 * z + r) + (100 * r + 10 * r + z) == (100 * b + 10 * b + b))
solutions = []
while True:
    ret = s.check()
    if str(ret) != "sat":
        break
    model = s.model()
    solution = f" {model[z]}{model[z]}{model[r]}\n"
    solution += f"+{model[r]}{model[r]}{model[z]}\n"
    solution += f" {model[b]}{model[b]}{model[b]}"
    solutions.append(solution)
    s.add(Or(z != model[z], r != model[r], b != model[b]))
solutions.sort()
# Expected to print the 5 solutions.
print("\n\n".join(solutions))
