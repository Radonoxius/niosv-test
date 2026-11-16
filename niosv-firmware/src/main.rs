#![no_main]
#![no_std]

pub(crate) mod setup;

use core::ptr::{read_volatile, write_volatile};

use niosv_firmware::{LED_CNTL_OUTR, SWITCH_CNTL_INR};

#[riscv_rt::entry]
fn main() -> ! {
    loop {
        unsafe {
            let sw0 = read_volatile(SWITCH_CNTL_INR) & 0x1;
            write_volatile(LED_CNTL_OUTR, sw0);
        }
    }
}