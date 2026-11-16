use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

// Anything that lands in slot 0 ends up here unless you add #[riscv_rt::exception(...)]
// handlers. Cause 19 (hardware/ECC error) is decoded here too.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn ExceptionHandler(_tf: &riscv_rt::TrapFrame) -> ! {
    let cause = riscv::register::mcause::read().bits();
    let tval = riscv::register::mtval::read();
    let _ = (cause, tval);
    loop {}
}