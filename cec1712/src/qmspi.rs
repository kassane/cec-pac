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
    pub tx_fifo: TX_FIFO,
    #[doc = "0x24 - QMSPI Receive Buffer Register"]
    pub rx_fifo: RX_FIFO,
    #[doc = "0x28 - QMSPI Chip Select Timing Register"]
    pub cstm: CSTM,
    _reserved11: [u8; 0x04],
    #[doc = "0x30..0x70 - QMSPI Description Buffer 0 Register"]
    pub descr: [DESCR; 16],
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
