use wasm_bindgen::prelude::*;

// By default, wasm-bindgen runs in a browser context, which doesn't have
// filesystem access. Therefore, instead of passing a file path, we need to
// pass the file's content as a string. The JavaScript host will be
// responsible for reading the file.
use univ_csv_stats_core::{
    SelectedStats as CoreSelectedStats, calculate_stats,
    calculate_stats_from_file as core_calculate_stats_from_file,
};

// The wasm-bindgen attribute generates the necessary JavaScript "glue" code to
// convert this Rust struct into a JavaScript object that can be easily used.
#[wasm_bindgen]
#[derive(Debug)]
pub struct SelectedStats {
    pub min: f64,
    pub max: f64,
    pub sum: f64,
    pub mean: f64,
}

// This implementation allows for a clean conversion from the core library's
// `SelectedStats` struct into our wasm-bindgen-compatible `SelectedStats` struct.
impl From<CoreSelectedStats> for SelectedStats {
    fn from(stats: CoreSelectedStats) -> Self {
        Self {
            min: stats.min,
            max: stats.max,
            sum: stats.sum,
            mean: stats.mean,
        }
    }
}

/// Calculates statistics from a string of CSV data.
///
/// This function is a wrapper around the `calculate_stats` function from the
/// core Rust library. It takes CSV data as a string, calls the core function,
/// and returns the result as a `Result`.
///
/// In case of success, it returns an `Ok` variant containing a `SelectedStats`
/// object, which will be converted to a JavaScript object.
///
/// In case of an error, it returns an `Err` variant containing a `JsValue`,
/// which will be thrown as a JavaScript exception.
#[wasm_bindgen]
pub fn calculate_stats_from_csv(csv_data: String) -> Result<SelectedStats, JsValue> {
    // Call the core function, mapping the success and error cases to types
    // that wasm-bindgen can understand.
    calculate_stats(csv_data.as_bytes())
        .map(|stats| stats.into()) // Convert CoreSelectedStats to SelectedStats
        .map_err(|err| JsValue::from_str(&err.to_string())) // Convert CoreStatsError to JsValue
}

#[wasm_bindgen]
pub fn calculate_stats_from_file(path: String) -> Result<SelectedStats, JsValue> {
    core_calculate_stats_from_file(&path)
        .map(|stats| stats.into()) // Convert CoreSelectedStats to SelectedStats
        .map_err(|err| JsValue::from_str(&err.to_string())) // Convert CoreStatsError to JsValue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_stats_from_csv_valid() {
        let csv_data = "header1,Amount Received\nvalue1,10.0\nvalue2,20.5\nvalue3,30.0".to_string();
        let result = calculate_stats_from_csv(csv_data).unwrap();
        assert_eq!(result.min, 10.0);
        assert_eq!(result.max, 30.0);
        assert_eq!(result.sum, 60.5);
        assert_eq!(result.mean, 60.5 / 3.0);
    }

    #[test]
    fn test_calculate_stats_from_csv_invalid() {
        let csv_data = "header1,Amount Received\nvalue1,ten".to_string();
        let result = calculate_stats_from_csv(csv_data);
        assert!(result.is_err());
    }
}
