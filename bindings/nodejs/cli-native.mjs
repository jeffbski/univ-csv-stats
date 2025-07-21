#!/usr/bin/env node

import path from 'node:path';
import fs from 'node:fs';
import { parse } from 'csv-parse/sync';

/**
 * A simple CLI to calculate statistics from a CSV file using a native Node.js
 * implementation.
 */
function main() {
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
  console.log(`Calculating statistics for '${absolutePath}'...`);

  try {
    // Call the core logic from our native implementation.
    const stats = calculateStatsFromFile(absolutePath);

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

function calculateStatsFromFile(filePath) {
  const fileContent = fs.readFileSync(filePath, { encoding: 'utf8' });

  // Use csv-parse to process the file.
  const records = parse(fileContent, {
    columns: true, // Treat the first line as headers.
    skip_empty_lines: true // Ignore empty lines.
  });

  let count = 0;
  let min = Infinity;
  let max = -Infinity;
  let sum = 0;

  // Process each record using a for...of loop.
  for (const record of records) {
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

  // Avoid division by zero if there are no valid data points.
  const mean = count > 0 ? sum / count : 0;

  // If no data was found, min/max should be 0.
  if (count === 0) {
    min = 0;
    max = 0;
  }

  return { count, min, max, sum, mean };
}

// Run the main function.
main();
