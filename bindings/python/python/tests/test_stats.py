import univ_csv_stats_python
from pathlib import Path
import pytest


def test_calculate_stats_from_file():
    """
    Tests that the calculate_stats_from_file function correctly calculates
    all statistics from a CSV file.
    """
    # Get the absolute path to the test CSV file.
    # The test runner will execute from a different directory, so we need
    # to construct the path relative to this test file.
    csv_path = Path(__file__).parent / "data" / "test.csv"

    # Call the function to calculate stats
    stats = univ_csv_stats_python.calculate_stats_from_file(str(csv_path))

    # Expected values are pre-calculated based on the data in test.csv
    # [3697.34, 0.01, 14675.57]
    expected_sum = 3697.34 + 0.01 + 14675.57
    expected_mean = expected_sum / 3.0

    # Assert that the calculated stats are correct
    assert stats.count == 3
    assert stats.min == pytest.approx(0.01)
    assert stats.max == pytest.approx(14675.57)
    assert stats.sum == pytest.approx(expected_sum)
    assert stats.mean == pytest.approx(expected_mean)
    assert stats.variance == pytest.approx(38840427.15282223)
    assert stats.standard_deviation == pytest.approx(6232.208850224953)
    assert stats.skewness == pytest.approx(0.5250785969178626)
    assert stats.kurtosis == pytest.approx(-1.5)

    # Also check the types
    assert isinstance(stats.count, int)
    assert isinstance(stats.min, float)
    assert isinstance(stats.max, float)
    assert isinstance(stats.sum, float)
    assert isinstance(stats.mean, float)
    assert isinstance(stats.variance, float)
    assert isinstance(stats.standard_deviation, float)
    assert isinstance(stats.skewness, float)
    assert isinstance(stats.kurtosis, float)


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
