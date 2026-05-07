# Changelog

## v0.2.1 -- 2026-05-07

Behavior change:

- Update dynamic loading logic
    - now check CONDA_PREFIX first, then python in PATH, then python3 in PATH
    - now all python in PATH will try to be parsed, not the first one

## v0.2.0 -- 2026-04-27

Following changes applied by PR RESTGroup/dftd4-rs#1.

API breaking changes:

- Cargo features added. Now default changes to `dynamic_loading` and `api-v4_0` (previously static loading is a requirement).

Enhancements:

- Added dynamic loading of the DFTD4 library (optional, enabled by default).
- Added cargo features to select version of the DFTD4 API (v4.0 `api-v4_0` is default, which have the same C API to v4.1.0).
- Added support for toml/json parsing of DFTD4 parameters (toml is builtin, json is optional).

Behavior changes:

- Change CMake that will download v4.1 of original DFTD4 library.

## v0.1.0 -- 2025-04-17
