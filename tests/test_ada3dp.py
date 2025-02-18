import py_adaone
import polars as pl


def test_sum_as_string():
    assert py_adaone.sum_as_string(1, 2) == "3"


def test_ada3dp_to_polars():
    parameters, df = py_adaone.ada3dp_to_polars("path/to/your/ada3dp/file.ada3dp")
    assert isinstance(parameters, bytes)
    assert isinstance(df, pl.DataFrame)


def test_polars_to_ada3dp():
    df = pl.DataFrame(
        {
            "position.x": [0.0],
            "position.y": [0.0],
            "position.z": [0.0],
            "direction.x": [0.0],
            "direction.y": [0.0],
            "direction.z": [0.0],
            "orientation.x": [0.0],
            "orientation.y": [0.0],
            "orientation.z": [0.0],
            "orientation.w": [1.0],
            "externalAxes": [[]],
            "deposition": [0.0],
            "speed": [0.0],
            "fans": [[]],
            "userEvents": [[]],
            "speedTCP": [0],
            "type": [0],
            "layerIndex": [0],
            "processOnDelay": [0.0],
            "processOffDelay": [0.0],
            "startDelay": [0.0],
            "equipmentID": [0],
            "toolID": [0],
            "materialID": [0],
            "segmentID": [0],
        }
    )
    parameters = ada3dp_pb2.Parameters()
    tool_path_data = py_adaone.polars_to_ada3dp(df, parameters.SerializeToString())
    assert isinstance(tool_path_data, bytes)
