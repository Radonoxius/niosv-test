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
    MachineTimer = 7
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