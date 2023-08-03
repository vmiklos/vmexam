# rubik

Cmdline tool, related to <https://en.wikipedia.org/wiki/Rubik%27s_Cube>.

## shuffle

This subcommand shuffles a cube, similar to what's described in [the
book](https://bookline.hu/product/home.action?_v=Rubik_Erno_A_buvos_kocka&type=20&id=147099), on
page 73 (see also an [online
version](https://www.worldcubeassociation.org/regulations/history/files/scrambles/scramble_cube.htm?size=3&num=1&len=24&col=yobwrg&subbutton=Scramble%21)).

## solve

Usage example:

```
$ cargo run --release -- solve --faces DRBLUURLDRBLRRBFLFFUBFFDRUDURRBDFBBULDUDLUDLBUFFDBFLRL
L2 B' D R F B' L U B R' U' B2 D L2 D2 R2 B2 D' B2 D F2 U
```

A solved state would be UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB.
