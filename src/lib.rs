// src/lib.rs
//
// Copyright (C) 2023 James Petersen <m@jamespetersen.ca>
// Licensed under Apache 2.0 OR MIT. See LICENSE-APACHE or LICENSE-MIT

#![warn(missing_docs)]

//! This crate exists to provide a portable method to getting any user's home
//! directory. The API is rather simple: there are two functions,
//! [`get_home`] and [`get_my_home`]. The former can get the home directory
//! of any user provided you have their username. The latter can get the home
//! directory of the user executing this process.
//!
//! This crate aims to work on both Windows and Unix systems. However,
//! Unix systems do not have a unified API. This crate may not work
//! on Unix systems which do not have the `getpwnam_r(3)`, `getpwuid_r(3)`,
//! and `getuid(2)` functions.
//!
//! # Usage
//! This crate is on [crates.io](https://crates.io/crates/homedir) and can be used by executing `cargo add homedir`
//! or adding the following to the dependencies in your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! homedir = "0.1.0"
//! ```
//!
//! By default, on Unix, if the library cannot find the current user's home directory
//! in the `/etc/passwd` file, it will return `None`. However, it is possible that
//! some users want it to check the `$HOME` environment variable instead.
//! The `check_env` feature flag can be set to enable this behaviour. To enable
//! this feature, replace the above lines in the `Cargo.toml` with
//!
//! ```toml
//! [dependencies]
//! homedir = { version = '0.1.0", features = ["fast_fail"] }
//! ```
//!
//! This feature is only useful on Unix systems; it has no effect on Windows.
//!
//! # Examples
//! ## Get the process' user's home directory.
//! ```no_run
//! use homedir::get_my_home;
//!
//! // This assumes that the process' user has "/home/jpetersen" as home directory.
//! assert_eq!(
//!     "/home/jpetersen".as_ref(),
//!     get_my_home().unwrap().unwrap().as_path()
//! );
//! ```
//!
//! ## Get an arbitrary user's home directory.
//! ```no_run
//! use homedir::get_home;
//!
//! // This assumes there is a user named `Administrator` which has
//! // `C:\Users\Administrator` as a home directory.
//! assert_eq!(
//!     "C:\\Users\\Administrator".as_ref(),
//!     get_home("Administrator").unwrap().unwrap().as_path()
//! );
//! assert!(get_home("NonExistentUser").unwrap().is_none());
//! ```

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(windows)] {
        /// Contains the implementation of the crate for Windows systems.
        pub mod windows;
        pub use windows::get_home;
        pub use windows::get_my_home;
    } else if #[cfg(unix)] {
        /// Contains the implementation of the crate for Unix systems.
        pub mod unix;
        pub use unix::get_home;
        pub use unix::get_my_home;
    } else {
        compile_error!("this crate only supports windows and unix systems");
    }
}
