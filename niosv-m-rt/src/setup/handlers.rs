use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(panic_info: &PanicInfo) -> ! {
    crate::diagnostics::record_panic(panic_info);
    halt()
}

// Anything that lands in slot 0 ends up here unless you add #[riscv_rt::exception(...)]
// handlers. Cause 19 (hardware/ECC error) is decoded here too.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn ExceptionHandler(tf: &riscv_rt::TrapFrame) -> ! {
    let cause = riscv::register::mcause::read().bits();
    let tval = riscv::register::mtval::read();
    let epc = riscv::register::mepc::read();
    crate::diagnostics::record_exception(cause, tval, epc, tf.ra);
    halt()
}

fn halt() -> ! {
    loop {}
}
