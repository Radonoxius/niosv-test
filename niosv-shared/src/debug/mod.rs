use core::panic::PanicInfo;

pub mod diagnostics;

use riscv::register::{mcause, mepc, mtval};

use diagnostics::{record_exception, record_panic};

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    record_panic(panic_info);

    halt()
}

// Anything that lands in slot 0 ends up here unless you add #[riscv_rt::exception(...)]
// handlers. Cause 19 (hardware/ECC error) is decoded here too.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn ExceptionHandler(tf: &riscv_rt::TrapFrame) -> ! {
    let cause = mcause::read().bits();
    let tval = mtval::read();
    let epc = mepc::read();
    record_exception(cause, tval, epc, tf.ra);

    halt()
}

fn halt() -> ! {
    loop {}
}