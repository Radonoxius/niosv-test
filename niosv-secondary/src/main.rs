#![no_main]
#![no_std]

use niosv_shared::interrupts::enable_global_interrupt;

use crate::setup::Interrupts;

pub(crate) mod setup;

#[riscv_rt::entry]
fn main() -> ! {
    unsafe {
        enable_global_interrupt(Interrupts::MachineSoft);
    }

    loop {
        core::hint::spin_loop();
    }
}