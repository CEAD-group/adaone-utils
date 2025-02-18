# %%
import py_adaone
import polars as pl
from pathlib import Path

input_file = Path(__file__).parent / "Protobuff_doubleBeads_backside.ada3dp"

df = py_adaone.ada3dp_to_polars(str(input_file))
df
# %%
py_adaone.polars_to_ada3dp(df,str(input_file.with_stem("test")))
# %%

# %%
%%timeit
df = py_adaone.ada3dp_to_polars(str(input_file))