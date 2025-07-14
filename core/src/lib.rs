use incr_stats::incr::Stats;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StatsError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Stats error: {0}")]
    Stats(#[from] incr_stats::error::StatsError),
}

/// Holds the calculated statistics for a dataset.
#[derive(Debug, PartialEq)]
pub struct SelectedStats {
    pub min: f64,
    pub max: f64,
    pub sum: f64,
    pub mean: f64,
}

/// Represents a transaction record from the CSV file.
/// We only need the 'Amount Received' field for our calculations.
#[derive(Debug, Deserialize)]
struct Transaction {
    #[serde(rename = "Amount Received")]
    amount_received: f64,
}

/// Calculates statistics for the 'Amount Received' column from a CSV file.
pub fn calculate_stats_from_file(path: &str) -> Result<SelectedStats, StatsError> {
    let file = File::open(path)?;
    calculate_stats(file)
}

/// Calculates statistics on a CSV data stream using header-based deserialization.
///
/// This function reads from any type that implements `std::io::Read`, expecting
/// a header row. It deserializes each record into a `Transaction` struct
/// and calculates statistics on the `amount_received` field.
/// Rows that fail to deserialize result in an error.
pub fn calculate_stats<R: Read>(reader: R) -> Result<SelectedStats, StatsError> {
    let mut rdr = csv::Reader::from_reader(reader);

    let mut iter = rdr.deserialize();
    let amount_received_stats = iter.try_fold(Stats::new(), |mut acc: Stats, trans| {
        let transaction: Transaction = trans?;
        acc.update(transaction.amount_received)?;
        Ok::<Stats, StatsError>(acc)
    })?;

    let min = amount_received_stats.min()?;
    let max = amount_received_stats.max()?;
    let sum = amount_received_stats.sum()?;
    let mean = amount_received_stats.mean()?;

    Ok(SelectedStats {
        min,
        max,
        sum,
        mean,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_stats_from_string() {
        let data = "Timestamp,From Bank,From Account,To Bank,To Account,Amount Received,Receiving Currency,Amount Paid,Payment Currency,Payment Format,Is Laundering\n\
                    2022/09/01 00:20,010,8000EBD30,010,8000EBD30,100.0,US Dollar,100.0,US Dollar,Reinvestment,0\n\
                    2022/09/01 00:20,03208,8000F4580,001,8000F5340,200.5,US Dollar,200.5,US Dollar,Cheque,0\n\
                    2022/09/01 00:00,03209,8000F4670,03209,8000F4670,50.25,US Dollar,50.25,US Dollar,Reinvestment,0\n\
                    2022/09/01 00:02,012,8000F5030,012,8000F5030,2806.97,US Dollar,2806.97,US Dollar,Reinvestment,0";

        let stats = calculate_stats(data.as_bytes()).unwrap();

        assert_eq!(stats.min, 50.25);
        assert_eq!(stats.max, 2806.97);
        assert_eq!(stats.sum, 3157.72);
        assert_eq!(stats.mean, 3157.72 / 4.0);
    }

    #[test]
    fn test_empty_data() {
        let data = "Timestamp,From Bank,From Account,To Bank,To Account,Amount Received,Receiving Currency,Amount Paid,Payment Currency,Payment Format,Is Laundering\n";

        // A CSV with only a header row, should result in a Stats error
        let result = calculate_stats(data.as_bytes());
        assert!(matches!(result, Err(StatsError::Stats(_))));
    }

    #[test]
    fn test_empty_file() {
        let data = "";

        // An empty file should result in a Stats error.
        let result = calculate_stats(data.as_bytes());
        assert!(matches!(result, Err(StatsError::Stats(_))));
    }

    #[test]
    fn test_data_with_unparsable_amount() {
        let data = "Timestamp,From Bank,From Account,To Bank,To Account,Amount Received,Receiving Currency,Amount Paid,Payment Currency,Payment Format,Is Laundering\n\
                    2022/09/01 00:20,010,8000EBD30,010,8000EBD30,100.0,US Dollar,100.0,US Dollar,Reinvestment,0\n\
                    2022/09/01 00:20,010,8000EBD30,010,8000EBD30,not-a-number,US Dollar,100.0,US Dollar,Reinvestment,0\n\
                    2022/09/01 00:00,03209,8000F4670,03209,8000F4670,50.25,US Dollar,50.25,US Dollar,Reinvestment,0";

        // The row with "not-a-number" should result in an Error
        let result = calculate_stats(data.as_bytes());
        assert!(matches!(result, Err(StatsError::Csv(_))));
    }
}
