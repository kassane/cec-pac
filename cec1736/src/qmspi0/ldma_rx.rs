#[doc = r"Register block"]
#[repr(C)]
pub struct LDMA_RX {
    #[doc = "0x00 - QMSPI RX Control Register"]
    pub ldma_rxctrl: LDMA_RXCTRL,
    #[doc = "0x04 - QMSPI Local DMA Rx Start Address Register"]
    pub ldma_rxstrt_addr: LDMA_RXSTRT_ADDR,
    #[doc = "0x08 - QMSPI Local DMA Rx Length Register"]
    pub ldma_rx_len: LDMA_RX_LEN,
    #[doc = "0x0c - Reserved Register"]
    pub rsvd: RSVD,
}
#[doc = "LDMA_RXCTRL (rw) register accessor: an alias for `Reg<LDMA_RXCTRL_SPEC>`"]
pub type LDMA_RXCTRL = crate::Reg<ldma_rxctrl::LDMA_RXCTRL_SPEC>;
#[doc = "QMSPI RX Control Register"]
pub mod ldma_rxctrl;
#[doc = "LDMA_RXSTRT_ADDR (rw) register accessor: an alias for `Reg<LDMA_RXSTRT_ADDR_SPEC>`"]
pub type LDMA_RXSTRT_ADDR = crate::Reg<ldma_rxstrt_addr::LDMA_RXSTRT_ADDR_SPEC>;
#[doc = "QMSPI Local DMA Rx Start Address Register"]
pub mod ldma_rxstrt_addr;
#[doc = "LDMA_RX_LEN (rw) register accessor: an alias for `Reg<LDMA_RX_LEN_SPEC>`"]
pub type LDMA_RX_LEN = crate::Reg<ldma_rx_len::LDMA_RX_LEN_SPEC>;
#[doc = "QMSPI Local DMA Rx Length Register"]
pub mod ldma_rx_len;
#[doc = "RSVD (r) register accessor: an alias for `Reg<RSVD_SPEC>`"]
pub type RSVD = crate::Reg<rsvd::RSVD_SPEC>;
#[doc = "Reserved Register"]
pub mod rsvd;
