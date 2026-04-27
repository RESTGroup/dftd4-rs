/*!

# DFTD4 API specification document entrance

For API users, the most important part of this crate is the [`interface`]
module. The commonly used functions and structs can be

- [`DFTD4Model`](interface::DFTD4Model): serve as main driver struct for DFTD4.
- [`dftd4_load_param`](interface::dftd4_load_param): load parameters with xc-functional and DFT-D4 version specified.
- [`dftd4_parse_damping_param_from_toml`](parsing::dftd4_parse_damping_param_from_toml): parse damping parameters from TOML string (supports method lookup and overrides). The `version` field is optional and defaults to **bj-eeq-atm**; `version = "d4"`, `"d4bj"`, `"bj"`, or `"atm"` all resolve to the same variant. Please refer to [parsing] module for more details and examples.

To specify custom DFT-D4 parameters, some structs you may interest.

- [`DFTD4RationalDampingParam`](interface::DFTD4RationalDampingParam) for rational damping.

You may also check [`DFTD4Param`](interface::DFTD4Param), but note that this struct is somehow low-level API, so use it with more care.

## Parameter database

The [`parameters`] module provides access to the built-in DFT-D4 parameter
database embedded from the original dftd4 library. This allows looking up
damping parameters for various XC functionals by method name and variant:

- [`dftd4_get_damping_param`](parameters::dftd4_get_damping_param): lookup parameters by method name and variant
- [`dftd4_list_methods`](parameters::dftd4_list_methods): list all available methods in the database
- [`dftd4_get_all_damping_params`](parameters::dftd4_get_all_damping_params): get all parameters for a given variant

*/
#![doc = include_str!("../readme.md")]

// Use ffi module from ffi/ directory for both static and dynamic loading
#[cfg(not(feature = "dynamic_loading"))]
pub mod ffi_static;
#[cfg(not(feature = "dynamic_loading"))]
pub use ffi_static as ffi;

#[cfg(feature = "dynamic_loading")]
pub mod ffi_dynamic;
#[cfg(feature = "dynamic_loading")]
pub use ffi_dynamic as ffi;

pub mod interface;
pub mod parameters;
pub mod parsing;

pub mod prelude {
    //! Use `dftd4::prelude::*` to import all the commonly used structs and
    //! functions.
    pub use crate::interface::*;
    pub use crate::parameters::*;
    pub use crate::parsing::*;
}
