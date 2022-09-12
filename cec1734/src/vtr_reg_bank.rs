#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The Power-Fail and Reset Status Register collects and retains the VBAT RST and WDT event status when VCC1 is unpowered."]
    pub pfrs: PFRS,
}
#[doc = "PFRS (rw) register accessor: an alias for `Reg<PFRS_SPEC>`"]
pub type PFRS = crate::Reg<pfrs::PFRS_SPEC>;
#[doc = "The Power-Fail and Reset Status Register collects and retains the VBAT RST and WDT event status when VCC1 is unpowered."]
pub mod pfrs;
