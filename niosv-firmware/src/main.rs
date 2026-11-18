#![no_main]
#![no_std]

use niosv_m_rt::{Interrupts, enable_global_interrupt};

pub(crate) mod consts;
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