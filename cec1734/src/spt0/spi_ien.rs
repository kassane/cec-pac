#[doc = "Register `SPI_IEN` reader"]
pub struct R(crate::R<SPI_IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_IEN` writer"]
pub struct W(crate::W<SPI_IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_IEN_SPEC>;
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
impl From<crate::W<SPI_IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_WR_DONE` reader - Enable Memory Write Done Interrupt to SPI Master."]
pub type MEM_WR_DONE_R = crate::BitReader<bool>;
#[doc = "Field `MEM_WR_DONE` writer - Enable Memory Write Done Interrupt to SPI Master."]
pub type MEM_WR_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `MEM_RD_DONE` reader - Enable Memory Read Done Interrupt to SPI Master."]
pub type MEM_RD_DONE_R = crate::BitReader<bool>;
#[doc = "Field `MEM_RD_DONE` writer - Enable Memory Read Done Interrupt to SPI Master."]
pub type MEM_RD_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `MEM_WR_BUSY` reader - Enable Memory Write Busy Interrupt to SPI Master."]
pub type MEM_WR_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `MEM_WR_BUSY` writer - Enable Memory Write Busy Interrupt to SPI Master."]
pub type MEM_WR_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `MEM_RD_BUSY` reader - Enable Memory Read Busy Interrupt to SPI Master."]
pub type MEM_RD_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `MEM_RD_BUSY` writer - Enable Memory Read Busy Interrupt to SPI Master."]
pub type MEM_RD_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `SREG_TRANS` reader - Enable SREG Trans Busy Interrupt to SPI Master."]
pub type SREG_TRANS_R = crate::BitReader<bool>;
#[doc = "Field `SREG_TRANS` writer - Enable SREG Trans Busy Interrupt to SPI Master."]
pub type SREG_TRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `POLL_HI` reader - Enable Poll High Request Interrupt to SPI Master."]
pub type POLL_HI_R = crate::BitReader<bool>;
#[doc = "Field `POLL_HI` writer - Enable Poll High Request Interrupt to SPI Master."]
pub type POLL_HI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `RXF_EMP` reader - Enable SREG RX FIFO Empty Interrupt to SPI Master."]
pub type RXF_EMP_R = crate::BitReader<bool>;
#[doc = "Field `RXF_EMP` writer - Enable SREG RX FIFO Empty Interrupt to SPI Master."]
pub type RXF_EMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `RXF_FUL` reader - Enable RX FIFO Full Interrupt to SPI Master."]
pub type RXF_FUL_R = crate::BitReader<bool>;
#[doc = "Field `RXF_FUL` writer - Enable RX FIFO Full Interrupt to SPI Master."]
pub type RXF_FUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `TXF_EMP` reader - Enable TX FIFO Empty Interrupt to SPI Master."]
pub type TXF_EMP_R = crate::BitReader<bool>;
#[doc = "Field `TXF_EMP` writer - Enable TX FIFO Empty Interrupt to SPI Master."]
pub type TXF_EMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `TXF_FUL` reader - Enable TX FIFI FULL Interrupt to SPI Master."]
pub type TXF_FUL_R = crate::BitReader<bool>;
#[doc = "Field `TXF_FUL` writer - Enable TX FIFI FULL Interrupt to SPI Master."]
pub type TXF_FUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `TMCLK_CNT_ERR` reader - Enable Test Mode SPI Clock Count Error Interrupt to SPI Master."]
pub type TMCLK_CNT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `TMCLK_CNT_ERR` writer - Enable Test Mode SPI Clock Count Error Interrupt to SPI Master."]
pub type TMCLK_CNT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `IBF_FLG` reader - Enable Input Buffer Signaling Interrupt to SPI Master."]
pub type IBF_FLG_R = crate::BitReader<bool>;
#[doc = "Field `IBF_FLG` writer - Enable Input Buffer Signaling Interrupt to SPI Master."]
pub type IBF_FLG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `OBF_FLG` reader - Enable Output Buffer signaling Interrupt to SPI Master."]
pub type OBF_FLG_R = crate::BitReader<bool>;
#[doc = "Field `OBF_FLG` writer - Enable Output Buffer signaling Interrupt to SPI Master."]
pub type OBF_FLG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `SPIM_RST_REQ` reader - Enable SPI Master Request Reset Interrupt to SPI Master."]
pub type SPIM_RST_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SPIM_RST_REQ` writer - Enable SPI Master Request Reset Interrupt to SPI Master."]
pub type SPIM_RST_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `RXF_RST_DN` reader - Enable RX FIFO Reset Done Interrupt to SPI Master."]
pub type RXF_RST_DN_R = crate::BitReader<bool>;
#[doc = "Field `RXF_RST_DN` writer - Enable RX FIFO Reset Done Interrupt to SPI Master."]
pub type RXF_RST_DN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `TXF_RST_DN` reader - Enable TX FIFO Reset Done Interrupt to SPI Master."]
pub type TXF_RST_DN_R = crate::BitReader<bool>;
#[doc = "Field `TXF_RST_DN` writer - Enable TX FIFO Reset Done Interrupt to SPI Master."]
pub type TXF_RST_DN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `OOL0_ERR` reader - Enable Out Of Limit 0 Error Interrupt to SPI Master."]
pub type OOL0_ERR_R = crate::BitReader<bool>;
#[doc = "Field `OOL0_ERR` writer - Enable Out Of Limit 0 Error Interrupt to SPI Master."]
pub type OOL0_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `OOL1_ERR` reader - Enable Out Of Limit 1 Error Interrupt to SPI Master."]
pub type OOL1_ERR_R = crate::BitReader<bool>;
#[doc = "Field `OOL1_ERR` writer - Enable Out Of Limit 1 Error Interrupt to SPI Master."]
pub type OOL1_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `ARMBUS_ERR` reader - Enable AHB BUS Error Interrupt to SPI Master."]
pub type ARMBUS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ARMBUS_ERR` writer - Enable AHB BUS Error Interrupt to SPI Master."]
pub type ARMBUS_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `UNDEF_CMD_ERR` reader - Enable Undefined Command Error Interrupt to SPI Master."]
pub type UNDEF_CMD_ERR_R = crate::BitReader<bool>;
#[doc = "Field `UNDEF_CMD_ERR` writer - Enable Undefined Command Error Interrupt to SPI Master."]
pub type UNDEF_CMD_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `DV_BUSY` reader - Enable Device Busy Interrupt to SPI Master."]
pub type DV_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `DV_BUSY` writer - Enable Device Busy Interrupt to SPI Master."]
pub type DV_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `RXF_SIZE_ERR` reader - Enable RX FIFO SIZE Error Interrupt to SPI Master."]
pub type RXF_SIZE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `RXF_SIZE_ERR` writer - Enable RX FIFO SIZE Error Interrupt to SPI Master."]
pub type RXF_SIZE_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `TXF_UNFLW` reader - Enable TX FIFO Underflow Interrupt to SPI Master."]
pub type TXF_UNFLW_R = crate::BitReader<bool>;
#[doc = "Field `TXF_UNFLW` writer - Enable TX FIFO Underflow Interrupt to SPI Master."]
pub type TXF_UNFLW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `TXF_OVRFLOW` reader - Enable TX FIFO Overflow Interrupt to SPI Master."]
pub type TXF_OVRFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TXF_OVRFLOW` writer - Enable TX FIFO Overflow Interrupt to SPI Master."]
pub type TXF_OVRFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `RXF_UNFLW` reader - Enable RX FIFO Underflow Interrupt to SPI Master."]
pub type RXF_UNFLW_R = crate::BitReader<bool>;
#[doc = "Field `RXF_UNFLW` writer - Enable RX FIFO Underflow Interrupt to SPI Master."]
pub type RXF_UNFLW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
#[doc = "Field `RXF_OVRFLW` reader - Enable RX FIFO Overflow Interrupt to SPI Master."]
pub type RXF_OVRFLW_R = crate::BitReader<bool>;
#[doc = "Field `RXF_OVRFLW` writer - Enable RX FIFO Overflow Interrupt to SPI Master."]
pub type RXF_OVRFLW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable Memory Write Done Interrupt to SPI Master."]
    #[inline(always)]
    pub fn mem_wr_done(&self) -> MEM_WR_DONE_R {
        MEM_WR_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Memory Read Done Interrupt to SPI Master."]
    #[inline(always)]
    pub fn mem_rd_done(&self) -> MEM_RD_DONE_R {
        MEM_RD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Memory Write Busy Interrupt to SPI Master."]
    #[inline(always)]
    pub fn mem_wr_busy(&self) -> MEM_WR_BUSY_R {
        MEM_WR_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Memory Read Busy Interrupt to SPI Master."]
    #[inline(always)]
    pub fn mem_rd_busy(&self) -> MEM_RD_BUSY_R {
        MEM_RD_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable SREG Trans Busy Interrupt to SPI Master."]
    #[inline(always)]
    pub fn sreg_trans(&self) -> SREG_TRANS_R {
        SREG_TRANS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Poll High Request Interrupt to SPI Master."]
    #[inline(always)]
    pub fn poll_hi(&self) -> POLL_HI_R {
        POLL_HI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable SREG RX FIFO Empty Interrupt to SPI Master."]
    #[inline(always)]
    pub fn rxf_emp(&self) -> RXF_EMP_R {
        RXF_EMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable RX FIFO Full Interrupt to SPI Master."]
    #[inline(always)]
    pub fn rxf_ful(&self) -> RXF_FUL_R {
        RXF_FUL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable TX FIFO Empty Interrupt to SPI Master."]
    #[inline(always)]
    pub fn txf_emp(&self) -> TXF_EMP_R {
        TXF_EMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable TX FIFI FULL Interrupt to SPI Master."]
    #[inline(always)]
    pub fn txf_ful(&self) -> TXF_FUL_R {
        TXF_FUL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Test Mode SPI Clock Count Error Interrupt to SPI Master."]
    #[inline(always)]
    pub fn tmclk_cnt_err(&self) -> TMCLK_CNT_ERR_R {
        TMCLK_CNT_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Input Buffer Signaling Interrupt to SPI Master."]
    #[inline(always)]
    pub fn ibf_flg(&self) -> IBF_FLG_R {
        IBF_FLG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Output Buffer signaling Interrupt to SPI Master."]
    #[inline(always)]
    pub fn obf_flg(&self) -> OBF_FLG_R {
        OBF_FLG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable SPI Master Request Reset Interrupt to SPI Master."]
    #[inline(always)]
    pub fn spim_rst_req(&self) -> SPIM_RST_REQ_R {
        SPIM_RST_REQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable RX FIFO Reset Done Interrupt to SPI Master."]
    #[inline(always)]
    pub fn rxf_rst_dn(&self) -> RXF_RST_DN_R {
        RXF_RST_DN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable TX FIFO Reset Done Interrupt to SPI Master."]
    #[inline(always)]
    pub fn txf_rst_dn(&self) -> TXF_RST_DN_R {
        TXF_RST_DN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Out Of Limit 0 Error Interrupt to SPI Master."]
    #[inline(always)]
    pub fn ool0_err(&self) -> OOL0_ERR_R {
        OOL0_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Out Of Limit 1 Error Interrupt to SPI Master."]
    #[inline(always)]
    pub fn ool1_err(&self) -> OOL1_ERR_R {
        OOL1_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable AHB BUS Error Interrupt to SPI Master."]
    #[inline(always)]
    pub fn armbus_err(&self) -> ARMBUS_ERR_R {
        ARMBUS_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Undefined Command Error Interrupt to SPI Master."]
    #[inline(always)]
    pub fn undef_cmd_err(&self) -> UNDEF_CMD_ERR_R {
        UNDEF_CMD_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Device Busy Interrupt to SPI Master."]
    #[inline(always)]
    pub fn dv_busy(&self) -> DV_BUSY_R {
        DV_BUSY_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable RX FIFO SIZE Error Interrupt to SPI Master."]
    #[inline(always)]
    pub fn rxf_size_err(&self) -> RXF_SIZE_ERR_R {
        RXF_SIZE_ERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable TX FIFO Underflow Interrupt to SPI Master."]
    #[inline(always)]
    pub fn txf_unflw(&self) -> TXF_UNFLW_R {
        TXF_UNFLW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable TX FIFO Overflow Interrupt to SPI Master."]
    #[inline(always)]
    pub fn txf_ovrflow(&self) -> TXF_OVRFLOW_R {
        TXF_OVRFLOW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable RX FIFO Underflow Interrupt to SPI Master."]
    #[inline(always)]
    pub fn rxf_unflw(&self) -> RXF_UNFLW_R {
        RXF_UNFLW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable RX FIFO Overflow Interrupt to SPI Master."]
    #[inline(always)]
    pub fn rxf_ovrflw(&self) -> RXF_OVRFLW_R {
        RXF_OVRFLW_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Memory Write Done Interrupt to SPI Master."]
    #[inline(always)]
    pub fn mem_wr_done(&mut self) -> MEM_WR_DONE_W<0> {
        MEM_WR_DONE_W::new(self)
    }
    #[doc = "Bit 1 - Enable Memory Read Done Interrupt to SPI Master."]
    #[inline(always)]
    pub fn mem_rd_done(&mut self) -> MEM_RD_DONE_W<1> {
        MEM_RD_DONE_W::new(self)
    }
    #[doc = "Bit 3 - Enable Memory Write Busy Interrupt to SPI Master."]
    #[inline(always)]
    pub fn mem_wr_busy(&mut self) -> MEM_WR_BUSY_W<3> {
        MEM_WR_BUSY_W::new(self)
    }
    #[doc = "Bit 4 - Enable Memory Read Busy Interrupt to SPI Master."]
    #[inline(always)]
    pub fn mem_rd_busy(&mut self) -> MEM_RD_BUSY_W<4> {
        MEM_RD_BUSY_W::new(self)
    }
    #[doc = "Bit 5 - Enable SREG Trans Busy Interrupt to SPI Master."]
    #[inline(always)]
    pub fn sreg_trans(&mut self) -> SREG_TRANS_W<5> {
        SREG_TRANS_W::new(self)
    }
    #[doc = "Bit 6 - Enable Poll High Request Interrupt to SPI Master."]
    #[inline(always)]
    pub fn poll_hi(&mut self) -> POLL_HI_W<6> {
        POLL_HI_W::new(self)
    }
    #[doc = "Bit 8 - Enable SREG RX FIFO Empty Interrupt to SPI Master."]
    #[inline(always)]
    pub fn rxf_emp(&mut self) -> RXF_EMP_W<8> {
        RXF_EMP_W::new(self)
    }
    #[doc = "Bit 9 - Enable RX FIFO Full Interrupt to SPI Master."]
    #[inline(always)]
    pub fn rxf_ful(&mut self) -> RXF_FUL_W<9> {
        RXF_FUL_W::new(self)
    }
    #[doc = "Bit 10 - Enable TX FIFO Empty Interrupt to SPI Master."]
    #[inline(always)]
    pub fn txf_emp(&mut self) -> TXF_EMP_W<10> {
        TXF_EMP_W::new(self)
    }
    #[doc = "Bit 11 - Enable TX FIFI FULL Interrupt to SPI Master."]
    #[inline(always)]
    pub fn txf_ful(&mut self) -> TXF_FUL_W<11> {
        TXF_FUL_W::new(self)
    }
    #[doc = "Bit 13 - Enable Test Mode SPI Clock Count Error Interrupt to SPI Master."]
    #[inline(always)]
    pub fn tmclk_cnt_err(&mut self) -> TMCLK_CNT_ERR_W<13> {
        TMCLK_CNT_ERR_W::new(self)
    }
    #[doc = "Bit 14 - Enable Input Buffer Signaling Interrupt to SPI Master."]
    #[inline(always)]
    pub fn ibf_flg(&mut self) -> IBF_FLG_W<14> {
        IBF_FLG_W::new(self)
    }
    #[doc = "Bit 15 - Enable Output Buffer signaling Interrupt to SPI Master."]
    #[inline(always)]
    pub fn obf_flg(&mut self) -> OBF_FLG_W<15> {
        OBF_FLG_W::new(self)
    }
    #[doc = "Bit 16 - Enable SPI Master Request Reset Interrupt to SPI Master."]
    #[inline(always)]
    pub fn spim_rst_req(&mut self) -> SPIM_RST_REQ_W<16> {
        SPIM_RST_REQ_W::new(self)
    }
    #[doc = "Bit 17 - Enable RX FIFO Reset Done Interrupt to SPI Master."]
    #[inline(always)]
    pub fn rxf_rst_dn(&mut self) -> RXF_RST_DN_W<17> {
        RXF_RST_DN_W::new(self)
    }
    #[doc = "Bit 18 - Enable TX FIFO Reset Done Interrupt to SPI Master."]
    #[inline(always)]
    pub fn txf_rst_dn(&mut self) -> TXF_RST_DN_W<18> {
        TXF_RST_DN_W::new(self)
    }
    #[doc = "Bit 19 - Enable Out Of Limit 0 Error Interrupt to SPI Master."]
    #[inline(always)]
    pub fn ool0_err(&mut self) -> OOL0_ERR_W<19> {
        OOL0_ERR_W::new(self)
    }
    #[doc = "Bit 20 - Enable Out Of Limit 1 Error Interrupt to SPI Master."]
    #[inline(always)]
    pub fn ool1_err(&mut self) -> OOL1_ERR_W<20> {
        OOL1_ERR_W::new(self)
    }
    #[doc = "Bit 21 - Enable AHB BUS Error Interrupt to SPI Master."]
    #[inline(always)]
    pub fn armbus_err(&mut self) -> ARMBUS_ERR_W<21> {
        ARMBUS_ERR_W::new(self)
    }
    #[doc = "Bit 22 - Enable Undefined Command Error Interrupt to SPI Master."]
    #[inline(always)]
    pub fn undef_cmd_err(&mut self) -> UNDEF_CMD_ERR_W<22> {
        UNDEF_CMD_ERR_W::new(self)
    }
    #[doc = "Bit 23 - Enable Device Busy Interrupt to SPI Master."]
    #[inline(always)]
    pub fn dv_busy(&mut self) -> DV_BUSY_W<23> {
        DV_BUSY_W::new(self)
    }
    #[doc = "Bit 24 - Enable RX FIFO SIZE Error Interrupt to SPI Master."]
    #[inline(always)]
    pub fn rxf_size_err(&mut self) -> RXF_SIZE_ERR_W<24> {
        RXF_SIZE_ERR_W::new(self)
    }
    #[doc = "Bit 25 - Enable TX FIFO Underflow Interrupt to SPI Master."]
    #[inline(always)]
    pub fn txf_unflw(&mut self) -> TXF_UNFLW_W<25> {
        TXF_UNFLW_W::new(self)
    }
    #[doc = "Bit 26 - Enable TX FIFO Overflow Interrupt to SPI Master."]
    #[inline(always)]
    pub fn txf_ovrflow(&mut self) -> TXF_OVRFLOW_W<26> {
        TXF_OVRFLOW_W::new(self)
    }
    #[doc = "Bit 27 - Enable RX FIFO Underflow Interrupt to SPI Master."]
    #[inline(always)]
    pub fn rxf_unflw(&mut self) -> RXF_UNFLW_W<27> {
        RXF_UNFLW_W::new(self)
    }
    #[doc = "Bit 28 - Enable RX FIFO Overflow Interrupt to SPI Master."]
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
#[doc = "SPI Peripheral Target Interrupt Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ien](index.html) module"]
pub struct SPI_IEN_SPEC;
impl crate::RegisterSpec for SPI_IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ien::R](R) reader structure"]
impl crate::Readable for SPI_IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ien::W](W) writer structure"]
impl crate::Writable for SPI_IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_IEN to value 0"]
impl crate::Resettable for SPI_IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
