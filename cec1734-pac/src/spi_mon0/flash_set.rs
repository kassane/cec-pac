#[doc = r"Register block"]
#[repr(C)]
pub struct FLASH_SET {
    #[doc = "0x00..0x20 - Permit Address Register"]
    pub op_prmt: [OP_PRMT; 8],
    #[doc = "0x20..0x40 - Kill Address Register"]
    pub op_killmd: [OP_KILLMD; 8],
    #[doc = "0x40..0x60 - Write Protect Address Register"]
    pub op_wprot: [OP_WPROT; 8],
    #[doc = "0x60..0x80 - Lock Address Register"]
    pub op_lock: [OP_LOCK; 8],
}
#[doc = "OP_PRMT (rw) register accessor: an alias for `Reg<OP_PRMT_SPEC>`"]
pub type OP_PRMT = crate::Reg<op_prmt::OP_PRMT_SPEC>;
#[doc = "Permit Address Register"]
pub mod op_prmt;
#[doc = "OP_KILLMD (rw) register accessor: an alias for `Reg<OP_KILLMD_SPEC>`"]
pub type OP_KILLMD = crate::Reg<op_killmd::OP_KILLMD_SPEC>;
#[doc = "Kill Address Register"]
pub mod op_killmd;
#[doc = "OP_WPROT (rw) register accessor: an alias for `Reg<OP_WPROT_SPEC>`"]
pub type OP_WPROT = crate::Reg<op_wprot::OP_WPROT_SPEC>;
#[doc = "Write Protect Address Register"]
pub mod op_wprot;
#[doc = "OP_LOCK (rw) register accessor: an alias for `Reg<OP_LOCK_SPEC>`"]
pub type OP_LOCK = crate::Reg<op_lock::OP_LOCK_SPEC>;
#[doc = "Lock Address Register"]
pub mod op_lock;
