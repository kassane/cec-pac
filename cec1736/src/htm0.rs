#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 15:0\\]
This register is used to set the Hibernation Timer Preload value."]
    pub prld: PRLD,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - HTimer Control Register"]
    pub ctrl: CTRL,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - The current state of the Hibernation Timer."]
    pub cnt: CNT,
}
#[doc = "PRLD (rw) register accessor: an alias for `Reg<PRLD_SPEC>`"]
pub type PRLD = crate::Reg<prld::PRLD_SPEC>;
#[doc = "15:0\\]
This register is used to set the Hibernation Timer Preload value."]
pub mod prld;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "HTimer Control Register"]
pub mod ctrl;
#[doc = "CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "The current state of the Hibernation Timer."]
pub mod cnt;
