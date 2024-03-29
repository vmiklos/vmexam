# Python → Rust porting notes

Based on porting osm-gimmisn over from Python to Rust. Read
<https://blog.waleedkhan.name/port-python-to-rust/> first.

I started porting on 2021-08-01 and finished on 2021-11-14, taking care of about 11000 lines of
Python code (including tests).

Before porting:

- Tests: make sure you have 100% statement coverage to avoid regressions.

- Have type hints, checked by mypy.

- Localization: if you use gettext: replace `_()` with `tr()`, which is not reserved in Rust.

- Avoid mocking in test code, replace with interfaces.

This can be a little complex. For example, a Time interface that wraps the current system time needs
these for incremental porting: `context.Time` (interface), `context.StdTime` (real implementation) and
`tests.test_context.TestTime` (test implementation) on the Python side. Then `context::Time` (Rust
trait), `context::StdTime` (real impl), `context::PyTime` (wraps a `dyn Time`), `PyAnyTime `(wraps a `PyAny`,
implements `Time`) and `TestTime` on the Rust side. But this is only temporary, at the end of the
porting you'll again have a single trait and 2 implementations.

- Kill inheritance (except interface classes), replace with encapsulation.

Actual porting:

- `Vec<u8>` is mapped to `List[int]` by default, need custom mapping if the wanted result type is bytes.

- Maintain Python type hints even for the API exposed from rust.

- Add python wrappers for the pyo3 glue code so code coverage points out when those can be removed.

I.e. each converted function has 3 wrappers: rust function that is a pyfunction, Python type hints
and the original Python function, which is now just a stub.

- Either avoid generics and callables or port the function cluster in one go.

- Don't worry about exceptions, it is possible to map rust errors to python exceptions.

- Understand ownership: you grow the Rust code from bottom up, so Rust can have (mutable) references
  to the Python heap, but not the other way around, see e.g.
<https://github.com/vmiklos/osm-gimmisn/blob/002d1611d9c98efb0e9287d459fd3326cb9dfce9/src/util.rs#L1112-L1116>.

- `std::fs::PathBuf` has an `ends_with()`, but it compares the whole filename to the provided
  suffix, you probably want `extension()` instead.

Conversions in Rust:

- string → bytes: `std::str::as_bytes()`.

- bytes → string: `std::str::from_utf8()`.

- vector of tuples → map: `let map: HashMap<_, _> = vec.into_iter().collect();`

Overall a very positive experience. If you have a larger hobby project, then can definitely
recommend the incremental porting way. It takes more time, but small fixes and features can be added
in parallel to porting, which means less frustration.
