#[doc = "Register `SPI_EC_STS` reader"]
pub struct R(crate::R<SPI_EC_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_EC_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_EC_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_EC_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_EC_STS` writer"]
pub struct W(crate::W<SPI_EC_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_EC_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_EC_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_EC_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_WR_DONE` reader - When the ARM BUS side has fully finished the last transaction from the FIFO to write the data to Memory for Posted Writes .- clear with new Write request."]
pub type MEM_WR_DONE_R = crate::BitReader<bool>;
#[doc = "Field `MEM_WR_DONE` writer - When the ARM BUS side has fully finished the last transaction from the FIFO to write the data to Memory for Posted Writes .- clear with new Write request."]
pub type MEM_WR_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `MEM_RD_DONE` reader - When the ARM BUS side has fully finished writing the last written DWord to the FIFO for a set of data read from Memory for Posted Reads. - cleared with new Read request."]
pub type MEM_RD_DONE_R = crate::BitReader<bool>;
#[doc = "Field `MEM_RD_DONE` writer - When the ARM BUS side has fully finished writing the last written DWord to the FIFO for a set of data read from Memory for Posted Reads. - cleared with new Read request."]
pub type MEM_RD_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `MEM_WR_BUSY` reader - When an Memory Write transaction is currently being processed."]
pub type MEM_WR_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `MEM_WR_BUSY` writer - When an Memory Write transaction is currently being processed."]
pub type MEM_WR_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `MEM_RD_BUSY` reader - When an Memory Read transaction is currently being processed."]
pub type MEM_RD_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `MEM_RD_BUSY` writer - When an Memory Read transaction is currently being processed."]
pub type MEM_RD_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `SREG_TRANS` reader - When an SREG transaction is currently being processed."]
pub type SREG_TRANS_R = crate::BitReader<bool>;
#[doc = "Field `SREG_TRANS` writer - When an SREG transaction is currently being processed."]
pub type SREG_TRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `POLL_HI` reader - If this bit is set, then something in the high 16-bit of status register is set and needs to be checked. SPI Master should take action to clear this."]
pub type POLL_HI_R = crate::BitReader<bool>;
#[doc = "Field `POLL_HI` writer - If this bit is set, then something in the high 16-bit of status register is set and needs to be checked. SPI Master should take action to clear this."]
pub type POLL_HI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `RXF_EMP` reader - Signifies all Memory write transactions for the SPI Masters requested size have been performed. New transactions are allowed."]
pub type RXF_EMP_R = crate::BitReader<bool>;
#[doc = "Field `RXF_EMP` writer - Signifies all Memory write transactions for the SPI Masters requested size have been performed. New transactions are allowed."]
pub type RXF_EMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `RXF_FUL` reader - The RX FIFO is full of data to be written to Memory."]
pub type RXF_FUL_R = crate::BitReader<bool>;
#[doc = "Field `RXF_FUL` writer - The RX FIFO is full of data to be written to Memory."]
pub type RXF_FUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `TXF_EMP` reader - Signifies SPI Master has read the data requested from Memory. Can be used to show there is data the SPI Master has requested and not been read yet. New read transactions will be aligned."]
pub type TXF_EMP_R = crate::BitReader<bool>;
#[doc = "Field `TXF_EMP` writer - Signifies SPI Master has read the data requested from Memory. Can be used to show there is data the SPI Master has requested and not been read yet. New read transactions will be aligned."]
pub type TXF_EMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `TXF_FUL` reader - The TX FIFO is full of data that was read from Memory."]
pub type TXF_FUL_R = crate::BitReader<bool>;
#[doc = "Field `TXF_FUL` writer - The TX FIFO is full of data that was read from Memory."]
pub type TXF_FUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `TMCLK_CNT_ERR` reader - This bit is set when the SPI Clock Count Test Mode is set and there is an uneven amount of clocks."]
pub type TMCLK_CNT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `TMCLK_CNT_ERR` writer - This bit is set when the SPI Clock Count Test Mode is set and there is an uneven amount of clocks."]
pub type TMCLK_CNT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `IBF_FLG` reader - Set when the Host writes to the Input Buffer signaling there is data for the EC to read."]
pub type IBF_FLG_R = crate::BitReader<bool>;
#[doc = "Field `IBF_FLG` writer - Set when the Host writes to the Input Buffer signaling there is data for the EC to read."]
pub type IBF_FLG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `OBF_FLG` reader - Set when the EC writes to the Output Buffer signaling there is data for the Host to read."]
pub type OBF_FLG_R = crate::BitReader<bool>;
#[doc = "Field `OBF_FLG` writer - Set when the EC writes to the Output Buffer signaling there is data for the Host to read."]
pub type OBF_FLG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `SPIM_RST_REQ` reader - Set when the SPI Master Requested a Configuration Reset."]
pub type SPIM_RST_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SPIM_RST_REQ` writer - Set when the SPI Master Requested a Configuration Reset."]
pub type SPIM_RST_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `RXF_RST_DN` reader - Set after the SPI Master initiates a RX FIFO reset and the reset has been processed. FIFO is cleared."]
pub type RXF_RST_DN_R = crate::BitReader<bool>;
#[doc = "Field `RXF_RST_DN` writer - Set after the SPI Master initiates a RX FIFO reset and the reset has been processed. FIFO is cleared."]
pub type RXF_RST_DN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `TXF_RST_DN` reader - Set after the SPI Master initiates a TX FIFO reset and the reset has been processed. FIFO is cleared."]
pub type TXF_RST_DN_R = crate::BitReader<bool>;
#[doc = "Field `TXF_RST_DN` writer - Set after the SPI Master initiates a TX FIFO reset and the reset has been processed. FIFO is cleared."]
pub type TXF_RST_DN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `OOL0_ERR` reader - This flag is set with the transfer address requested by the master is out of Limit 0 range or when the BAR is disabled."]
pub type OOL0_ERR_R = crate::BitReader<bool>;
#[doc = "Field `OOL0_ERR` writer - This flag is set with the transfer address requested by the master is out of Limit 0 range or when the BAR is disabled."]
pub type OOL0_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `OOL1_ERR` reader - This flag is set with the transfer address requested by the master is out of Limit 1 range or when the BAR is disabled."]
pub type OOL1_ERR_R = crate::BitReader<bool>;
#[doc = "Field `OOL1_ERR` writer - This flag is set with the transfer address requested by the master is out of Limit 1 range or when the BAR is disabled."]
pub type OOL1_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `ARMBUS_ERR` reader - ARM Bus Error returned for the curren data transfer."]
pub type ARMBUS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ARMBUS_ERR` writer - ARM Bus Error returned for the curren data transfer."]
pub type ARMBUS_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `UNDEF_CMD_ERR` reader - Undefined Command Error: The command received from the master isn't defined."]
pub type UNDEF_CMD_ERR_R = crate::BitReader<bool>;
#[doc = "Field `UNDEF_CMD_ERR` writer - Undefined Command Error: The command received from the master isn't defined."]
pub type UNDEF_CMD_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `DV_BUSY` reader - If the Master requested a transaction whose destination is busy the request is ignored. Should use the poll or wait for interrupts."]
pub type DV_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `DV_BUSY` writer - If the Master requested a transaction whose destination is busy the request is ignored. Should use the poll or wait for interrupts."]
pub type DV_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `RXF_SIZE_ERR` reader - If size requested is more than what Master provided and the Master terminates early error flag shut down request signal to ARM Bus. Size requested is less than what Master provided -- ignored and continue transaction, may be taking in garbage."]
pub type RXF_SIZE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `RXF_SIZE_ERR` writer - If size requested is more than what Master provided and the Master terminates early error flag shut down request signal to ARM Bus. Size requested is less than what Master provided -- ignored and continue transaction, may be taking in garbage."]
pub type RXF_SIZE_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `TXF_UNFLW` reader - If Master reads more than what is in FIFO, FIFO will flag an underflow error and the data returned will just be the last valid pointer value."]
pub type TXF_UNFLW_R = crate::BitReader<bool>;
#[doc = "Field `TXF_UNFLW` writer - If Master reads more than what is in FIFO, FIFO will flag an underflow error and the data returned will just be the last valid pointer value."]
pub type TXF_UNFLW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `TXF_OVRFLW` reader - If Master doesn't read all of the data it requested from the posted read block cycle, than data will still be left in the FIFO. This will cause misalignment with the following transactions and a new read cycle can cause overflow."]
pub type TXF_OVRFLW_R = crate::BitReader<bool>;
#[doc = "Field `TXF_OVRFLW` writer - If Master doesn't read all of the data it requested from the posted read block cycle, than data will still be left in the FIFO. This will cause misalignment with the following transactions and a new read cycle can cause overflow."]
pub type TXF_OVRFLW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `RXF_UNFLW` reader - If the SPI Peripheral Target reads RX FIFO when it is empty, RX FIFO Underflow flag will be set. This condition will never happen under normal situation."]
pub type RXF_UNFLW_R = crate::BitReader<bool>;
#[doc = "Field `RXF_UNFLW` writer - If the SPI Peripheral Target reads RX FIFO when it is empty, RX FIFO Underflow flag will be set. This condition will never happen under normal situation."]
pub type RXF_UNFLW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
#[doc = "Field `RXF_OVRFLW` reader - If SPI Master writes more than the space in the FIFO, the FIFO will flag an overflow error and data will not be stored."]
pub type RXF_OVRFLW_R = crate::BitReader<bool>;
#[doc = "Field `RXF_OVRFLW` writer - If SPI Master writes more than the space in the FIFO, the FIFO will flag an overflow error and data will not be stored."]
pub type RXF_OVRFLW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EC_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When the ARM BUS side has fully finished the last transaction from the FIFO to write the data to Memory for Posted Writes .- clear with new Write request."]
    #[inline(always)]
    pub fn mem_wr_done(&self) -> MEM_WR_DONE_R {
        MEM_WR_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When the ARM BUS side has fully finished writing the last written DWord to the FIFO for a set of data read from Memory for Posted Reads. - cleared with new Read request."]
    #[inline(always)]
    pub fn mem_rd_done(&self) -> MEM_RD_DONE_R {
        MEM_RD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - When an Memory Write transaction is currently being processed."]
    #[inline(always)]
    pub fn mem_wr_busy(&self) -> MEM_WR_BUSY_R {
        MEM_WR_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When an Memory Read transaction is currently being processed."]
    #[inline(always)]
    pub fn mem_rd_busy(&self) -> MEM_RD_BUSY_R {
        MEM_RD_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When an SREG transaction is currently being processed."]
    #[inline(always)]
    pub fn sreg_trans(&self) -> SREG_TRANS_R {
        SREG_TRANS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If this bit is set, then something in the high 16-bit of status register is set and needs to be checked. SPI Master should take action to clear this."]
    #[inline(always)]
    pub fn poll_hi(&self) -> POLL_HI_R {
        POLL_HI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Signifies all Memory write transactions for the SPI Masters requested size have been performed. New transactions are allowed."]
    #[inline(always)]
    pub fn rxf_emp(&self) -> RXF_EMP_R {
        RXF_EMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The RX FIFO is full of data to be written to Memory."]
    #[inline(always)]
    pub fn rxf_ful(&self) -> RXF_FUL_R {
        RXF_FUL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Signifies SPI Master has read the data requested from Memory. Can be used to show there is data the SPI Master has requested and not been read yet. New read transactions will be aligned."]
    #[inline(always)]
    pub fn txf_emp(&self) -> TXF_EMP_R {
        TXF_EMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The TX FIFO is full of data that was read from Memory."]
    #[inline(always)]
    pub fn txf_ful(&self) -> TXF_FUL_R {
        TXF_FUL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit is set when the SPI Clock Count Test Mode is set and there is an uneven amount of clocks."]
    #[inline(always)]
    pub fn tmclk_cnt_err(&self) -> TMCLK_CNT_ERR_R {
        TMCLK_CNT_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set when the Host writes to the Input Buffer signaling there is data for the EC to read."]
    #[inline(always)]
    pub fn ibf_flg(&self) -> IBF_FLG_R {
        IBF_FLG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set when the EC writes to the Output Buffer signaling there is data for the Host to read."]
    #[inline(always)]
    pub fn obf_flg(&self) -> OBF_FLG_R {
        OBF_FLG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set when the SPI Master Requested a Configuration Reset."]
    #[inline(always)]
    pub fn spim_rst_req(&self) -> SPIM_RST_REQ_R {
        SPIM_RST_REQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set after the SPI Master initiates a RX FIFO reset and the reset has been processed. FIFO is cleared."]
    #[inline(always)]
    pub fn rxf_rst_dn(&self) -> RXF_RST_DN_R {
        RXF_RST_DN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set after the SPI Master initiates a TX FIFO reset and the reset has been processed. FIFO is cleared."]
    #[inline(always)]
    pub fn txf_rst_dn(&self) -> TXF_RST_DN_R {
        TXF_RST_DN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This flag is set with the transfer address requested by the master is out of Limit 0 range or when the BAR is disabled."]
    #[inline(always)]
    pub fn ool0_err(&self) -> OOL0_ERR_R {
        OOL0_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This flag is set with the transfer address requested by the master is out of Limit 1 range or when the BAR is disabled."]
    #[inline(always)]
    pub fn ool1_err(&self) -> OOL1_ERR_R {
        OOL1_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ARM Bus Error returned for the curren data transfer."]
    #[inline(always)]
    pub fn armbus_err(&self) -> ARMBUS_ERR_R {
        ARMBUS_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Undefined Command Error: The command received from the master isn't defined."]
    #[inline(always)]
    pub fn undef_cmd_err(&self) -> UNDEF_CMD_ERR_R {
        UNDEF_CMD_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - If the Master requested a transaction whose destination is busy the request is ignored. Should use the poll or wait for interrupts."]
    #[inline(always)]
    pub fn dv_busy(&self) -> DV_BUSY_R {
        DV_BUSY_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - If size requested is more than what Master provided and the Master terminates early error flag shut down request signal to ARM Bus. Size requested is less than what Master provided -- ignored and continue transaction, may be taking in garbage."]
    #[inline(always)]
    pub fn rxf_size_err(&self) -> RXF_SIZE_ERR_R {
        RXF_SIZE_ERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - If Master reads more than what is in FIFO, FIFO will flag an underflow error and the data returned will just be the last valid pointer value."]
    #[inline(always)]
    pub fn txf_unflw(&self) -> TXF_UNFLW_R {
        TXF_UNFLW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If Master doesn't read all of the data it requested from the posted read block cycle, than data will still be left in the FIFO. This will cause misalignment with the following transactions and a new read cycle can cause overflow."]
    #[inline(always)]
    pub fn txf_ovrflw(&self) -> TXF_OVRFLW_R {
        TXF_OVRFLW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - If the SPI Peripheral Target reads RX FIFO when it is empty, RX FIFO Underflow flag will be set. This condition will never happen under normal situation."]
    #[inline(always)]
    pub fn rxf_unflw(&self) -> RXF_UNFLW_R {
        RXF_UNFLW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - If SPI Master writes more than the space in the FIFO, the FIFO will flag an overflow error and data will not be stored."]
    #[inline(always)]
    pub fn rxf_ovrflw(&self) -> RXF_OVRFLW_R {
        RXF_OVRFLW_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When the ARM BUS side has fully finished the last transaction from the FIFO to write the data to Memory for Posted Writes .- clear with new Write request."]
    #[inline(always)]
    pub fn mem_wr_done(&mut self) -> MEM_WR_DONE_W<0> {
        MEM_WR_DONE_W::new(self)
    }
    #[doc = "Bit 1 - When the ARM BUS side has fully finished writing the last written DWord to the FIFO for a set of data read from Memory for Posted Reads. - cleared with new Read request."]
    #[inline(always)]
    pub fn mem_rd_done(&mut self) -> MEM_RD_DONE_W<1> {
        MEM_RD_DONE_W::new(self)
    }
    #[doc = "Bit 3 - When an Memory Write transaction is currently being processed."]
    #[inline(always)]
    pub fn mem_wr_busy(&mut self) -> MEM_WR_BUSY_W<3> {
        MEM_WR_BUSY_W::new(self)
    }
    #[doc = "Bit 4 - When an Memory Read transaction is currently being processed."]
    #[inline(always)]
    pub fn mem_rd_busy(&mut self) -> MEM_RD_BUSY_W<4> {
        MEM_RD_BUSY_W::new(self)
    }
    #[doc = "Bit 5 - When an SREG transaction is currently being processed."]
    #[inline(always)]
    pub fn sreg_trans(&mut self) -> SREG_TRANS_W<5> {
        SREG_TRANS_W::new(self)
    }
    #[doc = "Bit 6 - If this bit is set, then something in the high 16-bit of status register is set and needs to be checked. SPI Master should take action to clear this."]
    #[inline(always)]
    pub fn poll_hi(&mut self) -> POLL_HI_W<6> {
        POLL_HI_W::new(self)
    }
    #[doc = "Bit 8 - Signifies all Memory write transactions for the SPI Masters requested size have been performed. New transactions are allowed."]
    #[inline(always)]
    pub fn rxf_emp(&mut self) -> RXF_EMP_W<8> {
        RXF_EMP_W::new(self)
    }
    #[doc = "Bit 9 - The RX FIFO is full of data to be written to Memory."]
    #[inline(always)]
    pub fn rxf_ful(&mut self) -> RXF_FUL_W<9> {
        RXF_FUL_W::new(self)
    }
    #[doc = "Bit 10 - Signifies SPI Master has read the data requested from Memory. Can be used to show there is data the SPI Master has requested and not been read yet. New read transactions will be aligned."]
    #[inline(always)]
    pub fn txf_emp(&mut self) -> TXF_EMP_W<10> {
        TXF_EMP_W::new(self)
    }
    #[doc = "Bit 11 - The TX FIFO is full of data that was read from Memory."]
    #[inline(always)]
    pub fn txf_ful(&mut self) -> TXF_FUL_W<11> {
        TXF_FUL_W::new(self)
    }
    #[doc = "Bit 13 - This bit is set when the SPI Clock Count Test Mode is set and there is an uneven amount of clocks."]
    #[inline(always)]
    pub fn tmclk_cnt_err(&mut self) -> TMCLK_CNT_ERR_W<13> {
        TMCLK_CNT_ERR_W::new(self)
    }
    #[doc = "Bit 14 - Set when the Host writes to the Input Buffer signaling there is data for the EC to read."]
    #[inline(always)]
    pub fn ibf_flg(&mut self) -> IBF_FLG_W<14> {
        IBF_FLG_W::new(self)
    }
    #[doc = "Bit 15 - Set when the EC writes to the Output Buffer signaling there is data for the Host to read."]
    #[inline(always)]
    pub fn obf_flg(&mut self) -> OBF_FLG_W<15> {
        OBF_FLG_W::new(self)
    }
    #[doc = "Bit 16 - Set when the SPI Master Requested a Configuration Reset."]
    #[inline(always)]
    pub fn spim_rst_req(&mut self) -> SPIM_RST_REQ_W<16> {
        SPIM_RST_REQ_W::new(self)
    }
    #[doc = "Bit 17 - Set after the SPI Master initiates a RX FIFO reset and the reset has been processed. FIFO is cleared."]
    #[inline(always)]
    pub fn rxf_rst_dn(&mut self) -> RXF_RST_DN_W<17> {
        RXF_RST_DN_W::new(self)
    }
    #[doc = "Bit 18 - Set after the SPI Master initiates a TX FIFO reset and the reset has been processed. FIFO is cleared."]
    #[inline(always)]
    pub fn txf_rst_dn(&mut self) -> TXF_RST_DN_W<18> {
        TXF_RST_DN_W::new(self)
    }
    #[doc = "Bit 19 - This flag is set with the transfer address requested by the master is out of Limit 0 range or when the BAR is disabled."]
    #[inline(always)]
    pub fn ool0_err(&mut self) -> OOL0_ERR_W<19> {
        OOL0_ERR_W::new(self)
    }
    #[doc = "Bit 20 - This flag is set with the transfer address requested by the master is out of Limit 1 range or when the BAR is disabled."]
    #[inline(always)]
    pub fn ool1_err(&mut self) -> OOL1_ERR_W<20> {
        OOL1_ERR_W::new(self)
    }
    #[doc = "Bit 21 - ARM Bus Error returned for the curren data transfer."]
    #[inline(always)]
    pub fn armbus_err(&mut self) -> ARMBUS_ERR_W<21> {
        ARMBUS_ERR_W::new(self)
    }
    #[doc = "Bit 22 - Undefined Command Error: The command received from the master isn't defined."]
    #[inline(always)]
    pub fn undef_cmd_err(&mut self) -> UNDEF_CMD_ERR_W<22> {
        UNDEF_CMD_ERR_W::new(self)
    }
    #[doc = "Bit 23 - If the Master requested a transaction whose destination is busy the request is ignored. Should use the poll or wait for interrupts."]
    #[inline(always)]
    pub fn dv_busy(&mut self) -> DV_BUSY_W<23> {
        DV_BUSY_W::new(self)
    }
    #[doc = "Bit 24 - If size requested is more than what Master provided and the Master terminates early error flag shut down request signal to ARM Bus. Size requested is less than what Master provided -- ignored and continue transaction, may be taking in garbage."]
    #[inline(always)]
    pub fn rxf_size_err(&mut self) -> RXF_SIZE_ERR_W<24> {
        RXF_SIZE_ERR_W::new(self)
    }
    #[doc = "Bit 25 - If Master reads more than what is in FIFO, FIFO will flag an underflow error and the data returned will just be the last valid pointer value."]
    #[inline(always)]
    pub fn txf_unflw(&mut self) -> TXF_UNFLW_W<25> {
        TXF_UNFLW_W::new(self)
    }
    #[doc = "Bit 26 - If Master doesn't read all of the data it requested from the posted read block cycle, than data will still be left in the FIFO. This will cause misalignment with the following transactions and a new read cycle can cause overflow."]
    #[inline(always)]
    pub fn txf_ovrflw(&mut self) -> TXF_OVRFLW_W<26> {
        TXF_OVRFLW_W::new(self)
    }
    #[doc = "Bit 27 - If the SPI Peripheral Target reads RX FIFO when it is empty, RX FIFO Underflow flag will be set. This condition will never happen under normal situation."]
    #[inline(always)]
    pub fn rxf_unflw(&mut self) -> RXF_UNFLW_W<27> {
        RXF_UNFLW_W::new(self)
    }
    #[doc = "Bit 28 - If SPI Master writes more than the space in the FIFO, the FIFO will flag an overflow error and data will not be stored."]
    #[inline(always)]
    pub fn rxf_ovrflw(&mut self) -> RXF_OVRFLW_W<28> {
        RXF_OVRFLW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Peripheral Target EC Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ec_sts](index.html) module"]
pub struct SPI_EC_STS_SPEC;
impl crate::RegisterSpec for SPI_EC_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ec_sts::R](R) reader structure"]
impl crate::Readable for SPI_EC_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ec_sts::W](W) writer structure"]
impl crate::Writable for SPI_EC_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_EC_STS to value 0x0500"]
impl crate::Resettable for SPI_EC_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0500
    }
}
