#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IMSPI Mode Register"]
    pub mode: MODE,
    #[doc = "0x04 - IMSPI Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - IMSPI Interrupt Enable Register"]
    pub int_enable: INT_ENABLE,
    #[doc = "0x0c - IMSPI Timeout Control Register"]
    pub timeout_control: TIMEOUT_CONTROL,
}
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "IMSPI Mode Register"]
pub mod mode;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "IMSPI Status Register"]
pub mod status;
#[doc = "INT_ENABLE (rw) register accessor: an alias for `Reg<INT_ENABLE_SPEC>`"]
pub type INT_ENABLE = crate::Reg<int_enable::INT_ENABLE_SPEC>;
#[doc = "IMSPI Interrupt Enable Register"]
pub mod int_enable;
#[doc = "TIMEOUT_CONTROL (rw) register accessor: an alias for `Reg<TIMEOUT_CONTROL_SPEC>`"]
pub type TIMEOUT_CONTROL = crate::Reg<timeout_control::TIMEOUT_CONTROL_SPEC>;
#[doc = "IMSPI Timeout Control Register"]
pub mod timeout_control;
