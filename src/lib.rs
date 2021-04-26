//! External Interrupt [Drone OS] driver for STM32F4 micro-controllers.
//!
//! This crate is for managing external interrupts from GPIO pins.
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
//! Example of initializing the driver for GPIO B4:
//!
//! ```no_run
//! # #![feature(const_fn_fn_ptr_basics)]
//! # use drone_stm32_map::stm32_reg_tokens;
//! # use drone_core::token::Token;
//! # stm32_reg_tokens! {
//! #     index => Regs;
//! #     exclude => {
//! #         scb_ccr,
//! #         mpu_type, mpu_ctrl, mpu_rnr, mpu_rbar, mpu_rasr,
//! #     }
//! # }
//! mod thr {
//!     pub use drone_cortexm::thr::init;
//!     pub use drone_stm32_map::thr::*;
//!
//!     use drone_cortexm::thr;
//!
//!     thr::nvic! {
//!         thread => pub Thr {};
//!         local => pub ThrLocal {};
//!         vtable => pub Vtable;
//!         index => pub Thrs;
//!         init => pub ThrsInit;
//!
//!         threads => {
//!             interrupts => {
//!                 /// EXTI Line4 interrupt.
//!                 10: pub exti4;
//!             };
//!         };
//!     }
//! }
//!
//! use crate::thr::ThrsInit;
//! use drone_cortexm::{reg::prelude::*, thr::prelude::*};
//! use drone_stm32_map::periph::{exti::periph_exti4, gpio::periph_gpio_b};
//! use smartoris_exti::{ExtiDrv, ExtiSetup};
//!
//! fn handler(reg: Regs, thr_init: ThrsInit) {
//!     let thr = thr::init(thr_init);
//!
//!     // Enable the interrupt.
//!     thr.exti4.enable_int();
//!
//!     // Configure the GPIO pin.
//!     let gpio_b = periph_gpio_b!(reg);
//!     gpio_b.rcc_busenr_gpioen.set_bit(); // IO port clock enable
//!     gpio_b.gpio_pupdr.pupdr4.write_bits(0b01); // pull-up
//!     gpio_b.gpio_moder.moder4.write_bits(0b00); // input
//!     gpio_b.rcc_busenr_gpioen.clear_bit(); // IO port clock disable
//!
//!     reg.rcc_apb2enr.syscfgen.set_bit(); // system configuration controller clock enabled
//!
//!     let exti4 = ExtiDrv::init(ExtiSetup {
//!         exti: periph_exti4!(reg),
//!         exti_int: thr.exti4,
//!         config: 0b0001, // PB4 pin
//!         falling: true,  // trigger the interrupt on a falling edge
//!         rising: false,  // don't trigger the interrupt on a rising edge
//!     });
//! }
//! # fn main() {
//! #     unsafe { handler(Regs::take(), ThrsInit::take()) };
//! # }
//! ```
//!
//! Example of usage:
//!
//! ```no_run
//! # #![feature(const_fn_fn_ptr_basics)]
//! # use drone_stm32_map::periph::exti::Exti4;
//! # mod thr {
//! #     use drone_stm32_map::thr::*;
//! #     drone_cortexm::thr::nvic! {
//! #         thread => pub Thr {};
//! #         local => pub ThrLocal {};
//! #         vtable => pub Vtable;
//! #         index => pub Thrs;
//! #         init => pub ThrsInit;
//! #         threads => {
//! #             interrupts => {
//! #                 10: pub exti4;
//! #             };
//! #         };
//! #     }
//! # }
//! # async fn handler() {
//! # let mut exti4: smartoris_exti::ExtiDrv<
//! #     Exti4,
//! #     thr::Exti4,
//! # > = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
//! use futures::prelude::*;
//!
//! let mut tachometer = exti4.create_saturating_stream();
//! while let Some(count) = tachometer.next().await {
//!     for _ in 0..count.get() {
//!         println!("rev");
//!     }
//! }
//! # }
//! # fn main() {}
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

pub use self::drv::{ExtiDrv, ExtiOverflow, ExtiSetup};

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
