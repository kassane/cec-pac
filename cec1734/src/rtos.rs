#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTOS Timer Count Register."]
    pub cnt: CNT,
    #[doc = "0x04 - RTOS Timer Preload Register"]
    pub prld: PRLD,
    #[doc = "0x08 - RTOS Timer Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Soft Interrupt Register"]
    pub softirq: SOFTIRQ,
}
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "RTOS Timer Count Register."]
pub mod cnt;
#[doc = "PRLD (rw) register accessor: an alias for `Reg<PRLD_SPEC>`"]
pub type PRLD = crate::Reg<prld::PRLD_SPEC>;
#[doc = "RTOS Timer Preload Register"]
pub mod prld;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RTOS Timer Control Register"]
pub mod ctrl;
#[doc = "SOFTIRQ (w) register accessor: an alias for `Reg<SOFTIRQ_SPEC>`"]
pub type SOFTIRQ = crate::Reg<softirq::SOFTIRQ_SPEC>;
#[doc = "Soft Interrupt Register"]
pub mod softirq;
