use core::ptr::{read_volatile, write_volatile};
use crate::setup::Interrupts;

use niosv_shared::consts::{LED_CNTL_OUTR, SWITCH_CNTL_IN_IRQCLRR};

#[riscv_rt::core_interrupt(Interrupts::Irq0)]
fn irq0() {
    unsafe {
        let switches = read_volatile(SWITCH_CNTL_IN_IRQCLRR);
        write_volatile(LED_CNTL_OUTR, switches);

        // Write-one-to-clear the switch controller's pending bit 0.
        write_volatile(SWITCH_CNTL_IN_IRQCLRR, 1);
    }
}