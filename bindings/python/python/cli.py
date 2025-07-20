#!/usr/bin/env python

import argparse
import sys
import univ_csv_stats_python

def main():
    """
    A simple CLI to calculate statistics from a CSV file using the Rust-powered
    Python library.
    """
    parser = argparse.ArgumentParser(
        description="Calculate statistics from a CSV file."
    )
    parser.add_argument(
        "file_path",
        type=str,
        help="The path to the CSV file to process.",
    )

    args = parser.parse_args()
    file_path = args.file_path

    print(f"Calculating statistics for '{file_path}'...")

    try:
        # Read the file content in Python first.
        with open(file_path, 'r', encoding='utf-8') as f:
            csv_data = f.read()

        # Call the core logic, passing the CSV content as a string.
        stats = univ_csv_stats_python.calculate_stats_from_csv(csv_data)

        # If successful, print the statistics in a formatted way.
        print("\n--- Statistics for 'Amount Received' ---")
        print(f"Count:              {stats.count}")
        print(f"Min:                {stats.min:.4f}")
        print(f"Max:                {stats.max:.4f}")
        print(f"Sum:                {stats.sum:.4f}")
        print(f"Mean:               {stats.mean:.4f}")
        print(f"Variance:           {stats.variance:.4f}")
        print(f"Standard Deviation: {stats.standard_deviation:.4f}")
        print(f"Skewness:           {stats.skewness:.4f}")
        print(f"Kurtosis:           {stats.kurtosis:.4f}")
        print("----------------------------------------")

    except (ValueError, IOError) as e:
        # Handle both calculation errors from Rust (ValueError) and file
        # reading errors from Python (IOError).
        # Print the error message to stderr and exit.
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
