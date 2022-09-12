#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Sleep Control"]
    pub sys_slp_ctrl: SYS_SLP_CTRL,
    #[doc = "0x04 - Processor Clock Control Register \\[7:0\\]
Processor Clock Divide Value (PROC_DIV)\n 1: divide 48 MHz Ring Oscillator by 1.\n 3: divide 48 MHz Ring Oscillator by 3.\n 4: divide 48 MHz Ring Oscillator by 4.\n 16: divide 48 MHz Ring Oscillator by 16.\n 48: divide 48 MHz Ring Oscillator by 48.\n No other values are supported."]
    pub proc_clk_ctrl: PROC_CLK_CTRL,
    #[doc = "0x08 - Configures the EC_CLK clock domain"]
    pub slow_clk_ctrl: SLOW_CLK_CTRL,
    #[doc = "0x0c - Oscillator ID Register"]
    pub osc_id: OSC_ID,
    #[doc = "0x10 - PCR Power Reset Status Register"]
    pub pwr_rst_sts: PWR_RST_STS,
    #[doc = "0x14 - Power Reset Control Register"]
    pub pwr_rst_ctrl: PWR_RST_CTRL,
    #[doc = "0x18 - System Reset Register"]
    pub sys_rst: SYS_RST,
    _reserved7: [u8; 0x14],
    #[doc = "0x30 - Sleep Enable 0 Register"]
    pub slp_en_0: SLP_EN_0,
    #[doc = "0x34 - Sleep Enable 1 Register"]
    pub slp_en_1: SLP_EN_1,
    #[doc = "0x38 - Sleep Enable 2 Register"]
    pub slp_en_2: SLP_EN_2,
    #[doc = "0x3c - Sleep Enable 3 Register"]
    pub slp_en_3: SLP_EN_3,
    #[doc = "0x40 - Sleep Enable 4 Register"]
    pub slp_en_4: SLP_EN_4,
    _reserved12: [u8; 0x0c],
    #[doc = "0x50 - Clock Required 0 Register"]
    pub clk_req_0: CLK_REQ_0,
    #[doc = "0x54 - Clock Required 1 Register"]
    pub clk_req_1: CLK_REQ_1,
    #[doc = "0x58 - Clock Required 2 Register"]
    pub clk_req_2: CLK_REQ_2,
    #[doc = "0x5c - Clock Required 3 Register"]
    pub clk_req_3: CLK_REQ_3,
    #[doc = "0x60 - Clock Required 4 Register"]
    pub clk_req_4: CLK_REQ_4,
    _reserved17: [u8; 0x0c],
    #[doc = "0x70 - Reset Enable 0 Register"]
    pub rst_en_0: RST_EN_0,
    #[doc = "0x74 - Reset Enable 1 Register"]
    pub rst_en_1: RST_EN_1,
    #[doc = "0x78 - Reset Enable 2 Register"]
    pub rst_en_2: RST_EN_2,
    #[doc = "0x7c - Reset Enable 3 Register"]
    pub rst_en_3: RST_EN_3,
    #[doc = "0x80 - Reset Enable 4 Register"]
    pub rst_en_4: RST_EN_4,
}
#[doc = "SYS_SLP_CTRL (rw) register accessor: an alias for `Reg<SYS_SLP_CTRL_SPEC>`"]
pub type SYS_SLP_CTRL = crate::Reg<sys_slp_ctrl::SYS_SLP_CTRL_SPEC>;
#[doc = "System Sleep Control"]
pub mod sys_slp_ctrl;
#[doc = "PROC_CLK_CTRL (rw) register accessor: an alias for `Reg<PROC_CLK_CTRL_SPEC>`"]
pub type PROC_CLK_CTRL = crate::Reg<proc_clk_ctrl::PROC_CLK_CTRL_SPEC>;
#[doc = "Processor Clock Control Register \\[7:0\\]
Processor Clock Divide Value (PROC_DIV)\n 1: divide 48 MHz Ring Oscillator by 1.\n 3: divide 48 MHz Ring Oscillator by 3.\n 4: divide 48 MHz Ring Oscillator by 4.\n 16: divide 48 MHz Ring Oscillator by 16.\n 48: divide 48 MHz Ring Oscillator by 48.\n No other values are supported."]
pub mod proc_clk_ctrl;
#[doc = "SLOW_CLK_CTRL (rw) register accessor: an alias for `Reg<SLOW_CLK_CTRL_SPEC>`"]
pub type SLOW_CLK_CTRL = crate::Reg<slow_clk_ctrl::SLOW_CLK_CTRL_SPEC>;
#[doc = "Configures the EC_CLK clock domain"]
pub mod slow_clk_ctrl;
#[doc = "OSC_ID (rw) register accessor: an alias for `Reg<OSC_ID_SPEC>`"]
pub type OSC_ID = crate::Reg<osc_id::OSC_ID_SPEC>;
#[doc = "Oscillator ID Register"]
pub mod osc_id;
#[doc = "PWR_RST_STS (rw) register accessor: an alias for `Reg<PWR_RST_STS_SPEC>`"]
pub type PWR_RST_STS = crate::Reg<pwr_rst_sts::PWR_RST_STS_SPEC>;
#[doc = "PCR Power Reset Status Register"]
pub mod pwr_rst_sts;
#[doc = "PWR_RST_CTRL (rw) register accessor: an alias for `Reg<PWR_RST_CTRL_SPEC>`"]
pub type PWR_RST_CTRL = crate::Reg<pwr_rst_ctrl::PWR_RST_CTRL_SPEC>;
#[doc = "Power Reset Control Register"]
pub mod pwr_rst_ctrl;
#[doc = "SYS_RST (rw) register accessor: an alias for `Reg<SYS_RST_SPEC>`"]
pub type SYS_RST = crate::Reg<sys_rst::SYS_RST_SPEC>;
#[doc = "System Reset Register"]
pub mod sys_rst;
#[doc = "SLP_EN_0 (rw) register accessor: an alias for `Reg<SLP_EN_0_SPEC>`"]
pub type SLP_EN_0 = crate::Reg<slp_en_0::SLP_EN_0_SPEC>;
#[doc = "Sleep Enable 0 Register"]
pub mod slp_en_0;
#[doc = "SLP_EN_1 (rw) register accessor: an alias for `Reg<SLP_EN_1_SPEC>`"]
pub type SLP_EN_1 = crate::Reg<slp_en_1::SLP_EN_1_SPEC>;
#[doc = "Sleep Enable 1 Register"]
pub mod slp_en_1;
#[doc = "SLP_EN_2 (rw) register accessor: an alias for `Reg<SLP_EN_2_SPEC>`"]
pub type SLP_EN_2 = crate::Reg<slp_en_2::SLP_EN_2_SPEC>;
#[doc = "Sleep Enable 2 Register"]
pub mod slp_en_2;
#[doc = "SLP_EN_3 (rw) register accessor: an alias for `Reg<SLP_EN_3_SPEC>`"]
pub type SLP_EN_3 = crate::Reg<slp_en_3::SLP_EN_3_SPEC>;
#[doc = "Sleep Enable 3 Register"]
pub mod slp_en_3;
#[doc = "SLP_EN_4 (rw) register accessor: an alias for `Reg<SLP_EN_4_SPEC>`"]
pub type SLP_EN_4 = crate::Reg<slp_en_4::SLP_EN_4_SPEC>;
#[doc = "Sleep Enable 4 Register"]
pub mod slp_en_4;
#[doc = "CLK_REQ_0 (rw) register accessor: an alias for `Reg<CLK_REQ_0_SPEC>`"]
pub type CLK_REQ_0 = crate::Reg<clk_req_0::CLK_REQ_0_SPEC>;
#[doc = "Clock Required 0 Register"]
pub mod clk_req_0;
#[doc = "CLK_REQ_1 (rw) register accessor: an alias for `Reg<CLK_REQ_1_SPEC>`"]
pub type CLK_REQ_1 = crate::Reg<clk_req_1::CLK_REQ_1_SPEC>;
#[doc = "Clock Required 1 Register"]
pub mod clk_req_1;
#[doc = "CLK_REQ_2 (rw) register accessor: an alias for `Reg<CLK_REQ_2_SPEC>`"]
pub type CLK_REQ_2 = crate::Reg<clk_req_2::CLK_REQ_2_SPEC>;
#[doc = "Clock Required 2 Register"]
pub mod clk_req_2;
#[doc = "CLK_REQ_3 (rw) register accessor: an alias for `Reg<CLK_REQ_3_SPEC>`"]
pub type CLK_REQ_3 = crate::Reg<clk_req_3::CLK_REQ_3_SPEC>;
#[doc = "Clock Required 3 Register"]
pub mod clk_req_3;
#[doc = "CLK_REQ_4 (rw) register accessor: an alias for `Reg<CLK_REQ_4_SPEC>`"]
pub type CLK_REQ_4 = crate::Reg<clk_req_4::CLK_REQ_4_SPEC>;
#[doc = "Clock Required 4 Register"]
pub mod clk_req_4;
#[doc = "RST_EN_0 (rw) register accessor: an alias for `Reg<RST_EN_0_SPEC>`"]
pub type RST_EN_0 = crate::Reg<rst_en_0::RST_EN_0_SPEC>;
#[doc = "Reset Enable 0 Register"]
pub mod rst_en_0;
#[doc = "RST_EN_1 (rw) register accessor: an alias for `Reg<RST_EN_1_SPEC>`"]
pub type RST_EN_1 = crate::Reg<rst_en_1::RST_EN_1_SPEC>;
#[doc = "Reset Enable 1 Register"]
pub mod rst_en_1;
#[doc = "RST_EN_2 (rw) register accessor: an alias for `Reg<RST_EN_2_SPEC>`"]
pub type RST_EN_2 = crate::Reg<rst_en_2::RST_EN_2_SPEC>;
#[doc = "Reset Enable 2 Register"]
pub mod rst_en_2;
#[doc = "RST_EN_3 (rw) register accessor: an alias for `Reg<RST_EN_3_SPEC>`"]
pub type RST_EN_3 = crate::Reg<rst_en_3::RST_EN_3_SPEC>;
#[doc = "Reset Enable 3 Register"]
pub mod rst_en_3;
#[doc = "RST_EN_4 (rw) register accessor: an alias for `Reg<RST_EN_4_SPEC>`"]
pub type RST_EN_4 = crate::Reg<rst_en_4::RST_EN_4_SPEC>;
#[doc = "Reset Enable 4 Register"]
pub mod rst_en_4;
