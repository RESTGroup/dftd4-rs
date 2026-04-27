//! Test the parsing module functionality.
//!
//! Tests various ways to parse TOML input into DFTD4DampingParam.

#![allow(clippy::excessive_precision, unused_imports)]

use approx::assert_abs_diff_eq;
use dftd4::prelude::*;

// --- Use case 1: Method lookup ---
#[test]
fn test_method_lookup() {
    let param =
        dftd4_parse_damping_param_from_toml_f(r#"{version = "d4bj", method = "b3lyp"}"#).unwrap();
    assert_abs_diff_eq!(param.param.a1, 0.40868035);
    assert_abs_diff_eq!(param.param.s8, 2.02929367);
    assert_abs_diff_eq!(param.param.a2, 4.53807137);
    assert_eq!(param.doi.as_deref(), Some("10.1063/1.5090222"));
}

// --- Use case 2: Version without d4 prefix ---
#[test]
fn test_version_without_prefix() {
    let param =
        dftd4_parse_damping_param_from_toml_f(r#"{version = "bj", method = "b3lyp"}"#).unwrap();
    assert_abs_diff_eq!(param.param.a1, 0.40868035);
}

// --- Version is case-insensitive ---
#[test]
fn test_version_case_insensitive() {
    let param =
        dftd4_parse_damping_param_from_toml_f(r#"{version = "BJ", method = "b3lyp"}"#).unwrap();
    assert_abs_diff_eq!(param.param.a1, 0.40868035);
}

// --- Use case 3: Direct parameters ---
#[test]
fn test_direct_params() {
    let param = dftd4_parse_damping_param_from_toml_f(
        r#"{version = "d4bj", a1 = 0.40868035, s8 = 2.02929367, a2 = 4.53807137}"#,
    )
    .unwrap();
    assert_abs_diff_eq!(param.param.s6, 1.0);
    assert_abs_diff_eq!(param.param.a1, 0.40868035);
    assert_abs_diff_eq!(param.param.s8, 2.02929367);
    assert_abs_diff_eq!(param.param.a2, 4.53807137);
    assert_abs_diff_eq!(param.param.alp, 16.0);
    assert_abs_diff_eq!(param.param.s9, 1.0);
    assert!(param.doi.is_none());
}

// --- Use case 4: atm = false sets s9 = 0.0 ---
#[test]
fn test_atm_false() {
    let param =
        dftd4_parse_damping_param_from_toml_f(r#"{version = "bj", method = "b3lyp", atm = false}"#)
            .unwrap();
    assert_abs_diff_eq!(param.param.a1, 0.40868035);
    assert_abs_diff_eq!(param.param.s9, 0.0);
}

// --- atm = true is default (s9 = 1.0) ---
#[test]
fn test_atm_true_default() {
    let param =
        dftd4_parse_damping_param_from_toml_f(r#"{version = "bj", method = "b3lyp"}"#).unwrap();
    assert_abs_diff_eq!(param.param.s9, 1.0);
}

// --- Use case 5: Method lookup with parameter override ---
#[test]
fn test_method_override() {
    let param =
        dftd4_parse_damping_param_from_toml_f(r#"{version = "d4bj", method = "b3lyp", a1 = 0.5}"#)
            .unwrap();
    assert_abs_diff_eq!(param.param.a1, 0.5);
    assert_abs_diff_eq!(param.param.s8, 2.02929367); // unchanged from database
}

// --- Use case 6: Direct parameters with atm = false ---
#[test]
fn test_direct_params_atm_false() {
    let param = dftd4_parse_damping_param_from_toml_f(
        r#"{version = "d4bj", a1 = 0.40868035, s8 = 2.02929367, a2 = 4.53807137, atm = false}"#,
    )
    .unwrap();
    assert_abs_diff_eq!(param.param.s9, 0.0);
    assert_abs_diff_eq!(param.param.a1, 0.40868035);
}

// --- s9 takes precedence over atm when both specified ---
#[test]
fn test_s9_precedence_over_atm() {
    let param = dftd4_parse_damping_param_from_toml_f(
        r#"{version = "bj", method = "b3lyp", atm = false, s9 = 1.0}"#,
    )
    .unwrap();
    assert_abs_diff_eq!(param.param.s9, 1.0); // s9 takes precedence
}

// --- Use case 7: Unknown field should raise error ---
#[test]
fn test_unknown_field_error() {
    let result =
        dftd4_parse_damping_param_from_toml_f(r#"{version = "d4bj", method = "b3lyp", rs6 = 0.5}"#);
    assert!(result.is_err());
    match result.unwrap_err() {
        DFTD4Error::ParametersError(ref msg) => {
            assert!(msg.contains("rs6"), "Error should mention 'rs6': {msg}")
        },
        e => panic!("Expected ParametersError, got: {e:?}"),
    }
}

// --- Use case 8: Method name normalization (remove separators) ---
#[test]
fn test_method_name_normalization() {
    let param =
        dftd4_parse_damping_param_from_toml_f(r#"{version = "bj", method = "r2-scan"}"#).unwrap();
    assert_abs_diff_eq!(param.param.s8, 0.60187490);
}

#[test]
fn test_not_sufficient_parameters() {
    let result = dftd4_parse_damping_param_from_toml_f(r#"{version = "d4bj", a1 = 0.40868035}"#);
    assert!(result.is_err());
    match result.unwrap_err() {
        DFTD4Error::ParametersError(ref msg) => {
            assert!(msg.contains("missing"), "Error should mention missing parameters: {msg}")
        },
        e => panic!("Expected ParametersError, got: {e:?}"),
    }
}

#[test]
fn test_method_name_with_underscores() {
    let param =
        dftd4_parse_damping_param_from_toml_f(r#"{version = "bj", method = "r2_scan"}"#).unwrap();
    assert_abs_diff_eq!(param.param.s8, 0.60187490);
}

#[test]
fn test_missing_version_error() {
    let result = dftd4_parse_damping_param_from_toml_f(r#"{method = "b3lyp"}"#);
    assert!(result.is_err());
}

// --- dftd4_parse_damping_param_from_toml_f with standard TOML ---
#[test]
fn test_parse_from_toml_standard() {
    let input = r#"version = "bj"
method = "b3lyp""#;
    let param = dftd4_parse_damping_param_from_toml_f(input).unwrap();
    assert_abs_diff_eq!(param.param.a1, 0.40868035);
}

// --- version = "d4" also resolves to bj-eeq-atm (the default) ---
#[test]
fn test_version_d4_alias() {
    let param =
        dftd4_parse_damping_param_from_toml_f(r#"{version = "d4", method = "b3lyp"}"#).unwrap();
    assert_abs_diff_eq!(param.param.a1, 0.40868035);
    assert_abs_diff_eq!(param.param.s9, 1.0); // atm is default
}

// --- MBD variant ---
#[test]
fn test_mbd_variant() {
    let param =
        dftd4_parse_damping_param_from_toml_f(r#"{version = "mbd", method = "b3lyp"}"#).unwrap();
    assert_abs_diff_eq!(param.param.s8, 2.00246246);
    assert_abs_diff_eq!(param.param.a1, 0.40276191);
}

// --- TWO variant ---
#[test]
fn test_two_variant() {
    let param =
        dftd4_parse_damping_param_from_toml_f(r#"{version = "bj-eeq-two", method = "dftb_mio"}"#)
            .unwrap();
    assert_abs_diff_eq!(param.param.s8, 1.1948145);
}

fn main() {
    println!("Run with: cargo test --example test_parsing");
}
