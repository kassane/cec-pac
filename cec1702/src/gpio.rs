#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x2b0 - GPIO Pin Control Register"]
    pub ctrl: [CTRL; 172],
    _reserved1: [u8; 0x50],
    #[doc = "0x300..0x318 - The GPIO Input Registers."]
    pub parin: [PARIN; 6],
    _reserved2: [u8; 0x68],
    #[doc = "0x380..0x398 - The GPIO Output Registers."]
    pub parout: [PAROUT; 6],
    _reserved3: [u8; 0x0168],
    #[doc = "0x500..0x7b0 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p: [CTRL2P; 172],
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl;
#[doc = "PARIN (rw) register accessor: an alias for `Reg<PARIN_SPEC>`"]
pub type PARIN = crate::Reg<parin::PARIN_SPEC>;
#[doc = "The GPIO Input Registers."]
pub mod parin;
#[doc = "PAROUT (rw) register accessor: an alias for `Reg<PAROUT_SPEC>`"]
pub type PAROUT = crate::Reg<parout::PAROUT_SPEC>;
#[doc = "The GPIO Output Registers."]
pub mod parout;
#[doc = "CTRL2P (rw) register accessor: an alias for `Reg<CTRL2P_SPEC>`"]
pub type CTRL2P = crate::Reg<ctrl2p::CTRL2P_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p;
