use pyo3::prelude::*;

/// An inclusive range of integers.
#[pyclass]
struct Range {
    start: u64,
    end: u64,
}

#[pymethods]
impl Range {
    #[new]
    fn new(start: u64, end: u64) -> Self {
        Range { start, end }
    }

    fn contains(&self, item: u64) -> PyResult<bool> {
        Ok(self.start <= item && item <= self.end)
    }
}

/// The ranges module contains functionality related to the Ranges class.
#[pymodule]
fn ranges(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Range>()?;
    Ok(())
}
