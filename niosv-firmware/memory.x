MEMORY
{
  RAM   : ORIGIN = 0x00010000, LENGTH = 32K
  FLASH : ORIGIN = 0x00000000, LENGTH = 64K
}

REGION_ALIAS("REGION_TEXT",   FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA",   RAM);
REGION_ALIAS("REGION_BSS",    RAM);
REGION_ALIAS("REGION_HEAP",   RAM);
REGION_ALIAS("REGION_STACK",  RAM);

_stext           = ORIGIN(FLASH);
_heap_size       = 2K;
_max_hart_id     = 0;
_hart_stack_size = 24K;
_stack_start     = ORIGIN(RAM) + LENGTH(RAM);

/* Slots with no #[core_interrupt] IRQ handlers fall back to the default handler */
PROVIDE(_start_MachineSoft_trap  = _start_DefaultHandler_trap);
PROVIDE(_start_MachineTimer_trap = _start_DefaultHandler_trap);
PROVIDE(_start_Irq0_trap  = _start_DefaultHandler_trap);
PROVIDE(_start_Irq1_trap  = _start_DefaultHandler_trap);
PROVIDE(_start_Irq2_trap  = _start_DefaultHandler_trap);
PROVIDE(_start_Irq3_trap  = _start_DefaultHandler_trap);
PROVIDE(_start_Irq4_trap  = _start_DefaultHandler_trap);
PROVIDE(_start_Irq5_trap  = _start_DefaultHandler_trap);
PROVIDE(_start_Irq6_trap  = _start_DefaultHandler_trap);
PROVIDE(_start_Irq7_trap  = _start_DefaultHandler_trap);
PROVIDE(_start_Irq8_trap  = _start_DefaultHandler_trap);
PROVIDE(_start_Irq9_trap  = _start_DefaultHandler_trap);
PROVIDE(_start_Irq10_trap = _start_DefaultHandler_trap);
PROVIDE(_start_Irq11_trap = _start_DefaultHandler_trap);
PROVIDE(_start_Irq12_trap = _start_DefaultHandler_trap);
PROVIDE(_start_Irq13_trap = _start_DefaultHandler_trap);
PROVIDE(_start_Irq14_trap = _start_DefaultHandler_trap);
PROVIDE(_start_Irq15_trap = _start_DefaultHandler_trap);

/* Exception slots with no #[exception] handler fall back to the default one */
PROVIDE(InstructionAddressMisaligned = ExceptionHandler);
PROVIDE(InstructionAccessFault       = ExceptionHandler);
PROVIDE(IllegalInstruction           = ExceptionHandler);
PROVIDE(Breakpoint                   = ExceptionHandler);
PROVIDE(LoadAddressMisaligned        = ExceptionHandler);
PROVIDE(LoadAccessFault              = ExceptionHandler);
PROVIDE(StoreAddressMisaligned       = ExceptionHandler);
PROVIDE(StoreAccessFault             = ExceptionHandler);
PROVIDE(MachineEnvironmentCall       = ExceptionHandler);
PROVIDE(HardwareError                = ExceptionHandler);

/* Build-time checks */
ASSERT(_vector_table % 128 == 0, "vector table not 128-byte aligned");
ASSERT(_start_trap - _vector_table >= 128, "vector table must cover causes 0..31");
