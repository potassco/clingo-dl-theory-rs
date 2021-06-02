# clingo-dl-sys [![Build Status](https://github.com/potassco/clingo-dl-sys/workflows/CI%20Test/badge.svg)](https://github.com/potassco/clingo-dl-sys)[![Latest Version](https://img.shields.io/crates/v/clingo-dl-sys.svg)](https://crates.io/crates/clingo-dl-sys)[![Rust Documentation](https://docs.rs/clingo-dl-sys/badge.svg)](https://docs.rs/clingo-dl-sys)

Rust raw FFI bindings to the C API of [clingo-dl](https://github.com/potassco/clingo-dl) library.


You have to define the environment variable `CLINGO_DL_LIBRARY_PATH` for example:

```sh
export CLINGO_DL_LIBRARY_PATH=/scratch/miniconda3/envs/clingo-dl/lib
```
