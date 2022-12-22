# ur5

A library enabling efficient and effective control of UR5 cobots, in std and no_std environments.

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

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
// Replace * with latest version
ur5 = "*"
```

## Features

This crate can be used without the standard library (`#![no_std]`) by disabling the default `std` feature. Use this in `Cargo.toml`:

```toml
[dependencies.ur5]
// Replace * with latest version
ur5 = "*"
default-features = false
```

The `libm` feature allows for the usage of `floats` in `no_std` builds.

More features to come as this library is developed.

## License

No license provided as of yet.