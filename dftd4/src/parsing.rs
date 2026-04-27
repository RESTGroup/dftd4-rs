//! Flexible parsing of DFT-D4 damping parameters from TOML input.
//!
//! This module provides functions to parse parameter specifications that can
//! combine method lookups from the database with direct parameter values,
//! overrides, and the `atm` flag for three-body dispersion control.
//!
//! # Supported input formats
//!
//! - **Usual case with method**: `{version = "d4bj", method = "b3lyp"}` Lookup
//!   B3LYP-D4(BJ-EEQ-ATM) parameters from the database.
//! - **Default version**: `version = "d4"`, `"d4bj"`, `"bj"`, or `"atm"` all
//!   resolve to the **bj-eeq-atm** variant (Becke-Johnson damping with ATM
//!   three-body dispersion), which is the standard and most commonly used
//!   DFT-D4 model. This is the default variant in the upstream dftd4 parameter
//!   database.
//! - **Omitting version**: The `version` field is optional. If omitted, it
//!   defaults to bj-eeq-atm. For example, `{method = "b3lyp"}` is equivalent to
//!   `{version = "d4", method = "b3lyp"}`.
//! - **Other variants**: Use `version = "two"` for bj-eeq-two (no three-body
//!   term, s9 = 0), or `version = "mbd"` for bj-eeq-mbd (MBD-style three-body).
//! - **Version aliases**: The `d4` prefix and `bj-eeq-` part are optional.
//!   Version is case-insensitive.
//! - **Direct parameters**: `{version = "d4bj", a1 = 0.40868035, s8 =
//!   2.02929367, a2 = 4.53807137}` Specify parameters directly without the
//!   database.
//! - **ATM flag (no three-body)**: `{version = "bj", method = "b3lyp", atm =
//!   false}` Sets `s9 = 0.0`. Default is `atm = true` (s9 = 1.0).
//! - **Parameter override**: `{version = "d4bj", method = "b3lyp", a1 = 0.5}`
//!   Use database values but override `a1` to 0.5.
//! - **Direct params + ATM**: `{version = "d4bj", a1 = 0.40868035, s8 =
//!   2.02929367, a2 = 4.53807137, atm = false}` Direct parameters with `s9 =
//!   0.0`. If both `s9` and `atm` are provided, `s9` takes precedence.
//! - **Method name normalization**: `{version = "bj", method = "r2-scan"}`
//!   Separators like `-`, `_` are removed automatically (normalized to `r2scan`
//!   for lookup).
//! - **Invalid field error**: `{version = "d4bj", method = "b3lyp", rs6 = 0.5}`
//!   Returns an error because `rs6` is not a valid parameter for DFT-D4.
//!
//! # Example
//!
//! ```
//! use dftd4::prelude::*;
//!
//! // B3LYP with Becke-Johnson damping, no overrides, atm = true (default)
//! let input = r#"{version = "d4bj", method = "b3lyp"}"#;
//! // toml parameter type
//! let damping_param = dftd4_parse_damping_param_from_toml(input);
//! // FFI parameter type
//! let dftd4_param = damping_param.new_param();
//!
//! let atom_numbers = vec![8, 1, 1];
//! // coordinates in bohr
//! #[rustfmt::skip]
//! let coordinates = vec![
//!     0.000000, 0.000000, 0.000000,
//!     0.000000, 0.000000, 1.807355,
//!     1.807355, 0.000000, -0.452500,
//! ];
//! let model = DFTD4Model::new(&atom_numbers, &coordinates, None, None, None);
//! let res = model.get_dispersion(&dftd4_param, false);
//! let eng = res.energy;
//! println!("Dispersion energy: {eng}");
//! ```

use crate::interface::DFTD4Error;
use crate::parameters::{
    convert_to_damping_param, get_default_param_table, get_merged_param_table, normalize_version,
    DFTD4DampingParam,
};
use toml::Table;

/// Meta-fields that control parsing but are not damping parameters themselves.
const META_FIELDS: &[&str] = &["version", "method", "atm"];

/// Valid damping parameter fields for DFT-D4 (all variants use the same
/// parameter set).
const VALID_FIELDS: &[&str] = &["s6", "s8", "s9", "a1", "a2", "alp"];

/// Parse damping parameters from a TOML table.
///
/// The `version` field is optional and defaults to bj-eeq-atm if omitted.
/// Optional `method` field triggers a database lookup, and `atm` controls
/// the three-body dispersion term (s9). Remaining fields are treated as
/// damping parameters or overrides.
///
/// # Errors
///
/// Returns an error if:
/// - `version` is unrecognized
/// - `method` is specified but not found in the database
/// - A field not valid for DFT-D4 is present
/// - Required damping parameters are missing
pub fn dftd4_parse_damping_param(input: &Table) -> DFTD4DampingParam {
    dftd4_parse_damping_param_f(input).unwrap()
}

pub fn dftd4_parse_damping_param_f(input: &Table) -> Result<DFTD4DampingParam, DFTD4Error> {
    // 1. Extract version (optional, defaults to bj-eeq-atm)
    let version_raw = input.get("version").and_then(|v| v.as_str()).unwrap_or("d4");
    let version = normalize_version(version_raw);

    // 2. Extract method (optional)
    let method = input.get("method").and_then(|v| v.as_str());

    // 3. Extract atm flag (optional, default true)
    let atm = input.get("atm").and_then(|v| v.as_bool()).unwrap_or(true);

    // 4. Check whether s9 is explicitly provided by user
    let s9_explicit = input.contains_key("s9");

    // 5. Collect user-provided parameter fields (excluding meta-fields)
    let user_param_keys: Vec<&str> =
        input.keys().map(|k| k.as_str()).filter(|k| !META_FIELDS.contains(k)).collect();

    // Validate unknown fields
    for key in &user_param_keys {
        if !VALID_FIELDS.contains(key) {
            return Err(DFTD4Error::ParametersError(format!(
                "Unknown parameter '{}' for variant '{}'",
                key, version
            )));
        }
    }

    // 6. Build the merged parameter table
    let mut merged = if let Some(method) = method {
        // Method lookup: get base table from database
        get_merged_param_table(method, &version)?
    } else {
        // No method: start from defaults
        get_default_param_table(&version)?
    };

    // 7. Apply user parameter overrides on top
    for key in &user_param_keys {
        if let Some(value) = input.get(*key) {
            merged.insert((*key).to_string(), value.clone());
        }
    }

    // 8. Handle atm -> s9 relationship If s9 is explicitly provided, it takes
    //    precedence over atm. If s9 is not explicit and atm is false, set s9 = 0.0.
    if !atm && !s9_explicit {
        merged.insert("s9".to_string(), toml::Value::Float(0.0));
    }

    // 9. Remove non-deserializable fields (like "damping", "mbd" from defaults)
    merged.remove("damping");
    merged.remove("mbd");

    // 10. Convert to DFTD4DampingParam
    convert_to_damping_param(&merged)
}

/// Parse a TOML string to a table. Supports both standard TOML documents
/// and inline table syntax like `{version = "bj", method = "b3lyp"}`.
fn parse_toml_table(input: &str) -> Result<Table, DFTD4Error> {
    let trimmed = input.trim();
    if trimmed.starts_with('{') {
        // Inline table: wrap in dummy key to make valid TOML document
        let wrapped = format!("x = {trimmed}");
        let mut doc: Table = toml::from_str(&wrapped)
            .map_err(|e| DFTD4Error::ParametersError(format!("TOML parsing error: {e}")))?;
        if let Some(toml::Value::Table(table)) = doc.remove("x") {
            Ok(table)
        } else {
            Err(DFTD4Error::ParametersError("Invalid TOML format".into()))
        }
    } else {
        toml::from_str(trimmed)
            .map_err(|e| DFTD4Error::ParametersError(format!("TOML parsing error: {e}")))
    }
}

/// Parse damping parameters from a TOML string (panics on error).
///
/// Supports both standard TOML documents and inline table syntax.
/// See the [module-level documentation](self) for all supported input formats.
///
/// # Panics
///
/// Panics if parsing fails. Use [`dftd4_parse_damping_param_from_toml_f`] for a
/// fallible version.
///
/// # Example
///
/// ```
/// use dftd4::prelude::*;
///
/// let input = r#"{version = "bj", method = "b3lyp"}"#;
/// let param = dftd4_parse_damping_param_from_toml(input);
/// ```
pub fn dftd4_parse_damping_param_from_toml(input: &str) -> DFTD4DampingParam {
    dftd4_parse_damping_param_from_toml_f(input).unwrap()
}

/// Parse damping parameters from a TOML string (fallible version).
///
/// Supports both standard TOML documents and inline table syntax.
/// See the [module-level documentation](self) for all supported input formats.
///
/// # Errors
///
/// Returns an error if:
/// - TOML parsing fails
/// - Method not found in database
/// - Unknown parameter field
/// - Parameter deserialization fails
pub fn dftd4_parse_damping_param_from_toml_f(input: &str) -> Result<DFTD4DampingParam, DFTD4Error> {
    let table = parse_toml_table(input)?;
    dftd4_parse_damping_param_f(&table)
}

#[test]
fn test_dftd4_parse_damping_param_from_toml_doc() {
    use crate::prelude::*;
    // B3LYP with Becke-Johnson damping, no overrides, atm = true (default)
    let input = r#"{version = "d4bj", method = "b3lyp"}"#;
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

    // custom parameter
    let input =
        r#"{version = "d4bj", a1 = 0.40868035, s8 = 2.02929367, a2 = 4.53807137, atm = false}"#;
    let damping_param = dftd4_parse_damping_param_from_toml(input);
    let dftd4_param = damping_param.new_param();
    let res = model.get_dispersion(&dftd4_param, false);
    let eng = res.energy;
    println!("Dispersion energy with custom params: {eng}");
}
