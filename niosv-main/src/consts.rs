// Keep the full Platform Designer address map available to sibling modules
#![allow(dead_code)]
pub(crate) const NIOSV_CORE_1_SRAM_BASE: *mut usize = 0x3_0000 as *mut usize;