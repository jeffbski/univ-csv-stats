#!/usr/bin/env python

import argparse
import sys
import csv

class Statistics:
    """A simple data class to hold the calculated statistics."""
    def __init__(self, count, min_val, max_val, total_sum, mean):
        self.count = count
        self.min = min_val
        self.max = max_val
        self.sum = total_sum
        self.mean = mean

def calculate_stats_from_file(file_path):
    """
    Reads a CSV file and calculates statistics for the "Amount Received" column.

    This implementation uses pure Python for performance comparison.

    Args:
        file_path (str): The path to the CSV file.

    Returns:
        Statistics: An object containing the count, min, max, sum, and mean.

    Raises:
        ValueError: If the file is not found, cannot be processed, is empty,
                    or does not contain the required column.
    """
    count = 0
    total_sum = 0.0
    min_val = float('inf')
    max_val = float('-inf')

    try:
        with open(file_path, mode='r', encoding='utf-8') as infile:
            reader = csv.DictReader(infile)

            if "Amount Received" not in reader.fieldnames:
                raise ValueError("CSV file must have a column named 'Amount Received'")

            for row in reader:
                try:
                    # Extract and convert the amount, skipping if invalid
                    amount = float(row["Amount Received"])
                    count += 1
                    total_sum += amount
                    if amount < min_val:
                        min_val = amount
                    if amount > max_val:
                        max_val = amount
                except (ValueError, TypeError):
                    # This handles rows with non-numeric data or empty values
                    continue
    except FileNotFoundError:
        raise ValueError(f"Error: File not found at '{file_path}'")
    except Exception as e:
        raise ValueError(f"Error processing file: {e}")

    if count == 0:
        raise ValueError("No valid numeric data found in 'Amount Received' column.")

    mean = total_sum / count

    return Statistics(
        count=count,
        min_val=min_val,
        max_val=max_val,
        total_sum=total_sum,
        mean=mean
    )

def main():
    """
    A simple CLI to calculate statistics from a CSV file using pure Python.
    This script serves as a baseline for performance comparison against other
    implementations (e.g., a Rust-powered one).
    """
    parser = argparse.ArgumentParser(
        description="Calculate statistics from a CSV file using pure Python."
    )
    parser.add_argument(
        "file_path",
        type=str,
        help="The path to the CSV file to process.",
    )

    args = parser.parse_args()
    file_path = args.file_path

    print(f"Calculating statistics for '{file_path}' (Native Python)...")

    try:
        # Call the pure Python implementation for statistics calculation.
        stats = calculate_stats_from_file(file_path)

        # If successful, print the statistics in a formatted way.
        print("\nOutput for python-native")
        print("--- Statistics for 'Amount Received' ---")
        print(f"Count:              {stats.count}")
        print(f"Min:                {stats.min:.4f}")
        print(f"Max:                {stats.max:.4f}")
        print(f"Sum:                {stats.sum:.4f}")
        print(f"Mean:               {stats.mean:.4f}")
        print("----------------------------------------")

    except ValueError as e:
        # The calculation function raises ValueError for known issues.
        # Print the error message to stderr and exit.
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
