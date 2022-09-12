#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Monitor Control Register"]
    pub mntr_ctrl: MNTR_CTRL,
    #[doc = "0x04 - SPI Configuration Status Register"]
    pub cfg_sts: CFG_STS,
    #[doc = "0x08 - SPI Monitor Configuration 2 Register"]
    pub spicfg2: SPICFG2,
    #[doc = "0x0c - Violation IRQ Control/Status Register"]
    pub vioctrlsts: VIOCTRLSTS,
    #[doc = "0x10 - SPI Intervention Status Register"]
    pub ivn_sts: IVN_STS,
    #[doc = "0x11 - SPI Intervention Recovery Register"]
    pub ivn_rec: IVN_REC,
    _reserved6: [u8; 0x02],
    #[doc = "0x14 - Violation Log Register"]
    pub vio_sts: VIO_STS,
    #[doc = "0x18 - Error Byte Address Register"]
    pub err_addr: ERR_ADDR,
    _reserved8: [u8; 0x04],
    #[doc = "0x20..0x120 - FLASH_SET\\[%s\\]"]
    pub flash_set: [FLASH_SET; 2],
    #[doc = "0x120..0x1a0 - RN_TM\\[%s\\]"]
    pub rn_tm: [RN_TM; 16],
    _reserved10: [u8; 0x30],
    #[doc = "0x1d0 - Match Monitor Control/Status Register"]
    pub mtmon_ctrlsts: MTMON_CTRLSTS,
    #[doc = "0x1d4 - Match Monitor Enable/Mode Register"]
    pub mtmon_enmd: MTMON_ENMD,
    #[doc = "0x1d8 - Match Fetch Timeout Control Register"]
    pub mtmon_tctrl: MTMON_TCTRL,
    _reserved13: [u8; 0x04],
    #[doc = "0x1e0..0x240 - MT_MON\\[%s\\]"]
    pub mt_mon: [MT_MON; 8],
    #[doc = "0x240 - Match Monitor Violation Log Register"]
    pub mtmon_viosts: MTMON_VIOSTS,
    #[doc = "0x244 - Match Monitor Violation Address Register"]
    pub mtmon_vioaddr: MTMON_VIOADDR,
    _reserved16: [u8; 0x08],
    #[doc = "0x250 - Loadtime (Hash) IRQ Aggregation Register"]
    pub ltmon_aggr: LTMON_AGGR,
    #[doc = "0x254..0x314 - LT_MON\\[%s\\]"]
    pub lt_mon: [LT_MON; 8],
    #[doc = "0x314 - Load Monitor Control/Status Register"]
    pub ltmon_ctrlsts: LTMON_CTRLSTS,
}
#[doc = "MNTR_CTRL (rw) register accessor: an alias for `Reg<MNTR_CTRL_SPEC>`"]
pub type MNTR_CTRL = crate::Reg<mntr_ctrl::MNTR_CTRL_SPEC>;
#[doc = "SPI Monitor Control Register"]
pub mod mntr_ctrl;
#[doc = "CFG_STS (rw) register accessor: an alias for `Reg<CFG_STS_SPEC>`"]
pub type CFG_STS = crate::Reg<cfg_sts::CFG_STS_SPEC>;
#[doc = "SPI Configuration Status Register"]
pub mod cfg_sts;
#[doc = "SPICFG2 (rw) register accessor: an alias for `Reg<SPICFG2_SPEC>`"]
pub type SPICFG2 = crate::Reg<spicfg2::SPICFG2_SPEC>;
#[doc = "SPI Monitor Configuration 2 Register"]
pub mod spicfg2;
#[doc = "VIOCTRLSTS (rw) register accessor: an alias for `Reg<VIOCTRLSTS_SPEC>`"]
pub type VIOCTRLSTS = crate::Reg<vioctrlsts::VIOCTRLSTS_SPEC>;
#[doc = "Violation IRQ Control/Status Register"]
pub mod vioctrlsts;
#[doc = "IVN_STS (r) register accessor: an alias for `Reg<IVN_STS_SPEC>`"]
pub type IVN_STS = crate::Reg<ivn_sts::IVN_STS_SPEC>;
#[doc = "SPI Intervention Status Register"]
pub mod ivn_sts;
#[doc = "IVN_REC (w) register accessor: an alias for `Reg<IVN_REC_SPEC>`"]
pub type IVN_REC = crate::Reg<ivn_rec::IVN_REC_SPEC>;
#[doc = "SPI Intervention Recovery Register"]
pub mod ivn_rec;
#[doc = "VIO_STS (rw) register accessor: an alias for `Reg<VIO_STS_SPEC>`"]
pub type VIO_STS = crate::Reg<vio_sts::VIO_STS_SPEC>;
#[doc = "Violation Log Register"]
pub mod vio_sts;
#[doc = "ERR_ADDR (r) register accessor: an alias for `Reg<ERR_ADDR_SPEC>`"]
pub type ERR_ADDR = crate::Reg<err_addr::ERR_ADDR_SPEC>;
#[doc = "Error Byte Address Register"]
pub mod err_addr;
#[doc = "FLASH_SET\\[%s\\]"]
pub use flash_set::FLASH_SET;
#[doc = r"Cluster"]
#[doc = "FLASH_SET\\[%s\\]"]
pub mod flash_set;
#[doc = "RN_TM\\[%s\\]"]
pub use rn_tm::RN_TM;
#[doc = r"Cluster"]
#[doc = "RN_TM\\[%s\\]"]
pub mod rn_tm;
#[doc = "MTMON_CTRLSTS (rw) register accessor: an alias for `Reg<MTMON_CTRLSTS_SPEC>`"]
pub type MTMON_CTRLSTS = crate::Reg<mtmon_ctrlsts::MTMON_CTRLSTS_SPEC>;
#[doc = "Match Monitor Control/Status Register"]
pub mod mtmon_ctrlsts;
#[doc = "MTMON_ENMD (rw) register accessor: an alias for `Reg<MTMON_ENMD_SPEC>`"]
pub type MTMON_ENMD = crate::Reg<mtmon_enmd::MTMON_ENMD_SPEC>;
#[doc = "Match Monitor Enable/Mode Register"]
pub mod mtmon_enmd;
#[doc = "MTMON_TCTRL (rw) register accessor: an alias for `Reg<MTMON_TCTRL_SPEC>`"]
pub type MTMON_TCTRL = crate::Reg<mtmon_tctrl::MTMON_TCTRL_SPEC>;
#[doc = "Match Fetch Timeout Control Register"]
pub mod mtmon_tctrl;
#[doc = "MT_MON\\[%s\\]"]
pub use mt_mon::MT_MON;
#[doc = r"Cluster"]
#[doc = "MT_MON\\[%s\\]"]
pub mod mt_mon;
#[doc = "MTMON_VIOSTS (rw) register accessor: an alias for `Reg<MTMON_VIOSTS_SPEC>`"]
pub type MTMON_VIOSTS = crate::Reg<mtmon_viosts::MTMON_VIOSTS_SPEC>;
#[doc = "Match Monitor Violation Log Register"]
pub mod mtmon_viosts;
#[doc = "MTMON_VIOADDR (r) register accessor: an alias for `Reg<MTMON_VIOADDR_SPEC>`"]
pub type MTMON_VIOADDR = crate::Reg<mtmon_vioaddr::MTMON_VIOADDR_SPEC>;
#[doc = "Match Monitor Violation Address Register"]
pub mod mtmon_vioaddr;
#[doc = "LTMON_AGGR (rw) register accessor: an alias for `Reg<LTMON_AGGR_SPEC>`"]
pub type LTMON_AGGR = crate::Reg<ltmon_aggr::LTMON_AGGR_SPEC>;
#[doc = "Loadtime (Hash) IRQ Aggregation Register"]
pub mod ltmon_aggr;
#[doc = "LT_MON\\[%s\\]"]
pub use lt_mon::LT_MON;
#[doc = r"Cluster"]
#[doc = "LT_MON\\[%s\\]"]
pub mod lt_mon;
#[doc = "LTMON_CTRLSTS (rw) register accessor: an alias for `Reg<LTMON_CTRLSTS_SPEC>`"]
pub type LTMON_CTRLSTS = crate::Reg<ltmon_ctrlsts::LTMON_CTRLSTS_SPEC>;
#[doc = "Load Monitor Control/Status Register"]
pub mod ltmon_ctrlsts;
