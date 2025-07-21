#!/usr/bin/env node

import { calculateStatsFromFile } from './index.js';
import path from 'node:path';

/**
 * A simple CLI to calculate statistics from a CSV file using the Rust-powered
 * Node.js addon.
 */
function main() {
  // Basic command-line argument parsing.
  // process.argv[0] is the node executable
  // process.argv[1] is the path to this script
  const filePath = process.argv[2];

  if (!filePath) {
    console.error('Usage: ./cli.js <path-to-csv-file>');
    process.exit(1);
  }

  // Resolve the path to an absolute path for clarity.
  const absolutePath = path.resolve(filePath);
  console.log(`Calculating statistics for '${absolutePath}'...`);

  try {
    // Call the core logic from the Rust native addon.
    const stats = calculateStatsFromFile(absolutePath);

    // If successful, print the statistics in a formatted way.
    console.log("\n--- Statistics for 'Amount Received' ---");
    console.log(`Count:              ${stats.count}`);
    console.log(`Min:                ${stats.min.toFixed(4)}`);
    console.log(`Max:                ${stats.max.toFixed(4)}`);
    console.log(`Sum:                ${stats.sum.toFixed(4)}`);
    console.log(`Mean:               ${stats.mean.toFixed(4)}`);
    console.log(`Variance:           ${stats.variance.toFixed(4)}`);
    console.log(`Standard Deviation: ${stats.standardDeviation.toFixed(4)}`);
    console.log(`Skewness:           ${stats.skewness.toFixed(4)}`);
    console.log(`Kurtosis:           ${stats.kurtosis.toFixed(4)}`);
    console.log('----------------------------------------');
  } catch (e) {
    // Errors from the Rust addon are thrown as JavaScript exceptions.
    // Print the error message to stderr and exit.
    console.error(`Error: ${e.message}`);
    process.exit(1);
  }
}

// Run the main function.
main();
