use std::{fs::OpenOptions, io::{Error, ErrorKind::InvalidInput, Read, Result}, ptr::{null_mut, write_volatile}};

use libc::{MAP_FAILED, MAP_SHARED, O_RDWR, O_SYNC, PROT_READ, PROT_WRITE, c_char, mmap, off_t, open};
use std::ptr::copy_nonoverlapping;

pub(crate) const FIRMWARE_SEARCH_PATH: &str = "/lib/firmware";
pub(crate) const FIRMWARE_BIN_NAME: &str = "niosv.bin";

const FIRMWARE_MAX_SIZE: usize = NIOSV_BOOT_ROM_LEN;

const LW_HPS_TO_FPGA_BASE: usize = 0xFF20_0000;
const LW_HPS_TO_FPGA_SPAN: usize = 0x0020_0000;

const NIOSV_BOOT_ROM_BASE: usize = 0x1_0000;
const NIOSV_BOOT_ROM_LEN: usize = 0x1_0000;

const NIOSV_MAIN_RESET_PIO_BASE: usize = 0x2_0000;
const _NIOSV_MAIN_RESET_PIO_LEN: usize = 0x10;

pub(crate) fn read_firmware_from_disk() -> Result<Vec<u8>> {
    let mut bin: Vec<u8> = Vec::with_capacity(FIRMWARE_MAX_SIZE);

    let res_file = OpenOptions::new()
        .read(true)
        .open(
            format!("{FIRMWARE_SEARCH_PATH}/{FIRMWARE_BIN_NAME}")
        );

    match res_file {
        Ok(mut file) => {
            let res_count = file.read_to_end(&mut bin);

            match res_count {
                Ok(count) => {
                    if count == 0 || count > FIRMWARE_MAX_SIZE {
                        Err(Error::new(InvalidInput, "Invalid binary size!"))
                    } else {
                        Ok(bin)
                    }
                },
                Err(e) => Err(e)
            }
        },
        Err(e) => Err(e)
    }
}

pub(crate) fn map_lwaxi_bus() -> Result<usize> {
    unsafe {
        let devmem_fd = open(
            b"/dev/mem\0".as_ptr() as *const c_char,
            O_RDWR | O_SYNC
        );
        if devmem_fd < 0 {
            return Err(std::io::Error::last_os_error());
        }
        let lwaxi_base = mmap(
            null_mut(),
            LW_HPS_TO_FPGA_SPAN,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            devmem_fd,
            LW_HPS_TO_FPGA_BASE as off_t,
        );
        if lwaxi_base == MAP_FAILED {
            return Err(std::io::Error::last_os_error());
        }

        Ok(lwaxi_base as usize)
    }
}

pub(crate) fn flash_firmware(bin: Vec<u8>, lwaxi_base: usize) -> Result<()> {
    unsafe {
        let lwaxi_base = lwaxi_base as *mut u8;
        let nios_rom = lwaxi_base.byte_add(NIOSV_BOOT_ROM_BASE);

        copy_nonoverlapping(bin.as_ptr(), nios_rom, bin.len());
    }

    Ok(())
}

pub(crate) unsafe fn start_niosv_main_core(lwaxi_base: usize) {
    unsafe {
        let lwaxi_base = lwaxi_base as *mut u32;
        let nios_main_reset_datar = lwaxi_base.byte_add(NIOSV_MAIN_RESET_PIO_BASE);

        write_volatile(nios_main_reset_datar, 0x0);
    }
}