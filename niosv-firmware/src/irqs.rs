#[allow(unused)]
use niosv_m_rt::Interrupts;

#[riscv_rt::core_interrupt(Interrupts::MachineTimer)]
fn machine_timer() {}

#[riscv_rt::core_interrupt(Interrupts::Irq0)]
fn irq0() {}
