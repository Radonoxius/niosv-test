use std::io::Result;

pub(crate) mod utils;

use utils::{flash_firmware, read_firmware_from_disk};

use crate::utils::{FIRMWARE_BIN_NAME, FIRMWARE_SEARCH_PATH, map_lwaxi_bus, start_niosv_main_core};

fn main() -> Result<()> {
    println!("Reading firmware from: {FIRMWARE_SEARCH_PATH}/{FIRMWARE_BIN_NAME}");
    let bin = read_firmware_from_disk()?;

    let lwaxi_base_addr = map_lwaxi_bus()?;

    flash_firmware(bin, lwaxi_base_addr)?;
    println!("Finished flashing firmware!");

    unsafe {
        start_niosv_main_core(lwaxi_base_addr);
    }
    println!("Started Nios-V main core!");

    Ok(())
}