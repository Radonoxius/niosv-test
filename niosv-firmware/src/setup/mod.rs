mod handlers;
mod irqs;

/// Nios V/m (pipelined, CLINT-vectored) core interrupts, numbered by mcause[30:0].
/// Platform irq[n] is cause 16 + n.
#[repr(usize)]
#[riscv::pac_enum(unsafe CoreInterruptNumber)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Interrupts {
    Irq0  = 16,
    Irq1  = 17,
    Irq2  = 18,
    Irq3  = 19,
    Irq4  = 20,
    Irq5  = 21,
    Irq6  = 22,
    Irq7  = 23,
    Irq8  = 24,
    Irq9  = 25,
    Irq10 = 26,
    Irq11 = 27,
    Irq12 = 28,
    Irq13 = 29,
    Irq14 = 30,
    Irq15 = 31,

    MachineSoft  = 3,
    MachineTimer = 7
}

#[repr(usize)]
#[riscv::pac_enum(unsafe ExceptionNumber)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Exceptions {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault       = 1,
    IllegalInstruction           = 2,
    Breakpoint                   = 3,
    LoadAddressMisaligned        = 4,
    LoadAccessFault              = 5,
    StoreAddressMisaligned       = 6,
    StoreAccessFault             = 7,
    MachineEnvironmentCall       = 11,
    HardwareError                = 19
}