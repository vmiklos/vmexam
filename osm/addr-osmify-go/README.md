# addr-osmify-cpp

Takes an nominatim query (e.g. 'Mészáros utca 58/a, Budapest') and turns it
into a string that is readable (so that you can save it to your contacts) and
is also machine-friendly, e.g. OsmAnd can parse it as well.

This implementation is written in go:

- [x] static typing

- [x] consistent code formatting (gofmt)

- [x] documentation (this file for users, code comments for developers)

- [x] tests (100% statement coverage, passing under the race detector)

- [x] static code analysis (golint)

## Install

```
go build
go test
```
