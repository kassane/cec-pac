#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Week Alarm Counter Register"]
    pub alarm_cnt: ALARM_CNT,
    #[doc = "0x08 - Week Timer Compare Register"]
    pub tmr_comp: TMR_COMP,
    #[doc = "0x0c - Clock Divider Register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x10 - Sub-Second Programmable Interrupt Select Register"]
    pub ss_intr_sel: SS_INTR_SEL,
    #[doc = "0x14 - Sub-Week Control Register"]
    pub swk_ctrl: SWK_CTRL,
    #[doc = "0x18 - Sub-Week Alarm Counter Register"]
    pub swk_alarm: SWK_ALARM,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "ALARM_CNT (rw) register accessor: an alias for `Reg<ALARM_CNT_SPEC>`"]
pub type ALARM_CNT = crate::Reg<alarm_cnt::ALARM_CNT_SPEC>;
#[doc = "Week Alarm Counter Register"]
pub mod alarm_cnt;
#[doc = "TMR_COMP (rw) register accessor: an alias for `Reg<TMR_COMP_SPEC>`"]
pub type TMR_COMP = crate::Reg<tmr_comp::TMR_COMP_SPEC>;
#[doc = "Week Timer Compare Register"]
pub mod tmr_comp;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Divider Register"]
pub mod clkdiv;
#[doc = "SS_INTR_SEL (rw) register accessor: an alias for `Reg<SS_INTR_SEL_SPEC>`"]
pub type SS_INTR_SEL = crate::Reg<ss_intr_sel::SS_INTR_SEL_SPEC>;
#[doc = "Sub-Second Programmable Interrupt Select Register"]
pub mod ss_intr_sel;
#[doc = "SWK_CTRL (r) register accessor: an alias for `Reg<SWK_CTRL_SPEC>`"]
pub type SWK_CTRL = crate::Reg<swk_ctrl::SWK_CTRL_SPEC>;
#[doc = "Sub-Week Control Register"]
pub mod swk_ctrl;
#[doc = "SWK_ALARM (r) register accessor: an alias for `Reg<SWK_ALARM_SPEC>`"]
pub type SWK_ALARM = crate::Reg<swk_alarm::SWK_ALARM_SPEC>;
#[doc = "Sub-Week Alarm Counter Register"]
pub mod swk_alarm;
