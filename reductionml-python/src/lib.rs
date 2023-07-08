use pyo3::prelude::*;

pub(crate) mod workspace;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass]
struct ScalarPrediction(reductionml_core::ScalarPrediction);

/// A Python module implemented in Rust.
#[pymodule]
fn _reductionml(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<workspace::Workspace>()?;
    m.add_class::<ScalarPrediction>()?;
    Ok(())
}
