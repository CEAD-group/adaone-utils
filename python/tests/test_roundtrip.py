# %%
from adaone_utils import Toolpath
from pathlib import Path


def test_roundtrip():
    # Load input file
    input_file = Path(__file__).parent / "cone.ada3dp"
    toolpath = Toolpath.from_file(input_file)

    # Write to output file
    output_file = input_file.with_stem("cone.roundtrip")
    toolpath.to_file(output_file)

    # asert files are equal
    assert input_file.read_bytes() == output_file.read_bytes()
