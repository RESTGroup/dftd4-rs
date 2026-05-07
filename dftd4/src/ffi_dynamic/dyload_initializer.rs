//! Library initializer implementation for dynamic loading.
//!
//! This file is generated automatically.

use super::*;
use libloading::{Library, Symbol};

unsafe fn get_symbol<'f, F>(libs: &'f [Library], name: &[u8]) -> Option<Symbol<'f, F>> {
    libs.iter().find_map(|lib| lib.get::<F>(name).ok())
}

impl DyLoadLib {
    pub unsafe fn new(libs: Vec<libloading::Library>, libs_path: Vec<String>) -> DyLoadLib {
        let mut result = DyLoadLib {
            __libraries: vec![],      // dummy, set later
            __libraries_path: vec![], // dummy, set later
            __error: None,
            dftd4_get_version: get_symbol(&libs, b"dftd4_get_version\0").map(|sym| *sym),
            dftd4_new_error: get_symbol(&libs, b"dftd4_new_error\0").map(|sym| *sym),
            dftd4_check_error: get_symbol(&libs, b"dftd4_check_error\0").map(|sym| *sym),
            dftd4_get_error: get_symbol(&libs, b"dftd4_get_error\0").map(|sym| *sym),
            dftd4_delete_error: get_symbol(&libs, b"dftd4_delete_error\0").map(|sym| *sym),
            dftd4_new_structure: get_symbol(&libs, b"dftd4_new_structure\0").map(|sym| *sym),
            dftd4_delete_structure: get_symbol(&libs, b"dftd4_delete_structure\0").map(|sym| *sym),
            dftd4_update_structure: get_symbol(&libs, b"dftd4_update_structure\0").map(|sym| *sym),
            dftd4_new_d4_model: get_symbol(&libs, b"dftd4_new_d4_model\0").map(|sym| *sym),
            dftd4_new_d4s_model: get_symbol(&libs, b"dftd4_new_d4s_model\0").map(|sym| *sym),
            dftd4_custom_d4_model: get_symbol(&libs, b"dftd4_custom_d4_model\0").map(|sym| *sym),
            dftd4_custom_d4s_model: get_symbol(&libs, b"dftd4_custom_d4s_model\0").map(|sym| *sym),
            dftd4_delete_model: get_symbol(&libs, b"dftd4_delete_model\0").map(|sym| *sym),
            dftd4_set_model_realspace_cutoff: get_symbol(
                &libs,
                b"dftd4_set_model_realspace_cutoff\0",
            )
            .map(|sym| *sym),
            dftd4_set_model_realspace_cutoff_smooth: get_symbol(
                &libs,
                b"dftd4_set_model_realspace_cutoff_smooth\0",
            )
            .map(|sym| *sym),
            dftd4_new_rational_damping: get_symbol(&libs, b"dftd4_new_rational_damping\0")
                .map(|sym| *sym),
            dftd4_load_rational_damping: get_symbol(&libs, b"dftd4_load_rational_damping\0")
                .map(|sym| *sym),
            dftd4_delete_param: get_symbol(&libs, b"dftd4_delete_param\0").map(|sym| *sym),
            dftd4_get_properties: get_symbol(&libs, b"dftd4_get_properties\0").map(|sym| *sym),
            dftd4_get_dispersion: get_symbol(&libs, b"dftd4_get_dispersion\0").map(|sym| *sym),
            dftd4_get_numerical_hessian: get_symbol(&libs, b"dftd4_get_numerical_hessian\0")
                .map(|sym| *sym),
            dftd4_get_pairwise_dispersion: get_symbol(&libs, b"dftd4_get_pairwise_dispersion\0")
                .map(|sym| *sym),
        };
        result.__libraries = libs;
        result.__libraries_path = libs_path;
        result
    }
}
