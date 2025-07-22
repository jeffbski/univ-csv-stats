const test = require('ava');
const { calculateStatsFromFile, sum } = require('./index.js');
const path = require('path');
const fs = require('fs');

/**
 * Test suite for the native Node.js addon.
 */

/**
 * Tests for the calculateStatsFromFile function.
 */

test('calculateStatsFromFile should return correct statistics for a valid CSV file', (t) => {
  // Construct the absolute path to the test CSV file.
  // __dirname is a global variable in CommonJS that gives the directory of the current module.
  const testFilePath = path.join(__dirname, '..', '..', 'test_data', 'test.csv');

  // Call the function from the native addon.
  const result = calculateStatsFromFile(testFilePath);

  // Define the expected statistical results.
  // These values are calculated from the data in `test.csv`.
  const expected = {
    count: 3,
    min: 0.01,
    max: 14675.57,
    sum: 18372.92,
    mean: 18372.92 / 3,
    variance: 38840427.15282223,
    standardDeviation: 6232.208850224953,
    skewness: 0.5250785969178626,
    kurtosis: -1.5
  };

  // Assert that the calculated stats match the expected values.
  t.is(result.count, expected.count, 'Count value should be correct');
  t.is(result.min, expected.min, 'Min value should be correct');
  t.is(result.max, expected.max, 'Max value should be correct');
  t.is(result.sum, expected.sum, 'Sum value should be correct');
  t.is(result.variance, expected.variance, 'Variance value should be correct');
  t.is(result.standardDeviation, expected.standardDeviation, 'Standard deviation value should be correct');
  t.is(result.skewness, expected.skewness, 'Skewness value should be correct');
  t.is(result.kurtosis, expected.kurtosis, 'Kurtosis value should be correct');

  // For floating-point numbers like the mean, it's best to check for
  // approximate equality to avoid precision issues.
  t.true(Math.abs(result.mean - expected.mean) < 1e-9, 'Mean value should be correct');
});

test('calculateStatsFromFile should throw an error if the file does not exist', (t) => {
  // Define a path to a file that does not exist.
  const nonExistentFilePath = 'non-existent-file.csv';

  // Use t.throws to assert that an error is thrown.
  // We expect a generic Error from the N-API wrapper.
  const error = t.throws(
    () => {
      calculateStatsFromFile(nonExistentFilePath);
    },
    { instanceOf: Error }
  );

  // The error message from our Rust core library should be passed through.
  // We check for a substring because the exact OS-level error message can vary.
  t.true(
    error.message.includes('No such file or directory'),
    'Error message should indicate that the file was not found'
  );
});

test('calculateStatsFromFile should throw an error for a malformed CSV file', (t) => {
  // Create a temporary, malformed CSV file for this test.
  // This approach is more robust than relying on a static malformed file.
  const malformedCsvPath = path.join(__dirname, '..', '..', 'test_data', 'malformed.csv');
  fs.writeFileSync(malformedCsvPath, 'header1,Amount Received\nvalue1,"not-a-number"\n');

  const error = t.throws(
    () => {
      calculateStatsFromFile(malformedCsvPath);
    },
    { instanceOf: Error }
  );

  // Check for the specific CSV parsing error.
  t.true(error.message.includes('CSV deserialize error'), 'Error message should indicate a CSV parsing issue');

  // Clean up the temporary file.
  fs.unlinkSync(malformedCsvPath);
});
