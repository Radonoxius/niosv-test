#[allow(unused)]
use crate::setup::Interrupts;

#[riscv_rt::core_interrupt(Interrupts::MachineTimer)]
fn machine_timer() {
    // reload mtimecmp / clear the timer source
}

#[riscv_rt::core_interrupt(Interrupts::Irq0)]
fn irq0() {

}