# API documentation for custom damping parameter specification

If your task is to retrive damping parameters of some specific xc-functionals, you may wish to try [`DFTD4Param::load_rational_damping`] function.

In this crate, you may have three ways to define customized parameters:

- By [`DFTD4RationalDampingParam`] struct. In this way, all parameters (include optional parameters with default value) must be provided. For example:

    ```rust
  # use dftd4::prelude::*;
  # let atm = true;
    let param = DFTD4RationalDampingParam {
        s6: 1.0,
        s8: 1.683,
        s9: 1.0,
        a1: 1.139,
        a2: 1.5,
        alp: 16.0,
    };
    let param = param.new_param();
    // this will give param: DFTD4Param
    ```

- By [`DFTD4RationalDampingParamBuilder`] struct. In this way, optional parameters can be omitted. For example:

    ```rust
  # use dftd4::prelude::*;
  # let atm = true;
    let param = DFTD4RationalDampingParamBuilder::default()
        .s8(1.683)
        .a1(1.139)
        .a2(1.5)
        .init();
    // this will give param: DFTD4Param
    ```

- By [`DFTD4Param`] utility. In this way, all parameters must be provided. For example of B3-Zero:

    ```rust
  # use dftd4::prelude::*;
  # let atm = true;
    let param = DFTD4Param::new_rational_damping(
        1.0,   // s6
        1.683, // s8
        1.0,   // s9
        1.139, // a1
        1.0,   // a2
        16.0,  // alp
    );
    ```
