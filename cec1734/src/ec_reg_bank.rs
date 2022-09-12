#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM Configuration Register"]
    pub sram_cnfg: SRAM_CNFG,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - AHB Error Address \\[0:0\\]"]
    pub ahb_err_addr: AHB_ERR_ADDR,
    _reserved2: [u8; 0x0c],
    #[doc = "0x14 - AHB Error Control \\[0:0\\]
AHB_ERROR_DISABLE, 0: EC memory exceptions are enabled. 1: EC memory exceptions are disabled."]
    pub ahb_err_ctrl: AHB_ERR_CTRL,
    _reserved3: [u8; 0x03],
    #[doc = "0x18 - Interrupt Control \\[0:0\\]
NVIC_EN (NVIC_EN) This bit enables Alternate NVIC IRQ's Vectors. The Alternate NVIC Vectors provides each interrupt event with a dedicated (direct) NVIC vector. 0 = Alternate NVIC vectors disabled, 1= Alternate NVIC vectors enabled"]
    pub intr_ctrl: INTR_CTRL,
    #[doc = "0x1c - ETM TRACE Enable \\[0:0\\]
TRACE_EN (TRACE_EN) This bit enables the ARM TRACE debug port (ETM/ITM). The Trace Debug Interface pins are forced to the TRACE functions. 0 = ARM TRACE port disabled, 1= ARM TRACE port enabled"]
    pub etm_ctrl: ETM_CTRL,
    #[doc = "0x20 - Debug Enable Register"]
    pub debug_ctrl: DEBUG_CTRL,
    #[doc = "0x24 - Lock Register"]
    pub otp_lock: OTP_LOCK,
    #[doc = "0x28 - WDT Event Count \\[3:0\\]
WDT_COUNT (WDT_COUNT) These EC R/W bits are cleared to 0 on VCC1 POR, but not on a WDT. Note: This field is written by Boot ROM firmware to indicate the number of times a WDT fired before loading a good EC code image."]
    pub wdt_cnt: WDT_CNT,
    #[doc = "0x2c - AES HASH Byte Swap Control Register."]
    pub aesh_bswap_ctrl: AESH_BSWAP_CTRL,
    _reserved9: [u8; 0x34],
    #[doc = "0x64 - GPIO Bank Power Register"]
    pub gpio_bank_pwr: GPIO_BANK_PWR,
    _reserved10: [u8; 0x08],
    #[doc = "0x70 - Security Monitor SRAM Bank Swap Register"]
    pub sram_bnk_swp: SRAM_BNK_SWP,
    _reserved11: [u8; 0x1c],
    #[doc = "0x90 - Virtual Wire Source Configuration Register"]
    pub vw_src_cngf: VW_SRC_CNGF,
    _reserved12: [u8; 0x30],
    #[doc = "0xc4 - SPI Monitor's Inter-Bus Configuration Register"]
    pub spimon_ib_cngf: SPIMON_IB_CNGF,
    _reserved13: [u8; 0x0178],
    #[doc = "0x240 - PAD Monitor Control Register"]
    pub pd_mon_ctrl: PD_MON_CTRL,
    #[doc = "0x244 - PAD Monitor Interrupt Enable Register"]
    pub pd_mon_int_en: PD_MON_INT_EN,
    #[doc = "0x248 - PAD Monitor Status Register"]
    pub pd_mon_sts: PD_MON_STS,
}
#[doc = "SRAM_CNFG (rw) register accessor: an alias for `Reg<SRAM_CNFG_SPEC>`"]
pub type SRAM_CNFG = crate::Reg<sram_cnfg::SRAM_CNFG_SPEC>;
#[doc = "SRAM Configuration Register"]
pub mod sram_cnfg;
#[doc = "AHB_ERR_ADDR (rw) register accessor: an alias for `Reg<AHB_ERR_ADDR_SPEC>`"]
pub type AHB_ERR_ADDR = crate::Reg<ahb_err_addr::AHB_ERR_ADDR_SPEC>;
#[doc = "AHB Error Address \\[0:0\\]"]
pub mod ahb_err_addr;
#[doc = "AHB_ERR_CTRL (rw) register accessor: an alias for `Reg<AHB_ERR_CTRL_SPEC>`"]
pub type AHB_ERR_CTRL = crate::Reg<ahb_err_ctrl::AHB_ERR_CTRL_SPEC>;
#[doc = "AHB Error Control \\[0:0\\]
AHB_ERROR_DISABLE, 0: EC memory exceptions are enabled. 1: EC memory exceptions are disabled."]
pub mod ahb_err_ctrl;
#[doc = "INTR_CTRL (rw) register accessor: an alias for `Reg<INTR_CTRL_SPEC>`"]
pub type INTR_CTRL = crate::Reg<intr_ctrl::INTR_CTRL_SPEC>;
#[doc = "Interrupt Control \\[0:0\\]
NVIC_EN (NVIC_EN) This bit enables Alternate NVIC IRQ's Vectors. The Alternate NVIC Vectors provides each interrupt event with a dedicated (direct) NVIC vector. 0 = Alternate NVIC vectors disabled, 1= Alternate NVIC vectors enabled"]
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
WDT_COUNT (WDT_COUNT) These EC R/W bits are cleared to 0 on VCC1 POR, but not on a WDT. Note: This field is written by Boot ROM firmware to indicate the number of times a WDT fired before loading a good EC code image."]
pub mod wdt_cnt;
#[doc = "AESH_BSWAP_CTRL (rw) register accessor: an alias for `Reg<AESH_BSWAP_CTRL_SPEC>`"]
pub type AESH_BSWAP_CTRL = crate::Reg<aesh_bswap_ctrl::AESH_BSWAP_CTRL_SPEC>;
#[doc = "AES HASH Byte Swap Control Register."]
pub mod aesh_bswap_ctrl;
#[doc = "GPIO_BANK_PWR (rw) register accessor: an alias for `Reg<GPIO_BANK_PWR_SPEC>`"]
pub type GPIO_BANK_PWR = crate::Reg<gpio_bank_pwr::GPIO_BANK_PWR_SPEC>;
#[doc = "GPIO Bank Power Register"]
pub mod gpio_bank_pwr;
#[doc = "SRAM_BNK_SWP (rw) register accessor: an alias for `Reg<SRAM_BNK_SWP_SPEC>`"]
pub type SRAM_BNK_SWP = crate::Reg<sram_bnk_swp::SRAM_BNK_SWP_SPEC>;
#[doc = "Security Monitor SRAM Bank Swap Register"]
pub mod sram_bnk_swp;
#[doc = "VW_SRC_CNGF (rw) register accessor: an alias for `Reg<VW_SRC_CNGF_SPEC>`"]
pub type VW_SRC_CNGF = crate::Reg<vw_src_cngf::VW_SRC_CNGF_SPEC>;
#[doc = "Virtual Wire Source Configuration Register"]
pub mod vw_src_cngf;
#[doc = "SPIMON_IB_CNGF (rw) register accessor: an alias for `Reg<SPIMON_IB_CNGF_SPEC>`"]
pub type SPIMON_IB_CNGF = crate::Reg<spimon_ib_cngf::SPIMON_IB_CNGF_SPEC>;
#[doc = "SPI Monitor's Inter-Bus Configuration Register"]
pub mod spimon_ib_cngf;
#[doc = "PD_MON_CTRL (rw) register accessor: an alias for `Reg<PD_MON_CTRL_SPEC>`"]
pub type PD_MON_CTRL = crate::Reg<pd_mon_ctrl::PD_MON_CTRL_SPEC>;
#[doc = "PAD Monitor Control Register"]
pub mod pd_mon_ctrl;
#[doc = "PD_MON_INT_EN (rw) register accessor: an alias for `Reg<PD_MON_INT_EN_SPEC>`"]
pub type PD_MON_INT_EN = crate::Reg<pd_mon_int_en::PD_MON_INT_EN_SPEC>;
#[doc = "PAD Monitor Interrupt Enable Register"]
pub mod pd_mon_int_en;
#[doc = "PD_MON_STS (rw) register accessor: an alias for `Reg<PD_MON_STS_SPEC>`"]
pub type PD_MON_STS = crate::Reg<pd_mon_sts::PD_MON_STS_SPEC>;
#[doc = "PAD Monitor Status Register"]
pub mod pd_mon_sts;
