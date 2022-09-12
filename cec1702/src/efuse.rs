#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - eFUSE CTRL Register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Manual Control Register"]
    pub man_ctrl: MAN_CTRL,
    #[doc = "0x06 - MANUAL MODE ADDRESS REGISTER"]
    pub man_mod_addr: MAN_MOD_ADDR,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - MANUAL MODE DATA REGISTER"]
    pub man_mod_data: MAN_MOD_DATA,
    #[doc = "0x10..0x210 - 512 Bytes of EFUSE Memory (IP_MEM) Represented in 128 DW chunks:\n eFUSE memory read-back data. Access to this region depends on the operating mode: NORMAL MODE: Reading any of the bytes\n starting at this base will automatically start the controller to sequence all eFUSE signals to generate read data. Wait cycles added\n to the read cycle as appropriate. MANUAL MODE: Refer to the manual mode section for the proper procedure for accessing data in this mode.\n See REG_MAN_CTRL.MAN_EN and REG_CTRL.EXT_PRGM bits for controlling the operating mode of the block."]
    pub mem_dw: [MEM_DW; 128],
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "eFUSE CTRL Register"]
pub mod ctrl;
#[doc = "MAN_CTRL (rw) register accessor: an alias for `Reg<MAN_CTRL_SPEC>`"]
pub type MAN_CTRL = crate::Reg<man_ctrl::MAN_CTRL_SPEC>;
#[doc = "Manual Control Register"]
pub mod man_ctrl;
#[doc = "MAN_MOD_ADDR (rw) register accessor: an alias for `Reg<MAN_MOD_ADDR_SPEC>`"]
pub type MAN_MOD_ADDR = crate::Reg<man_mod_addr::MAN_MOD_ADDR_SPEC>;
#[doc = "MANUAL MODE ADDRESS REGISTER"]
pub mod man_mod_addr;
#[doc = "MAN_MOD_DATA (rw) register accessor: an alias for `Reg<MAN_MOD_DATA_SPEC>`"]
pub type MAN_MOD_DATA = crate::Reg<man_mod_data::MAN_MOD_DATA_SPEC>;
#[doc = "MANUAL MODE DATA REGISTER"]
pub mod man_mod_data;
#[doc = "MEM_DW (rw) register accessor: an alias for `Reg<MEM_DW_SPEC>`"]
pub type MEM_DW = crate::Reg<mem_dw::MEM_DW_SPEC>;
#[doc = "512 Bytes of EFUSE Memory (IP_MEM) Represented in 128 DW chunks:\n eFUSE memory read-back data. Access to this region depends on the operating mode: NORMAL MODE: Reading any of the bytes\n starting at this base will automatically start the controller to sequence all eFUSE signals to generate read data. Wait cycles added\n to the read cycle as appropriate. MANUAL MODE: Refer to the manual mode section for the proper procedure for accessing data in this mode.\n See REG_MAN_CTRL.MAN_EN and REG_CTRL.EXT_PRGM bits for controlling the operating mode of the block."]
pub mod mem_dw;
