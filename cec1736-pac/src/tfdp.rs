#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Debug data to be shifted out on the TFDP Debug port. While data is being shifted out, the Host Interface will 'hold-off' additional writes to the data register until the transfer is complete."]
    pub msdata: MSDATA,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Debug Control Register"]
    pub ctrl: CTRL,
}
#[doc = "MSDATA (rw) register accessor: an alias for `Reg<MSDATA_SPEC>`"]
pub type MSDATA = crate::Reg<msdata::MSDATA_SPEC>;
#[doc = "Debug data to be shifted out on the TFDP Debug port. While data is being shifted out, the Host Interface will 'hold-off' additional writes to the data register until the transfer is complete."]
pub mod msdata;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Debug Control Register"]
pub mod ctrl;
