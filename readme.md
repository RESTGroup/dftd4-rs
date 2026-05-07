# dftd4 FFI bindings

This project contains dftd4 FFI bindings, wrapper and build-from-source.

Current binding of dftd4: [![v4.1.0](https://img.shields.io/github/v/release/dftd4/dftd4?filter=v4.1.0)](https://github.com/dftd4/dftd4/releases/v4.1.0)

Source code of dftd4 is available on [github](https://github.com/dftd4/dftd4).

This crate is not official bindgen project. It is originally intended to potentially serve rust electronic structure toolkit [REST](https://gitee.com/RESTGroup/rest).

## Crate `dftd4`

This crate contains dftd4 FFI bindings and wrapper.

| Resources | Badges |
|--|--|
| Crate | [![Crate](https://img.shields.io/crates/v/dftd4.svg)](https://crates.io/crates/dftd4) |
| API Document | [![API Documentation](https://docs.rs/dftd4/badge.svg)](https://docs.rs/dftd4) |
| FFI Binding | [![v4.1.0](https://img.shields.io/github/v/release/dftd4/dftd4?filter=v4.1.0)](https://github.com/dftd4/dftd4/releases/v4.1.0) |

### Dynamic loading guide

This crate will default to dynamic loading (unless unset cargo feature `dynamic_loading`). Please refer to module [`ffi_dynamic`](https://docs.rs/dftd4/latest/dftd4/ffi_dynamic/index.html) for more details of dynamic loading.

### Example: B97m with D4

For example, full code for computing B97m dispersion energy with D4:

```rust
use dftd4::prelude::*;

// atom indices
let numbers = vec![1, 6, 6, 6, 6, 6, 6, 1, 1, 1, 1, 1, 35, 6, 9, 9, 9];
// geometry in angstrom
#[rustfmt::skip]
let positions = vec![
    0.002144194,   0.361043475,   0.029799709,
    0.015020592,   0.274789738,   1.107648016,
    1.227632658,   0.296655040,   1.794629427,
    1.243958826,   0.183702791,   3.183703934,
    0.047958213,   0.048915002,   3.886484583,
   -1.165135654,   0.026954348,   3.200213281,
   -1.181832083,   0.139828643,   1.810376587,
    2.155807907,   0.399177037,   1.249441585,
    2.184979344,   0.198598553,   3.716170761,
    0.060934662,  -0.040672756,   4.964014252,
   -2.093220602,  -0.078628959,   3.745125056,
   -2.122845437,   0.123257119,   1.277645797,
   -0.268325907,  -3.194209024,   1.994458950,
    0.049999933,  -5.089197474,   1.929391171,
    0.078949601,  -5.512441335,   0.671851563,
    1.211983937,  -5.383996300,   2.498664281,
   -0.909987405,  -5.743747328,   2.570721738,
];
// convert angstrom to bohr
let positions = positions.iter().map(|&x| x / 0.52917721067).collect::<Vec<f64>>();
// generate DFTD4 model
let model = DFTD4Model::new(&numbers, &positions, None, None, None);
// retrive the DFTD4 parameters
let param = DFTD4Param::load_rational_damping("b97m", true);
// obtain the dispersion energy and gradient, without sigma
let (energy, gradient, _) = model.get_dispersion(&param, true).into();
let gradient = gradient.unwrap();

println!("Dispersion energy: {}", energy);
let energy_ref = -0.025765015532807658;
assert!((energy - energy_ref).abs() < 1e-9);
println!("Dispersion gradient:");
gradient.chunks(3).for_each(|chunk| println!("{:16.9?}", chunk));
```

### Example: Custom parameters by toml

```rust
use dftd4::prelude::*;

// Use custom parameters by toml string
// Note: version = "d4", "d4bj", "bj", or "atm" all resolve to bj-eeq-atm (the default)
// The version field is optional — omitting it also defaults to bj-eeq-atm.
let input = r#"{version = "d4bj", a1 = 0.40868035, s8 = 2.02929367, a2 = 4.53807137, atm = false}"#;
// You can also use the following input to specify B3LYP-D4 parameters
// let input = r#"{version = "d4bj", method = "b3lyp"}"#;
// Or simply omit version (defaults to bj-eeq-atm):
// let input = r#"{method = "b3lyp"}"#;
// toml parameter type
let damping_param = dftd4_parse_damping_param_from_toml(input);
// FFI parameter type
let dftd4_param = damping_param.new_param();

let atom_numbers = vec![8, 1, 1];
// coordinates in bohr
#[rustfmt::skip]
let coordinates = vec![
    0.000000, 0.000000, 0.000000,
    0.000000, 0.000000, 1.807355,
    1.807355, 0.000000, -0.452500,
];
let model = DFTD4Model::new(&atom_numbers, &coordinates, None, None, None);
let res = model.get_dispersion(&dftd4_param, false);
let eng = res.energy;
println!("Dispersion energy: {eng}");
```

### Cargo features of `dftd4`

Default cargo features of `dftd4` are:
- **`api-v4_0`**: Corresponding to the original dftd4 [v4.0](https://github.com/dftd4/dftd4/releases/tag/v4.0.0). This will enable rational damping (BJ), custom D4 model, pairwise dispersion, properties, and numerical hessian. D4S model also included in `api-v4_0`.
- **`dynamic_loading`**: This will enable dynamic loading of `libdftd4` library, which can be more flexible for users who do not want to perform static linking. Please place `libdftd4.so` in `LD_LIBRARY_PATH` (for macos, place `libdftd4.dylib` in `DYLD_LIBRARY_PATH`), or make dftd4 available in your python environment, and function symbols will be loaded at runtime.

Other cargo features of `dftd4` are:
- **`api-v3_0`** through **`api-v4_0`**: Versioned API features (cumulative). Each version enables all functions introduced in that version. Note: dynamic loading ignores API version features — all functions are available at runtime.
- **`api-v4_0`**: Enables D4S dispersion model support.

## Installation guide and Crate `dftd4-src`

**This crate is only useful for static loading**: If you enabled cargo feature `dynamic_loading` (which is enabled by default), you just need to place `libdftd4.so` in `LD_LIBRARY_PATH`, or make dftd4 available in your python environment. The library will be loaded at runtime, so you do not need to perform any static loading.

If you need to link `dftd4` library by static loading (either static linking by `libdftd4.a` or dynamic linking by `libdftd4.so`), proceed to the following instructions.

| Resources | Badges |
|--|--|
| Crate | [![Crate](https://img.shields.io/crates/v/dftd4-src.svg)](https://crates.io/crates/dftd4-src) |

To use crate `dftd4` in rust, you may need to perform some configuration to properly link `libdftd4.so` into your own program.

### Install `dftd4`

Please refer to original [github](https://github.com/dftd4/dftd4) repository for more instructions.

The easiest way is install from conda/mamba, and you can retrive the shared/static library therein.

### Manually link `dftd4` into your project

Similar to other projects, after library search path properly defined
```rust,ignore
println!("cargo:rustc-link-search=native={}", path);
```
you may link `dftd4` and `mctc-lib`, `multicharge` by cargo instructions **in your own project**:
```rust,ignore
// following code is for static linking
println!("cargo:rustc-link-lib=static=dftd4");
println!("cargo:rustc-link-lib=static=mctc-lib");
println!("cargo:rustc-link-lib=static=multicharge");
// following code is for dynamic linking
println!("cargo:rustc-link-lib=dftd4");
println!("cargo:rustc-link-lib=mctc-lib");
println!("cargo:rustc-link-lib=multicharge");
```
It should be noted that, for static linking, you may also need to dynamic link Fortran, OpenMP, BLAS, LAPACK libraries (for the library installed by conda or mamba, it is usually `gfortran` and `gomp`, `blas`, `lapack`).

### Link `dftd4` by crate `dftd4-src`

You can also link `dftd4` by crate `dftd4-src`.

First, **in your own project**'s `lib.rs` or `main.rs`, you need to add a line for explicitly importing this library:
```rust,ignore
extern crate dftd4_src;
```

If you have compiled `dftd4` library, make sure path of it (together with `mctc-lib`) is either in
- `DFTD4_DIR`
- `REST_EXT_DIR`
- `LD_LIBRARY_PATH`
- or in other common system library paths.

If you have not compiled `dftd4` library, you may try out cargo feature `build_from_source`, but that can also cause trobule when distributing your program binary, so use with caution.

### Cargo features of `dftd4-src`

- **`build_from_source`**: This will use CMake and meson, and pull code from github to first perform build for dftd4. Though this option can be developer-friendly (you do not need to perform any other configurations to make program compile and run by cargo), `build_from_source` does not provide customized compilation.

    CMake configurable variables (can be defined as environment variables):
    - `DFTD4_SRC`: git repository source directory or URL;
    - `DFTD4_VER`: version of DFT-D4 (default v4.1.0);

- **`static`**: This will link static libary instead of dynamic one. Please note that 1. static linking may require additional Fortran and OpenMP linking, which is not provided in this crate; 2. staticly linking LGPL-3.0 license may require your project to be GPL-3.0.

## License

This repository is licensed under LGPL-3.0, the same to dftd4.

Some parts of this project is derivative work of original library [dftd4](https://github.com/dftd4/dftd4), and contains some source code (headers, toml parameters) and AI-translated/generated code.
