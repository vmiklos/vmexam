# csp

## puzzle-solver sample code

Given a 2x2 table with numbers, select cells, so that the sum of the selected cells will equal to
the provided sum.

(Real-life math exercise for 2nd grade students in primary school.)

Example:

```console
$ csp --sum 16 5 4 8 7
5 4
0 7
```

## cube

See `src/cube.rs` for the problem description. To install the solver:

```
cargo install --git https://github.com/vmiklos/vmexam --bin cube csp
```
