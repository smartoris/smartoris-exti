[![crates.io](https://img.shields.io/crates/v/smartoris-exti.svg)](https://crates.io/crates/smartoris-exti)
![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# smartoris-exti

External Interrupt [Drone OS] driver for STM32F4 micro-controllers.

This crate is for managing external interrupts from GPIO pins.

## Usage

Add the crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
smartoris-exti = { version = "0.1.0" }
```

Add or extend `std` feature as follows:

```toml
[features]
std = ["smartoris-exti/std"]
```

Example of initializing the driver for GPIO B4:

```rust
mod thr {
    pub use drone_cortexm::thr::init;
    pub use drone_stm32_map::thr::*;

    use drone_cortexm::thr;

    thr::nvic! {
        thread => pub Thr {};
        local => pub ThrLocal {};
        vtable => pub Vtable;
        index => pub Thrs;
        init => pub ThrsInit;

        threads => {
            interrupts => {
                /// EXTI Line4 interrupt.
                10: pub exti4;
            };
        };
    }
}

use crate::thr::ThrsInit;
use drone_cortexm::{reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{exti::periph_exti4, gpio::periph_gpio_b};
use smartoris_exti::{ExtiDrv, ExtiSetup};

fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    // Enable the interrupt.
    thr.exti4.enable_int();

    // Configure the GPIO pin.
    let gpio_b = periph_gpio_b!(reg);
    gpio_b.rcc_busenr_gpioen.set_bit(); // IO port clock enable
    gpio_b.gpio_pupdr.pupdr4.write_bits(0b01); // pull-up
    gpio_b.gpio_moder.moder4.write_bits(0b00); // input
    gpio_b.rcc_busenr_gpioen.clear_bit(); // IO port clock disable

    reg.rcc_apb2enr.syscfgen.set_bit(); // system configuration controller clock enabled

    let exti4 = ExtiDrv::init(ExtiSetup {
        exti: periph_exti4!(reg),
        exti_int: thr.exti4,
        config: 0b0001, // PB4 pin
        falling: true,  // trigger the interrupt on a falling edge
        rising: false,  // don't trigger the interrupt on a rising edge
    });
}
```

Example of usage:

```rust
use futures::prelude::*;

let mut tachometer = exti4.create_saturating_stream();
while let Some(count) = tachometer.next().await {
    for _ in 0..count.get() {
        println!("rev");
    }
}
```

[Drone OS]: https://www.drone-os.com/

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
