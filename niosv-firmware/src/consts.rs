// Keep the full Platform Designer address map available to sibling modules
#![allow(dead_code)]

pub(crate) const MAIN_CORE_HART_ID: usize = 0x0;

pub(crate) const NIOSV_CORE_ROM_BASE: usize = 0x0;
pub(crate) const NIOSV_CORE_ROM_LEN: usize = 0x8000;

pub(crate) const NIOSV_CORE_SRAM_BASE: *mut usize = 0x1_0000 as *mut usize;
pub(crate) const NIOSV_CORE_SRAM_LEN: usize = 0x8000;

pub(crate) const NIOSV_CORES_RESET_BASE: *mut usize = 0x8000 as *mut usize;
pub(crate) const NIOSV_CORES_RESETR_LEN: usize = 0x10;

pub(crate) const LED_CNTL_OUTR: *mut usize = 0x8140 as *mut usize;
pub(crate) const SWITCH_CNTL_INR: *const usize = 0x8144 as *const usize;

pub(crate) const NIOSV_TIM_SW_AGENT_BASE: *mut usize = 0x8100 as *mut usize;
pub(crate) const NIOSV_TIM_SW_AGENT_LEN: usize = 0x40;

pub(crate) const NIOSV_DM_AGENT_BASE: *mut usize = 0x2_0000 as *mut usize;
pub(crate) const NIOSV_DM_AGENT_LEN: usize = 0x1_0000;

pub(crate) const NIOSV_CORE_1_SRAM_BASE: *mut usize = 0x3_0000 as *mut usize;
