# addr-osmify-js

Takes an nominatim query (e.g. 'Mészáros utca 58/a, Budapest') and turns it
into a string that is readable (so that you can save it to your contacts) and
is also machine-friendly, e.g. OsmAnd can parse it as well.

This implementation is written in JavaScript:

- [ ] static typing (this is JavaScript after all)

- [x] consistent code formatting (clang-format)

- [x] documentation (this file for users, code comments for developers)

- [x] tests (100% statement coverage)

- [x] static code analysis (eslint)

## Install

```
autoconf
./configure
make
make check
make run
```
