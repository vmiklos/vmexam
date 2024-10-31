#!/usr/bin/env python3
#
# Copyright 2024 Miklos Vajna
#
# SPDX-License-Identifier: MIT

from z3 import Int
from z3 import And
from z3 import Solver
from z3 import sat

"""
Each number in a square is the sum of the numbers in the two squares below it. Use the given numbers
to move to the top of the pyramid!

          |163|
        | j | k |
      | g | h | i |
    |14 | d | e | f |
  | 9 |   | b | c | 10 |
| 5 |   |   | a |   | 3 |
"""

a = Int('a')
b = Int('b')
c = Int('c')
d = Int('d')
e = Int('e')
f = Int('f')
g = Int('g')
h = Int('h')
i = Int('i')
j = Int('j')
k = Int('k')

s = Solver()
s.add(a == b - 1)
s.add(b == d - 5)
s.add(d == g - 14)
s.add(c == a + 7)
s.add(e == b + c)
s.add(h == d + e)
s.add(j == g + h)
s.add(f == c + 10)
s.add(i == e + f)
s.add(k == h + i)
s.add(163 == j + k)
while True:
    if s.check() != sat:
        break
    model = s.model()
    solution = f"a = {model[a]}, "
    solution += f"b = {model[b]}, "
    solution += f"c = {model[c]}, "
    solution += f"d = {model[d]}, "
    solution += f"e = {model[e]}, "
    solution += f"f = {model[f]}, "
    solution += f"g = {model[g]}, "
    solution += f"h = {model[h]}, "
    solution += f"i = {model[i]}, "
    solution += f"j = {model[j]}, "
    solution += f"k = {model[k]}"
    print(solution)
    s.add(And(a != model[a],
              b != model[b],
              c != model[c],
              d != model[d],
              e != model[e],
              f != model[f],
              g != model[g],
              h != model[h],
              i != model[i],
              j != model[j],
              k != model[k]))
