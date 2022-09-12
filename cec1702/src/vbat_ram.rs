#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x80 - 32-bits of VBAT powered RAM."]
    pub mem_u32: [MEM_U32; 32],
}
#[doc = "MEM_u32 (rw) register accessor: an alias for `Reg<MEM_U32_SPEC>`"]
pub type MEM_U32 = crate::Reg<mem_u32::MEM_U32_SPEC>;
#[doc = "32-bits of VBAT powered RAM."]
pub mod mem_u32;
