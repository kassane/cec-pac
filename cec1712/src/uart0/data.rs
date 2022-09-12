#[doc = r"Register block"]
#[repr(C)]
pub struct DATA {
    _reserved_0_rx_dat: [u8; 0x01],
    #[doc = "0x01 - UART Interrupt Enable Register (DLAB=0)"]
    pub ien: IEN,
    _reserved_2_int_id: [u8; 0x01],
    #[doc = "0x03 - UART Line Control Register"]
    pub lcr: LCR,
    #[doc = "0x04 - UART Modem Control Register"]
    pub mcr: MCR,
    #[doc = "0x05 - UART Line Status Register"]
    pub lsr: LSR,
    #[doc = "0x06 - UART Modem Status Register"]
    pub msr: MSR,
    #[doc = "0x07 - UART Scratchpad Register This 8 bit read/write register has no effect on the operation of the Serial Port. It is intended as a scratchpad register to be used by the programmer to hold data temporarily."]
    pub scr: SCR,
    _reserved8: [u8; 0x0328],
    #[doc = "0x330 - UART Activate Register. \\[0:0\\]
ACTIVATE When this bit is 1, the UART logical device is powered and functional. When this bit is 0, the UART logical device is powered down and inactive."]
    pub activate: ACTIVATE,
    _reserved9: [u8; 0xbf],
    #[doc = "0x3f0 - UART Config Select Register"]
    pub cfg_sel: CFG_SEL,
}
impl DATA {
    #[doc = "0x00 - UART Transmit (Write) Buffer Register (DLAB=0)"]
    #[inline(always)]
    pub fn tx_dat(&self) -> &TX_DAT {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const TX_DAT) }
    }
    #[doc = "0x00 - UART Receive (Read) Buffer Register (DLAB=0)"]
    #[inline(always)]
    pub fn rx_dat(&self) -> &RX_DAT {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const RX_DAT) }
    }
    #[doc = "0x02 - UART Interrupt Identification Register"]
    #[inline(always)]
    pub fn int_id(&self) -> &INT_ID {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const INT_ID) }
    }
    #[doc = "0x02 - UART FIFO Control Register"]
    #[inline(always)]
    pub fn fifo_cr(&self) -> &FIFO_CR {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const FIFO_CR) }
    }
}
#[doc = "RX_DAT (r) register accessor: an alias for `Reg<RX_DAT_SPEC>`"]
pub type RX_DAT = crate::Reg<rx_dat::RX_DAT_SPEC>;
#[doc = "UART Receive (Read) Buffer Register (DLAB=0)"]
pub mod rx_dat;
#[doc = "TX_DAT (w) register accessor: an alias for `Reg<TX_DAT_SPEC>`"]
pub type TX_DAT = crate::Reg<tx_dat::TX_DAT_SPEC>;
#[doc = "UART Transmit (Write) Buffer Register (DLAB=0)"]
pub mod tx_dat;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "UART Interrupt Enable Register (DLAB=0)"]
pub mod ien;
#[doc = "FIFO_CR (w) register accessor: an alias for `Reg<FIFO_CR_SPEC>`"]
pub type FIFO_CR = crate::Reg<fifo_cr::FIFO_CR_SPEC>;
#[doc = "UART FIFO Control Register"]
pub mod fifo_cr;
#[doc = "INT_ID (r) register accessor: an alias for `Reg<INT_ID_SPEC>`"]
pub type INT_ID = crate::Reg<int_id::INT_ID_SPEC>;
#[doc = "UART Interrupt Identification Register"]
pub mod int_id;
#[doc = "LCR (rw) register accessor: an alias for `Reg<LCR_SPEC>`"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "UART Line Control Register"]
pub mod lcr;
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "UART Modem Control Register"]
pub mod mcr;
#[doc = "LSR (r) register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "UART Line Status Register"]
pub mod lsr;
#[doc = "MSR (r) register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "UART Modem Status Register"]
pub mod msr;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "UART Scratchpad Register This 8 bit read/write register has no effect on the operation of the Serial Port. It is intended as a scratchpad register to be used by the programmer to hold data temporarily."]
pub mod scr;
#[doc = "ACTIVATE (rw) register accessor: an alias for `Reg<ACTIVATE_SPEC>`"]
pub type ACTIVATE = crate::Reg<activate::ACTIVATE_SPEC>;
#[doc = "UART Activate Register. \\[0:0\\]
ACTIVATE When this bit is 1, the UART logical device is powered and functional. When this bit is 0, the UART logical device is powered down and inactive."]
pub mod activate;
#[doc = "CFG_SEL (rw) register accessor: an alias for `Reg<CFG_SEL_SPEC>`"]
pub type CFG_SEL = crate::Reg<cfg_sel::CFG_SEL_SPEC>;
#[doc = "UART Config Select Register"]
pub mod cfg_sel;
