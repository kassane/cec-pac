#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Enable this channel for operation. The DMA Main Control: Activate must also be enabled for this channel to be operational."]
    pub activate: ACTIVATE,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - This is the starting address for the Memory device."]
    pub mstart: MSTART,
    #[doc = "0x08 - This is the ending address for the Memory device."]
    pub mend: MEND,
    #[doc = "0x0c - This is the Master Device address."]
    pub dstart: DSTART,
    #[doc = "0x10 - DMA Channel N Control"]
    pub ctrl: CTRL,
    #[doc = "0x14 - DMA Channel N Interrupt Status"]
    pub ists: ISTS,
    _reserved6: [u8; 0x03],
    #[doc = "0x18 - DMA CHANNEL N INTERRUPT ENABLE"]
    pub ien: IEN,
    _reserved7: [u8; 0x07],
    #[doc = "0x20 - DMA CHANNEL N FILL ENABLE"]
    pub fill_en: FILL_EN,
    #[doc = "0x24 - DMA CHANNEL N FILL DATA"]
    pub fill_data: FILL_DATA,
    #[doc = "0x28 - DMA CHANNEL N FILL STATUS"]
    pub fill_sts: FILL_STS,
}
#[doc = "ACTIVATE (rw) register accessor: an alias for `Reg<ACTIVATE_SPEC>`"]
pub type ACTIVATE = crate::Reg<activate::ACTIVATE_SPEC>;
#[doc = "Enable this channel for operation. The DMA Main Control: Activate must also be enabled for this channel to be operational."]
pub mod activate;
#[doc = "MSTART (rw) register accessor: an alias for `Reg<MSTART_SPEC>`"]
pub type MSTART = crate::Reg<mstart::MSTART_SPEC>;
#[doc = "This is the starting address for the Memory device."]
pub mod mstart;
#[doc = "MEND (rw) register accessor: an alias for `Reg<MEND_SPEC>`"]
pub type MEND = crate::Reg<mend::MEND_SPEC>;
#[doc = "This is the ending address for the Memory device."]
pub mod mend;
#[doc = "DSTART (rw) register accessor: an alias for `Reg<DSTART_SPEC>`"]
pub type DSTART = crate::Reg<dstart::DSTART_SPEC>;
#[doc = "This is the Master Device address."]
pub mod dstart;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DMA Channel N Control"]
pub mod ctrl;
#[doc = "ISTS (rw) register accessor: an alias for `Reg<ISTS_SPEC>`"]
pub type ISTS = crate::Reg<ists::ISTS_SPEC>;
#[doc = "DMA Channel N Interrupt Status"]
pub mod ists;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "DMA CHANNEL N INTERRUPT ENABLE"]
pub mod ien;
#[doc = "FILL_EN (rw) register accessor: an alias for `Reg<FILL_EN_SPEC>`"]
pub type FILL_EN = crate::Reg<fill_en::FILL_EN_SPEC>;
#[doc = "DMA CHANNEL N FILL ENABLE"]
pub mod fill_en;
#[doc = "FILL_DATA (rw) register accessor: an alias for `Reg<FILL_DATA_SPEC>`"]
pub type FILL_DATA = crate::Reg<fill_data::FILL_DATA_SPEC>;
#[doc = "DMA CHANNEL N FILL DATA"]
pub mod fill_data;
#[doc = "FILL_STS (rw) register accessor: an alias for `Reg<FILL_STS_SPEC>`"]
pub type FILL_STS = crate::Reg<fill_sts::FILL_STS_SPEC>;
#[doc = "DMA CHANNEL N FILL STATUS"]
pub mod fill_sts;
