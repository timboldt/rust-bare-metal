//! Sends "Hello, world!" through the ITM port 0
//!
//! ITM is much faster than semihosting. Like 4 orders of magnitude or so.
//!
//! **NOTE** Cortex-M0 chips don't support ITM.
//!
//! You'll have to connect the microcontroller's SWO pin to the SWD interface. Note that some
//! development boards don't provide this option.
//!
//! You'll need [`itmdump`] to receive the message on the host plus you'll need to uncomment two
//! `monitor` commands in the `.gdbinit` file.
//!
//! [`itmdump`]: https://docs.rs/itm/0.2.1/itm/
//!
//! ---

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::{asm, iprintln};
use cortex_m_rt::entry;
use stm32f3;

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f3::stm32f303::Peripherals::take().unwrap();
    let stim = &mut cp.ITM.stim[0];
    let rcc = dp.RCC;
    rcc.ahbenr.modify(|_, w| {
        w.iopaen()
            .set_bit() // Enable GPIOA.
            .iopeen()
            .set_bit() // Enable GPIOE.
    });

    let gpioa = dp.GPIOA;
    gpioa.moder.write(
        |w| w.moder0().input(), // PA0 is user button 1.
    );

    let gpioe = dp.GPIOE;
    gpioe.moder.write(
        |w| w.moder11().output(), // PE11 is the green LED.
    );

    iprintln!(stim, "Init complete");

    loop {
        if gpioa.idr.read().idr0().is_high() {
            gpioe.bsrr.write(|w| w.bs11().set_bit());
        } else {
            gpioe.bsrr.write(|w| w.br11().set_bit());
        }
        // for _ in 0..10000 {
        //     asm::nop();
        // }
    }
}
