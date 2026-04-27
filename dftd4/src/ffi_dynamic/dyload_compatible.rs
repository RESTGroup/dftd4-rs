//! Compatible wrapper functions for dynamic loading.
//!
//! This file is generated automatically.
//!
//! Note: For dynamic loading, API version features are ignored.
//! All functions are available at runtime.

use super::*;
use core::ffi::{c_char, c_int};

pub unsafe fn dftd4_get_version() -> c_int {
    dyload_lib().dftd4_get_version.unwrap()()
}

pub unsafe fn dftd4_new_error() -> dftd4_error {
    dyload_lib().dftd4_new_error.unwrap()()
}

pub unsafe fn dftd4_check_error(arg1: dftd4_error) -> c_int {
    dyload_lib().dftd4_check_error.unwrap()(arg1)
}

pub unsafe fn dftd4_get_error(arg1: dftd4_error, arg2: *mut c_char, arg3: *const c_int) {
    dyload_lib().dftd4_get_error.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dftd4_delete_error(arg1: *mut dftd4_error) {
    dyload_lib().dftd4_delete_error.unwrap()(arg1)
}

pub unsafe fn dftd4_new_structure(
    arg1: dftd4_error,
    arg2: c_int,
    arg3: *const c_int,
    arg4: *const f64,
    arg5: *const f64,
    arg6: *const f64,
    arg7: *const bool,
) -> dftd4_structure {
    dyload_lib().dftd4_new_structure.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dftd4_delete_structure(arg1: *mut dftd4_structure) {
    dyload_lib().dftd4_delete_structure.unwrap()(arg1)
}

pub unsafe fn dftd4_update_structure(
    arg1: dftd4_error,
    arg2: dftd4_structure,
    arg3: *const f64,
    arg4: *const f64,
) {
    dyload_lib().dftd4_update_structure.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn dftd4_new_d4_model(arg1: dftd4_error, arg2: dftd4_structure) -> dftd4_model {
    dyload_lib().dftd4_new_d4_model.unwrap()(arg1, arg2)
}

pub unsafe fn dftd4_new_d4s_model(arg1: dftd4_error, arg2: dftd4_structure) -> dftd4_model {
    dyload_lib().dftd4_new_d4s_model.unwrap()(arg1, arg2)
}

pub unsafe fn dftd4_custom_d4_model(
    arg1: dftd4_error,
    arg2: dftd4_structure,
    arg3: f64,
    arg4: f64,
    arg5: f64,
) -> dftd4_model {
    dyload_lib().dftd4_custom_d4_model.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dftd4_custom_d4s_model(
    arg1: dftd4_error,
    arg2: dftd4_structure,
    arg3: f64,
    arg4: f64,
) -> dftd4_model {
    dyload_lib().dftd4_custom_d4s_model.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn dftd4_delete_model(arg1: *mut dftd4_model) {
    dyload_lib().dftd4_delete_model.unwrap()(arg1)
}

pub unsafe fn dftd4_new_rational_damping(
    arg1: dftd4_error,
    arg2: f64,
    arg3: f64,
    arg4: f64,
    arg5: f64,
    arg6: f64,
    arg7: f64,
) -> dftd4_param {
    dyload_lib().dftd4_new_rational_damping.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dftd4_load_rational_damping(
    arg1: dftd4_error,
    arg2: *mut c_char,
    arg3: bool,
) -> dftd4_param {
    dyload_lib().dftd4_load_rational_damping.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dftd4_delete_param(arg1: *mut dftd4_param) {
    dyload_lib().dftd4_delete_param.unwrap()(arg1)
}

pub unsafe fn dftd4_get_properties(
    arg1: dftd4_error,
    arg2: dftd4_structure,
    arg3: dftd4_model,
    arg4: *mut f64,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut f64,
) {
    dyload_lib().dftd4_get_properties.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dftd4_get_dispersion(
    arg1: dftd4_error,
    arg2: dftd4_structure,
    arg3: dftd4_model,
    arg4: dftd4_param,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut f64,
) {
    dyload_lib().dftd4_get_dispersion.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dftd4_get_numerical_hessian(
    arg1: dftd4_error,
    arg2: dftd4_structure,
    arg3: dftd4_model,
    arg4: dftd4_param,
    arg5: *mut f64,
) {
    dyload_lib().dftd4_get_numerical_hessian.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dftd4_get_pairwise_dispersion(
    arg1: dftd4_error,
    arg2: dftd4_structure,
    arg3: dftd4_model,
    arg4: dftd4_param,
    arg5: *mut f64,
    arg6: *mut f64,
) {
    dyload_lib().dftd4_get_pairwise_dispersion.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}
