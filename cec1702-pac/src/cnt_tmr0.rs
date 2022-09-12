#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This bit reflects the current state of the timer's Clock_Required output signal."]
    pub timerx_ctrl: TIMERX_CTRL,
    #[doc = "0x04 - This is the value of the Timer pre-load for the counter.\n This is used by H/W when the counter is to be restarted automatically; this will become the new value of the counter upon restart."]
    pub prld: PRLD,
    #[doc = "0x08 - This register is used in Timer and One-Shot modes to set the lower limit of the timer."]
    pub timerx_rld: TIMERX_RLD,
    #[doc = "0x0c - This register returns the current value of the timer in all modes."]
    pub timerx_cnt: TIMERX_CNT,
}
#[doc = "TIMERX_CTRL (rw) register accessor: an alias for `Reg<TIMERX_CTRL_SPEC>`"]
pub type TIMERX_CTRL = crate::Reg<timerx_ctrl::TIMERX_CTRL_SPEC>;
#[doc = "This bit reflects the current state of the timer's Clock_Required output signal."]
pub mod timerx_ctrl;
#[doc = "PRLD (rw) register accessor: an alias for `Reg<PRLD_SPEC>`"]
pub type PRLD = crate::Reg<prld::PRLD_SPEC>;
#[doc = "This is the value of the Timer pre-load for the counter.\n This is used by H/W when the counter is to be restarted automatically; this will become the new value of the counter upon restart."]
pub mod prld;
#[doc = "TIMERX_RLD (rw) register accessor: an alias for `Reg<TIMERX_RLD_SPEC>`"]
pub type TIMERX_RLD = crate::Reg<timerx_rld::TIMERX_RLD_SPEC>;
#[doc = "This register is used in Timer and One-Shot modes to set the lower limit of the timer."]
pub mod timerx_rld;
#[doc = "TIMERX_CNT (rw) register accessor: an alias for `Reg<TIMERX_CNT_SPEC>`"]
pub type TIMERX_CNT = crate::Reg<timerx_cnt::TIMERX_CNT_SPEC>;
#[doc = "This register returns the current value of the timer in all modes."]
pub mod timerx_cnt;
