#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Soft reset the entire module. Enable the blocks operation."]
    pub actrst: ACTRST,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Debug register that has the data that is stored in the Data Packet. This data is read data from the currently active transfer source."]
    pub data_pkt: DATA_PKT,
}
#[doc = "ACTRST (rw) register accessor: an alias for `Reg<ACTRST_SPEC>`"]
pub type ACTRST = crate::Reg<actrst::ACTRST_SPEC>;
#[doc = "Soft reset the entire module. Enable the blocks operation."]
pub mod actrst;
#[doc = "DATA_PKT (r) register accessor: an alias for `Reg<DATA_PKT_SPEC>`"]
pub type DATA_PKT = crate::Reg<data_pkt::DATA_PKT_SPEC>;
#[doc = "Debug register that has the data that is stored in the Data Packet. This data is read data from the currently active transfer source."]
pub mod data_pkt;
