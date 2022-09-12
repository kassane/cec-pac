#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - AHB Error Address \\[0:0\\]
AHB_ERR_ADDR, In priority order:\n 1. AHB address is registered when an AHB error occurs on the processor's AHB master port and the register value was\n already 0. This way only the first address to generate an exception is captured.\n 2. The processor can clear this register by writing any 32-bit value to this register."]
    pub ahb_err_addr: AHB_ERR_ADDR,
    _reserved1: [u8; 0x0c],
    #[doc = "0x14 - AHB Error Control \\[0:0\\]
AHB_ERROR_DISABLE, 0: EC memory exceptions are enabled. 1: EC memory exceptions are disabled."]
    pub ahb_err_ctrl: AHB_ERR_CTRL,
    _reserved2: [u8; 0x03],
    #[doc = "0x18 - Interrupt Control \\[0:0\\]
NVIC_EN (NVIC_EN) This bit enables Alternate NVIC IRQ's Vectors. The Alternate NVIC Vectors provides each interrupt event with a dedicated (direct) NVIC vector.\n 0 = Alternate NVIC vectors disabled, 1= Alternate NVIC vectors enabled"]
    pub intr_ctrl: INTR_CTRL,
    #[doc = "0x1c - ETM TRACE Enable \\[0:0\\]
TRACE_EN (TRACE_EN) This bit enables the ARM TRACE debug port (ETM/ITM). The Trace Debug Interface pins are forced to the TRACE functions. 0 = ARM TRACE port disabled, 1= ARM TRACE port enabled"]
    pub etm_ctrl: ETM_CTRL,
    #[doc = "0x20 - Debug Enable Register"]
    pub debug_ctrl: DEBUG_CTRL,
    #[doc = "0x24 - Lock Register"]
    pub otp_lock: OTP_LOCK,
    #[doc = "0x28 - WDT Event Count \\[3:0\\]
WDT_COUNT (WDT_COUNT) These EC R/W bits are cleared to 0 on VCC1 POR, but not on a WDT.\n Note: This field is written by Boot ROM firmware to indicate the number of times a WDT fired before loading a good EC code image."]
    pub wdt_cnt: WDT_CNT,
    #[doc = "0x2c - AES HASH Byte Swap Control Register."]
    pub aesh_bswap_ctrl: AESH_BSWAP_CTRL,
    _reserved8: [u8; 0x10],
    #[doc = "0x40 - PECI Disable"]
    pub peci_dis: PECI_DIS,
    _reserved9: [u8; 0x08],
    #[doc = "0x4c - This register is a mirror of the Boot Control Register.\n"]
    pub stap_tmir: STAP_TMIR,
    _reserved10: [u8; 0x04],
    #[doc = "0x54 - This register contains the VTR Reset Status for BOOT ROM.\n"]
    pub brom_sts: BROM_STS,
    _reserved11: [u8; 0x07],
    #[doc = "0x5c - System Shutdown Reset"]
    pub crypto_srst: CRYPTO_SRST,
    _reserved12: [u8; 0x04],
    #[doc = "0x64 - GPIO Bank Power Register"]
    pub gpio_bank_pwr: GPIO_BANK_PWR,
    _reserved13: [u8; 0x08],
    #[doc = "0x70 - JTAG Master Configuration Register"]
    pub jtag_mcfg: JTAG_MCFG,
    #[doc = "0x74 - JTAG Master Status Register"]
    pub jtag_msts: JTAG_MSTS,
    #[doc = "0x78 - JTAG Master TDO Register"]
    pub jtag_mtdo: JTAG_MTDO,
    #[doc = "0x7c - JTAG Master TDI Register"]
    pub jtag_mtdi: JTAG_MTDI,
    #[doc = "0x80 - JTAG Master TMS Register"]
    pub jtag_mtms: JTAG_MTMS,
    #[doc = "0x84 - JTAG Master Command Register"]
    pub jtag_mcmd: JTAG_MCMD,
    _reserved19: [u8; 0xf8],
    #[doc = "0x180 - BOOT ROM Scratch 0 Register"]
    pub fw_scr0: FW_SCR0,
    #[doc = "0x184 - BOOT ROM Scratch 1 Register"]
    pub fw_scr1: FW_SCR1,
    #[doc = "0x188 - BOOT ROM Scratch 2 Register"]
    pub fw_scr2: FW_SCR2,
    #[doc = "0x18c - BOOT ROM Scratch 3 Register"]
    pub fw_scr3: FW_SCR3,
}
#[doc = "AHB_ERR_ADDR (rw) register accessor: an alias for `Reg<AHB_ERR_ADDR_SPEC>`"]
pub type AHB_ERR_ADDR = crate::Reg<ahb_err_addr::AHB_ERR_ADDR_SPEC>;
#[doc = "AHB Error Address \\[0:0\\]
AHB_ERR_ADDR, In priority order:\n 1. AHB address is registered when an AHB error occurs on the processor's AHB master port and the register value was\n already 0. This way only the first address to generate an exception is captured.\n 2. The processor can clear this register by writing any 32-bit value to this register."]
pub mod ahb_err_addr;
#[doc = "AHB_ERR_CTRL (rw) register accessor: an alias for `Reg<AHB_ERR_CTRL_SPEC>`"]
pub type AHB_ERR_CTRL = crate::Reg<ahb_err_ctrl::AHB_ERR_CTRL_SPEC>;
#[doc = "AHB Error Control \\[0:0\\]
AHB_ERROR_DISABLE, 0: EC memory exceptions are enabled. 1: EC memory exceptions are disabled."]
pub mod ahb_err_ctrl;
#[doc = "INTR_CTRL (rw) register accessor: an alias for `Reg<INTR_CTRL_SPEC>`"]
pub type INTR_CTRL = crate::Reg<intr_ctrl::INTR_CTRL_SPEC>;
#[doc = "Interrupt Control \\[0:0\\]
NVIC_EN (NVIC_EN) This bit enables Alternate NVIC IRQ's Vectors. The Alternate NVIC Vectors provides each interrupt event with a dedicated (direct) NVIC vector.\n 0 = Alternate NVIC vectors disabled, 1= Alternate NVIC vectors enabled"]
pub mod intr_ctrl;
#[doc = "ETM_CTRL (rw) register accessor: an alias for `Reg<ETM_CTRL_SPEC>`"]
pub type ETM_CTRL = crate::Reg<etm_ctrl::ETM_CTRL_SPEC>;
#[doc = "ETM TRACE Enable \\[0:0\\]
TRACE_EN (TRACE_EN) This bit enables the ARM TRACE debug port (ETM/ITM). The Trace Debug Interface pins are forced to the TRACE functions. 0 = ARM TRACE port disabled, 1= ARM TRACE port enabled"]
pub mod etm_ctrl;
#[doc = "DEBUG_CTRL (rw) register accessor: an alias for `Reg<DEBUG_CTRL_SPEC>`"]
pub type DEBUG_CTRL = crate::Reg<debug_ctrl::DEBUG_CTRL_SPEC>;
#[doc = "Debug Enable Register"]
pub mod debug_ctrl;
#[doc = "OTP_LOCK (rw) register accessor: an alias for `Reg<OTP_LOCK_SPEC>`"]
pub type OTP_LOCK = crate::Reg<otp_lock::OTP_LOCK_SPEC>;
#[doc = "Lock Register"]
pub mod otp_lock;
#[doc = "WDT_CNT (rw) register accessor: an alias for `Reg<WDT_CNT_SPEC>`"]
pub type WDT_CNT = crate::Reg<wdt_cnt::WDT_CNT_SPEC>;
#[doc = "WDT Event Count \\[3:0\\]
WDT_COUNT (WDT_COUNT) These EC R/W bits are cleared to 0 on VCC1 POR, but not on a WDT.\n Note: This field is written by Boot ROM firmware to indicate the number of times a WDT fired before loading a good EC code image."]
pub mod wdt_cnt;
#[doc = "AESH_BSWAP_CTRL (rw) register accessor: an alias for `Reg<AESH_BSWAP_CTRL_SPEC>`"]
pub type AESH_BSWAP_CTRL = crate::Reg<aesh_bswap_ctrl::AESH_BSWAP_CTRL_SPEC>;
#[doc = "AES HASH Byte Swap Control Register."]
pub mod aesh_bswap_ctrl;
#[doc = "PECI_DIS (rw) register accessor: an alias for `Reg<PECI_DIS_SPEC>`"]
pub type PECI_DIS = crate::Reg<peci_dis::PECI_DIS_SPEC>;
#[doc = "PECI Disable"]
pub mod peci_dis;
#[doc = "STAP_TMIR (r) register accessor: an alias for `Reg<STAP_TMIR_SPEC>`"]
pub type STAP_TMIR = crate::Reg<stap_tmir::STAP_TMIR_SPEC>;
#[doc = "This register is a mirror of the Boot Control Register.\n"]
pub mod stap_tmir;
#[doc = "BROM_STS (rw) register accessor: an alias for `Reg<BROM_STS_SPEC>`"]
pub type BROM_STS = crate::Reg<brom_sts::BROM_STS_SPEC>;
#[doc = "This register contains the VTR Reset Status for BOOT ROM.\n"]
pub mod brom_sts;
#[doc = "CRYPTO_SRST (rw) register accessor: an alias for `Reg<CRYPTO_SRST_SPEC>`"]
pub type CRYPTO_SRST = crate::Reg<crypto_srst::CRYPTO_SRST_SPEC>;
#[doc = "System Shutdown Reset"]
pub mod crypto_srst;
#[doc = "GPIO_BANK_PWR (rw) register accessor: an alias for `Reg<GPIO_BANK_PWR_SPEC>`"]
pub type GPIO_BANK_PWR = crate::Reg<gpio_bank_pwr::GPIO_BANK_PWR_SPEC>;
#[doc = "GPIO Bank Power Register"]
pub mod gpio_bank_pwr;
#[doc = "JTAG_MCFG (rw) register accessor: an alias for `Reg<JTAG_MCFG_SPEC>`"]
pub type JTAG_MCFG = crate::Reg<jtag_mcfg::JTAG_MCFG_SPEC>;
#[doc = "JTAG Master Configuration Register"]
pub mod jtag_mcfg;
#[doc = "JTAG_MSTS (r) register accessor: an alias for `Reg<JTAG_MSTS_SPEC>`"]
pub type JTAG_MSTS = crate::Reg<jtag_msts::JTAG_MSTS_SPEC>;
#[doc = "JTAG Master Status Register"]
pub mod jtag_msts;
#[doc = "JTAG_MTDO (rw) register accessor: an alias for `Reg<JTAG_MTDO_SPEC>`"]
pub type JTAG_MTDO = crate::Reg<jtag_mtdo::JTAG_MTDO_SPEC>;
#[doc = "JTAG Master TDO Register"]
pub mod jtag_mtdo;
#[doc = "JTAG_MTDI (rw) register accessor: an alias for `Reg<JTAG_MTDI_SPEC>`"]
pub type JTAG_MTDI = crate::Reg<jtag_mtdi::JTAG_MTDI_SPEC>;
#[doc = "JTAG Master TDI Register"]
pub mod jtag_mtdi;
#[doc = "JTAG_MTMS (rw) register accessor: an alias for `Reg<JTAG_MTMS_SPEC>`"]
pub type JTAG_MTMS = crate::Reg<jtag_mtms::JTAG_MTMS_SPEC>;
#[doc = "JTAG Master TMS Register"]
pub mod jtag_mtms;
#[doc = "JTAG_MCMD (rw) register accessor: an alias for `Reg<JTAG_MCMD_SPEC>`"]
pub type JTAG_MCMD = crate::Reg<jtag_mcmd::JTAG_MCMD_SPEC>;
#[doc = "JTAG Master Command Register"]
pub mod jtag_mcmd;
#[doc = "FW_SCR0 (rw) register accessor: an alias for `Reg<FW_SCR0_SPEC>`"]
pub type FW_SCR0 = crate::Reg<fw_scr0::FW_SCR0_SPEC>;
#[doc = "BOOT ROM Scratch 0 Register"]
pub mod fw_scr0;
#[doc = "FW_SCR1 (rw) register accessor: an alias for `Reg<FW_SCR1_SPEC>`"]
pub type FW_SCR1 = crate::Reg<fw_scr1::FW_SCR1_SPEC>;
#[doc = "BOOT ROM Scratch 1 Register"]
pub mod fw_scr1;
#[doc = "FW_SCR2 (rw) register accessor: an alias for `Reg<FW_SCR2_SPEC>`"]
pub type FW_SCR2 = crate::Reg<fw_scr2::FW_SCR2_SPEC>;
#[doc = "BOOT ROM Scratch 2 Register"]
pub mod fw_scr2;
#[doc = "FW_SCR3 (rw) register accessor: an alias for `Reg<FW_SCR3_SPEC>`"]
pub type FW_SCR3 = crate::Reg<fw_scr3::FW_SCR3_SPEC>;
#[doc = "BOOT ROM Scratch 3 Register"]
pub mod fw_scr3;
