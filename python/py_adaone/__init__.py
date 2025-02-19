from ._internal import ada3dp_to_polars as _ada3dp_to_polars
from ._internal import polars_to_ada3dp as _polars_to_ada3dp
from ._internal import PyParameters as _Parameters
from pathlib import Path
import polars as pl
from enum import Enum
from dataclasses import dataclass

__all__ = ["ada3dp_to_polars", "polars_to_ada3dp"]


class PathPlanningStrategy(Enum):
    PLANAR_HORIZONTAL = 0
    PLANAR_ANGLED = 1
    PLANAR_ALONG_GUIDE_CURVE = 2
    REVOLVED_SURFACE = 3
    RADIAL = 4
    NON_PLANAR_SURFACE = 5
    GEODESIC = 6
    CONICAL_FIELDS = 7
    RADIAL_360 = 8
    CLADDING = 9


@dataclass
class Parameters:
    layer_height: float
    path_planning_strategy: PathPlanningStrategy
    posi_axis1_val: float
    posi_axis2_val: float
    posi_axis1_dynamic: bool
    posi_axis2_dynamic: bool
    deposition_width: float

    def to_internal_parameters(self) -> _Parameters:
        internal_parameters = _Parameters(
            layer_height=self.layer_height,
            path_planning_strategy=self.path_planning_strategy.value,
            posi_axis1_val=self.posi_axis1_val,
            posi_axis2_val=self.posi_axis2_val,
            posi_axis1_dynamic=self.posi_axis1_dynamic,
            posi_axis2_dynamic=self.posi_axis2_dynamic,
            deposition_width=self.deposition_width,
        )
        return internal_parameters

    @classmethod
    def from_internal_parameters(cls, internal_parameters: _Parameters) -> "Parameters":
        return cls(
            layer_height=internal_parameters.layer_height,
            path_planning_strategy=PathPlanningStrategy(
                internal_parameters.path_planning_strategy
            ),
            posi_axis1_val=internal_parameters.posi_axis1_val,
            posi_axis2_val=internal_parameters.posi_axis2_val,
            posi_axis1_dynamic=internal_parameters.posi_axis1_dynamic,
            posi_axis2_dynamic=internal_parameters.posi_axis2_dynamic,
            deposition_width=internal_parameters.deposition_width,
        )


def ada3dp_to_polars(file_path: str | Path) -> tuple[pl.DataFrame, _Parameters]:
    """
    Convert a *.ada3dp file to a Polars pl.DataFrame and extract _parameters.

    _Parameters:
    file_path (str | Path): The path to the *.ada3dp file.

    Returns:
    (DataFrame, _Parameters): The converted Polars DataFrame and extracted _parameters.
    """
    df, internal_parameters = _ada3dp_to_polars(str(file_path))
    return df, Parameters.from_internal_parameters(internal_parameters)


def polars_to_ada3dp(
    df: pl.DataFrame, parameters: Parameters, file_path: str | Path
) -> None:
    """
    Convert a Polars DataFrame and _parameters to a *.ada3dp file.

    _Parameters:
    df (DataFrame): The Polars DataFrame to convert.
    _parameters (Dict[str, Any]): The _parameters to include.
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

    _polars_to_ada3dp(df, parameters.to_internal_parameters(), str(file_path))
