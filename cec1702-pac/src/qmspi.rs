#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - QMSPI Mode Register"]
    pub mode: MODE,
    #[doc = "0x04 - QMSPI SPI Control"]
    pub ctrl: CTRL,
    #[doc = "0x08 - QMSPI Execute Register"]
    pub exe: EXE,
    #[doc = "0x0c - QMSPI Interface Control Register"]
    pub ifctrl: IFCTRL,
    #[doc = "0x10 - QMSPI Status Register"]
    pub sts: STS,
    #[doc = "0x14 - QMSPI Buffer Count Status Register"]
    pub buf_cnt_sts: BUF_CNT_STS,
    #[doc = "0x18 - QMSPI Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x1c - QMSPI Buffer Count Trigger Register"]
    pub buf_cnt_trig: BUF_CNT_TRIG,
    _reserved_8_tx_fifo_u: [u8; 0x04],
    _reserved_9_rx_fifo_u: [u8; 0x04],
    _reserved10: [u8; 0x08],
    #[doc = "0x30..0x44 - QMSPI Description Buffer Register"]
    pub descr: [DESCR; 5],
}
impl RegisterBlock {
    #[doc = "0x20 - QMSPI Transmit Buffer Register"]
    #[inline(always)]
    pub fn tx_fifo_u08(&self) -> &[TX_FIFO_U08; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const [TX_FIFO_U08; 4]) }
    }
    #[doc = "0x20 - QMSPI Transmit Buffer Register"]
    #[inline(always)]
    pub fn tx_fifo_u16(&self) -> &[TX_FIFO_U16; 2] {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const [TX_FIFO_U16; 2]) }
    }
    #[doc = "0x20 - QMSPI Transmit Buffer Register"]
    #[inline(always)]
    pub fn tx_fifo_u32(&self) -> &TX_FIFO_U32 {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const TX_FIFO_U32) }
    }
    #[doc = "0x24 - QMSPI Receive Buffer Register"]
    #[inline(always)]
    pub fn rx_fifo_u08(&self) -> &[RX_FIFO_U08; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const [RX_FIFO_U08; 4]) }
    }
    #[doc = "0x24 - QMSPI Receive Buffer Register"]
    #[inline(always)]
    pub fn rx_fifo_u16(&self) -> &[RX_FIFO_U16; 2] {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const [RX_FIFO_U16; 2]) }
    }
    #[doc = "0x24 - QMSPI Receive Buffer Register"]
    #[inline(always)]
    pub fn rx_fifo_u32(&self) -> &RX_FIFO_U32 {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const RX_FIFO_U32) }
    }
}
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "QMSPI Mode Register"]
pub mod mode;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "QMSPI SPI Control"]
pub mod ctrl;
#[doc = "EXE (rw) register accessor: an alias for `Reg<EXE_SPEC>`"]
pub type EXE = crate::Reg<exe::EXE_SPEC>;
#[doc = "QMSPI Execute Register"]
pub mod exe;
#[doc = "IFCTRL (rw) register accessor: an alias for `Reg<IFCTRL_SPEC>`"]
pub type IFCTRL = crate::Reg<ifctrl::IFCTRL_SPEC>;
#[doc = "QMSPI Interface Control Register"]
pub mod ifctrl;
#[doc = "STS (rw) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "QMSPI Status Register"]
pub mod sts;
#[doc = "BUF_CNT_STS (rw) register accessor: an alias for `Reg<BUF_CNT_STS_SPEC>`"]
pub type BUF_CNT_STS = crate::Reg<buf_cnt_sts::BUF_CNT_STS_SPEC>;
#[doc = "QMSPI Buffer Count Status Register"]
pub mod buf_cnt_sts;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "QMSPI Interrupt Enable Register"]
pub mod ien;
#[doc = "BUF_CNT_TRIG (rw) register accessor: an alias for `Reg<BUF_CNT_TRIG_SPEC>`"]
pub type BUF_CNT_TRIG = crate::Reg<buf_cnt_trig::BUF_CNT_TRIG_SPEC>;
#[doc = "QMSPI Buffer Count Trigger Register"]
pub mod buf_cnt_trig;
#[doc = "TX_FIFO_u32 (rw) register accessor: an alias for `Reg<TX_FIFO_U32_SPEC>`"]
pub type TX_FIFO_U32 = crate::Reg<tx_fifo_u32::TX_FIFO_U32_SPEC>;
#[doc = "QMSPI Transmit Buffer Register"]
pub mod tx_fifo_u32;
#[doc = "TX_FIFO_u16 (rw) register accessor: an alias for `Reg<TX_FIFO_U16_SPEC>`"]
pub type TX_FIFO_U16 = crate::Reg<tx_fifo_u16::TX_FIFO_U16_SPEC>;
#[doc = "QMSPI Transmit Buffer Register"]
pub mod tx_fifo_u16;
#[doc = "TX_FIFO_u08 (rw) register accessor: an alias for `Reg<TX_FIFO_U08_SPEC>`"]
pub type TX_FIFO_U08 = crate::Reg<tx_fifo_u08::TX_FIFO_U08_SPEC>;
#[doc = "QMSPI Transmit Buffer Register"]
pub mod tx_fifo_u08;
#[doc = "RX_FIFO_u32 (rw) register accessor: an alias for `Reg<RX_FIFO_U32_SPEC>`"]
pub type RX_FIFO_U32 = crate::Reg<rx_fifo_u32::RX_FIFO_U32_SPEC>;
#[doc = "QMSPI Receive Buffer Register"]
pub mod rx_fifo_u32;
#[doc = "RX_FIFO_u16 (rw) register accessor: an alias for `Reg<RX_FIFO_U16_SPEC>`"]
pub type RX_FIFO_U16 = crate::Reg<rx_fifo_u16::RX_FIFO_U16_SPEC>;
#[doc = "QMSPI Receive Buffer Register"]
pub mod rx_fifo_u16;
#[doc = "RX_FIFO_u08 (rw) register accessor: an alias for `Reg<RX_FIFO_U08_SPEC>`"]
pub type RX_FIFO_U08 = crate::Reg<rx_fifo_u08::RX_FIFO_U08_SPEC>;
#[doc = "QMSPI Receive Buffer Register"]
pub mod rx_fifo_u08;
#[doc = "DESCR (rw) register accessor: an alias for `Reg<DESCR_SPEC>`"]
pub type DESCR = crate::Reg<descr::DESCR_SPEC>;
#[doc = "QMSPI Description Buffer Register"]
pub mod descr;
