#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The Power-Fail and Reset Status Register collects and retains the VBAT RST and WDT event status when VCC1 is unpowered."]
    pub pfrs: PFRS,
    _reserved1: [u8; 0x07],
    #[doc = "0x08 - CLOCK ENABLE"]
    pub clk32_en: CLK32_EN,
    #[doc = "0x0c - System Shutdown Enable register.\n"]
    pub sys_shdn: SYS_SHDN,
    _reserved3: [u8; 0x10],
    #[doc = "0x20 - MONOTONIC COUNTER"]
    pub mcnt_lo: MCNT_LO,
    #[doc = "0x24 - COUNTER HIWORD"]
    pub mcnt_hi: MCNT_HI,
    #[doc = "0x28 - VWIRE_BACKUP"]
    pub vwr_bckp: VWR_BCKP,
}
#[doc = "PFRS (rw) register accessor: an alias for `Reg<PFRS_SPEC>`"]
pub type PFRS = crate::Reg<pfrs::PFRS_SPEC>;
#[doc = "The Power-Fail and Reset Status Register collects and retains the VBAT RST and WDT event status when VCC1 is unpowered."]
pub mod pfrs;
#[doc = "CLK32_EN (rw) register accessor: an alias for `Reg<CLK32_EN_SPEC>`"]
pub type CLK32_EN = crate::Reg<clk32_en::CLK32_EN_SPEC>;
#[doc = "CLOCK ENABLE"]
pub mod clk32_en;
#[doc = "SYS_SHDN (rw) register accessor: an alias for `Reg<SYS_SHDN_SPEC>`"]
pub type SYS_SHDN = crate::Reg<sys_shdn::SYS_SHDN_SPEC>;
#[doc = "System Shutdown Enable register.\n"]
pub mod sys_shdn;
#[doc = "MCNT_LO (rw) register accessor: an alias for `Reg<MCNT_LO_SPEC>`"]
pub type MCNT_LO = crate::Reg<mcnt_lo::MCNT_LO_SPEC>;
#[doc = "MONOTONIC COUNTER"]
pub mod mcnt_lo;
#[doc = "MCNT_HI (rw) register accessor: an alias for `Reg<MCNT_HI_SPEC>`"]
pub type MCNT_HI = crate::Reg<mcnt_hi::MCNT_HI_SPEC>;
#[doc = "COUNTER HIWORD"]
pub mod mcnt_hi;
#[doc = "VWR_BCKP (rw) register accessor: an alias for `Reg<VWR_BCKP_SPEC>`"]
pub type VWR_BCKP = crate::Reg<vwr_bckp::VWR_BCKP_SPEC>;
#[doc = "VWIRE_BACKUP"]
pub mod vwr_bckp;
