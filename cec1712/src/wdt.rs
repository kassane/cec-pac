#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Writing this field reloads the Watch Dog Timer counter."]
    pub load: LOAD,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - WDT Control Register"]
    pub ctrl: CTRL,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - The WDT Kick Register is a strobe. Reads of this register return 0. Writes to this register cause the WDT to reload\n the WDT Load Register value and start decrementing when the WDT_ENABLE bit in the WDT Control Register is set to '1'. When the WDT_ENABLE\n bit in the WDT Control Register is cleared to '0', writes to the WDT Kick Register have no effect."]
    pub kick: KICK,
    _reserved3: [u8; 0x03],
    #[doc = "0x0c - This read-only register provides the current WDT count."]
    pub cnt: CNT,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - This register provides the current WDT count."]
    pub sts: STS,
    _reserved5: [u8; 0x03],
    #[doc = "0x14 - Watch Dog Interrupt Enable Register.\n"]
    pub ien: IEN,
}
#[doc = "LOAD (rw) register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "Writing this field reloads the Watch Dog Timer counter."]
pub mod load;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "WDT Control Register"]
pub mod ctrl;
#[doc = "KICK (w) register accessor: an alias for `Reg<KICK_SPEC>`"]
pub type KICK = crate::Reg<kick::KICK_SPEC>;
#[doc = "The WDT Kick Register is a strobe. Reads of this register return 0. Writes to this register cause the WDT to reload\n the WDT Load Register value and start decrementing when the WDT_ENABLE bit in the WDT Control Register is set to '1'. When the WDT_ENABLE\n bit in the WDT Control Register is cleared to '0', writes to the WDT Kick Register have no effect."]
pub mod kick;
#[doc = "CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "This read-only register provides the current WDT count."]
pub mod cnt;
#[doc = "STS (rw) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "This register provides the current WDT count."]
pub mod sts;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Watch Dog Interrupt Enable Register.\n"]
pub mod ien;
