import pytest
from pathlib import Path
from polars import DataFrame
from py_adaone import ada3dp_to_polars, polars_to_ada3dp

TEST_FILE_PATH = Path(__file__).parent / "test.ada3dp"
OUTPUT_FILE_PATH = Path(__file__).parent / "test_output.ada3dp"


def test_read_ada3dp():
    df = ada3dp_to_polars(TEST_FILE_PATH)
    assert isinstance(df, DataFrame)
    assert not df.is_empty()


def test_write_ada3dp():
    df = ada3dp_to_polars(TEST_FILE_PATH)
    polars_to_ada3dp(df, OUTPUT_FILE_PATH)
    assert OUTPUT_FILE_PATH.exists()


def test_roundtrip_ada3dp():
    df = ada3dp_to_polars(TEST_FILE_PATH)
    polars_to_ada3dp(df, OUTPUT_FILE_PATH)
    df_roundtrip = ada3dp_to_polars(OUTPUT_FILE_PATH)
    assert df.equals(df_roundtrip)


if __name__ == "__main__":
    pytest.main()
