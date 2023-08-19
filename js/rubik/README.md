# Rubik solver, browser side

## What does this do?

This is a simple Rubik cube solver, where the emphasis is on allowing you to paint a cube based on a
real-world, scrambled cube and then showing the steps to solve the cube.

You only have to care to hold the cube with the correct orientation while painting (start with the
facing side: red on the facing side, yellow on the right side), because the generated solution won't
rotate the cube, so bad orientation results in no solution.

## Motivation

The motivation is to return to the solved state while you learn solving the cube yourself, so you
never need to physically disassemble the cube while practicing.

## Credits

The code builds on top of [Joe's rubik-js.git](https://github.com/joews/rubik-js) for visualization,
and uses [my rubik solver
service](https://github.com/vmiklos/vmexam/blob/master/share-vmiklos-hu-apps/src/rubik.rs) to solve
the cube.

In other words, this is free software and a browser version of the [Cube
Solver](https://play.google.com/store/apps/details?id=com.jeffprod.cubesolver) Android app.
