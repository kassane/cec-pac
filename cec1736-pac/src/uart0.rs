#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_data: [u8; 0x03f1],
}
impl RegisterBlock {
    #[doc = "0x00..0x3f1 - UART when DLAB=1"]
    #[inline(always)]
    pub fn dlab(&self) -> &DLAB {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DLAB) }
    }
    #[doc = "0x00..0x3f1 - UART when DLAB=0"]
    #[inline(always)]
    pub fn data(&self) -> &DATA {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DATA) }
    }
}
#[doc = "UART when DLAB=0"]
pub use data::DATA;
#[doc = r"Cluster"]
#[doc = "UART when DLAB=0"]
pub mod data;
#[doc = "UART when DLAB=1"]
pub use dlab::DLAB;
#[doc = r"Cluster"]
#[doc = "UART when DLAB=1"]
pub mod dlab;
