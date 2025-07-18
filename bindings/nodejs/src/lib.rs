#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{Error as NapiError, Status};
use univ_csv_stats_core::{
  SelectedStats as CoreSelectedStats, calculate_stats_from_file as core_calculate_stats,
};

/// A struct that will be exposed to Node.js as a plain JavaScript object.
///
/// The `#[napi(object)]` attribute directs `napi-rs` to generate the
/// necessary code to convert this struct to and from a JavaScript object.
#[napi(object)]
#[derive(Debug)]
pub struct SelectedStats {
  pub count: u32,
  pub min: f64,
  pub max: f64,
  pub sum: f64,
  pub mean: f64,
  pub variance: f64,
  pub standard_deviation: f64,
  pub skewness: f64,
  pub kurtosis: f64,
}

/// This implementation allows for a clean conversion from the core library's
/// `SelectedStats` struct into our Node.js-compatible `SelectedStats` struct.
impl From<CoreSelectedStats> for SelectedStats {
  fn from(stats: CoreSelectedStats) -> Self {
    Self {
      count: stats.count,
      min: stats.min,
      max: stats.max,
      sum: stats.sum,
      mean: stats.mean,
      variance: stats.variance,
      standard_deviation: stats.standard_deviation,
      skewness: stats.skewness,
      kurtosis: stats.kurtosis,
    }
  }
}

/// A Node.js function that calculates statistics from a CSV file.
///
/// This function wraps the `calculate_stats_from_file` function from the core
/// Rust library. It manually handles the `Result` from the core library.
/// If the core function returns an `Ok`, the stats are converted and returned.
/// If it returns an `Err`, the error is converted into a `napi::Error`, which
/// `napi-rs` will then throw as a JavaScript exception.
#[napi]
pub fn calculate_stats_from_file(path: String) -> Result<SelectedStats, NapiError> {
  match core_calculate_stats(&path) {
    Ok(stats) => {
      // If successful, convert the core stats struct into our N-API compatible
      // struct and return it.
      Ok(stats.into())
    }
    Err(err) => {
      // If an error occurs, create a new NapiError and return it.
      // This will be thrown as an exception in the Node.js environment.
      Err(NapiError::new(Status::GenericFailure, err.to_string()))
    }
  }
}
