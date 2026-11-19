use riscv::CoreInterruptNumber;
use riscv::interrupt::enable;
use riscv::interrupt::enable_interrupt;

/// Enables the selected machine interrupt and global machine interrupts.
///
/// `interrupt` selects the cause bit to enable in `mie`; setting `mstatus.MIE`
/// then allows enabled machine interrupts to be delivered. Call this only
/// after the runtime has configured the trap vector and it is safe to receive
/// interrupts.
///
/// # Safety
///
/// The selected handler and any state it accesses must be ready, and enabling
/// interrupts must be safe at the call site.
pub unsafe fn enable_global_interrupt<I>(interrupt: I) where I: CoreInterruptNumber {
    // SAFETY: The caller guarantees it is safe to enable interrupt delivery.
    unsafe {
        enable_interrupt(interrupt);
        enable();
    }
}