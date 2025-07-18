import test from 'ava';
import { calculate_stats_from_csv } from 'univ-csv-stats-nodejs-wasm';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

/**
 * Test suite for the `calculate_stats_from_csv` function.
 */

test('calculate_stats_from_csv should return correct statistics for a valid CSV file', (t) => {
  // Construct the absolute path to the test CSV file.
  const testFilePath = path.join(__dirname, '..', 'test_data', 'test.csv');
  const csvData = fs.readFileSync(testFilePath, 'utf8');

  // Call the wasm function.
  const result = calculate_stats_from_csv(csvData);

  const expected = {
    count: 3,
    min: 0.01,
    max: 14675.57,
    sum: 18372.92,
    mean: 18372.92 / 3,
    variance: 38840427.15282223,
    standard_deviation: 6232.208850224953,
    skewness: 0.5250785969178626,
    kurtosis: -1.5
  };

  // Assert that the calculated stats match the expected values.
  t.is(result.count, expected.count, 'Count value should be correct');
  t.is(result.min, expected.min, 'Min value should be correct');
  t.is(result.max, expected.max, 'Max value should be correct');
  t.is(result.sum, expected.sum, 'Sum value should be correct');
  t.is(result.variance, expected.variance, 'Variance value should be correct');
  t.is(result.standard_deviation, expected.standard_deviation, 'Standard deviation value should be correct');
  t.is(result.skewness, expected.skewness, 'Skewness value should be correct');
  t.is(result.kurtosis, expected.kurtosis, 'Kurtosis value should be correct');

  // For floating-point numbers like the mean, it's best to check for
  // approximate equality to avoid precision issues.
  t.true(Math.abs(result.mean - expected.mean) < 1e-9, 'Mean value should be correct');

  // In wasm-bindgen, when a struct is returned, memory is allocated on the
  // wasm heap. We must explicitly free it to avoid memory leaks.
  result.free();
});

test('calculate_stats_from_csv should throw an error for malformed CSV data', async (t) => {
  // This CSV data has a non-numeric value where a number is expected.
  const csvData = `header1,Amount Received\nvalue1,"not-a-number"`;

  try {
    calculate_stats_from_csv(csvData);
    // If the above line doesn't throw, the test should fail.
    t.fail('Expected an error to be thrown for malformed CSV data.');
  } catch (error) {
    // Check that the error message from the Rust core library is propagated correctly.
    // wasm-bindgen throws the error as a string primitive.
    t.true(
      typeof error === 'string' && error.includes('CSV deserialize error'),
      'Error should be a string indicating a CSV parsing issue'
    );
  }
});

test('calculate_stats_from_csv should throw an error for empty CSV data', async (t) => {
  const csvData = ''; // An empty string.

  try {
    calculate_stats_from_csv(csvData);
    t.fail('Expected an error to be thrown for empty CSV data.');
  } catch (error) {
    // The core library returns a specific error when no data is available to calculate stats.
    t.true(
      typeof error === 'string' && error.includes('Stats error: not enough data'),
      'Error message should indicate a stats calculation issue for empty data'
    );
  }
});

test('calculate_stats_from_csv should throw an error for header-only CSV data', async (t) => {
  const csvData = `header1,Amount Received`; // Only a header row, no data.

  try {
    calculate_stats_from_csv(csvData);
    t.fail('Expected an error to be thrown for header-only CSV data.');
  } catch (error) {
    // This should also result in a stats error because there are no records to process.
    t.true(
      typeof error === 'string' && error.includes('Stats error: not enough data'),
      'Error message should indicate a stats calculation issue for header-only data'
    );
  }
});
