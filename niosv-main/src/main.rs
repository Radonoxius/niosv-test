#![no_main]
#![no_std]

use niosv_shared::interrupts::enable_global_interrupt;
use setup::Interrupts;

pub(crate) mod consts;
pub(crate) mod setup;

mod irqs;

#[riscv_rt::entry]
fn main() -> ! {
    // Enable IRQ0
    unsafe {
        enable_global_interrupt(Interrupts::Irq0)
    };

    loop {
        core::hint::spin_loop();
    }
}