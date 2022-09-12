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
    #[doc = "0x20 - QMSPI Transmit Buffer Register"]
    pub tx_fifo: [TX_FIFO; 1],
    #[doc = "0x24 - QMSPI Receive Buffer Register"]
    pub rx_fifo: [RX_FIFO; 1],
    #[doc = "0x28 - QMSPI Chip Select Timing Register"]
    pub cstm: CSTM,
    _reserved11: [u8; 0x04],
    #[doc = "0x30..0x70 - QMSPI Description Buffer 0 Register"]
    pub descr: [DESCR; 16],
    _reserved12: [u8; 0x40],
    #[doc = "0xb0 - QMSPI Alias Control Register"]
    pub alias_ctrl: ALIAS_CTRL,
    _reserved13: [u8; 0x0c],
    #[doc = "0xc0 - QMSPI Mode Alternate 1 Register"]
    pub mode_alt1: MODE_ALT1,
    _reserved14: [u8; 0x0c],
    #[doc = "0xd0 - QMSPI TAPs Register"]
    pub taps: TAPS,
    #[doc = "0xd4 - QMSPI TAP Control Register"]
    pub tap_adj: TAP_ADJ,
    #[doc = "0xd8 - QMSPI TAP Adjustment Register"]
    pub tap_ctrl: TAP_CTRL,
    _reserved17: [u8; 0x24],
    #[doc = "0x100 - QMSPI Descriptor Local DMA Rx Enable Register"]
    pub desc_ldma_rxen: DESC_LDMA_RXEN,
    #[doc = "0x104 - QMSPI Descriptor Local DMA Tx Enable Register"]
    pub desc_ldma_txen: DESC_LDMA_TXEN,
    _reserved19: [u8; 0x08],
    #[doc = "0x110..0x140 - LDMA_RX\\[%s\\]"]
    pub ldma_rx: [LDMA_RX; 3],
    #[doc = "0x140..0x170 - LDMA_TX\\[%s\\]"]
    pub ldma_tx: [LDMA_TX; 3],
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
#[doc = "TX_FIFO (rw) register accessor: an alias for `Reg<TX_FIFO_SPEC>`"]
pub type TX_FIFO = crate::Reg<tx_fifo::TX_FIFO_SPEC>;
#[doc = "QMSPI Transmit Buffer Register"]
pub mod tx_fifo;
#[doc = "RX_FIFO (rw) register accessor: an alias for `Reg<RX_FIFO_SPEC>`"]
pub type RX_FIFO = crate::Reg<rx_fifo::RX_FIFO_SPEC>;
#[doc = "QMSPI Receive Buffer Register"]
pub mod rx_fifo;
#[doc = "CSTM (rw) register accessor: an alias for `Reg<CSTM_SPEC>`"]
pub type CSTM = crate::Reg<cstm::CSTM_SPEC>;
#[doc = "QMSPI Chip Select Timing Register"]
pub mod cstm;
#[doc = "DESCR (rw) register accessor: an alias for `Reg<DESCR_SPEC>`"]
pub type DESCR = crate::Reg<descr::DESCR_SPEC>;
#[doc = "QMSPI Description Buffer 0 Register"]
pub mod descr;
#[doc = "ALIAS_CTRL (w) register accessor: an alias for `Reg<ALIAS_CTRL_SPEC>`"]
pub type ALIAS_CTRL = crate::Reg<alias_ctrl::ALIAS_CTRL_SPEC>;
#[doc = "QMSPI Alias Control Register"]
pub mod alias_ctrl;
#[doc = "MODE_ALT1 (rw) register accessor: an alias for `Reg<MODE_ALT1_SPEC>`"]
pub type MODE_ALT1 = crate::Reg<mode_alt1::MODE_ALT1_SPEC>;
#[doc = "QMSPI Mode Alternate 1 Register"]
pub mod mode_alt1;
#[doc = "TAPS (rw) register accessor: an alias for `Reg<TAPS_SPEC>`"]
pub type TAPS = crate::Reg<taps::TAPS_SPEC>;
#[doc = "QMSPI TAPs Register"]
pub mod taps;
#[doc = "TAP_ADJ (rw) register accessor: an alias for `Reg<TAP_ADJ_SPEC>`"]
pub type TAP_ADJ = crate::Reg<tap_adj::TAP_ADJ_SPEC>;
#[doc = "QMSPI TAP Control Register"]
pub mod tap_adj;
#[doc = "TAP_CTRL (rw) register accessor: an alias for `Reg<TAP_CTRL_SPEC>`"]
pub type TAP_CTRL = crate::Reg<tap_ctrl::TAP_CTRL_SPEC>;
#[doc = "QMSPI TAP Adjustment Register"]
pub mod tap_ctrl;
#[doc = "DESC_LDMA_TXEN (rw) register accessor: an alias for `Reg<DESC_LDMA_TXEN_SPEC>`"]
pub type DESC_LDMA_TXEN = crate::Reg<desc_ldma_txen::DESC_LDMA_TXEN_SPEC>;
#[doc = "QMSPI Descriptor Local DMA Tx Enable Register"]
pub mod desc_ldma_txen;
#[doc = "DESC_LDMA_RXEN (rw) register accessor: an alias for `Reg<DESC_LDMA_RXEN_SPEC>`"]
pub type DESC_LDMA_RXEN = crate::Reg<desc_ldma_rxen::DESC_LDMA_RXEN_SPEC>;
#[doc = "QMSPI Descriptor Local DMA Rx Enable Register"]
pub mod desc_ldma_rxen;
#[doc = "LDMA_RX\\[%s\\]"]
pub use ldma_rx::LDMA_RX;
#[doc = r"Cluster"]
#[doc = "LDMA_RX\\[%s\\]"]
pub mod ldma_rx;
#[doc = "LDMA_TX\\[%s\\]"]
pub use ldma_tx::LDMA_TX;
#[doc = r"Cluster"]
#[doc = "LDMA_TX\\[%s\\]"]
pub mod ldma_tx;
