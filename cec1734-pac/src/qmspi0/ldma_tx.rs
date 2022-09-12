#[doc = r"Register block"]
#[repr(C)]
pub struct LDMA_TX {
    #[doc = "0x00 - QMSPI TX Control Register"]
    pub ldma_txctrl: LDMA_TXCTRL,
    #[doc = "0x04 - QMSPI Local DMA Tx Start Address Register"]
    pub ldma_txstrt_addr: LDMA_TXSTRT_ADDR,
    #[doc = "0x08 - QMSPI Local DMA Tx Length Register"]
    pub ldma_tx_len: LDMA_TX_LEN,
    #[doc = "0x0c - Reserved Register"]
    pub rsvd: RSVD,
}
#[doc = "LDMA_TXCTRL (rw) register accessor: an alias for `Reg<LDMA_TXCTRL_SPEC>`"]
pub type LDMA_TXCTRL = crate::Reg<ldma_txctrl::LDMA_TXCTRL_SPEC>;
#[doc = "QMSPI TX Control Register"]
pub mod ldma_txctrl;
#[doc = "LDMA_TXSTRT_ADDR (rw) register accessor: an alias for `Reg<LDMA_TXSTRT_ADDR_SPEC>`"]
pub type LDMA_TXSTRT_ADDR = crate::Reg<ldma_txstrt_addr::LDMA_TXSTRT_ADDR_SPEC>;
#[doc = "QMSPI Local DMA Tx Start Address Register"]
pub mod ldma_txstrt_addr;
#[doc = "LDMA_TX_LEN (rw) register accessor: an alias for `Reg<LDMA_TX_LEN_SPEC>`"]
pub type LDMA_TX_LEN = crate::Reg<ldma_tx_len::LDMA_TX_LEN_SPEC>;
#[doc = "QMSPI Local DMA Tx Length Register"]
pub mod ldma_tx_len;
#[doc = "RSVD (r) register accessor: an alias for `Reg<RSVD_SPEC>`"]
pub type RSVD = crate::Reg<rsvd::RSVD_SPEC>;
#[doc = "Reserved Register"]
pub mod rsvd;
