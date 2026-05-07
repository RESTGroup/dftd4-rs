//! DFTD4 interface (safe wrapper).

use crate::ffi;
use derive_builder::{Builder, UninitializedFieldError};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use std::ffi::{c_char, c_int, CStr};
use std::ptr::{null, null_mut};
use std::result::Result;

/* #region DFTD4 version */

/// Get the version of the DFTD4 library.
///
/// The version is returned as a string in the format "major.minor.patch".
pub fn dftd4_get_api_version() -> String {
    let version = unsafe { ffi::dftd4_get_version() };
    format!("{}.{}.{}", version / 10000, version / 100 % 100, version % 100)
}

/// Get the version of the DFTD4 library in list of integers (major, minor,
/// patch).
pub fn dftd4_get_api_version_compact() -> [usize; 3] {
    let version = unsafe { ffi::dftd4_get_version() } as usize;
    [version / 10000, version / 100 % 100, version % 100]
}

/* #endregion */

/* #region DFTD4Error */

/// DFTD4 error class.
///
/// This is enum type to handle C error (from dftd4 itself) or rust error
/// (always presented as `String`).
pub enum DFTD4Error {
    C(ffi::dftd4_error),
    Rust(String),
    BuilderError(UninitializedFieldError),
    ParametersError(String),
}

impl From<UninitializedFieldError> for DFTD4Error {
    fn from(ufe: UninitializedFieldError) -> DFTD4Error {
        DFTD4Error::BuilderError(ufe)
    }
}

impl Drop for DFTD4Error {
    fn drop(&mut self) {
        if let DFTD4Error::C(ptr) = self {
            unsafe { ffi::dftd4_delete_error(&mut ptr.clone()) }
        }
    }
}

impl Default for DFTD4Error {
    fn default() -> Self {
        DFTD4Error::new()
    }
}

impl std::error::Error for DFTD4Error {}

impl DFTD4Error {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::dftd4_new_error() };
        DFTD4Error::C(ptr)
    }

    /// Check if the error is set.
    ///
    /// True if the error is set, false otherwise.
    pub fn check(&self) -> bool {
        match self {
            DFTD4Error::C(ptr) => unsafe { ffi::dftd4_check_error(*ptr) != 0 },
            _ => true,
        }
    }

    pub fn get_c_ptr(&mut self) -> ffi::dftd4_error {
        match self {
            DFTD4Error::C(ptr) => *ptr,
            _ => std::ptr::null_mut(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            DFTD4Error::C(ptr) => {
                const LEN_BUFFER: usize = 512;
                let buffer = [0u8; LEN_BUFFER];
                let raw = buffer.as_ptr() as *mut c_char;
                let msg = unsafe {
                    ffi::dftd4_get_error(*ptr, raw, &(LEN_BUFFER as c_int));
                    CStr::from_ptr(raw)
                };
                msg.to_string_lossy().to_string()
            },
            DFTD4Error::Rust(msg) => msg.clone(),
            DFTD4Error::BuilderError(ufe) => {
                format!("Builder error: {:?}", ufe)
            },
            DFTD4Error::ParametersError(msg) => msg.clone(),
        }
    }
}

impl std::fmt::Debug for DFTD4Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.check() {
            write!(f, "DFTD4Error: {}", self.get_message())
        } else {
            write!(f, "DFTD4Error: No error")
        }
    }
}

impl std::fmt::Display for DFTD4Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.check() {
            write!(f, "DFTD4Error: {}", self.get_message())
        } else {
            write!(f, "")
        }
    }
}

/* #endregion */

/* #region DFTD4Structure */

/// Molecular structure data.
///
/// Represents a wrapped structure object in `dftd4`. The molecular structure
/// data object has a fixed number of atoms and immutable atomic identifiers.
///
/// Note that except for number of atoms is stored in this struct, geometric
/// positions and lattice is not retrivable. API caller should handle these
/// information for themselves.
///
/// # Note
///
/// In most cases, this struct should not be used directly. Instead, use
/// [`DFTD4Model`].
pub struct DFTD4Structure {
    /// Pointer to the internal DFTD4 structure object.
    pub(crate) ptr: ffi::dftd4_structure,
    /// Number of atoms in the structure.
    natoms: usize,
}

impl Drop for DFTD4Structure {
    fn drop(&mut self) {
        unsafe { ffi::dftd4_delete_structure(&mut self.ptr) };
    }
}

impl DFTD4Structure {
    /// Create new molecular structure data from arrays (in Bohr).
    ///
    /// The returned object has immutable atomic species and boundary condition,
    /// also the total number of atoms cannot be changed.
    ///
    /// - `numbers` - element index (6 for O, 7 for N) in the structure
    /// - `positions` - atomic positions in Bohr (natom * 3)
    /// - `charge` - optional, total charge of the system
    /// - `lattice` - optional, lattice parameters (3 * 3)
    /// - `periodic` - optional, periodicity (3)
    ///
    /// # See also
    ///
    /// [`DFTD4Model::new`]
    pub fn new(
        numbers: &[usize],
        positions: &[f64],
        charge: Option<f64>,
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Self {
        Self::new_f(numbers, positions, charge, lattice, periodic).unwrap()
    }

    /// Update coordinates and lattice parameters (in Bohr).
    ///
    /// The lattice update is optional also for periodic structures.
    ///
    /// Generally, only the cartesian coordinates and the lattice parameters can
    /// be updated, every other modification, boundary condition, atomic types
    /// or number of atoms requires the complete reconstruction of the object.
    ///
    /// - `positions` - atomic positions in Bohr (natom * 3)
    /// - `lattice` - optional, lattice parameters (3 * 3)
    pub fn update(&mut self, positions: &[f64], lattice: Option<&[f64]>) {
        self.update_f(positions, lattice).unwrap()
    }

    /// Get number of atoms for this current structure.
    pub fn get_natoms(&self) -> usize {
        self.natoms
    }

    /// Create new molecular structure data from arrays (in Bohr, failable).
    ///
    /// # See also
    ///
    /// [`DFTD4Structure::new`]
    pub fn new_f(
        numbers: &[usize],
        positions: &[f64],
        charge: Option<f64>,
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Result<Self, DFTD4Error> {
        let natoms = numbers.len();
        // check dimension
        if positions.len() != 3 * natoms {
            return Err(DFTD4Error::Rust(format!(
                "Invalid dimension for positions, expected {}, got {}",
                3 * natoms,
                positions.len()
            )));
        }
        if lattice.is_some_and(|lattice| lattice.len() != 9) {
            return Err(DFTD4Error::Rust(format!(
                "Invalid dimension for lattice, expected 9, got {}",
                lattice.unwrap().len()
            )));
        }
        if periodic.is_some_and(|periodic| periodic.len() != 3) {
            return Err(DFTD4Error::Rust(format!(
                "Invalid dimension for periodic, expected 3, got {}",
                periodic.unwrap().len()
            )));
        }
        // unwrap optional values
        let charge_ptr = charge.map_or(null(), |x| &x as *const f64);
        let lattice_ptr = lattice.map_or(null(), |x| x.as_ptr());
        let periodic_ptr = periodic.map_or(null(), |x| x.as_ptr());
        // type conversion from usual definitions
        let natoms_c_int = natoms as c_int;
        let atomic_numbers = numbers.iter().map(|&x| x as c_int).collect::<Vec<c_int>>();
        // actual driver for creating the structure
        let mut error = DFTD4Error::new();
        let ptr = unsafe {
            ffi::dftd4_new_structure(
                error.get_c_ptr(),
                natoms_c_int,
                atomic_numbers.as_ptr(),
                positions.as_ptr(),
                charge_ptr,
                lattice_ptr,
                periodic_ptr,
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr, natoms }),
        }
    }

    /// Update coordinates and lattice parameters (in Bohr, failable).
    ///
    /// # See also
    ///
    /// [`DFTD4Structure::update`]
    pub fn update_f(
        &mut self,
        positions: &[f64],
        lattice: Option<&[f64]>,
    ) -> Result<(), DFTD4Error> {
        // check dimension
        if positions.len() != 3 * self.natoms {
            return Err(DFTD4Error::Rust(format!(
                "Invalid dimension for positions, expected {}, got {}",
                3 * self.natoms,
                positions.len()
            )));
        }
        if lattice.is_some_and(|lattice| lattice.len() != 9) {
            return Err(DFTD4Error::Rust(format!(
                "Invalid dimension for lattice, expected 9, got {}",
                lattice.unwrap().len()
            )));
        }
        // unwrap optional values
        let lattice_ptr = lattice.map_or(null(), |x| x.as_ptr());
        // actual driver for updating the structure
        let mut error = DFTD4Error::new();
        unsafe {
            ffi::dftd4_update_structure(
                error.get_c_ptr(),
                self.ptr,
                positions.as_ptr(),
                lattice_ptr,
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(()),
        }
    }
}

/* #endregion */

/* #region DFTD4Param */

/// Basic struct for damping parameters, representing a parametrization of
/// a DFT-D4 method.
///
/// The damping parameters contained in the object are immutable. To change the
/// parametrization, a new object must be created. Furthermore, the object is
/// opaque to the user and the contained data cannot be accessed directly.
pub struct DFTD4Param {
    ptr: ffi::dftd4_param,
}

impl Drop for DFTD4Param {
    fn drop(&mut self) {
        unsafe { ffi::dftd4_delete_param(&mut self.ptr) };
    }
}

impl DFTD4Param {
    /// Create new damping parameters.
    ///
    /// The parameters are:
    /// - `s6` - Scaling factor for C6 contribution
    /// - `s8` - Scaling factor for C8 contribution
    /// - `s9` - Scaling factor for C9 contribution
    /// - `a1` - Scaling factor for critical radii
    /// - `a2` - Offset distance in Bohr for critical radii
    pub fn new_rational_damping(s6: f64, s8: f64, s9: f64, a1: f64, a2: f64, alp: f64) -> Self {
        Self::new_rational_damping_f(s6, s8, s9, a1, a2, alp).unwrap()
    }

    /// Crate new damping parameters (failable).
    pub fn new_rational_damping_f(
        s6: f64,
        s8: f64,
        s9: f64,
        a1: f64,
        a2: f64,
        alp: f64,
    ) -> Result<Self, DFTD4Error> {
        let mut error = DFTD4Error::new();
        let ptr =
            unsafe { ffi::dftd4_new_rational_damping(error.get_c_ptr(), s6, s8, s9, a1, a2, alp) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Load damping parameters from internal storage.
    ///
    /// - `name` - name of the xc-functional
    /// - `atm` - use three-body specific parametrization
    pub fn load_rational_damping(name: &str, atm: bool) -> Self {
        Self::load_rational_damping_f(name, atm).unwrap()
    }

    /// Load damping parameters from internal storage (failable).
    pub fn load_rational_damping_f(name: &str, atm: bool) -> Result<Self, DFTD4Error> {
        let mut error = DFTD4Error::new();
        let name = std::ffi::CString::new(name).unwrap();
        let ptr = unsafe {
            ffi::dftd4_load_rational_damping(error.get_c_ptr(), name.as_ptr() as *mut c_char, atm)
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }
}

/* #region DFTD4ParamAPI trait */

/// Trait for creating DFTD4Param from damping parameter structs.
pub trait DFTD4ParamAPI: Sized {
    fn new_param(self) -> DFTD4Param {
        self.new_param_f().unwrap()
    }
    fn new_param_f(self) -> Result<DFTD4Param, DFTD4Error>;
}

/// Trait for loading DFTD4Param by method name.
pub trait DFTD4LoadParamAPI {
    fn load_param_f(method: &str, atm: bool) -> Result<DFTD4Param, DFTD4Error>;
}

/* #endregion */

/// DFTD4 rational damping parameters.
#[doc = include_str!("damping_param_usage.md")]
#[serde_inline_default]
#[derive(Builder, Debug, Clone, Deserialize, Serialize)]
#[builder(pattern = "owned", build_fn(error = "DFTD4Error"))]
pub struct DFTD4RationalDampingParam {
    #[builder(default = 1.0)]
    #[serde_inline_default(1.0)]
    #[doc = r"optional, default 1.0"]
    pub s6: f64,
    pub s8: f64,
    #[builder(default = 1.0)]
    #[serde_inline_default(1.0)]
    #[doc = r"optional, default 1.0"]
    pub s9: f64,
    pub a1: f64,
    pub a2: f64,
    #[builder(default = 16.0)]
    #[serde_inline_default(16.0)]
    #[doc = r"optional, default 16.0"]
    pub alp: f64,
}

impl DFTD4ParamAPI for DFTD4RationalDampingParam {
    fn new_param_f(self) -> Result<DFTD4Param, DFTD4Error> {
        let Self { s6, s8, s9, a1, a2, alp } = self;
        DFTD4Param::new_rational_damping_f(s6, s8, s9, a1, a2, alp)
    }
}

impl DFTD4LoadParamAPI for DFTD4RationalDampingParam {
    fn load_param_f(method: &str, atm: bool) -> Result<DFTD4Param, DFTD4Error> {
        DFTD4Param::load_rational_damping_f(method, atm)
    }
}

/* #region Macro-based implementations */

macro_rules! impl_damping_param_builder {
    ($feature:literal: $type:ty) => {
        #[cfg(feature = $feature)]
        impl $type {
            pub fn init(self) -> DFTD4Param {
                self.init_f().unwrap()
            }
            pub fn init_f(self) -> Result<DFTD4Param, DFTD4Error> {
                self.build()?.new_param_f()
            }
        }
    };
}

impl_damping_param_builder!("api-v3_0": DFTD4RationalDampingParamBuilder);

/* #endregion */

/// Load damping parameters by method name and DFT-D4 variant.
///
/// This is a convenience function that loads rational damping parameters
/// for a given method name and variant specification.
///
/// - `variant` - damping variant, currently only `"d4bj"` is supported
/// - `method` - name of the xc-functional (e.g., `"B3LYP"`)
/// - `atm` - use MBD parametrization (true) or ATM (false)
pub fn dftd4_load_param(variant: &str, method: &str, atm: bool) -> DFTD4Param {
    match variant {
        "d4bj" => DFTD4RationalDampingParam::load_param_f(method, atm).unwrap(),
        _ => panic!("Unknown damping variant: {}", variant),
    }
}

/* #endregion */

/* #region DFTD4 outputs */

/// DFTD4 returned result.
///
/// This struct implements `From` trait to convert to tuple. So you can use this
/// struct in this way:
///
/// ```ignore
/// let (energy, grad, sigma) = dftd4_model.get_dispersion(param, eval_grad).into();
/// ```
pub struct DFTD4Output {
    /// Dispersion energy.
    pub energy: f64,
    /// Gradient of the dispersion energy (natom * 3).
    pub grad: Option<Vec<f64>>,
    /// Strain derivatives (3 * 3).
    pub sigma: Option<Vec<f64>>,
}

impl From<DFTD4Output> for (f64, Option<Vec<f64>>, Option<Vec<f64>>) {
    fn from(output: DFTD4Output) -> Self {
        (output.energy, output.grad, output.sigma)
    }
}

/// DFTD4 pairwise returned result.
///
/// This struct implements `From` trait to convert to tuple. So you can use this
/// struct in this way:
///
/// ```ignore
/// let (pair_energy2, pair_energy3) = dftd4_model.get_pairwise_dispersion(param).into();
/// ```
#[cfg(feature = "api-v3_2")]
pub struct DFTD4PairwiseOutput {
    /// Pairwise additive pairwise energy (natom * natom)
    pub pair_energy2: Vec<f64>,
    /// Pairwise non-additive pairwise energy (natom * natom)
    pub pair_energy3: Vec<f64>,
}

#[cfg(feature = "api-v3_2")]
impl From<DFTD4PairwiseOutput> for (Vec<f64>, Vec<f64>) {
    fn from(output: DFTD4PairwiseOutput) -> Self {
        (output.pair_energy2, output.pair_energy3)
    }
}

/// DFTD4 property returned result.
///
/// This struct implements `From` trait to convert to tuple. So you can use this
/// struct in this way:
///
/// ```ignore
/// let (cn, charges, c6, alpha) = dftd4_model.get_properties(param).into();
/// ```
#[cfg(feature = "api-v3_1")]
pub struct DFTD4PropertyOutput {
    /// Coordination number for all atoms (natoms).
    pub cn: Vec<f64>,
    /// Partial charges for all atoms (natoms).
    pub charges: Vec<f64>,
    /// C6 coefficients for all atom pairs (natoms * natoms).
    pub c6: Vec<f64>,
    /// Static polarizability for all atoms (natoms).
    pub alpha: Vec<f64>,
}

#[cfg(feature = "api-v3_1")]
impl From<DFTD4PropertyOutput> for (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    fn from(output: DFTD4PropertyOutput) -> Self {
        (output.cn, output.charges, output.c6, output.alpha)
    }
}

/* #endregion */

/* #region DFTD4Model */

/// DFTD4 dispersion model.
///
/// Representation of a dispersion model to evaluate C6 coefficients.
/// The model is coupled to the molecular structure it has been created
/// from and cannot be transfered to another molecular structure without
/// recreating it.
pub struct DFTD4Model {
    /// Pointer to the internal DFTD4 model object.
    ptr: ffi::dftd4_model,
    /// Internal DFTD4 structure object.
    structure: DFTD4Structure,
}

impl Drop for DFTD4Model {
    fn drop(&mut self) {
        unsafe { ffi::dftd4_delete_model(&mut self.ptr) };
    }
}

impl DFTD4Model {
    /// Create new molecular structure data from arrays (in Bohr).
    ///
    /// The returned object has immutable atomic species and boundary condition,
    /// also the total number of atoms cannot be changed.
    ///
    /// - `numbers` - element index (6 for O, 7 for N) in the structure
    /// - `positions` - atomic positions in Bohr (natom * 3)
    /// - `charge` - optional, total charge of the system
    /// - `lattice` - optional, lattice parameters (3 * 3)
    /// - `periodic` - optional, periodicity (3)
    pub fn new(
        numbers: &[usize],
        positions: &[f64],
        charge: Option<f64>,
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Self {
        Self::new_f(numbers, positions, charge, lattice, periodic).unwrap()
    }

    /// Create new D4S dispersion model from arrays (in Bohr).
    #[cfg(feature = "api-v4_0")]
    pub fn new_d4s(
        numbers: &[usize],
        positions: &[f64],
        charge: Option<f64>,
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Self {
        Self::new_d4s_f(numbers, positions, charge, lattice, periodic).unwrap()
    }

    /// Create new D4S dispersion model with custom parameters.
    ///
    /// - `ga` - Charge scaling height
    /// - `gc` - Charge scaling steepness
    #[cfg(feature = "api-v4_0")]
    pub fn custom_d4s(
        numbers: &[usize],
        positions: &[f64],
        ga: f64,
        gc: f64,
        charge: Option<f64>,
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Self {
        Self::custom_d4s_f(numbers, positions, ga, gc, charge, lattice, periodic).unwrap()
    }

    /// Create custom D4 model with parameters.
    ///
    /// - `ga` - Charge scaling height
    /// - `gc` - Charge scaling steepness
    /// - `wf` - Weighting factor
    #[cfg(feature = "api-v3_1")]
    #[allow(clippy::too_many_arguments)]
    pub fn custom_d4(
        numbers: &[usize],
        positions: &[f64],
        ga: f64,
        gc: f64,
        wf: f64,
        charge: Option<f64>,
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Self {
        Self::custom_d4_f(numbers, positions, ga, gc, wf, charge, lattice, periodic).unwrap()
    }

    /// Set realspace cutoffs (quantities in Bohr).
    ///
    /// - `disp2` - Cutoff for two-body dispersion
    /// - `disp3` - Cutoff for three-body dispersion
    /// - `cn` - Cutoff for coordination number
    #[cfg(feature = "api-v4_2")]
    pub fn set_realspace_cutoff(&mut self, disp2: f64, disp3: f64, cn: f64) {
        self.set_realspace_cutoff_f(disp2, disp3, cn).unwrap()
    }

    /// Set realspace cutoffs with smoothing widths (quantities in Bohr).
    ///
    /// - `disp2` - Cutoff for two-body dispersion
    /// - `disp3` - Cutoff for three-body dispersion
    /// - `cn` - Cutoff for coordination number
    /// - `width2` - Smoothing width for two-body dispersion
    /// - `width3` - Smoothing width for three-body dispersion
    #[cfg(feature = "api-v4_2")]
    pub fn set_realspace_cutoff_smooth(
        &mut self,
        disp2: f64,
        disp3: f64,
        cn: f64,
        width2: f64,
        width3: f64,
    ) {
        self.set_realspace_cutoff_smooth_f(disp2, disp3, cn, width2, width3).unwrap()
    }

    /// Evaluate the dispersion energy and its derivatives.
    ///
    /// Output `DFTD4Output` contains
    ///
    /// - `energy` - dispersion energy
    /// - `grad` - gradient of the dispersion energy (natom * 3)
    /// - `sigma` - strain derivatives (3 * 3)
    pub fn get_dispersion(&self, param: &DFTD4Param, eval_grad: bool) -> DFTD4Output {
        self.get_dispersion_f(param, eval_grad).unwrap()
    }

    /// Evaluate the pairwise dispersion energy.
    ///
    /// Output `DFTD4PairwiseOutput` contains
    ///
    /// - `pair_energy2` - pairwise additive pairwise energy (natom * natom)
    /// - `pair_energy3` - pairwise non-additive pairwise energy (natom * natom)
    #[cfg(feature = "api-v3_2")]
    pub fn get_pairwise_dispersion(&self, param: &DFTD4Param) -> DFTD4PairwiseOutput {
        self.get_pairwise_dispersion_f(param).unwrap()
    }

    /// Evaluate the numerical hessian of the dispersion.
    ///
    /// Returns a vector of size (natoms * 3 * natoms * 3) containing the
    /// hessian matrix elements.
    #[cfg(feature = "api-v3_5")]
    pub fn get_numerical_hessian(&self, param: &DFTD4Param) -> Vec<f64> {
        self.get_numerical_hessian_f(param).unwrap()
    }

    /// Evaluate properties related to the dispersion model.
    ///
    /// Output `DFTD4PropertyOutput` contains
    ///
    /// - `cn` - Coordination number for all atoms (natoms)
    /// - `charges` - Partial charges for all atoms (natoms)
    /// - `c6` - C6 coefficients for all atom pairs (natoms * natoms)
    /// - `alpha` - Static polarizability for all atoms (natoms)
    #[cfg(feature = "api-v3_1")]
    pub fn get_properties(&self) -> DFTD4PropertyOutput {
        self.get_properties_f().unwrap()
    }

    /// Create new D4 dispersion model from structure.
    pub fn from_structure(structure: DFTD4Structure) -> Self {
        Self::from_structure_f(structure).unwrap()
    }

    /// Update coordinates and lattice parameters (in Bohr).
    ///
    /// The lattice update is optional also for periodic structures.
    ///
    /// Generally, only the cartesian coordinates and the lattice parameters can
    /// be updated, every other modification, boundary condition, atomic types
    /// or number of atoms requires the complete reconstruction of the object.
    ///
    /// - `positions` - atomic positions in Bohr (natom * 3)
    /// - `lattice` - optional, lattice parameters (3 * 3)
    pub fn update(&mut self, positions: &[f64], lattice: Option<&[f64]>) {
        self.structure.update(positions, lattice)
    }

    /// Update coordinates and lattice parameters (in Bohr, failable).
    ///
    /// # See also
    ///
    /// [`DFTD4Model::new`]
    pub fn new_f(
        numbers: &[usize],
        positions: &[f64],
        charge: Option<f64>,
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Result<Self, DFTD4Error> {
        let structure = DFTD4Structure::new_f(numbers, positions, charge, lattice, periodic)?;
        let mut error = DFTD4Error::new();
        let ptr = unsafe { ffi::dftd4_new_d4_model(error.get_c_ptr(), structure.ptr) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr, structure }),
        }
    }

    /// Create new D4S dispersion model from structure.
    ///
    /// # See also
    ///
    /// [`DFTD4Model::new_d4s`]
    #[cfg(feature = "api-v4_0")]
    pub fn new_d4s_f(
        numbers: &[usize],
        positions: &[f64],
        charge: Option<f64>,
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Result<Self, DFTD4Error> {
        let structure = DFTD4Structure::new_f(numbers, positions, charge, lattice, periodic)?;
        let mut error = DFTD4Error::new();
        let ptr = unsafe { ffi::dftd4_new_d4s_model(error.get_c_ptr(), structure.ptr) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr, structure }),
        }
    }

    /// Create new D4S dispersion model with custom parameters.
    ///
    /// # See also
    ///
    /// [`DFTD4Model::custom_d4s`]
    #[cfg(feature = "api-v4_0")]
    pub fn custom_d4s_f(
        numbers: &[usize],
        positions: &[f64],
        ga: f64,
        gc: f64,
        charge: Option<f64>,
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Result<Self, DFTD4Error> {
        let structure = DFTD4Structure::new_f(numbers, positions, charge, lattice, periodic)?;
        let mut error = DFTD4Error::new();
        let ptr = unsafe { ffi::dftd4_custom_d4s_model(error.get_c_ptr(), structure.ptr, ga, gc) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr, structure }),
        }
    }

    /// Create custom D4 model with parameters (failable).
    ///
    /// # See also
    ///
    /// [`DFTD4Model::custom_d4`]
    #[cfg(feature = "api-v3_1")]
    #[allow(clippy::too_many_arguments)]
    pub fn custom_d4_f(
        numbers: &[usize],
        positions: &[f64],
        ga: f64,
        gc: f64,
        wf: f64,
        charge: Option<f64>,
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Result<Self, DFTD4Error> {
        let structure = DFTD4Structure::new_f(numbers, positions, charge, lattice, periodic)?;
        let mut error = DFTD4Error::new();
        let ptr =
            unsafe { ffi::dftd4_custom_d4_model(error.get_c_ptr(), structure.ptr, ga, gc, wf) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr, structure }),
        }
    }

    /// Set realspace cutoffs (quantities in Bohr, failable).
    ///
    /// # See also
    ///
    /// [`DFTD4Model::set_realspace_cutoff`]
    #[cfg(feature = "api-v4_2")]
    pub fn set_realspace_cutoff_f(
        &mut self,
        disp2: f64,
        disp3: f64,
        cn: f64,
    ) -> Result<(), DFTD4Error> {
        let mut error = DFTD4Error::new();
        unsafe {
            ffi::dftd4_set_model_realspace_cutoff(error.get_c_ptr(), self.ptr, disp2, disp3, cn)
        };
        match error.check() {
            true => Err(error),
            false => Ok(()),
        }
    }

    /// Set realspace cutoffs with smoothing widths (quantities in Bohr,
    /// failable).
    ///
    /// # See also
    ///
    /// [`DFTD4Model::set_realspace_cutoff_smooth`]
    #[cfg(feature = "api-v4_2")]
    pub fn set_realspace_cutoff_smooth_f(
        &mut self,
        disp2: f64,
        disp3: f64,
        cn: f64,
        width2: f64,
        width3: f64,
    ) -> Result<(), DFTD4Error> {
        let mut error = DFTD4Error::new();
        unsafe {
            ffi::dftd4_set_model_realspace_cutoff_smooth(
                error.get_c_ptr(),
                self.ptr,
                disp2,
                disp3,
                cn,
                width2,
                width3,
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(()),
        }
    }

    /// Evaluate the dispersion energy and its derivatives (failable).
    ///
    /// # See also
    ///
    /// [`DFTD4Model::get_dispersion`]
    pub fn get_dispersion_f(
        &self,
        param: &DFTD4Param,
        eval_grad: bool,
    ) -> Result<DFTD4Output, DFTD4Error> {
        let structure = &self.structure;
        let natoms = structure.get_natoms();
        let mut energy = 0.0;
        let mut grad = match eval_grad {
            true => Some(vec![0.0; 3 * natoms]),
            false => None,
        };
        let mut sigma = match eval_grad {
            true => Some(vec![0.0; 9]),
            false => None,
        };
        let mut error = DFTD4Error::new();
        unsafe {
            ffi::dftd4_get_dispersion(
                error.get_c_ptr(),
                structure.ptr,
                self.ptr,
                param.ptr,
                &mut energy,
                grad.as_mut().map_or(null_mut(), |x| x.as_mut_ptr()),
                sigma.as_mut().map_or(null_mut(), |x| x.as_mut_ptr()),
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(DFTD4Output { energy, grad, sigma }),
        }
    }

    /// Evaluate the pairwise dispersion energy (failable).
    ///
    /// # See also
    ///
    /// [`DFTD4Model::get_pairwise_dispersion`]
    #[cfg(feature = "api-v3_2")]
    pub fn get_pairwise_dispersion_f(
        &self,
        param: &DFTD4Param,
    ) -> Result<DFTD4PairwiseOutput, DFTD4Error> {
        let structure = &self.structure;
        let natoms = structure.get_natoms();
        let mut pair_energy2 = vec![0.0; natoms * natoms];
        let mut pair_energy3 = vec![0.0; natoms * natoms];
        let mut error = DFTD4Error::new();

        unsafe {
            ffi::dftd4_get_pairwise_dispersion(
                error.get_c_ptr(),
                structure.ptr,
                self.ptr,
                param.ptr,
                pair_energy2.as_mut_ptr(),
                pair_energy3.as_mut_ptr(),
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(DFTD4PairwiseOutput { pair_energy2, pair_energy3 }),
        }
    }

    /// Evaluate the numerical hessian (failable).
    ///
    /// # See also
    ///
    /// [`DFTD4Model::get_numerical_hessian`]
    #[cfg(feature = "api-v3_5")]
    pub fn get_numerical_hessian_f(&self, param: &DFTD4Param) -> Result<Vec<f64>, DFTD4Error> {
        let structure = &self.structure;
        let natoms = structure.get_natoms();
        let n = 3 * natoms;
        let mut hess = vec![0.0; n * n];
        let mut error = DFTD4Error::new();

        unsafe {
            ffi::dftd4_get_numerical_hessian(
                error.get_c_ptr(),
                structure.ptr,
                self.ptr,
                param.ptr,
                hess.as_mut_ptr(),
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(hess),
        }
    }

    /// Evaluate properties related to the dispersion model (failable).
    ///
    /// # See also
    ///
    /// [`DFTD4Model::get_properties`]
    #[cfg(feature = "api-v3_1")]
    pub fn get_properties_f(&self) -> Result<DFTD4PropertyOutput, DFTD4Error> {
        let structure = &self.structure;
        let natoms = structure.get_natoms();
        let mut cn = vec![0.0; natoms];
        let mut charges = vec![0.0; natoms];
        let mut c6 = vec![0.0; natoms * natoms];
        let mut alpha = vec![0.0; natoms];
        let mut error = DFTD4Error::new();

        unsafe {
            ffi::dftd4_get_properties(
                error.get_c_ptr(),
                structure.ptr,
                self.ptr,
                cn.as_mut_ptr(),
                charges.as_mut_ptr(),
                c6.as_mut_ptr(),
                alpha.as_mut_ptr(),
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(DFTD4PropertyOutput { cn, charges, c6, alpha }),
        }
    }

    /// Create new D4 dispersion model from structure (failable).
    ///
    /// # See also
    ///
    /// [`DFTD4Model::from_structure`]
    pub fn from_structure_f(structure: DFTD4Structure) -> Result<Self, DFTD4Error> {
        let mut error = DFTD4Error::new();
        let ptr = unsafe { ffi::dftd4_new_d4_model(error.get_c_ptr(), structure.ptr) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr, structure }),
        }
    }

    /// Update coordinates and lattice parameters (in Bohr, failable).
    ///
    /// # See also
    ///
    /// [`DFTD4Model::update`]
    pub fn update_f(
        &mut self,
        positions: &[f64],
        lattice: Option<&[f64]>,
    ) -> Result<(), DFTD4Error> {
        self.structure.update_f(positions, lattice)
    }
}

/* #endregion */

#[cfg(test)]
mod tests {
    use crate::ffi::dftd4_load_rational_damping;

    use super::*;

    #[test]
    fn test_get_api_version() {
        println!("API version: {}", dftd4_get_api_version());
    }

    #[test]
    fn test_get_api_version_compact() {
        println!("API version: {:?}", dftd4_get_api_version_compact());
    }

    #[test]
    fn test_dftd4_error() {
        let mut error = DFTD4Error::new();
        println!("Error check   : {}", error.check());
        println!("Error message : {}", error.get_message());
        let token = std::ffi::CString::new("Hello").unwrap();
        unsafe { dftd4_load_rational_damping(error.get_c_ptr(), token.into_raw(), false) };
        println!("Error check   : {}", error.check());
        println!("Error message : {}", error.get_message());
        let token = std::ffi::CString::new("B3LYP").unwrap();
        unsafe { dftd4_load_rational_damping(error.get_c_ptr(), token.into_raw(), false) };
        println!("Error check   : {}", error.check());
        println!("Error message : {}", error.get_message());
    }

    #[test]
    fn test_get_dispersion() {
        let numbers = vec![1, 1];
        let positions = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        let model = DFTD4Model::new(&numbers, &positions, None, None, None);
        let param = DFTD4Param::load_rational_damping("B3LYP", false);
        let (energy, grad, sigma) = model.get_dispersion(&param, true).into();
        println!("Dispersion energy: {}", energy);
        println!("Dispersion gradient: {:?}", grad);
        println!("Dispersion sigma: {:?}", sigma);
    }
}
