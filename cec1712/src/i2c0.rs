#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_rsts: [u8; 0x04],
    #[doc = "0x04 - Own Address Register\n Note that the Data Register and Own Address fields are offset by one bit, so that programming Own Address 1 with a value of 55h will result in the value AAh being recognized as the SMB Controller Core slave address."]
    pub own_addr: OWN_ADDR,
    #[doc = "0x08 - This register holds the data that are either shifted out to or shifted in from the I2C port."]
    pub i2cdata: I2CDATA,
    _reserved3: [u8; 0x0f],
    #[doc = "0x18 - Repeated Start Hold Time Register"]
    pub rshtm: RSHTM,
    #[doc = "0x1c - Reserved"]
    pub rsvd1: RSVD1,
    #[doc = "0x20 - Completion Register"]
    pub compl: COMPL,
    #[doc = "0x24 - Reserved"]
    pub rsvd2: RSVD2,
    #[doc = "0x28 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x2c - Bus Clock Register"]
    pub busclk: BUSCLK,
    #[doc = "0x30 - Block ID Register"]
    pub blkid: BLKID,
    _reserved10: [u8; 0x03],
    #[doc = "0x34 - Revision Register"]
    pub blkrev: BLKREV,
    _reserved11: [u8; 0x03],
    #[doc = "0x38 - Bit-Bang Control Register"]
    pub bb_ctrl: BB_CTRL,
    #[doc = "0x3c - This is Clock Sync Register. This register must not be written, or undesirable results may occur.\n"]
    pub clksync: CLKSYNC,
    _reserved13: [u8; 0x03],
    #[doc = "0x40 - Data Timing Register"]
    pub datatm: DATATM,
    #[doc = "0x44 - Time-Out Scaling Register"]
    pub tmoutsc: TMOUTSC,
    _reserved15: [u8; 0x18],
    #[doc = "0x60 - WAKE STATUS Register"]
    pub wake_sts: WAKE_STS,
    #[doc = "0x64 - WAKE ENABLE Register"]
    pub wake_en: WAKE_EN,
    _reserved17: [u8; 0x04],
    #[doc = "0x6c - This is the Slave Address Register.\n"]
    pub slv_addr: SLV_ADDR,
    _reserved18: [u8; 0x03],
    #[doc = "0x70 - This is the Promiscuous Interrupt Register. This register bit will be functional only in Promiscuous mode.\n"]
    pub prm_sts: PRM_STS,
    _reserved19: [u8; 0x03],
    #[doc = "0x74 - This is the Promiscuous Interrupt Enable Register.\n"]
    pub prm_ien: PRM_IEN,
    _reserved20: [u8; 0x03],
    #[doc = "0x78 - This is the Promiscuous Control Register. This register is functional only in Promiscuous mode.\n"]
    pub prm_ctrl: PRM_CTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Status Register"]
    #[inline(always)]
    pub fn rsts(&self) -> &RSTS {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const RSTS) }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn wctrl(&self) -> &WCTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const WCTRL) }
    }
}
#[doc = "WCTRL (w) register accessor: an alias for `Reg<WCTRL_SPEC>`"]
pub type WCTRL = crate::Reg<wctrl::WCTRL_SPEC>;
#[doc = "Control Register"]
pub mod wctrl;
#[doc = "RSTS (r) register accessor: an alias for `Reg<RSTS_SPEC>`"]
pub type RSTS = crate::Reg<rsts::RSTS_SPEC>;
#[doc = "Status Register"]
pub mod rsts;
#[doc = "OWN_ADDR (rw) register accessor: an alias for `Reg<OWN_ADDR_SPEC>`"]
pub type OWN_ADDR = crate::Reg<own_addr::OWN_ADDR_SPEC>;
#[doc = "Own Address Register\n Note that the Data Register and Own Address fields are offset by one bit, so that programming Own Address 1 with a value of 55h will result in the value AAh being recognized as the SMB Controller Core slave address."]
pub mod own_addr;
#[doc = "I2CDATA (rw) register accessor: an alias for `Reg<I2CDATA_SPEC>`"]
pub type I2CDATA = crate::Reg<i2cdata::I2CDATA_SPEC>;
#[doc = "This register holds the data that are either shifted out to or shifted in from the I2C port."]
pub mod i2cdata;
#[doc = "RSHTM (rw) register accessor: an alias for `Reg<RSHTM_SPEC>`"]
pub type RSHTM = crate::Reg<rshtm::RSHTM_SPEC>;
#[doc = "Repeated Start Hold Time Register"]
pub mod rshtm;
#[doc = "RSVD1 (r) register accessor: an alias for `Reg<RSVD1_SPEC>`"]
pub type RSVD1 = crate::Reg<rsvd1::RSVD1_SPEC>;
#[doc = "Reserved"]
pub mod rsvd1;
#[doc = "COMPL (rw) register accessor: an alias for `Reg<COMPL_SPEC>`"]
pub type COMPL = crate::Reg<compl::COMPL_SPEC>;
#[doc = "Completion Register"]
pub mod compl;
#[doc = "RSVD2 (r) register accessor: an alias for `Reg<RSVD2_SPEC>`"]
pub type RSVD2 = crate::Reg<rsvd2::RSVD2_SPEC>;
#[doc = "Reserved"]
pub mod rsvd2;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "BUSCLK (rw) register accessor: an alias for `Reg<BUSCLK_SPEC>`"]
pub type BUSCLK = crate::Reg<busclk::BUSCLK_SPEC>;
#[doc = "Bus Clock Register"]
pub mod busclk;
#[doc = "BLKID (r) register accessor: an alias for `Reg<BLKID_SPEC>`"]
pub type BLKID = crate::Reg<blkid::BLKID_SPEC>;
#[doc = "Block ID Register"]
pub mod blkid;
#[doc = "BLKREV (r) register accessor: an alias for `Reg<BLKREV_SPEC>`"]
pub type BLKREV = crate::Reg<blkrev::BLKREV_SPEC>;
#[doc = "Revision Register"]
pub mod blkrev;
#[doc = "BB_CTRL (rw) register accessor: an alias for `Reg<BB_CTRL_SPEC>`"]
pub type BB_CTRL = crate::Reg<bb_ctrl::BB_CTRL_SPEC>;
#[doc = "Bit-Bang Control Register"]
pub mod bb_ctrl;
#[doc = "CLKSYNC (r) register accessor: an alias for `Reg<CLKSYNC_SPEC>`"]
pub type CLKSYNC = crate::Reg<clksync::CLKSYNC_SPEC>;
#[doc = "This is Clock Sync Register. This register must not be written, or undesirable results may occur.\n"]
pub mod clksync;
#[doc = "DATATM (rw) register accessor: an alias for `Reg<DATATM_SPEC>`"]
pub type DATATM = crate::Reg<datatm::DATATM_SPEC>;
#[doc = "Data Timing Register"]
pub mod datatm;
#[doc = "TMOUTSC (rw) register accessor: an alias for `Reg<TMOUTSC_SPEC>`"]
pub type TMOUTSC = crate::Reg<tmoutsc::TMOUTSC_SPEC>;
#[doc = "Time-Out Scaling Register"]
pub mod tmoutsc;
#[doc = "WAKE_STS (rw) register accessor: an alias for `Reg<WAKE_STS_SPEC>`"]
pub type WAKE_STS = crate::Reg<wake_sts::WAKE_STS_SPEC>;
#[doc = "WAKE STATUS Register"]
pub mod wake_sts;
#[doc = "WAKE_EN (rw) register accessor: an alias for `Reg<WAKE_EN_SPEC>`"]
pub type WAKE_EN = crate::Reg<wake_en::WAKE_EN_SPEC>;
#[doc = "WAKE ENABLE Register"]
pub mod wake_en;
#[doc = "SLV_ADDR (rw) register accessor: an alias for `Reg<SLV_ADDR_SPEC>`"]
pub type SLV_ADDR = crate::Reg<slv_addr::SLV_ADDR_SPEC>;
#[doc = "This is the Slave Address Register.\n"]
pub mod slv_addr;
#[doc = "PRM_STS (rw) register accessor: an alias for `Reg<PRM_STS_SPEC>`"]
pub type PRM_STS = crate::Reg<prm_sts::PRM_STS_SPEC>;
#[doc = "This is the Promiscuous Interrupt Register. This register bit will be functional only in Promiscuous mode.\n"]
pub mod prm_sts;
#[doc = "PRM_IEN (rw) register accessor: an alias for `Reg<PRM_IEN_SPEC>`"]
pub type PRM_IEN = crate::Reg<prm_ien::PRM_IEN_SPEC>;
#[doc = "This is the Promiscuous Interrupt Enable Register.\n"]
pub mod prm_ien;
#[doc = "PRM_CTRL (rw) register accessor: an alias for `Reg<PRM_CTRL_SPEC>`"]
pub type PRM_CTRL = crate::Reg<prm_ctrl::PRM_CTRL_SPEC>;
#[doc = "This is the Promiscuous Control Register. This register is functional only in Promiscuous mode.\n"]
pub mod prm_ctrl;
