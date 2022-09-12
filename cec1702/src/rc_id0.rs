#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RC_ID Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Reads of this register provide the result of an RC_ID measurement."]
    pub dat: DAT,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RC_ID Control Register"]
pub mod ctrl;
#[doc = "DAT (rw) register accessor: an alias for `Reg<DAT_SPEC>`"]
pub type DAT = crate::Reg<dat::DAT_SPEC>;
#[doc = "Reads of this register provide the result of an RC_ID measurement."]
pub mod dat;
