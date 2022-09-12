#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 0:0\\]
1=Enabled. The device is fully operational\n 0=Disabled. Clocks are gated to conserve power and the SPDOUT and SPI_CLK signals are set to their inactive state"]
    pub enable: ENABLE,
    #[doc = "0x04 - SPI Control"]
    pub ctrl: CTRL,
    #[doc = "0x08 - SPI Status"]
    pub sts: STS,
    #[doc = "0x0c - 7:0\\]
A write to this register when the \n Tx_Data buffer is empty (TXBE in the SPI Status Register is '1') initiates a SPI transaction."]
    pub tx_dat: TX_DAT,
    #[doc = "0x10 - 7:0\\]
This register is used to read the value returned by the external SPI device."]
    pub rx_dat: RX_DAT,
    #[doc = "0x14 - SPI Clock Control. This register should not be changed during an active SPI transaction."]
    pub clk_ctrl: CLK_CTRL,
    #[doc = "0x18 - 5:0\\]
PRELOAD SPI Clock Generator Preload value."]
    pub clk_gen: CLK_GEN,
}
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "0:0\\]
1=Enabled. The device is fully operational\n 0=Disabled. Clocks are gated to conserve power and the SPDOUT and SPI_CLK signals are set to their inactive state"]
pub mod enable;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI Control"]
pub mod ctrl;
#[doc = "STS (r) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "SPI Status"]
pub mod sts;
#[doc = "TX_DAT (rw) register accessor: an alias for `Reg<TX_DAT_SPEC>`"]
pub type TX_DAT = crate::Reg<tx_dat::TX_DAT_SPEC>;
#[doc = "7:0\\]
A write to this register when the \n Tx_Data buffer is empty (TXBE in the SPI Status Register is '1') initiates a SPI transaction."]
pub mod tx_dat;
#[doc = "RX_DAT (rw) register accessor: an alias for `Reg<RX_DAT_SPEC>`"]
pub type RX_DAT = crate::Reg<rx_dat::RX_DAT_SPEC>;
#[doc = "7:0\\]
This register is used to read the value returned by the external SPI device."]
pub mod rx_dat;
#[doc = "CLK_CTRL (rw) register accessor: an alias for `Reg<CLK_CTRL_SPEC>`"]
pub type CLK_CTRL = crate::Reg<clk_ctrl::CLK_CTRL_SPEC>;
#[doc = "SPI Clock Control. This register should not be changed during an active SPI transaction."]
pub mod clk_ctrl;
#[doc = "CLK_GEN (rw) register accessor: an alias for `Reg<CLK_GEN_SPEC>`"]
pub type CLK_GEN = crate::Reg<clk_gen::CLK_GEN_SPEC>;
#[doc = "5:0\\]
PRELOAD SPI Clock Generator Preload value."]
pub mod clk_gen;
