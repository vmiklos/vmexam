# rubik

Cmdline tool, related to <https://en.wikipedia.org/wiki/Rubik%27s_Cube>.

## shuffle

This subcommand shuffles a cube, similar to what's described in [the
book](https://bookline.hu/product/home.action?_v=Rubik_Erno_A_buvos_kocka&type=20&id=147099), on
page 73 (see also an [online
version](https://www.worldcubeassociation.org/regulations/history/files/scrambles/scramble_cube.htm?size=3&num=1&len=24&col=yobwrg&subbutton=Scramble%21)).

Example output:

```
$ cargo run -- shuffle
F L' B R' U R U B' L2 R' F2 U2 L' F2 D F U R' D R U' L' R2 D2
```

## solve

Usage example:

```
$ cargo run -- solve --colors GYOWBBYWGYOWYYORWRRBORRGYBGBYYOGROOBWGBGWBGWOBRRGORWYW
L2 B' D R F B' L U B R' U' B2 D L2 D2 R2 B2 D' B2 D F2 U
```

A solved state would be BBBBBBBBBYYYYYYYYYRRRRRRRRRGGGGGGGGGWWWWWWWWWOOOOOOOOO, for blue, yellow,
red, green, white and orange.

You can also omit `--colors` and enter each side separately. In that case the cube is laid out like
this:

```
            |************|
            |*B1**B2**U3*|
            |************|
            |*B4**B5**U6*|
            |************|
            |*B7**B8**B9*|
            |************|
************|************|************|************|
*W1**W2**W3*|*R1**R2**R3*|*Y1**Y2**Y3*|*O1**O2**O3*|
************|************|************|************|
*W4**W5**W6*|*R4**R5**R6*|*Y4**Y5**Y6*|*O4**O5**O6*|
************|************|************|************|
*W7**W8**W9*|*R7**R8**R9*|*Y7**Y8**Y9*|*O7**O8**O9*|
************|************|************|************|
            |************|
            |*G1**G2**G3*|
            |************|
            |*G4**G5**G6*|
            |************|
            |*G7**G8**G9*|
            |************|
```

(Internally, this is facelet DRBLUURLDRBLRRBFLFFUBFFDRUDURRBDFBBULDUDLUDLBUFFDBFLRL, a solved state
would be UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB).
