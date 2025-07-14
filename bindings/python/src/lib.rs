use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use thiserror::Error;
use univ_csv_stats_core::{
    SelectedStats as CoreSelectedStats, StatsError as CoreStatsError,
    calculate_stats_from_file as core_calculate_stats,
};

// Create a new error type for the Python bindings. This allows us to have a
// single error type that can be converted from the core library's error type
// and then seamlessly exposed to Python.
#[derive(Error, Debug)]
enum PyStatsError {
    #[error(transparent)]
    Core(#[from] CoreStatsError),
}

// PyO3 needs a way to convert our custom error type into a Python exception.
// This implementation handles that conversion, turning any `PyStatsError`
// into a `PyValueError` in the Python runtime.
impl From<PyStatsError> for PyErr {
    fn from(err: PyStatsError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

/// A Python-compatible struct to hold the calculated statistics.
/// The `#[pyclass]` attribute makes this struct usable as a Python class.
/// The `#[pyo3(get)]` attribute on each field automatically creates a getter,
/// making the fields accessible from Python.
#[pyclass]
#[derive(Debug, PartialEq)]
pub struct PySelectedStats {
    #[pyo3(get)]
    pub min: f64,
    #[pyo3(get)]
    pub max: f64,
    #[pyo3(get)]
    pub sum: f64,
    #[pyo3(get)]
    pub mean: f64,
}

// This implementation allows us to easily convert the core library's
// `SelectedStats` struct into our Python-compatible `PySelectedStats`.
impl From<CoreSelectedStats> for PySelectedStats {
    fn from(stats: CoreSelectedStats) -> Self {
        Self {
            min: stats.min,
            max: stats.max,
            sum: stats.sum,
            mean: stats.mean,
        }
    }
}

/// A Python function that calculates statistics from a CSV file.
///
/// This function is a wrapper around the `calculate_stats_from_file` function
/// from the core Rust library. It takes a file path as a string, calls the
/// core function, and returns the result as a Python object. Errors are
/// converted into Python exceptions.
#[pyfunction]
fn calculate_stats_from_file(path: &str) -> Result<PySelectedStats, PyStatsError> {
    let stats = core_calculate_stats(path)?;
    Ok(stats.into())
}

/// Defines the Python module `univ_csv_stats_python`.
///
/// This function is called by the Python interpreter when the module is imported.
/// It adds the `calculate_stats_from_file` function and the `PySelectedStats`
/// class to the module, making them available in Python.
#[pymodule]
fn univ_csv_stats_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_stats_from_file, m)?)?;
    m.add_class::<PySelectedStats>()?;
    Ok(())
}
