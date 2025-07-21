#!/usr/bin/env node

import path from 'node:path';
import fs from 'node:fs';
import { parse } from 'csv-parse';

/**
 * A simple CLI to calculate statistics from a CSV file using a native Node.js
 * stream-based implementation for large file support.
 */
async function main() {
  // Basic command-line argument parsing.
  // process.argv[0] is the node executable
  // process.argv[1] is the path to this script
  const filePath = process.argv[2];

  if (!filePath) {
    console.error('Usage: ./cli-native.mjs <path-to-csv-file>');
    process.exit(1);
  }

  // Resolve the path to an absolute path for clarity.
  const absolutePath = path.resolve(filePath);
  console.log(`Calculating statistics for '${absolutePath}' (Native Nodejs streaming)...`);

  try {
    // Call the async core logic from our native implementation.
    const stats = await calculateStatsFromFile(absolutePath);

    // If successful, print the statistics in a formatted way.
    console.log("\n--- Statistics for 'Amount Received' ---");
    console.log(`Count:              ${stats.count}`);
    console.log(`Min:                ${stats.min.toFixed(4)}`);
    console.log(`Max:                ${stats.max.toFixed(4)}`);
    console.log(`Sum:                ${stats.sum.toFixed(4)}`);
    console.log(`Mean:               ${stats.mean.toFixed(4)}`);
    console.log('----------------------------------------');
  } catch (e) {
    // Print the error message to stderr and exit.
    console.error(`Error: ${e.message}`);
    process.exit(1);
  }
}

/**
 * Reads a CSV file using a readable stream and calculates statistics for the
 * "Amount Received" column.
 *
 * This async implementation is suitable for very large files as it does not
 * load the entire file into memory.
 *
 * @param {string} filePath The absolute path to the CSV file.
 * @returns {Promise<object>} A promise that resolves with an object containing
 *                            the calculated statistics.
 */
async function calculateStatsFromFile(filePath) {
  // Immediately check for file existence for a clearer error message.
  if (!fs.existsSync(filePath)) {
    throw new Error(`File not found at '${filePath}'`);
  }

  let count = 0;
  let min = Infinity;
  let max = -Infinity;
  let sum = 0;

  // Create a readable stream and pipe it to the csv-parser.
  const parser = fs.createReadStream(filePath).pipe(
    parse({
      // Use the 'columns' option to treat the first row as headers and also
      // to validate that the required column exists.
      columns: (headers) => {
        if (!headers.includes('Amount Received')) {
          // Throwing an error here will cause the stream to emit an error.
          throw new Error("CSV file must have a column named 'Amount Received'");
        }
        return headers; // Return headers to be used for mapping.
      },
      skip_empty_lines: true // Ignore empty lines.
    })
  );

  // Process each record asynchronously as it comes through the stream.
  for await (const record of parser) {
    const amountStr = record['Amount Received'];

    if (amountStr) {
      const amount = parseFloat(amountStr);
      // Ensure the parsed amount is a valid number.
      if (!isNaN(amount)) {
        count++;
        sum += amount;
        if (amount < min) min = amount;
        if (amount > max) max = amount;
      }
    }
  }

  // If no valid data was found after processing the whole file, throw an error.
  if (count === 0) {
    throw new Error("No valid numeric data found in 'Amount Received' column.");
  }

  // Calculate the mean, avoiding division by zero.
  const mean = sum / count;

  return { count, min, max, sum, mean };
}

// Run the main async function.
await main();
