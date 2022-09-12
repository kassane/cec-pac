#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Processor Clock Control Register \\[7:0\\]
Processor Clock Divide Value (PROC_DIV)"]
    pub proc_clk_ctrl: PROC_CLK_CTRL,
    #[doc = "0x08 - Configures the EC_CLK clock domain"]
    pub slow_clk_ctrl: SLOW_CLK_CTRL,
    #[doc = "0x0c - Oscillator ID Register"]
    pub osc_id: OSC_ID,
    #[doc = "0x10 - PCR Power Reset Status Register"]
    pub pwr_rst_sts: PWR_RST_STS,
    _reserved4: [u8; 0x04],
    #[doc = "0x18 - System Reset Register"]
    pub sys_rst: SYS_RST,
    _reserved5: [u8; 0x08],
    #[doc = "0x24 - Peripheral Privilege Register"]
    pub priv_en_lock: PRIV_EN_LOCK,
    _reserved6: [u8; 0x08],
    #[doc = "0x30 - Sleep Enable 0 Register"]
    pub slp_en_0: SLP_EN_0,
    #[doc = "0x34 - Sleep Enable 1 Register"]
    pub slp_en_1: SLP_EN_1,
    _reserved8: [u8; 0x04],
    #[doc = "0x3c - Sleep Enable 3 Register"]
    pub slp_en_3: SLP_EN_3,
    #[doc = "0x40 - Sleep Enable 4 Register"]
    pub slp_en_4: SLP_EN_4,
    _reserved10: [u8; 0x2c],
    #[doc = "0x70 - Reset Enable 0 Register"]
    pub rst_en_0: RST_EN_0,
    #[doc = "0x74 - Reset Enable 1 Register"]
    pub rst_en_1: RST_EN_1,
    _reserved12: [u8; 0x04],
    #[doc = "0x7c - Reset Enable 3 Register"]
    pub rst_en_3: RST_EN_3,
    #[doc = "0x80 - Reset Enable 4 Register"]
    pub rst_en_4: RST_EN_4,
    #[doc = "0x84 - Peripheral Reset Lock Register"]
    pub periph_rst_en_lock: PERIPH_RST_EN_LOCK,
    _reserved15: [u8; 0x68],
    #[doc = "0xf0 - EC Priviliges 0 Register"]
    pub ec_priv_en0: EC_PRIV_EN0,
    #[doc = "0xf4 - EC Priviliges 1 Register"]
    pub ec_priv_en1: EC_PRIV_EN1,
    #[doc = "0xf8 - EC Priviliges 3 Register"]
    pub ec_priv_en3: EC_PRIV_EN3,
    #[doc = "0xfc - EC Priviliges 4 Register"]
    pub ec_priv_en4: EC_PRIV_EN4,
}
#[doc = "PROC_CLK_CTRL (rw) register accessor: an alias for `Reg<PROC_CLK_CTRL_SPEC>`"]
pub type PROC_CLK_CTRL = crate::Reg<proc_clk_ctrl::PROC_CLK_CTRL_SPEC>;
#[doc = "Processor Clock Control Register \\[7:0\\]
Processor Clock Divide Value (PROC_DIV)"]
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
#[doc = "SYS_RST (rw) register accessor: an alias for `Reg<SYS_RST_SPEC>`"]
pub type SYS_RST = crate::Reg<sys_rst::SYS_RST_SPEC>;
#[doc = "System Reset Register"]
pub mod sys_rst;
#[doc = "PRIV_EN_LOCK (rw) register accessor: an alias for `Reg<PRIV_EN_LOCK_SPEC>`"]
pub type PRIV_EN_LOCK = crate::Reg<priv_en_lock::PRIV_EN_LOCK_SPEC>;
#[doc = "Peripheral Privilege Register"]
pub mod priv_en_lock;
#[doc = "SLP_EN_0 (rw) register accessor: an alias for `Reg<SLP_EN_0_SPEC>`"]
pub type SLP_EN_0 = crate::Reg<slp_en_0::SLP_EN_0_SPEC>;
#[doc = "Sleep Enable 0 Register"]
pub mod slp_en_0;
#[doc = "SLP_EN_1 (rw) register accessor: an alias for `Reg<SLP_EN_1_SPEC>`"]
pub type SLP_EN_1 = crate::Reg<slp_en_1::SLP_EN_1_SPEC>;
#[doc = "Sleep Enable 1 Register"]
pub mod slp_en_1;
#[doc = "SLP_EN_3 (rw) register accessor: an alias for `Reg<SLP_EN_3_SPEC>`"]
pub type SLP_EN_3 = crate::Reg<slp_en_3::SLP_EN_3_SPEC>;
#[doc = "Sleep Enable 3 Register"]
pub mod slp_en_3;
#[doc = "SLP_EN_4 (rw) register accessor: an alias for `Reg<SLP_EN_4_SPEC>`"]
pub type SLP_EN_4 = crate::Reg<slp_en_4::SLP_EN_4_SPEC>;
#[doc = "Sleep Enable 4 Register"]
pub mod slp_en_4;
#[doc = "RST_EN_0 (rw) register accessor: an alias for `Reg<RST_EN_0_SPEC>`"]
pub type RST_EN_0 = crate::Reg<rst_en_0::RST_EN_0_SPEC>;
#[doc = "Reset Enable 0 Register"]
pub mod rst_en_0;
#[doc = "RST_EN_1 (rw) register accessor: an alias for `Reg<RST_EN_1_SPEC>`"]
pub type RST_EN_1 = crate::Reg<rst_en_1::RST_EN_1_SPEC>;
#[doc = "Reset Enable 1 Register"]
pub mod rst_en_1;
#[doc = "RST_EN_3 (rw) register accessor: an alias for `Reg<RST_EN_3_SPEC>`"]
pub type RST_EN_3 = crate::Reg<rst_en_3::RST_EN_3_SPEC>;
#[doc = "Reset Enable 3 Register"]
pub mod rst_en_3;
#[doc = "RST_EN_4 (rw) register accessor: an alias for `Reg<RST_EN_4_SPEC>`"]
pub type RST_EN_4 = crate::Reg<rst_en_4::RST_EN_4_SPEC>;
#[doc = "Reset Enable 4 Register"]
pub mod rst_en_4;
#[doc = "PERIPH_RST_EN_LOCK (rw) register accessor: an alias for `Reg<PERIPH_RST_EN_LOCK_SPEC>`"]
pub type PERIPH_RST_EN_LOCK = crate::Reg<periph_rst_en_lock::PERIPH_RST_EN_LOCK_SPEC>;
#[doc = "Peripheral Reset Lock Register"]
pub mod periph_rst_en_lock;
#[doc = "EC_PRIV_EN0 (rw) register accessor: an alias for `Reg<EC_PRIV_EN0_SPEC>`"]
pub type EC_PRIV_EN0 = crate::Reg<ec_priv_en0::EC_PRIV_EN0_SPEC>;
#[doc = "EC Priviliges 0 Register"]
pub mod ec_priv_en0;
#[doc = "EC_PRIV_EN1 (rw) register accessor: an alias for `Reg<EC_PRIV_EN1_SPEC>`"]
pub type EC_PRIV_EN1 = crate::Reg<ec_priv_en1::EC_PRIV_EN1_SPEC>;
#[doc = "EC Priviliges 1 Register"]
pub mod ec_priv_en1;
#[doc = "EC_PRIV_EN3 (rw) register accessor: an alias for `Reg<EC_PRIV_EN3_SPEC>`"]
pub type EC_PRIV_EN3 = crate::Reg<ec_priv_en3::EC_PRIV_EN3_SPEC>;
#[doc = "EC Priviliges 3 Register"]
pub mod ec_priv_en3;
#[doc = "EC_PRIV_EN4 (rw) register accessor: an alias for `Reg<EC_PRIV_EN4_SPEC>`"]
pub type EC_PRIV_EN4 = crate::Reg<ec_priv_en4::EC_PRIV_EN4_SPEC>;
#[doc = "EC Priviliges 4 Register"]
pub mod ec_priv_en4;
