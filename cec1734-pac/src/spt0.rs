#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Peripheral Target Communication Configuration Register."]
    pub spi_cfg: SPI_CFG,
    #[doc = "0x04 - SPI Peripheral Target Status Register."]
    pub spi_sts: SPI_STS,
    #[doc = "0x08 - SPI Peripheral Target EC Status Register."]
    pub spi_ec_sts: SPI_EC_STS,
    #[doc = "0x0c - SPI Peripheral Target Interrupt Enable Register."]
    pub spi_ien: SPI_IEN,
    #[doc = "0x10 - SPI Peripheral Target EC Interrupt Enable Register."]
    pub ec_ien: EC_IEN,
    #[doc = "0x14 - SPI Peripheral Target Memory Configuration Register."]
    pub mem_cfg: MEM_CFG,
    #[doc = "0x18 - SPI Peripheral Target Memory Base Address0 Register."]
    pub mem_bar0: MEM_BAR0,
    #[doc = "0x1c - SPI Peripheral Target Memory Write LIMIT 0 Register."]
    pub mem_wr_lim0: MEM_WR_LIM0,
    #[doc = "0x20 - SPI Peripheral Target Memory Read LIMIT 0 Register."]
    pub mem_rd_lim0: MEM_RD_LIM0,
    #[doc = "0x24 - SPI Peripheral Target Memory Base Address1 Register."]
    pub mem_bar1: MEM_BAR1,
    #[doc = "0x28 - SPI Peripheral Target Memory Write LIMIT 1 Register."]
    pub mem_wr_lim1: MEM_WR_LIM1,
    #[doc = "0x2c - SPI Peripheral Target Memory Read LIMIT 1 Register."]
    pub mem_rd_lim1: MEM_RD_LIM1,
    #[doc = "0x30 - SPI Peripheral Target RX FIFO Host Bar Register."]
    pub rxf_host_bar: RXF_HOST_BAR,
    #[doc = "0x34 - SPI Peripheral Target RX FIFO Byte Counter Register."]
    pub rxf_byte_cnt: RXF_BYTE_CNT,
    #[doc = "0x38 - SPI Peripheral Target TX FIFO Host Bar Register."]
    pub txf_host_bar: TXF_HOST_BAR,
    #[doc = "0x3c - SPI Peripheral Target TX FIFO Byte Counter Register."]
    pub txf_byte_cnt: TXF_BYTE_CNT,
    #[doc = "0x40 - SPI Peripheral Target System Configuration Register."]
    pub sys_cfg: SYS_CFG,
    #[doc = "0x44 - SPI Peripheral Target Master to EC Mailbox Register."]
    pub spim2ec_mbx: SPIM2EC_MBX,
    #[doc = "0x48 - SPI Peripheral Target Master to EC Mailbox Register."]
    pub ec2spim_mbx: EC2SPIM_MBX,
}
#[doc = "SPI_CFG (rw) register accessor: an alias for `Reg<SPI_CFG_SPEC>`"]
pub type SPI_CFG = crate::Reg<spi_cfg::SPI_CFG_SPEC>;
#[doc = "SPI Peripheral Target Communication Configuration Register."]
pub mod spi_cfg;
#[doc = "SPI_STS (rw) register accessor: an alias for `Reg<SPI_STS_SPEC>`"]
pub type SPI_STS = crate::Reg<spi_sts::SPI_STS_SPEC>;
#[doc = "SPI Peripheral Target Status Register."]
pub mod spi_sts;
#[doc = "SPI_EC_STS (rw) register accessor: an alias for `Reg<SPI_EC_STS_SPEC>`"]
pub type SPI_EC_STS = crate::Reg<spi_ec_sts::SPI_EC_STS_SPEC>;
#[doc = "SPI Peripheral Target EC Status Register."]
pub mod spi_ec_sts;
#[doc = "SPI_IEN (rw) register accessor: an alias for `Reg<SPI_IEN_SPEC>`"]
pub type SPI_IEN = crate::Reg<spi_ien::SPI_IEN_SPEC>;
#[doc = "SPI Peripheral Target Interrupt Enable Register."]
pub mod spi_ien;
#[doc = "EC_IEN (rw) register accessor: an alias for `Reg<EC_IEN_SPEC>`"]
pub type EC_IEN = crate::Reg<ec_ien::EC_IEN_SPEC>;
#[doc = "SPI Peripheral Target EC Interrupt Enable Register."]
pub mod ec_ien;
#[doc = "MEM_CFG (rw) register accessor: an alias for `Reg<MEM_CFG_SPEC>`"]
pub type MEM_CFG = crate::Reg<mem_cfg::MEM_CFG_SPEC>;
#[doc = "SPI Peripheral Target Memory Configuration Register."]
pub mod mem_cfg;
#[doc = "MEM_BAR0 (rw) register accessor: an alias for `Reg<MEM_BAR0_SPEC>`"]
pub type MEM_BAR0 = crate::Reg<mem_bar0::MEM_BAR0_SPEC>;
#[doc = "SPI Peripheral Target Memory Base Address0 Register."]
pub mod mem_bar0;
#[doc = "MEM_WR_LIM0 (rw) register accessor: an alias for `Reg<MEM_WR_LIM0_SPEC>`"]
pub type MEM_WR_LIM0 = crate::Reg<mem_wr_lim0::MEM_WR_LIM0_SPEC>;
#[doc = "SPI Peripheral Target Memory Write LIMIT 0 Register."]
pub mod mem_wr_lim0;
#[doc = "MEM_RD_LIM0 (rw) register accessor: an alias for `Reg<MEM_RD_LIM0_SPEC>`"]
pub type MEM_RD_LIM0 = crate::Reg<mem_rd_lim0::MEM_RD_LIM0_SPEC>;
#[doc = "SPI Peripheral Target Memory Read LIMIT 0 Register."]
pub mod mem_rd_lim0;
#[doc = "MEM_BAR1 (rw) register accessor: an alias for `Reg<MEM_BAR1_SPEC>`"]
pub type MEM_BAR1 = crate::Reg<mem_bar1::MEM_BAR1_SPEC>;
#[doc = "SPI Peripheral Target Memory Base Address1 Register."]
pub mod mem_bar1;
#[doc = "MEM_WR_LIM1 (rw) register accessor: an alias for `Reg<MEM_WR_LIM1_SPEC>`"]
pub type MEM_WR_LIM1 = crate::Reg<mem_wr_lim1::MEM_WR_LIM1_SPEC>;
#[doc = "SPI Peripheral Target Memory Write LIMIT 1 Register."]
pub mod mem_wr_lim1;
#[doc = "MEM_RD_LIM1 (rw) register accessor: an alias for `Reg<MEM_RD_LIM1_SPEC>`"]
pub type MEM_RD_LIM1 = crate::Reg<mem_rd_lim1::MEM_RD_LIM1_SPEC>;
#[doc = "SPI Peripheral Target Memory Read LIMIT 1 Register."]
pub mod mem_rd_lim1;
#[doc = "RXF_HOST_BAR (r) register accessor: an alias for `Reg<RXF_HOST_BAR_SPEC>`"]
pub type RXF_HOST_BAR = crate::Reg<rxf_host_bar::RXF_HOST_BAR_SPEC>;
#[doc = "SPI Peripheral Target RX FIFO Host Bar Register."]
pub mod rxf_host_bar;
#[doc = "RXF_BYTE_CNT (r) register accessor: an alias for `Reg<RXF_BYTE_CNT_SPEC>`"]
pub type RXF_BYTE_CNT = crate::Reg<rxf_byte_cnt::RXF_BYTE_CNT_SPEC>;
#[doc = "SPI Peripheral Target RX FIFO Byte Counter Register."]
pub mod rxf_byte_cnt;
#[doc = "TXF_HOST_BAR (r) register accessor: an alias for `Reg<TXF_HOST_BAR_SPEC>`"]
pub type TXF_HOST_BAR = crate::Reg<txf_host_bar::TXF_HOST_BAR_SPEC>;
#[doc = "SPI Peripheral Target TX FIFO Host Bar Register."]
pub mod txf_host_bar;
#[doc = "TXF_BYTE_CNT (r) register accessor: an alias for `Reg<TXF_BYTE_CNT_SPEC>`"]
pub type TXF_BYTE_CNT = crate::Reg<txf_byte_cnt::TXF_BYTE_CNT_SPEC>;
#[doc = "SPI Peripheral Target TX FIFO Byte Counter Register."]
pub mod txf_byte_cnt;
#[doc = "SYS_CFG (rw) register accessor: an alias for `Reg<SYS_CFG_SPEC>`"]
pub type SYS_CFG = crate::Reg<sys_cfg::SYS_CFG_SPEC>;
#[doc = "SPI Peripheral Target System Configuration Register."]
pub mod sys_cfg;
#[doc = "SPIM2EC_MBX (rw) register accessor: an alias for `Reg<SPIM2EC_MBX_SPEC>`"]
pub type SPIM2EC_MBX = crate::Reg<spim2ec_mbx::SPIM2EC_MBX_SPEC>;
#[doc = "SPI Peripheral Target Master to EC Mailbox Register."]
pub mod spim2ec_mbx;
#[doc = "EC2SPIM_MBX (rw) register accessor: an alias for `Reg<EC2SPIM_MBX_SPEC>`"]
pub type EC2SPIM_MBX = crate::Reg<ec2spim_mbx::EC2SPIM_MBX_SPEC>;
#[doc = "SPI Peripheral Target Master to EC Mailbox Register."]
pub mod ec2spim_mbx;
