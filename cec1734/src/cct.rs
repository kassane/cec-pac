#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register controls the capture and compare timer."]
    pub ctrl: CTRL,
    #[doc = "0x04 - This register is used to configure capture and compare timers 0-3."]
    pub cap0_ctrl: CAP0_CTRL,
    #[doc = "0x08 - This register is used to configure capture and compare timers 4-5."]
    pub cap1_ctrl: CAP1_CTRL,
    #[doc = "0x0c - This register contains the current value of the Free Running Timer."]
    pub free_run: FREE_RUN,
    #[doc = "0x10 - This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
    pub cap0: CAP0,
    #[doc = "0x14 - This register saves the value copied from the Free Running timer on a programmed edge of ICT1."]
    pub cap1: CAP1,
    #[doc = "0x18 - This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
    pub cap2: CAP2,
    #[doc = "0x1c - This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
    pub cap3: CAP3,
    #[doc = "0x20 - This register saves the value copied from the Free Running timer on a programmed edge of ICT4."]
    pub cap4: CAP4,
    #[doc = "0x24 - This register saves the value copied from the Free Running timer on a programmed edge of ICT5."]
    pub cap5: CAP5,
    #[doc = "0x28 - A COMPARE 0 interrupt is generated when this register matches the value in the Free Running Timer."]
    pub comp0: COMP0,
    #[doc = "0x2c - A COMPARE 1 interrupt is generated when this register matches the value in the Free Running Timer."]
    pub comp1: COMP1,
    #[doc = "0x30 - This register selects the pin mapping to the capture register."]
    pub mux_sel: MUX_SEL,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "This register controls the capture and compare timer."]
pub mod ctrl;
#[doc = "CAP0_CTRL (rw) register accessor: an alias for `Reg<CAP0_CTRL_SPEC>`"]
pub type CAP0_CTRL = crate::Reg<cap0_ctrl::CAP0_CTRL_SPEC>;
#[doc = "This register is used to configure capture and compare timers 0-3."]
pub mod cap0_ctrl;
#[doc = "CAP1_CTRL (rw) register accessor: an alias for `Reg<CAP1_CTRL_SPEC>`"]
pub type CAP1_CTRL = crate::Reg<cap1_ctrl::CAP1_CTRL_SPEC>;
#[doc = "This register is used to configure capture and compare timers 4-5."]
pub mod cap1_ctrl;
#[doc = "FREE_RUN (rw) register accessor: an alias for `Reg<FREE_RUN_SPEC>`"]
pub type FREE_RUN = crate::Reg<free_run::FREE_RUN_SPEC>;
#[doc = "This register contains the current value of the Free Running Timer."]
pub mod free_run;
#[doc = "CAP0 (rw) register accessor: an alias for `Reg<CAP0_SPEC>`"]
pub type CAP0 = crate::Reg<cap0::CAP0_SPEC>;
#[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
pub mod cap0;
#[doc = "CAP1 (rw) register accessor: an alias for `Reg<CAP1_SPEC>`"]
pub type CAP1 = crate::Reg<cap1::CAP1_SPEC>;
#[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT1."]
pub mod cap1;
#[doc = "CAP2 (rw) register accessor: an alias for `Reg<CAP2_SPEC>`"]
pub type CAP2 = crate::Reg<cap2::CAP2_SPEC>;
#[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
pub mod cap2;
#[doc = "CAP3 (rw) register accessor: an alias for `Reg<CAP3_SPEC>`"]
pub type CAP3 = crate::Reg<cap3::CAP3_SPEC>;
#[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
pub mod cap3;
#[doc = "CAP4 (rw) register accessor: an alias for `Reg<CAP4_SPEC>`"]
pub type CAP4 = crate::Reg<cap4::CAP4_SPEC>;
#[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT4."]
pub mod cap4;
#[doc = "CAP5 (rw) register accessor: an alias for `Reg<CAP5_SPEC>`"]
pub type CAP5 = crate::Reg<cap5::CAP5_SPEC>;
#[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT5."]
pub mod cap5;
#[doc = "COMP0 (rw) register accessor: an alias for `Reg<COMP0_SPEC>`"]
pub type COMP0 = crate::Reg<comp0::COMP0_SPEC>;
#[doc = "A COMPARE 0 interrupt is generated when this register matches the value in the Free Running Timer."]
pub mod comp0;
#[doc = "COMP1 (rw) register accessor: an alias for `Reg<COMP1_SPEC>`"]
pub type COMP1 = crate::Reg<comp1::COMP1_SPEC>;
#[doc = "A COMPARE 1 interrupt is generated when this register matches the value in the Free Running Timer."]
pub mod comp1;
#[doc = "MUX_SEL (rw) register accessor: an alias for `Reg<MUX_SEL_SPEC>`"]
pub type MUX_SEL = crate::Reg<mux_sel::MUX_SEL_SPEC>;
#[doc = "This register selects the pin mapping to the capture register."]
pub mod mux_sel;
