#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x07],
    #[doc = "0x07 - A write to this register selects the current logical device. This allows access to the control and configuration\n registers for each logical device. Note: The Activate command operates only on the selected logical device."]
    pub ldn: LDN,
    _reserved1: [u8; 0x18],
    #[doc = "0x20 - A read-only register which provides device identification."]
    pub dev_id: DEV_ID,
    #[doc = "0x21 - A read-only register which provides device revision information."]
    pub dev_rev: DEV_REV,
}
#[doc = "LDN (rw) register accessor: an alias for `Reg<LDN_SPEC>`"]
pub type LDN = crate::Reg<ldn::LDN_SPEC>;
#[doc = "A write to this register selects the current logical device. This allows access to the control and configuration\n registers for each logical device. Note: The Activate command operates only on the selected logical device."]
pub mod ldn;
#[doc = "DEV_ID (r) register accessor: an alias for `Reg<DEV_ID_SPEC>`"]
pub type DEV_ID = crate::Reg<dev_id::DEV_ID_SPEC>;
#[doc = "A read-only register which provides device identification."]
pub mod dev_id;
#[doc = "DEV_REV (r) register accessor: an alias for `Reg<DEV_REV_SPEC>`"]
pub type DEV_REV = crate::Reg<dev_rev::DEV_REV_SPEC>;
#[doc = "A read-only register which provides device revision information."]
pub mod dev_rev;
