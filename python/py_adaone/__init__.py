from ._internal import ada3dp_to_polars as _ada3dp_to_polars
from ._internal import polars_to_ada3dp as _polars_to_ada3dp
from pathlib import Path
import polars as pl

__all__ = ["ada3dp_to_polars", "polars_to_ada3dp"]


def ada3dp_to_polars(file_path: str | Path) -> pl.DataFrame:
    """
    Convert a *.ada3dp file to a Polars pl.DataFrame.

    Parameters:
    file_path (str | Path): The path to the *.ada3dp file.

    Returns:
    DataFrame: The converted Polars DataFrame.
    """
    return _ada3dp_to_polars(str(file_path))


def polars_to_ada3dp(df: pl.DataFrame, file_path: str | Path) -> None:
    """
    Convert a Polars DataFrame to a *.ada3dp file.

    Parameters:
    df (DataFrame): The Polars DataFrame to convert.
    file_path (str | Path): The path to save the *.ada3dp file.

    Raises:
    ValueError: If the DataFrame does not contain the required columns.
    """
    required_columns = [
        "position.x",
        "position.y",
        "position.z",
        "direction.x",
        "direction.y",
        "direction.z",
        "orientation.x",
        "orientation.y",
        "orientation.z",
        "orientation.w",
        "deposition",
        "speed",
        "speedTCP",
        "segment_type",
        "layerIndex",
        "processOnDelay",
        "processOffDelay",
        "startDelay",
        "equipmentID",
        "toolID",
        "materialID",
        "segmentID",
        "fans.num",
        "fans.speed",
        "userEvents.num",
        "externalAxes",
    ]

    missing_columns = [col for col in required_columns if col not in df.columns]
    if missing_columns:
        raise ValueError(f"Missing required columns: {', '.join(missing_columns)}")

    _polars_to_ada3dp(df, str(file_path))
