#[doc = r"Register block"]
#[repr(C)]
pub struct LT_MON {
    #[doc = "0x00 - Loadtime Monitor Control/Status Register"]
    pub lm_ctrlsts: LM_CTRLSTS,
    #[doc = "0x04 - Loadtime Monitor Channel Control Register"]
    pub lm_chn_ctrl: LM_CHN_CTRL,
    #[doc = "0x08 - Loadtime Monitor Channel Begin Register"]
    pub lm_begin: LM_BEGIN,
    #[doc = "0x0c - Loadtime Monitor Channel End Register"]
    pub lm_end: LM_END,
    #[doc = "0x10 - Loadtime Monitor Channel Count Register"]
    pub lm_count: LM_COUNT,
    #[doc = "0x14 - Loadtime Monitor Channel Digest Register"]
    pub lm_digest: LM_DIGEST,
}
#[doc = "LM_CTRLSTS (rw) register accessor: an alias for `Reg<LM_CTRLSTS_SPEC>`"]
pub type LM_CTRLSTS = crate::Reg<lm_ctrlsts::LM_CTRLSTS_SPEC>;
#[doc = "Loadtime Monitor Control/Status Register"]
pub mod lm_ctrlsts;
#[doc = "LM_CHN_CTRL (rw) register accessor: an alias for `Reg<LM_CHN_CTRL_SPEC>`"]
pub type LM_CHN_CTRL = crate::Reg<lm_chn_ctrl::LM_CHN_CTRL_SPEC>;
#[doc = "Loadtime Monitor Channel Control Register"]
pub mod lm_chn_ctrl;
#[doc = "LM_BEGIN (rw) register accessor: an alias for `Reg<LM_BEGIN_SPEC>`"]
pub type LM_BEGIN = crate::Reg<lm_begin::LM_BEGIN_SPEC>;
#[doc = "Loadtime Monitor Channel Begin Register"]
pub mod lm_begin;
#[doc = "LM_END (rw) register accessor: an alias for `Reg<LM_END_SPEC>`"]
pub type LM_END = crate::Reg<lm_end::LM_END_SPEC>;
#[doc = "Loadtime Monitor Channel End Register"]
pub mod lm_end;
#[doc = "LM_COUNT (r) register accessor: an alias for `Reg<LM_COUNT_SPEC>`"]
pub type LM_COUNT = crate::Reg<lm_count::LM_COUNT_SPEC>;
#[doc = "Loadtime Monitor Channel Count Register"]
pub mod lm_count;
#[doc = "LM_DIGEST (r) register accessor: an alias for `Reg<LM_DIGEST_SPEC>`"]
pub type LM_DIGEST = crate::Reg<lm_digest::LM_DIGEST_SPEC>;
#[doc = "Loadtime Monitor Channel Digest Register"]
pub mod lm_digest;
