#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - 32-bits of VBAT powered RAM."]
    pub mem: [MEM; 16],
}
#[doc = "MEM (rw) register accessor: an alias for `Reg<MEM_SPEC>`"]
pub type MEM = crate::Reg<mem::MEM_SPEC>;
#[doc = "32-bits of VBAT powered RAM."]
pub mod mem;
