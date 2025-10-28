use pyo3::prelude::*;

#[pyfunction]
fn is_valid_syntax(code: &str) -> bool {
    ruff_python_parser::parse_module(code).is_ok()
}

#[pyfunction]
fn format_string(code: &str) -> PyResult<String> {
    ruff_python_formatter::format_module_source(
        code,
        ruff_python_formatter::PyFormatOptions::default(),
    )
    .map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to format code: {}", e))
    })
    .map(|formatted| formatted.into_code())
}

/// A Python module implemented in Rust.
#[pymodule(gil_used = false)]
fn ruff_format(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(format_string, m)?)?;
    m.add_function(wrap_pyfunction!(is_valid_syntax, m)?)?;
    Ok(())
}
