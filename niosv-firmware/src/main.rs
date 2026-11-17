#![no_main]
#![no_std]

use core::ptr::{read_volatile, write_volatile};

use crate::consts::{LED_CNTL_OUTR, SWITCH_CNTL_INR};

pub(crate) mod consts;
mod irqs;

#[riscv_rt::entry]
fn main() -> ! {
    loop {
        unsafe {
            let sw0 = read_volatile(SWITCH_CNTL_INR) & 0x1;
            write_volatile(LED_CNTL_OUTR, sw0);
        }
    }
}
