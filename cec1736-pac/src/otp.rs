#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x44],
    #[doc = "0x44 - This is the Write Lock Register."]
    pub wr_lock0: WR_LOCK0,
    #[doc = "0x45 - This is the Write Lock Register."]
    pub wr_lock1: WR_LOCK1,
    #[doc = "0x46 - This is the Write Lock Register."]
    pub wr_lock2: WR_LOCK2,
    #[doc = "0x47 - This is the Write Lock Register."]
    pub wr_lock3: WR_LOCK3,
    #[doc = "0x48 - This is the Read Lock Register."]
    pub rd_lock0: RD_LOCK0,
    #[doc = "0x49 - This is the Read Lock Register."]
    pub rd_lock1: RD_LOCK1,
    #[doc = "0x4a - This is the Read Lock Register."]
    pub rd_lock2: RD_LOCK2,
    #[doc = "0x4b - This is the Read Lock Register."]
    pub rd_lock3: RD_LOCK3,
    #[doc = "0x4c - This is the Write Fine Lock Register."]
    pub wr_fine_lck: WR_FINE_LCK,
    #[doc = "0x50 - This is the Read Fine Lock Register."]
    pub rd_fine_lck: RD_FINE_LCK,
}
#[doc = "WR_LOCK0 (rw) register accessor: an alias for `Reg<WR_LOCK0_SPEC>`"]
pub type WR_LOCK0 = crate::Reg<wr_lock0::WR_LOCK0_SPEC>;
#[doc = "This is the Write Lock Register."]
pub mod wr_lock0;
#[doc = "WR_LOCK1 (rw) register accessor: an alias for `Reg<WR_LOCK1_SPEC>`"]
pub type WR_LOCK1 = crate::Reg<wr_lock1::WR_LOCK1_SPEC>;
#[doc = "This is the Write Lock Register."]
pub mod wr_lock1;
#[doc = "WR_LOCK2 (rw) register accessor: an alias for `Reg<WR_LOCK2_SPEC>`"]
pub type WR_LOCK2 = crate::Reg<wr_lock2::WR_LOCK2_SPEC>;
#[doc = "This is the Write Lock Register."]
pub mod wr_lock2;
#[doc = "WR_LOCK3 (rw) register accessor: an alias for `Reg<WR_LOCK3_SPEC>`"]
pub type WR_LOCK3 = crate::Reg<wr_lock3::WR_LOCK3_SPEC>;
#[doc = "This is the Write Lock Register."]
pub mod wr_lock3;
#[doc = "RD_LOCK0 (rw) register accessor: an alias for `Reg<RD_LOCK0_SPEC>`"]
pub type RD_LOCK0 = crate::Reg<rd_lock0::RD_LOCK0_SPEC>;
#[doc = "This is the Read Lock Register."]
pub mod rd_lock0;
#[doc = "RD_LOCK1 (rw) register accessor: an alias for `Reg<RD_LOCK1_SPEC>`"]
pub type RD_LOCK1 = crate::Reg<rd_lock1::RD_LOCK1_SPEC>;
#[doc = "This is the Read Lock Register."]
pub mod rd_lock1;
#[doc = "RD_LOCK2 (rw) register accessor: an alias for `Reg<RD_LOCK2_SPEC>`"]
pub type RD_LOCK2 = crate::Reg<rd_lock2::RD_LOCK2_SPEC>;
#[doc = "This is the Read Lock Register."]
pub mod rd_lock2;
#[doc = "RD_LOCK3 (rw) register accessor: an alias for `Reg<RD_LOCK3_SPEC>`"]
pub type RD_LOCK3 = crate::Reg<rd_lock3::RD_LOCK3_SPEC>;
#[doc = "This is the Read Lock Register."]
pub mod rd_lock3;
#[doc = "WR_FINE_LCK (rw) register accessor: an alias for `Reg<WR_FINE_LCK_SPEC>`"]
pub type WR_FINE_LCK = crate::Reg<wr_fine_lck::WR_FINE_LCK_SPEC>;
#[doc = "This is the Write Fine Lock Register."]
pub mod wr_fine_lck;
#[doc = "RD_FINE_LCK (rw) register accessor: an alias for `Reg<RD_FINE_LCK_SPEC>`"]
pub type RD_FINE_LCK = crate::Reg<rd_fine_lck::RD_FINE_LCK_SPEC>;
#[doc = "This is the Read Fine Lock Register."]
pub mod rd_fine_lck;
