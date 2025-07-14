import univ_csv_stats_python
from pathlib import Path
import pytest


def test_calculate_stats_from_file():
    """
    Tests that the calculate_stats_from_file function correctly calculates
    statistics from a CSV file.
    """
    # Get the absolute path to the test CSV file.
    # The test runner will execute from a different directory, so we need
    # to construct the path relative to this test file.
    csv_path = Path(__file__).parent / "data" / "test.csv"

    # Call the function to calculate stats
    stats = univ_csv_stats_python.calculate_stats_from_file(str(csv_path))

    # Assert that the calculated stats are correct
    assert stats.min == 0.01
    assert stats.max == 14675.57
    assert stats.sum == 18372.92
    assert stats.mean == 6124.306666666667

    # Also check the types
    assert isinstance(stats.min, float)
    assert isinstance(stats.max, float)
    assert isinstance(stats.sum, float)
    assert isinstance(stats.mean, float)


def test_calculate_stats_from_file_not_found():
    """
    Tests that a ValueError is raised when the file is not found.
    """
    # We expect a PyValueError, which translates to a ValueError in Python.
    # The error message comes from the Rust core library's error handling.
    with pytest.raises(ValueError) as excinfo:
        univ_csv_stats_python.calculate_stats_from_file("non_existent_file.csv")

    # Check that the error message contains a hint that the file is not found.
    # The exact message depends on the OS and the Rust io::Error formatting.
    assert "No such file or directory" in str(excinfo.value)
