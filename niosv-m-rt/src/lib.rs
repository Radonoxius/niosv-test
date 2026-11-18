#![no_std]

use riscv::interrupt::enable;
use riscv::interrupt::enable_interrupt;

pub mod diagnostics;
mod setup;

/// Machine-level interrupt cause numbers used by the Nios V/m core.
///
/// The enum discriminants match the low cause bits in `mcause`; the top bit of
/// `mcause` indicates that the cause is an interrupt. In CLINT vectored mode,
/// platform interrupt `n` uses cause `16 + n`, so the vector-table entry is at
/// `mtvec.BASE + 4 * (16 + n)`. The `pac_enum` attribute generates the vector
/// table entries for these causes.
#[repr(usize)]
#[riscv::pac_enum(unsafe CoreInterruptNumber)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Interrupts {
    /// Core-local machine software interrupt (cause 3).
    MachineSoft = 3,
    /// Core-local machine timer interrupt (cause 7).
    MachineTimer = 7,

    /// First platform interrupt input (cause 16).
    Irq0 = 16,
    /// Platform interrupt input 1 (cause 17).
    Irq1 = 17,
    /// Platform interrupt input 2 (cause 18).
    Irq2 = 18,
    /// Platform interrupt input 3 (cause 19).
    Irq3 = 19,
    /// Platform interrupt input 4 (cause 20).
    Irq4 = 20,
    /// Platform interrupt input 5 (cause 21).
    Irq5 = 21,
    /// Platform interrupt input 6 (cause 22).
    Irq6 = 22,
    /// Platform interrupt input 7 (cause 23).
    Irq7 = 23,
    /// Platform interrupt input 8 (cause 24).
    Irq8 = 24,
    /// Platform interrupt input 9 (cause 25).
    Irq9 = 25,
    /// Platform interrupt input 10 (cause 26).
    Irq10 = 26,
    /// Platform interrupt input 11 (cause 27).
    Irq11 = 27,
    /// Platform interrupt input 12 (cause 28).
    Irq12 = 28,
    /// Platform interrupt input 13 (cause 29).
    Irq13 = 29,
    /// Platform interrupt input 14 (cause 30).
    Irq14 = 30,
    /// Platform interrupt input 15 (cause 31).
    Irq15 = 31,
}

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
pub unsafe fn enable_global_interrupt(interrupt: Interrupts) {
    // SAFETY: The caller guarantees it is safe to enable interrupt delivery.
    unsafe {
        enable_interrupt(interrupt);
        enable();
    }
}

/// Synchronous machine exception cause numbers recognized by this runtime.
///
/// Each discriminant matches the exception code in `mcause` when its interrupt
/// bit is clear. In vectored mode, exceptions all enter at `mtvec.BASE`; the
/// runtime's exception handler then uses the cause code to identify the fault.
/// `pac_enum` associates these cause numbers with the runtime's exception
/// handling support.
#[repr(usize)]
#[riscv::pac_enum(unsafe ExceptionNumber)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Exceptions {
    /// Instruction address is not aligned to the instruction boundary.
    InstructionAddressMisaligned = 0,
    /// The instruction fetch could not access its target address.
    InstructionAccessFault = 1,
    /// The processor encountered an unsupported or invalid instruction.
    IllegalInstruction = 2,
    /// A breakpoint instruction or debug breakpoint was reached.
    Breakpoint = 3,
    /// A load used an address that is not aligned for its access width.
    LoadAddressMisaligned = 4,
    /// A load could not access its target address.
    LoadAccessFault = 5,
    /// A store used an address that is not aligned for its access width.
    StoreAddressMisaligned = 6,
    /// A store could not access its target address.
    StoreAccessFault = 7,
    /// Code requested a machine-mode environment service with `ecall`.
    MachineEnvironmentCall = 11,
    /// The processor reported a hardware error, such as an ECC error.
    HardwareError = 19,
}
