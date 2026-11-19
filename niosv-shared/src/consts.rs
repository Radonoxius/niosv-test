pub const MAIN_CORE_HART_ID: usize = 0x0;

pub const NIOSV_CORE_ROM_BASE: *const usize = 0x0 as *const usize;
pub const NIOSV_CORE_ROM_LEN: usize = 0x1_0000;

pub const NIOSV_CORE_SRAM_BASE: *mut usize = 0x1_0000 as *mut usize;
pub const NIOSV_CORE_SRAM_LEN: usize = 0x8000;

pub const NIOSV_CORES_RESET_BASE: *mut usize = 0x1_8000 as *mut usize;
pub const NIOSV_CORES_RESETR_LEN: usize = 0x10;

pub const LED_CNTL_OUTR: *mut usize = 0x1_8140 as *mut usize;
pub const SWITCH_CNTL_IN_IRQCLRR: *mut usize = 0x1_8144 as *mut usize;

pub const NIOSV_TIM_SW_AGENT_BASE: *mut usize = 0x1_8100 as *mut usize;
pub const NIOSV_TIM_SW_AGENT_LEN: usize = 0x40;

pub const NIOSV_DM_AGENT_BASE: *mut usize = 0x2_0000 as *mut usize;
pub const NIOSV_DM_AGENT_LEN: usize = 0x1_0000;