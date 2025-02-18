# py-adaone

A Python interface to *.ada3dp files, the protobuf based AdaOne toolpath format.

## Description

`py-adaone` provides a Python interface to read and write *.ada3dp files using Polars DataFrames. It leverages Rust for performance and uses the `pyo3` and `polars` libraries.

## Installation

To install `py-adaone`, you need to have Rust and Maturin installed. You can install Maturin using pip:

```sh
pip install maturin
```

Then, you can build and install the package using Maturin:

```sh
maturin develop
```

## Usage

### Reading a *.ada3dp file

```python
from py_adaone import ada3dp_to_polars
from pathlib import Path

file_path = Path("path/to/your/file.ada3dp")
df = ada3dp_to_polars(file_path)
print(df)
```

### Writing a *.ada3dp file

```python
from py_adaone import polars_to_ada3dp
from polars import DataFrame
from pathlib import Path

# Assuming df is your DataFrame
output_path = Path("path/to/your/output.ada3dp")
polars_to_ada3dp(df, output_path)
```

## Development

To run tests, use pytest:

```sh
pytest
```

## License

This project is licensed under the MIT License.