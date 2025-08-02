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
        # Call the core logic from the Rust extension module.
        stats = univ_csv_stats_python.calculate_stats_from_file(file_path)

        # If successful, print the statistics in a formatted way.
        print("\nOutput for python-rust")
        print("--- Statistics for 'Amount Received' ---")
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

    except ValueError as e:
        # The Rust errors are converted to Python's ValueError.
        # Print the error message to stderr and exit.
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
