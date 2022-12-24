#![no_std]

/*!
# `URScript` for Rust

A library enabling efficient and effective control of UR e-series cobots, in both `std` and `no_std` environments.

This library currently provides:
* Support for `no-std` environemnts
* Preproccessing of variables related to `URScript`
* Implementation of the pose type seen in `URScript`

This library is aiming to provide:
* Formatting of variables and functions in a way compatible with `URScript`
* Preproccessing of some functions and variables
* Implementation of all types related to `URScript`
* Async helper functions
* Ability to deserialize data from UR5 robot into strings or variables

## Example

```
# #[cfg(any(feature = "std", feature = "libm"))]
use ur_script::vars::pose::*;
use core::f32::consts::PI;

# #[cfg(any(feature = "std", feature = "libm"))]
let home = Pose::new_pose(0., 1., 2., 0., PI, 0.);
# #[cfg(any(feature = "std", feature = "libm"))]
let wobj_diagonal = Pose::new_pos(0., 2., 3.);
# #[cfg(any(feature = "std", feature = "libm"))]
let target1 = wobj_diagonal*home;
# #[cfg(any(feature = "std", feature = "libm"))]
let target2 = wobj_diagonal*target1;

# #[cfg(any(feature = "std", feature = "libm"))]
println!("{}", target2);
```

## Compatibility

The `ur_script` crate is tested for:
* rustc 1.65.0
* [URScript 5.12](https://www.universal-robots.com/download/manuals-e-series/script/script-manual-e-series-sw-512)
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

#[cfg(any(feature = "std", feature = "libm"))]
pub use vars::pose::Pose;
