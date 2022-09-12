#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_rsts: [u8; 0x04],
    #[doc = "0x04 - Own Address Register\n Note that the Data Register and Own Address fields are offset by one bit, so that programming Own Address 1 with a value of 55h will result in the value AAh being recognized as the SMB Controller Core slave address."]
    pub own_addr: OWN_ADDR,
    #[doc = "0x08 - This register holds the data that are either shifted out to or shifted in from the I2C port."]
    pub i2cdata: I2CDATA,
    #[doc = "0x09 - Reserved"]
    pub rsvd1: [RSVD1; 3],
    #[doc = "0x0c - SMBus Master Command Register"]
    pub mcmd: MCMD,
    #[doc = "0x10 - SMBus Slave Command Register"]
    pub scmd: SCMD,
    #[doc = "0x14 - Packet Error Check (PEC) Register"]
    pub pec: PEC,
    #[doc = "0x18 - Repeated Start Hold Time Register"]
    pub rshtm: RSHTM,
    #[doc = "0x1c - Reserved"]
    pub rsvd2: RSVD2,
    #[doc = "0x20 - Completion Register"]
    pub compl: COMPL,
    #[doc = "0x24 - Idle Scaling Register"]
    pub idlsc: IDLSC,
    #[doc = "0x28 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x2c - Bus Clock Register"]
    pub busclk: BUSCLK,
    #[doc = "0x30 - Block ID Register"]
    pub blkid: BLKID,
    _reserved14: [u8; 0x03],
    #[doc = "0x34 - Revision Register"]
    pub blkrev: BLKREV,
    _reserved15: [u8; 0x03],
    #[doc = "0x38 - Bit-Bang Control Register"]
    pub bbctrl: BBCTRL,
    #[doc = "0x3c - Test"]
    pub test: TEST,
    _reserved17: [u8; 0x03],
    #[doc = "0x40 - Data Timing Register"]
    pub datatm: DATATM,
    #[doc = "0x44 - Time-Out Scaling Register"]
    pub tmoutsc: TMOUTSC,
    #[doc = "0x48 - SMBus Slave Transmit Buffer Register"]
    pub slv_txb: SLV_TXB,
    #[doc = "0x4c - SMBus Slave Receive Buffer Register"]
    pub slv_rxb: SLV_RXB,
    #[doc = "0x50 - SMBus Master Transmit Buffer Register"]
    pub mtr_txb: MTR_TXB,
    #[doc = "0x54 - SMBus Master Receive Buffer Register"]
    pub mtr_rxb: MTR_RXB,
    _reserved23: [u8; 0x08],
    #[doc = "0x60 - WAKE STATUS Register"]
    pub wake_sts: WAKE_STS,
    #[doc = "0x64 - WAKE ENABLE Register"]
    pub wake_en: WAKE_EN,
    _reserved25: [u8; 0x04],
    #[doc = "0x6c - This is the Slave Address Register"]
    pub slv_addr: SLV_ADDR,
    _reserved26: [u8; 0x03],
    #[doc = "0x70 - This is the Promiscuous Interrupt Register"]
    pub prm_sts: PRM_STS,
    _reserved27: [u8; 0x03],
    #[doc = "0x74 - This is the Promiscuous Interrupt Enable Register"]
    pub prm_ien: PRM_IEN,
    _reserved28: [u8; 0x03],
    #[doc = "0x78 - This is the Promiscuous Control Register"]
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
#[doc = "RSVD1 (r) register accessor: an alias for `Reg<RSVD1_SPEC>`"]
pub type RSVD1 = crate::Reg<rsvd1::RSVD1_SPEC>;
#[doc = "Reserved"]
pub mod rsvd1;
#[doc = "MCMD (rw) register accessor: an alias for `Reg<MCMD_SPEC>`"]
pub type MCMD = crate::Reg<mcmd::MCMD_SPEC>;
#[doc = "SMBus Master Command Register"]
pub mod mcmd;
#[doc = "SCMD (rw) register accessor: an alias for `Reg<SCMD_SPEC>`"]
pub type SCMD = crate::Reg<scmd::SCMD_SPEC>;
#[doc = "SMBus Slave Command Register"]
pub mod scmd;
#[doc = "PEC (rw) register accessor: an alias for `Reg<PEC_SPEC>`"]
pub type PEC = crate::Reg<pec::PEC_SPEC>;
#[doc = "Packet Error Check (PEC) Register"]
pub mod pec;
#[doc = "RSHTM (rw) register accessor: an alias for `Reg<RSHTM_SPEC>`"]
pub type RSHTM = crate::Reg<rshtm::RSHTM_SPEC>;
#[doc = "Repeated Start Hold Time Register"]
pub mod rshtm;
#[doc = "RSVD2 (r) register accessor: an alias for `Reg<RSVD2_SPEC>`"]
pub type RSVD2 = crate::Reg<rsvd2::RSVD2_SPEC>;
#[doc = "Reserved"]
pub mod rsvd2;
#[doc = "COMPL (rw) register accessor: an alias for `Reg<COMPL_SPEC>`"]
pub type COMPL = crate::Reg<compl::COMPL_SPEC>;
#[doc = "Completion Register"]
pub mod compl;
#[doc = "IDLSC (rw) register accessor: an alias for `Reg<IDLSC_SPEC>`"]
pub type IDLSC = crate::Reg<idlsc::IDLSC_SPEC>;
#[doc = "Idle Scaling Register"]
pub mod idlsc;
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
#[doc = "BBCTRL (rw) register accessor: an alias for `Reg<BBCTRL_SPEC>`"]
pub type BBCTRL = crate::Reg<bbctrl::BBCTRL_SPEC>;
#[doc = "Bit-Bang Control Register"]
pub mod bbctrl;
#[doc = "TEST (r) register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test"]
pub mod test;
#[doc = "DATATM (rw) register accessor: an alias for `Reg<DATATM_SPEC>`"]
pub type DATATM = crate::Reg<datatm::DATATM_SPEC>;
#[doc = "Data Timing Register"]
pub mod datatm;
#[doc = "TMOUTSC (rw) register accessor: an alias for `Reg<TMOUTSC_SPEC>`"]
pub type TMOUTSC = crate::Reg<tmoutsc::TMOUTSC_SPEC>;
#[doc = "Time-Out Scaling Register"]
pub mod tmoutsc;
#[doc = "SLV_TXB (rw) register accessor: an alias for `Reg<SLV_TXB_SPEC>`"]
pub type SLV_TXB = crate::Reg<slv_txb::SLV_TXB_SPEC>;
#[doc = "SMBus Slave Transmit Buffer Register"]
pub mod slv_txb;
#[doc = "SLV_RXB (rw) register accessor: an alias for `Reg<SLV_RXB_SPEC>`"]
pub type SLV_RXB = crate::Reg<slv_rxb::SLV_RXB_SPEC>;
#[doc = "SMBus Slave Receive Buffer Register"]
pub mod slv_rxb;
#[doc = "MTR_TXB (rw) register accessor: an alias for `Reg<MTR_TXB_SPEC>`"]
pub type MTR_TXB = crate::Reg<mtr_txb::MTR_TXB_SPEC>;
#[doc = "SMBus Master Transmit Buffer Register"]
pub mod mtr_txb;
#[doc = "MTR_RXB (rw) register accessor: an alias for `Reg<MTR_RXB_SPEC>`"]
pub type MTR_RXB = crate::Reg<mtr_rxb::MTR_RXB_SPEC>;
#[doc = "SMBus Master Receive Buffer Register"]
pub mod mtr_rxb;
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
#[doc = "This is the Slave Address Register"]
pub mod slv_addr;
#[doc = "PRM_STS (rw) register accessor: an alias for `Reg<PRM_STS_SPEC>`"]
pub type PRM_STS = crate::Reg<prm_sts::PRM_STS_SPEC>;
#[doc = "This is the Promiscuous Interrupt Register"]
pub mod prm_sts;
#[doc = "PRM_IEN (rw) register accessor: an alias for `Reg<PRM_IEN_SPEC>`"]
pub type PRM_IEN = crate::Reg<prm_ien::PRM_IEN_SPEC>;
#[doc = "This is the Promiscuous Interrupt Enable Register"]
pub mod prm_ien;
#[doc = "PRM_CTRL (rw) register accessor: an alias for `Reg<PRM_CTRL_SPEC>`"]
pub type PRM_CTRL = crate::Reg<prm_ctrl::PRM_CTRL_SPEC>;
#[doc = "This is the Promiscuous Control Register"]
pub mod prm_ctrl;
