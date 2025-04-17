# dftd4 FFI bindings

This project contains dftd4 FFI bindings, wrapper and build-from-source.

Current binding of dftd4: [bb9bd94](https://github.com/dftd4/dftd4/commit/bb9bd9459566a4199ea1a73488008c27ad029ecb) (after [![v3.7.0](https://img.shields.io/github/v/release/dftd4/dftd4)](https://github.com/dftd4/dftd4/releases/v3.7.0))

Source code of dftd4 is available on [github](https://github.com/dftd4/dftd4).

This crate is not official bindgen project. It is originally intended to potentially serve rust electronic structure toolkit [REST](https://gitee.com/RESTGroup/rest).

## Crate `dftd4`

This crate contains dftd4 FFI bindings and wrapper.

| Resources | Badges |
|--|--|
| Crate | [![Crate](https://img.shields.io/crates/v/dftd4.svg)](https://crates.io/crates/dftd4) |
| API Document | [![API Documentation](https://docs.rs/dftd4/badge.svg)](https://docs.rs/dftd4) |
| FFI Binding | [bb9bd94](https://github.com/dftd4/dftd4/commit/bb9bd9459566a4199ea1a73488008c27ad029ecb) after [![v3.7.0](https://img.shields.io/github/v/release/dftd4/dftd4)](https://github.com/dftd4/dftd4/releases/v3.7.0) |

### Cargo features of `dftd4`

- **`d4s`**: Support of D4S dispersion model. Please note that this is not available in latest stable release of dftd4 (at the time writing this readme, is v3.7.0). Unless you build dftd4 from git repository, you may not use this feature (especially installed dftd4 from conda or similar).

### Example: B97m with D4

For example, full code for computing B97m dispersion energy with D4:

```rust
fn main() {
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
        1.211983937,  -5.383996300,   2.498664481,
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
}
```

## Installation guide and Crate `dftd4-src`

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
- **`static`**: This will link static libary instead of dynamic one. Please note that 1. static linking may require additional Fortran and OpenMP linking, which is not provided in this crate; 2. staticly linking LGPL-3.0 license may require your project to be GPL-3.0.

## License

This repository is licensed under LGPL-3.0, the same to dftd4.

Some parts of this project is derivative work of original library [dftd4](https://github.com/dftd4/dftd4), and contains some source code (headers) and AI-translated code (pytest).
