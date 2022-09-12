#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This is the value of the Timer counter. This is updated by Hardware but may be set by Firmware."]
    pub cnt: CNT,
    #[doc = "0x04 - This is the value of the Timer pre-load for the counter. This is used by H/W when the counter is to be restarted\n automatically; this will become the new value of the counter upon restart."]
    pub prld: PRLD,
    #[doc = "0x08 - This is the interrupt status that fires when the timer reaches its limit"]
    pub sts: STS,
    #[doc = "0x0c - This is the interrupt enable for the status EVENT_INTERRUPT bit in the Timer Status Register"]
    pub ien: IEN,
    #[doc = "0x10 - Timer Control Register"]
    pub ctrl: CTRL,
}
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "This is the value of the Timer counter. This is updated by Hardware but may be set by Firmware."]
pub mod cnt;
#[doc = "PRLD (rw) register accessor: an alias for `Reg<PRLD_SPEC>`"]
pub type PRLD = crate::Reg<prld::PRLD_SPEC>;
#[doc = "This is the value of the Timer pre-load for the counter. This is used by H/W when the counter is to be restarted\n automatically; this will become the new value of the counter upon restart."]
pub mod prld;
#[doc = "STS (rw) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "This is the interrupt status that fires when the timer reaches its limit"]
pub mod sts;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "This is the interrupt enable for the status EVENT_INTERRUPT bit in the Timer Status Register"]
pub mod ien;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Timer Control Register"]
pub mod ctrl;
