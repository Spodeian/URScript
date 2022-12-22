# URScript for Rust

A library enabling efficient and effective control of UR5 cobots, in `std` and `no_std` environments.

This library currently provides:
* Support for `no-std` environemnts
* Preproccessing of variables related to **URScript**
* Implementation of the pose type seen in **URScript**

This library is aiming to provide:
* Formatting of variables and functions in a way compatible with **URScript**
* Preproccessing of some functions and variables
* Implementation of all types related to **URScript**
* Async helper functions
* Ability to deserialize data from UR5 robot into `Strings` or native variables

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
// Replace * with latest version
ur_script = "*"
```

## Features

This crate can be used with the standard library by enabling the `std` feature and optionally disabling the default `libm` feature. Use this in `Cargo.toml`:

```toml
[dependencies.ur_script]
// Replace * with latest version
ur_script = "*"
features = ["std"]
```

The `libm` feature allows for the usage of `floats` in `no_std` builds.

More features to come as this library is developed.

## License
<a rel="license" href="http://creativecommons.org/licenses/by-nc/4.0/"><img alt="Creative Commons Licence" style="border-width:0" src="https://i.creativecommons.org/l/by-nc/4.0/88x31.png" /></a><br />This work is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-nc/4.0/">Creative Commons Attribution-NonCommercial 4.0 International License</a>.

## Attributions
This version of this library is built with [nalgebra](https://nalgebra.org) as a core dependency, which licensed under the [Apache 2.0 license](http://www.apache.org/licenses/LICENSE-2.0).
