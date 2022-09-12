#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TACHx Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - TACHx Status Register"]
    pub sts: STS,
    #[doc = "0x08 - TACH HIGH LIMIT Register"]
    pub lim_hi: LIM_HI,
    #[doc = "0x0c - TACHx Low Limit Register"]
    pub lim_lo: LIM_LO,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "TACHx Control Register"]
pub mod ctrl;
#[doc = "STS (rw) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "TACHx Status Register"]
pub mod sts;
#[doc = "LIM_HI (rw) register accessor: an alias for `Reg<LIM_HI_SPEC>`"]
pub type LIM_HI = crate::Reg<lim_hi::LIM_HI_SPEC>;
#[doc = "TACH HIGH LIMIT Register"]
pub mod lim_hi;
#[doc = "LIM_LO (rw) register accessor: an alias for `Reg<LIM_LO_SPEC>`"]
pub type LIM_LO = crate::Reg<lim_lo::LIM_LO_SPEC>;
#[doc = "TACHx Low Limit Register"]
pub mod lim_lo;
