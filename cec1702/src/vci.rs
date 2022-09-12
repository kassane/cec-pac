#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - VCI Register"]
    pub ctrl_sts: CTRL_STS,
    #[doc = "0x04 - Latch Enable Register"]
    pub latch_en: LATCH_EN,
    #[doc = "0x08 - Latch Resets Register"]
    pub latch_rst: LATCH_RST,
    #[doc = "0x0c - VCI Input Enable Register"]
    pub input_en: INPUT_EN,
    #[doc = "0x10 - Holdoff Count Register"]
    pub hldoff_cnt: HLDOFF_CNT,
    #[doc = "0x14 - VCI Polarity Register"]
    pub polarity: POLARITY,
    #[doc = "0x18 - VCI Posedge Detect Register"]
    pub pedge_det: PEDGE_DET,
    #[doc = "0x1c - VCI Negedge Detect Register"]
    pub nedge_det: NEDGE_DET,
    #[doc = "0x20 - VCI Buffer Enable Register"]
    pub buffer_en: BUFFER_EN,
}
#[doc = "CTRL_STS (rw) register accessor: an alias for `Reg<CTRL_STS_SPEC>`"]
pub type CTRL_STS = crate::Reg<ctrl_sts::CTRL_STS_SPEC>;
#[doc = "VCI Register"]
pub mod ctrl_sts;
#[doc = "LATCH_EN (rw) register accessor: an alias for `Reg<LATCH_EN_SPEC>`"]
pub type LATCH_EN = crate::Reg<latch_en::LATCH_EN_SPEC>;
#[doc = "Latch Enable Register"]
pub mod latch_en;
#[doc = "LATCH_RST (rw) register accessor: an alias for `Reg<LATCH_RST_SPEC>`"]
pub type LATCH_RST = crate::Reg<latch_rst::LATCH_RST_SPEC>;
#[doc = "Latch Resets Register"]
pub mod latch_rst;
#[doc = "INPUT_EN (rw) register accessor: an alias for `Reg<INPUT_EN_SPEC>`"]
pub type INPUT_EN = crate::Reg<input_en::INPUT_EN_SPEC>;
#[doc = "VCI Input Enable Register"]
pub mod input_en;
#[doc = "HLDOFF_CNT (rw) register accessor: an alias for `Reg<HLDOFF_CNT_SPEC>`"]
pub type HLDOFF_CNT = crate::Reg<hldoff_cnt::HLDOFF_CNT_SPEC>;
#[doc = "Holdoff Count Register"]
pub mod hldoff_cnt;
#[doc = "POLARITY (rw) register accessor: an alias for `Reg<POLARITY_SPEC>`"]
pub type POLARITY = crate::Reg<polarity::POLARITY_SPEC>;
#[doc = "VCI Polarity Register"]
pub mod polarity;
#[doc = "PEDGE_DET (rw) register accessor: an alias for `Reg<PEDGE_DET_SPEC>`"]
pub type PEDGE_DET = crate::Reg<pedge_det::PEDGE_DET_SPEC>;
#[doc = "VCI Posedge Detect Register"]
pub mod pedge_det;
#[doc = "NEDGE_DET (rw) register accessor: an alias for `Reg<NEDGE_DET_SPEC>`"]
pub type NEDGE_DET = crate::Reg<nedge_det::NEDGE_DET_SPEC>;
#[doc = "VCI Negedge Detect Register"]
pub mod nedge_det;
#[doc = "BUFFER_EN (rw) register accessor: an alias for `Reg<BUFFER_EN_SPEC>`"]
pub type BUFFER_EN = crate::Reg<buffer_en::BUFFER_EN_SPEC>;
#[doc = "VCI Buffer Enable Register"]
pub mod buffer_en;
