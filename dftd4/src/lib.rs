/*!

# DFTD4 API specification document entrance

For API users, the most important part of this crate is the [`interface`]
module. The commonly used functions and structs can be

- [`DFTD4Model`](interface::DFTD4Model): serve as main driver struct for DFTD4.
- [`DFTD4Param`](interface::DFTD4Param): parameter utilities for DFTD4. Function [`DFTD4Param::load_rational_damping`](interface::DFTD4Param::load_rational_damping) is the most commonly used one.

To specify custom DFT-D4 parameters, some structs you may interest.

- [`DFTD4RationalDampingParam`](interface::DFTD4RationalDampingParam) for rational damping.

*/
#![doc = include_str!("../readme.md")]

pub mod ffi;
pub mod interface;

pub mod prelude {
    //! Use `dftd4::prelude::*` to import all the commonly used structs and
    //! functions.
    pub use crate::interface::*;
}
