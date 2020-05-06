//! External Interrupt [Drone OS] driver for STM32F4 micro-controllers.
//!
//! # Usage
//!
//! Add the crate to your `Cargo.toml` dependencies:
//!
//! ```toml
//! [dependencies]
//! smartoris-exti = { version = "0.1.0" }
//! ```
//!
//! Add or extend `std` feature as follows:
//!
//! ```toml
//! [features]
//! std = ["smartoris-exti/std"]
//! ```
//!
//! [Drone OS]: https://www.drone-os.com/

#![feature(prelude_import)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(not(feature = "std"), no_std)]

mod diverged;
mod drv;

pub use self::drv::{ExtiDrv, ExtiSetup};

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
