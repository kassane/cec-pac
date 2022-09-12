#[doc = r"Register block"]
#[repr(C)]
pub struct RN_TM {
    #[doc = "0x00 - Runtime Monitoring Start Register"]
    pub rt_start: RT_START,
    #[doc = "0x04 - Runtime Monitoring Limit Register"]
    pub rt_limit: RT_LIMIT,
}
#[doc = "RT_START (rw) register accessor: an alias for `Reg<RT_START_SPEC>`"]
pub type RT_START = crate::Reg<rt_start::RT_START_SPEC>;
#[doc = "Runtime Monitoring Start Register"]
pub mod rt_start;
#[doc = "RT_LIMIT (rw) register accessor: an alias for `Reg<RT_LIMIT_SPEC>`"]
pub type RT_LIMIT = crate::Reg<rt_limit::RT_LIMIT_SPEC>;
#[doc = "Runtime Monitoring Limit Register"]
pub mod rt_limit;
