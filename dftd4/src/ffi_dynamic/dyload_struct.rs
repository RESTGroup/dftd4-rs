//! Library struct definition for dynamic loading.
//!
//! This file is generated automatically.
//!
//! Note: For dynamic loading, API version features are ignored.
//! All functions are available at runtime. Runtime panic occurs if a function
//! is not found in the loaded library.

use super::*;
use core::ffi::{c_char, c_int};

pub struct DyLoadLib {
    pub __libraries: Vec<libloading::Library>,
    pub __libraries_path: Vec<String>,
    pub __error: Option<String>,
    pub dftd4_get_version: Option<unsafe extern "C" fn() -> c_int>,
    pub dftd4_new_error: Option<unsafe extern "C" fn() -> dftd4_error>,
    pub dftd4_check_error: Option<unsafe extern "C" fn(arg1: dftd4_error) -> c_int>,
    pub dftd4_get_error:
        Option<unsafe extern "C" fn(arg1: dftd4_error, arg2: *mut c_char, arg3: *const c_int)>,
    pub dftd4_delete_error: Option<unsafe extern "C" fn(arg1: *mut dftd4_error)>,
    pub dftd4_new_structure: Option<
        unsafe extern "C" fn(
            arg1: dftd4_error,
            arg2: c_int,
            arg3: *const c_int,
            arg4: *const f64,
            arg5: *const f64,
            arg6: *const f64,
            arg7: *const bool,
        ) -> dftd4_structure,
    >,
    pub dftd4_delete_structure: Option<unsafe extern "C" fn(arg1: *mut dftd4_structure)>,
    pub dftd4_update_structure: Option<
        unsafe extern "C" fn(
            arg1: dftd4_error,
            arg2: dftd4_structure,
            arg3: *const f64,
            arg4: *const f64,
        ),
    >,
    pub dftd4_new_d4_model:
        Option<unsafe extern "C" fn(arg1: dftd4_error, arg2: dftd4_structure) -> dftd4_model>,
    pub dftd4_new_d4s_model:
        Option<unsafe extern "C" fn(arg1: dftd4_error, arg2: dftd4_structure) -> dftd4_model>,
    pub dftd4_custom_d4_model: Option<
        unsafe extern "C" fn(
            arg1: dftd4_error,
            arg2: dftd4_structure,
            arg3: f64,
            arg4: f64,
            arg5: f64,
        ) -> dftd4_model,
    >,
    pub dftd4_custom_d4s_model: Option<
        unsafe extern "C" fn(
            arg1: dftd4_error,
            arg2: dftd4_structure,
            arg3: f64,
            arg4: f64,
        ) -> dftd4_model,
    >,
    pub dftd4_delete_model: Option<unsafe extern "C" fn(arg1: *mut dftd4_model)>,
    pub dftd4_new_rational_damping: Option<
        unsafe extern "C" fn(
            arg1: dftd4_error,
            arg2: f64,
            arg3: f64,
            arg4: f64,
            arg5: f64,
            arg6: f64,
            arg7: f64,
        ) -> dftd4_param,
    >,
    pub dftd4_load_rational_damping: Option<
        unsafe extern "C" fn(arg1: dftd4_error, arg2: *mut c_char, arg3: bool) -> dftd4_param,
    >,
    pub dftd4_delete_param: Option<unsafe extern "C" fn(arg1: *mut dftd4_param)>,
    pub dftd4_get_properties: Option<
        unsafe extern "C" fn(
            arg1: dftd4_error,
            arg2: dftd4_structure,
            arg3: dftd4_model,
            arg4: *mut f64,
            arg5: *mut f64,
            arg6: *mut f64,
            arg7: *mut f64,
        ),
    >,
    pub dftd4_get_dispersion: Option<
        unsafe extern "C" fn(
            arg1: dftd4_error,
            arg2: dftd4_structure,
            arg3: dftd4_model,
            arg4: dftd4_param,
            arg5: *mut f64,
            arg6: *mut f64,
            arg7: *mut f64,
        ),
    >,
    pub dftd4_get_numerical_hessian: Option<
        unsafe extern "C" fn(
            arg1: dftd4_error,
            arg2: dftd4_structure,
            arg3: dftd4_model,
            arg4: dftd4_param,
            arg5: *mut f64,
        ),
    >,
    pub dftd4_get_pairwise_dispersion: Option<
        unsafe extern "C" fn(
            arg1: dftd4_error,
            arg2: dftd4_structure,
            arg3: dftd4_model,
            arg4: dftd4_param,
            arg5: *mut f64,
            arg6: *mut f64,
        ),
    >,
}
