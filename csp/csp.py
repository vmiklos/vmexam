#!/usr/bin/env python3
#
# Copyright 2023 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#

"""
Given a 2x2 table with numbers, select cells, so that the sum of the selected cells will equal to
the provided sum.

(Real-life math exercise for 2nd grade students in primary school.)
"""

from constraint import *

def solve(s, a1, b1, a2, b2):
    problem = Problem()
    problem.addVariable("a1", [0, a1])
    problem.addVariable("b1", [0, b1])
    problem.addVariable("a2", [0, a2])
    problem.addVariable("b2", [0, b2])
    problem.addConstraint(ExactSumConstraint(s))
    solutions = problem.getSolutions()
    if solutions:
        for s in solutions:
            print("{} {}\n{} {}".format(s["a1"], s["b1"], s["a2"], s["b2"]))
        print("\n")
    else:
        print("{} {}\n{} {}\n".format(0, 0, 0, 0))

print("16:")
solve(16, 5, 4, 8, 7)
solve(16, 4, 4, 4, 7)
solve(16, 5, 5, 9, 4)

print("22:")
solve(22, 6, 5, 10, 5)
solve(22, 8, 7, 6, 6)
solve(22, 5, 7, 5, 10)

print("14:")
solve(14, 5, 3, 3, 4)
solve(14, 7, 5, 3, 3)
solve(14, 4, 6, 4, 7)

print("27:")
solve(27, 7, 12, 7, 7)
solve(27, 8, 11, 7, 12)
solve(27, 12, 12, 7, 6)

# vim:set shiftwidth=4 softtabstop=4 expandtab:
