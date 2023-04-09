from constraint import *

problem = Problem()
problem.addVariable("a1", [0, 5])
problem.addVariable("b1", [0, 4])
problem.addVariable("a2", [0, 8])
problem.addVariable("b2", [0, 7])
problem.addConstraint(ExactSumConstraint(16))
solutions = problem.getSolutions()
if solutions:
    for s in solutions:
        print("{} {}\n{} {}".format(s["a1"], s["b1"], s["a2"], s["b2"]))
