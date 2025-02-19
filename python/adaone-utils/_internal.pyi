from polars import DataFrame
from typing import Any, TypedDict, tuple

class Parameters(TypedDict):
    layerHeight: float
    depositionWidth: float
    posiAxis1Val: float
    posiAxis2Val: float
    posiAxis1Dynamic: bool
    posiAxis2Dynamic: bool
    pathPlanningStrategy: int

def ada3dp_to_adaone(file_path: str) -> tuple[DataFrame, Parameters]: ...
def adaone_to_ada3dp(df: DataFrame, parameters: Parameters, file_path: str) -> Any: ...
