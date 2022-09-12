#[doc = r"Register block"]
#[repr(C)]
pub struct GIRQ {
    #[doc = "0x00 - Status R/W1C"]
    pub src: SRC,
    #[doc = "0x04 - Write to set source enables"]
    pub en_set: EN_SET,
    #[doc = "0x08 - Read-only bitwise OR of Source and Enable"]
    pub result: RESULT,
    #[doc = "0x0c - Write to clear source enables"]
    pub en_clr: EN_CLR,
}
#[doc = "SRC (rw) register accessor: an alias for `Reg<SRC_SPEC>`"]
pub type SRC = crate::Reg<src::SRC_SPEC>;
#[doc = "Status R/W1C"]
pub mod src;
#[doc = "EN_SET (rw) register accessor: an alias for `Reg<EN_SET_SPEC>`"]
pub type EN_SET = crate::Reg<en_set::EN_SET_SPEC>;
#[doc = "Write to set source enables"]
pub mod en_set;
#[doc = "RESULT (r) register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Read-only bitwise OR of Source and Enable"]
pub mod result;
#[doc = "EN_CLR (rw) register accessor: an alias for `Reg<EN_CLR_SPEC>`"]
pub type EN_CLR = crate::Reg<en_clr::EN_CLR_SPEC>;
#[doc = "Write to clear source enables"]
pub mod en_clr;
