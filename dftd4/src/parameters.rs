//! DFTD4 damping parameter database loaded from TOML.
//!
//! This module provides access to DFT-D4 damping parameters by reading from an
//! embedded TOML database. Unlike the FFI-based parameter loading
//! (`DFTD4Param`), this module exposes the actual parameter values and allows
//! inspection of damping parameters for various XC functionals.

use crate::interface::*;
use serde::Deserialize;
use std::collections::HashMap;
use toml::Table;

// Embed TOML at compile time
const PARAMETERS_TOML: &str = include_str!("parameters.toml");

/* #region TOML data structures */

/// D4 variants under a method (e.g., d4.bj-eeq-atm under [parameter.b3lyp]).
/// TOML creates nested structure: parameter.b3lyp.d4.bj-eeq-atm
#[derive(Debug, Clone, Deserialize, Default)]
struct D4Variants {
    d4: D4MethodParams,
}

/// D4 parameters for a specific method.
/// Each variant is stored as a raw TOML table for direct deserialization.
#[derive(Debug, Clone, Deserialize, Default)]
struct D4MethodParams {
    #[serde(rename = "bj-eeq-two")]
    bj_eeq_two: Option<Table>,
    #[serde(rename = "bj-eeq-atm")]
    bj_eeq_atm: Option<Table>,
    #[serde(rename = "bj-eeq-mbd")]
    bj_eeq_mbd: Option<Table>,
}

/// Full TOML structure.
#[derive(Debug, Clone, Deserialize)]
struct ParameterDataBase {
    default: DefaultSection,
    parameter: HashMap<String, D4Variants>,
}

/// Default section with default damping types and base parameters.
#[derive(Debug, Clone, Deserialize)]
struct DefaultSection {
    #[allow(dead_code)]
    d4: Vec<String>,
    parameter: DefaultParameterSection,
}

/// Nested section for default parameters under [default.parameter].
#[derive(Debug, Clone, Deserialize)]
struct DefaultParameterSection {
    d4: D4DefaultParams,
}

/// Default D4 parameters for each damping variant.
/// Each variant is stored as a raw TOML table.
#[derive(Debug, Clone, Deserialize)]
struct D4DefaultParams {
    #[serde(rename = "bj-eeq-two")]
    bj_eeq_two: Table,
    #[serde(rename = "bj-eeq-atm")]
    bj_eeq_atm: Table,
    #[serde(rename = "bj-eeq-mbd")]
    bj_eeq_mbd: Table,
}

/* #endregion */

/* #region Public parameter structs */

/// Damping parameters with actual values exposed, plus optional metadata like
/// DOI. This wraps the rational damping param struct and provides DOI
/// reference.
#[derive(Debug, Clone)]
pub struct DFTD4DampingParam {
    /// The actual damping parameters
    pub param: DFTD4RationalDampingParam,
    /// Reference DOI if available
    pub doi: Option<String>,
}

/* #endregion */

/* #region DFTD4ParamAPI implementation */

impl DFTD4ParamAPI for DFTD4DampingParam {
    fn new_param_f(self) -> Result<DFTD4Param, DFTD4Error> {
        self.param.new_param_f()
    }
}

/* #endregion */

/* #region Public API functions */

/// Load the parameter database from embedded TOML.
fn load_data_base() -> Result<ParameterDataBase, DFTD4Error> {
    toml::from_str(PARAMETERS_TOML)
        .map_err(|e| DFTD4Error::ParametersError(format!("TOML parsing error: {}", e)))
}

/// Look up a method in the parameter database using normalized key matching.
/// The input method name is normalized, and keys in the database are also
/// normalized for comparison (e.g., "dftb_mio" matches input "dftb-mio").
fn lookup_method<'a>(
    db: &'a ParameterDataBase,
    method: &str,
) -> Result<(String, &'a D4Variants), DFTD4Error> {
    let method_lower = normalize_method(method);
    // First try exact match (most common case)
    if let Some(entry) = db.parameter.get(&method_lower) {
        return Ok((method_lower, entry));
    }
    // Fall back to normalized key comparison
    for (key, entry) in &db.parameter {
        if normalize_method(key) == method_lower {
            return Ok((key.clone(), entry));
        }
    }
    Err(DFTD4Error::ParametersError(format!("Method '{}' not found in database", method)))
}

/// Get damping parameters for a specific method and variant.
///
/// # Arguments
///
/// - `method`: XC functional name (e.g., "b3lyp", "pbe0", "r2scan")
/// - `version`: DFT-D4 variant ("bj-eeq-two", "bj-eeq-atm", "bj-eeq-mbd", or
///   aliases "d4bj", "bj", "two", "atm", "mbd")
///
/// # Returns
///
/// A `DFTD4DampingParam` containing the damping parameters and DOI reference.
pub fn dftd4_get_damping_param(method: &str, version: &str) -> DFTD4DampingParam {
    dftd4_get_damping_param_f(method, version).unwrap()
}

pub fn dftd4_get_damping_param_f(
    method: &str,
    version: &str,
) -> Result<DFTD4DampingParam, DFTD4Error> {
    let db = load_data_base()?;
    let version_normalized = normalize_version(version);

    // Get method entry using normalized lookup
    let (_, method_entry) = lookup_method(&db, method)?;

    // Get variant entry
    let (entry_raw, default_entry) = get_variant_entry(method_entry, &version_normalized, &db)?;

    // Merge with defaults
    let merged = merge_tables(&entry_raw, &default_entry);

    // Convert to public struct
    convert_to_damping_param(&merged)
}

/// Get the merged TOML table for a method and variant (method values override
/// defaults).
///
/// This is useful for programmatic parameter overrides before final
/// deserialization.
#[allow(dead_code)]
pub(crate) fn get_merged_param_table(method: &str, version: &str) -> Result<Table, DFTD4Error> {
    let db = load_data_base()?;
    let version_normalized = normalize_version(version);

    let (_, method_entry) = lookup_method(&db, method)?;

    let (entry_raw, default_entry) = get_variant_entry(method_entry, &version_normalized, &db)?;
    Ok(merge_tables(&entry_raw, &default_entry))
}

/// Normalize method name: lowercase and remove separators (`-`, `_`, spaces).
pub(crate) fn normalize_method(method: &str) -> String {
    method.to_lowercase().replace(['-', '_', ' '], "")
}

/// Get the default parameter table for a variant.
#[allow(dead_code)]
pub(crate) fn get_default_param_table(version: &str) -> Result<Table, DFTD4Error> {
    let db = load_data_base()?;
    let version_normalized = normalize_version(version);
    let (_, default_entry) = get_variant_entry_for_defaults(&version_normalized, &db)?;
    Ok(default_entry)
}

/// Get all damping parameters for all methods for a given variant.
///
/// # Arguments
///
/// - `version`: DFT-D4 variant ("bj-eeq-two", "bj-eeq-atm", "bj-eeq-mbd", or
///   aliases)
///
/// # Returns
///
/// A HashMap mapping method names to their damping parameters.
pub fn dftd4_get_all_damping_params(version: &str) -> HashMap<String, DFTD4DampingParam> {
    dftd4_get_all_damping_params_f(version).unwrap()
}

pub fn dftd4_get_all_damping_params_f(
    version: &str,
) -> Result<HashMap<String, DFTD4DampingParam>, DFTD4Error> {
    let db = load_data_base()?;
    let version_normalized = normalize_version(version);
    let (_, default_entry) = get_variant_entry_for_defaults(&version_normalized, &db)?;

    let mut result = HashMap::new();
    for (method, method_entry) in &db.parameter {
        if let Ok((entry_raw, _)) = get_variant_entry(method_entry, &version_normalized, &db) {
            let merged = merge_tables(&entry_raw, &default_entry);
            if let Ok(param) = convert_to_damping_param(&merged) {
                result.insert(method.clone(), param);
            }
        }
    }
    Ok(result)
}

/// List all available methods in the database.
pub fn dftd4_list_methods() -> Vec<String> {
    let db = load_data_base().unwrap_or_else(|_| ParameterDataBase {
        default: DefaultSection {
            d4: vec!["bj-eeq-atm".to_string()],
            parameter: DefaultParameterSection {
                d4: D4DefaultParams {
                    bj_eeq_two: Table::new(),
                    bj_eeq_atm: Table::new(),
                    bj_eeq_mbd: Table::new(),
                },
            },
        },
        parameter: HashMap::new(),
    });
    db.parameter.keys().cloned().collect()
}

/* #endregion */

/* #region Internal helper functions */

/// Normalize version string (handle aliases like "d4bj" -> "bj-eeq-atm").
pub(crate) fn normalize_version(version: &str) -> String {
    let version_lower = version.to_lowercase().replace(['-', '_', ' '], "");
    match version_lower.as_str() {
        "d4bj" | "bj" | "bjeeqatm" | "atm" => "bj-eeq-atm",
        "bjeeqtwo" | "two" => "bj-eeq-two",
        "bjeeqmbd" | "mbd" => "bj-eeq-mbd",
        _ => &version_lower,
    }
    .to_string()
}

/// Get variant entry from method and database.
fn get_variant_entry(
    method_entry: &D4Variants,
    version: &str,
    db: &ParameterDataBase,
) -> Result<(Table, Table), DFTD4Error> {
    let d4_params = &method_entry.d4;
    let entry = match version {
        "bj-eeq-two" => d4_params.bj_eeq_two.clone(),
        "bj-eeq-atm" => d4_params.bj_eeq_atm.clone(),
        "bj-eeq-mbd" => d4_params.bj_eeq_mbd.clone(),
        _ => None,
    };

    let entry = entry.ok_or_else(|| {
        DFTD4Error::ParametersError(format!("Variant '{}' not found for this method", version))
    })?;

    let default_entry = match version {
        "bj-eeq-two" => db.default.parameter.d4.bj_eeq_two.clone(),
        "bj-eeq-atm" => db.default.parameter.d4.bj_eeq_atm.clone(),
        "bj-eeq-mbd" => db.default.parameter.d4.bj_eeq_mbd.clone(),
        _ => Table::new(),
    };

    Ok((entry, default_entry))
}

/// Get default entry for a variant.
fn get_variant_entry_for_defaults(
    version: &str,
    db: &ParameterDataBase,
) -> Result<(Option<Table>, Table), DFTD4Error> {
    let default_entry = match version {
        "bj-eeq-two" => db.default.parameter.d4.bj_eeq_two.clone(),
        "bj-eeq-atm" => db.default.parameter.d4.bj_eeq_atm.clone(),
        "bj-eeq-mbd" => db.default.parameter.d4.bj_eeq_mbd.clone(),
        _ => {
            return Err(DFTD4Error::ParametersError(format!(
                "Variant '{version}' not found in defaults",
            )))
        },
    };
    Ok((None, default_entry))
}

/// Merge method-specific entry table with defaults table.
/// Method values override defaults.
pub(crate) fn merge_tables(entry: &Table, defaults: &Table) -> Table {
    let mut merged = defaults.clone();
    for (key, value) in entry {
        merged.insert(key.clone(), value.clone());
    }
    merged
}

/// Extract DOI from merged table.
fn extract_doi(table: &Table) -> Option<String> {
    table.get("doi").and_then(|v| v.as_str()).map(|s| s.to_string())
}

/// Convert merged TOML table directly to DFTD4DampingParam via serde.
pub(crate) fn convert_to_damping_param(merged: &Table) -> Result<DFTD4DampingParam, DFTD4Error> {
    let doi = extract_doi(merged);
    let param = deserialize_table(merged)?;
    Ok(DFTD4DampingParam { param, doi })
}

/// Deserialize a TOML table directly into a serde-deserializable type.
pub(crate) fn deserialize_table<T: for<'de> Deserialize<'de>>(
    table: &Table,
) -> Result<T, DFTD4Error> {
    T::deserialize(table.clone()).map_err(|e| e.into())
}

impl From<toml::de::Error> for DFTD4Error {
    fn from(e: toml::de::Error) -> Self {
        DFTD4Error::ParametersError(format!("Deserialization error: {e}"))
    }
}

/* #endregion */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data_base() {
        let db = load_data_base();
        match &db {
            Ok(db) => {
                assert!(db.parameter.contains_key("b3lyp"));
                assert!(db.parameter.contains_key("pbe"));
                assert!(db.parameter.contains_key("r2scan"));
            },
            Err(e) => {
                println!("Error: {:?}", e);
                panic!("TOML parsing failed");
            },
        }
    }

    #[test]
    fn test_normalize_version() {
        assert_eq!(normalize_version("d4bj"), "bj-eeq-atm");
        assert_eq!(normalize_version("bj"), "bj-eeq-atm");
        assert_eq!(normalize_version("atm"), "bj-eeq-atm");
        assert_eq!(normalize_version("two"), "bj-eeq-two");
        assert_eq!(normalize_version("mbd"), "bj-eeq-mbd");
        assert_eq!(normalize_version("bj-eeq-atm"), "bj-eeq-atm");
    }
}
