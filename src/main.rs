use libc::{
    MAP_FAILED, MAP_SHARED, O_RDWR, O_SYNC, PROT_READ, PROT_WRITE, c_char, mmap, off_t, open,
};
use std::ptr::{self, read_volatile, write_volatile};

pub const LW_HPS_TO_FPGA_BASE: usize = 0xFF20_0000;
pub const LW_HPS_TO_FPGA_SPAN: usize = 0x0020_0000;

pub const SWITCH_IN_REGISTER_OFFSET: usize = 0x4;
pub const LED_OUT_REGISTER_OFFSET: usize = 0x0;

fn main() {
    unsafe {
        let fd = open(
            b"/dev/mem\0".as_ptr() as *const c_char,
            O_RDWR | O_SYNC
        );
        if fd < 0 {
            panic!("open /dev/mem failed: {}", std::io::Error::last_os_error());
        }
        let base = mmap(
            ptr::null_mut(),
            LW_HPS_TO_FPGA_SPAN,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            LW_HPS_TO_FPGA_BASE as off_t,
        );
        if base == MAP_FAILED {
            panic!("mmap failed: {}", std::io::Error::last_os_error());
        }

        let base = base as *mut u32;

        let switch_inr = base.byte_add(SWITCH_IN_REGISTER_OFFSET);
        let led_outr = base.byte_add(LED_OUT_REGISTER_OFFSET);

        loop {
            write_volatile(
                led_outr,
                read_volatile(switch_inr)
            );
        }
    }
}