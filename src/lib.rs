#![no_std]

/*!
# URScript

A library enabling efficient and effective control of UR e-series cobots, in std and no_std environments.

This library currently provides:
* Support for no-std environemnts
* Preproccessing of variables related to URScript
* Implementation of the pose type seen in URScript

This library is aiming to provide:
* Formatting of variables and functions in a way compatible with URScript
* Preproccessing of some functions and variables
* Implementation of all types related to URScript
* Async helper functions
* Ability to deserialize data from UR5 robot into strings or variables

## Example

```
todo!();
```

## Compatibility

The `ur_script` crate is tested for:
* rustc 1.65.0
* URScript 5.12
*/

#![deny(
    missing_docs,
    nonstandard_style,
    unused_variables,
    unused_mut,
    unused_parens,
    unused_qualifications,
    rust_2018_idioms,
    rust_2018_compatibility,
    future_incompatible,
    missing_copy_implementations
)]

pub mod vars;
// pub mod functions;

// #[cfg(feature = "tcp")]
// pub mod control;
