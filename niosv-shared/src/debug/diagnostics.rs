//! Panic and exception details that can be inspected with a debugger.
//!
//! This runtime has no configured console. The handlers save diagnostics in
//! [`NIOSV_DIAGNOSTIC`] and then halt, so a debugger can inspect the record.

use core::{
    cmp::min,
    fmt::{self, Write},
    ptr::write_volatile,
};

/// Maximum number of bytes kept from a panic message and source location.
pub const MESSAGE_CAPACITY: usize = 128;

/// Identifies which handler last wrote [`NIOSV_DIAGNOSTIC`].
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticKind {
    /// No panic or exception has been recorded since reset.
    Empty = 0,
    /// A panic was recorded.
    Panic = 1,
    /// A synchronous exception was recorded.
    Exception = 2,
}

/// Fixed-size diagnostic data preserved for inspection after the core halts.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DiagnosticRecord {
    /// Kind of event recorded.
    pub kind: DiagnosticKind,
    /// Raw `mcause` value for an exception; zero for a panic.
    pub mcause: usize,
    /// `mtval`, often the faulting address or illegal instruction; zero for a panic.
    pub mtval: usize,
    /// `mepc`, the address where the exception occurred; zero for a panic.
    pub mepc: usize,
    /// Return address from the saved trap frame; zero for a panic.
    pub ra: usize,
    /// Source line for a panic, or zero when unavailable.
    pub line: u32,
    /// Source column for a panic, or zero when unavailable.
    pub column: u32,
    /// Number of valid bytes in `message`.
    pub message_len: usize,
    /// True when the panic message and location did not fit in `message`.
    pub message_truncated: bool,
    /// Panic message and source location as UTF-8 bytes, when `kind` is `Panic`.
    pub message: [u8; MESSAGE_CAPACITY],
}

impl DiagnosticRecord {
    const fn empty() -> Self {
        Self {
            kind: DiagnosticKind::Empty,
            mcause: 0,
            mtval: 0,
            mepc: 0,
            ra: 0,
            line: 0,
            column: 0,
            message_len: 0,
            message_truncated: false,
            message: [0; MESSAGE_CAPACITY],
        }
    }
}

/// Last panic or exception, exported by name for GDB/JTAG inspection.
///
/// In GDB, inspect it with `p NIOSV_DIAGNOSTIC`. Its contents are updated with
/// a volatile write immediately before the runtime halts.
#[unsafe(no_mangle)]
pub static mut NIOSV_DIAGNOSTIC: DiagnosticRecord = DiagnosticRecord::empty();

/// Record panic details without allocating, then publish them for the debugger.
pub fn record_panic(info: &core::panic::PanicInfo<'_>) {
    let mut record = DiagnosticRecord {
        kind: DiagnosticKind::Panic,
        ..DiagnosticRecord::empty()
    };

    let location = info.location();
    if let Some(location) = location {
        record.line = location.line();
        record.column = location.column();
    }

    let mut message = MessageWriter(&mut record);
    if let Some(location) = location {
        let _ = write!(
            message,
            "{}:{}:{}: ",
            location.file(),
            location.line(),
            location.column()
        );
    }
    let _ = write!(message, "{}", info.message());
    publish(record);
}

/// Record the machine trap CSRs and saved return address for an exception.
pub fn record_exception(mcause: usize, mtval: usize, mepc: usize, ra: usize) {
    publish(DiagnosticRecord {
        kind: DiagnosticKind::Exception,
        mcause,
        mtval,
        mepc,
        ra,
        ..DiagnosticRecord::empty()
    });
}

fn publish(record: DiagnosticRecord) {
    // A volatile store keeps the record visible to JTAG/GDB before the halt loop.
    unsafe { write_volatile(core::ptr::addr_of_mut!(NIOSV_DIAGNOSTIC), record) };
}

struct MessageWriter<'a>(&'a mut DiagnosticRecord);

impl Write for MessageWriter<'_> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let record = &mut *self.0;
        let available = MESSAGE_CAPACITY - record.message_len;
        let mut count = min(available, text.len());
        while !text.is_char_boundary(count) {
            count -= 1;
        }
        let end = record.message_len + count;
        record.message[record.message_len..end].copy_from_slice(&text.as_bytes()[..count]);
        record.message_len = end;
        record.message_truncated |= count < text.len();
        Ok(())
    }
}
