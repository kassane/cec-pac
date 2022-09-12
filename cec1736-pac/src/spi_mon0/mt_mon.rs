#[doc = r"Register block"]
#[repr(C)]
pub struct MT_MON {
    #[doc = "0x00 - Match Monitor Region Begin Register"]
    pub mtmon_begin: MTMON_BEGIN,
    #[doc = "0x04 - Match Monitor Region End Register"]
    pub mtmon_end: MTMON_END,
    #[doc = "0x08 - Match Monitor Region Map Register"]
    pub map: MAP,
}
#[doc = "MTMON_BEGIN (rw) register accessor: an alias for `Reg<MTMON_BEGIN_SPEC>`"]
pub type MTMON_BEGIN = crate::Reg<mtmon_begin::MTMON_BEGIN_SPEC>;
#[doc = "Match Monitor Region Begin Register"]
pub mod mtmon_begin;
#[doc = "MTMON_END (rw) register accessor: an alias for `Reg<MTMON_END_SPEC>`"]
pub type MTMON_END = crate::Reg<mtmon_end::MTMON_END_SPEC>;
#[doc = "Match Monitor Region End Register"]
pub mod mtmon_end;
#[doc = "MAP (rw) register accessor: an alias for `Reg<MAP_SPEC>`"]
pub type MAP = crate::Reg<map::MAP_SPEC>;
#[doc = "Match Monitor Region Map Register"]
pub mod map;
