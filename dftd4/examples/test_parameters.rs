//! Test the parameters module functionality.
//!
//! Translated from s-dftd4's python/dftd4/test_parameters.py.

#![allow(clippy::excessive_precision, unused_imports)]

use approx::assert_abs_diff_eq;
use dftd4::prelude::*;

#[test]
fn test_list_methods() {
    let methods = dftd4_list_methods();
    assert!(methods.contains(&"b3lyp".to_string()));
    assert!(methods.contains(&"pbe0".to_string()));
    assert!(methods.len() > 50);
}

#[test]
fn test_get_b3lyp() {
    let param = dftd4_get_damping_param_f("b3lyp", "bj-eeq-atm").unwrap();
    assert_abs_diff_eq!(param.param.s6, 1.0);
    assert_abs_diff_eq!(param.param.s9, 1.0);
    assert_abs_diff_eq!(param.param.alp, 16.0);
    assert_abs_diff_eq!(param.param.a1, 0.40868035);
    assert_abs_diff_eq!(param.param.s8, 2.02929367);
    assert_abs_diff_eq!(param.param.a2, 4.53807137);
    assert_eq!(param.doi.as_deref(), Some("10.1063/1.5090222"));
}

#[test]
fn test_get_r2scan() {
    let param = dftd4_get_damping_param_f("r2scan", "bj-eeq-atm").unwrap();
    assert_abs_diff_eq!(param.param.s8, 0.60187490);
    assert_abs_diff_eq!(param.param.a1, 0.51559235);
    assert_abs_diff_eq!(param.param.a2, 5.77342911);
    assert_eq!(param.doi.as_deref(), Some("10.1063/5.0041008"));
}

#[test]
fn test_get_pbe_mbd() {
    let param = dftd4_get_damping_param_f("pbe", "bj-eeq-mbd").unwrap();
    assert_abs_diff_eq!(param.param.s8, 0.99924614);
    assert_abs_diff_eq!(param.param.a1, 0.38142528);
    assert_abs_diff_eq!(param.param.a2, 4.81839284);
}

#[test]
fn test_get_b2plyp() {
    let param = dftd4_get_damping_param_f("b2plyp", "bj-eeq-atm").unwrap();
    assert_abs_diff_eq!(param.param.s6, 0.6400);
    assert_abs_diff_eq!(param.param.s8, 1.16888646);
    assert_abs_diff_eq!(param.param.a1, 0.44154604);
    assert_abs_diff_eq!(param.param.a2, 4.73114642);
}

#[test]
fn test_version_alias() {
    // "d4bj" and "bj" should both resolve to "bj-eeq-atm"
    let param1 = dftd4_get_damping_param_f("b3lyp", "d4bj").unwrap();
    let param2 = dftd4_get_damping_param_f("b3lyp", "bj").unwrap();
    let param3 = dftd4_get_damping_param_f("b3lyp", "bj-eeq-atm").unwrap();
    assert_abs_diff_eq!(param1.param.s8, param2.param.s8);
    assert_abs_diff_eq!(param2.param.s8, param3.param.s8);
}

#[test]
fn test_method_not_found() {
    let result = dftd4_get_damping_param_f("nonexistent", "bj-eeq-atm");
    assert!(result.is_err());
    match &result.unwrap_err() {
        DFTD4Error::ParametersError(msg) => assert!(msg.contains("nonexistent")),
        _ => panic!("Expected ParametersError"),
    }
}

#[test]
fn test_variant_not_found() {
    let result = dftd4_get_damping_param_f("am05", "bj-eeq-mbd");
    assert!(result.is_err());
}

#[test]
fn test_all_parameters() {
    let params = dftd4_get_all_damping_params_f("bj-eeq-atm").unwrap();
    assert!(params.contains_key("b3lyp"));
    assert!(params.contains_key("b2plyp"));
    assert!(params.contains_key("pbe0"));
    assert!(params.len() > 50);
}

fn main() {
    println!("Run with: cargo test --example test_parameters");
}
