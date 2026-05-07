# Agent instruction of DFTD4 Rust FFI and Wrapper

## Notes to human developers

You should also create a file `CLAUDE.local.md` to place local resources:
- `DFTD4_REPO_PATH`: local of original dftd4 repository. The source code can help you understand how dftd4 works.

## DFTD4 original library

General rules
- This repository should live at `DFTD4_REPO_PATH`, which is defined in `CLAUDE.local.md`.
- **This repository should not be modified**, unless you are going to checkout specific tags (versions) of dftd4.

Important files for FFI and wrapper development:
- `include/dftd4.h`: the headers. Note that these files are also copied to this project under `dftd4/header` folder.
- `python/dftd4`: the python wrapper of dftd4. We should at least implement all major features of the certain wrapper:
  - `interface.py`, corresponding to this project `dftd4/src/interface.rs`.
  - `parameters.py`, corresponding to this project `dftd4/src/parameters.rs`.
  - Make sure the functionalities are tested. We use `dftd4/examples/test_interface.rs` corresponding to `test_interface.py` in the original wrapper for testing.
- `assets/parameters.toml`: the parameters file, which should be copied to `dftd4/src/parameters.toml` in this project.

## The additional feature in this crate

- We support toml parsing of DFTD4 parameters. The related code is at `/dftd4/src/parsing.rs`. The related test is at `dftd4/examples/test_parsing.rs`.
- We support dynamic loading of the DFTD4 library.
- We use tags such as `api-v3_5` to reflect the API version of DFTD4 we are using.

## Naming convention

- For functions and structs that will be exposed to users, add prefix `dftd4_` for general functions, and `DFTD4` for structs.
- If some function is to be fallible, we can add suffix `_f` (`fn <func>_f -> Result<_, DFTD4Error>`).

## Header handling

We use bindgen (python script at `scripts/generate_ffi.py`) to generate Rust bindings for the C header files. **Not modify the generated files directly**.

Exception is `ffi_dynamic/mod.rs`. This file can be manually modified.
