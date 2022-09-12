#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRANS_COMPL_EN` reader - 1=Enable an interrupt if TRANSFER_COMPLETE is asserted\n 0=Disable the interrupt."]
pub type TRANS_COMPL_EN_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_COMPL_EN` writer - 1=Enable an interrupt if TRANSFER_COMPLETE is asserted\n 0=Disable the interrupt."]
pub type TRANS_COMPL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DMA_COMPL_EN` reader - 1=Enable an interrupt if DMA_COMPLETE is asserted\n 0=Disable the interrupt."]
pub type DMA_COMPL_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_COMPL_EN` writer - 1=Enable an interrupt if DMA_COMPLETE is asserted\n 0=Disable the interrupt."]
pub type DMA_COMPL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TX_BUF_ERR_EN` reader - 1=Enable an interrupt if TRANSMIT_BUFFER_ERROR is asserted\n 0=Disable the interrupt."]
pub type TX_BUF_ERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_BUF_ERR_EN` writer - 1=Enable an interrupt if TRANSMIT_BUFFER_ERROR is asserted\n 0=Disable the interrupt."]
pub type TX_BUF_ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `RX_BUF_ERR_EN` reader - 1=Enable an interrupt if RECEIVE_BUFFER_ERROR is asserted\n 0=Disable the interrupt."]
pub type RX_BUF_ERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_BUF_ERR_EN` writer - 1=Enable an interrupt if RECEIVE_BUFFER_ERROR is asserted\n 0=Disable the interrupt."]
pub type RX_BUF_ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `PRGM_ERR_EN` reader - 1=Enable an interrupt if PROGRAMMING_ERROR is asserted\n 0=Disable the interrupt."]
pub type PRGM_ERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `PRGM_ERR_EN` writer - 1=Enable an interrupt if PROGRAMMING_ERROR is asserted\n 0=Disable the interrupt."]
pub type PRGM_ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TX_BUF_FULL_EN` reader - 1=Enable an interrupt if TRANSMIT_BUFFER_FULL is asserted\n 0=Disable the interrupt."]
pub type TX_BUF_FULL_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_BUF_FULL_EN` writer - 1=Enable an interrupt if TRANSMIT_BUFFER_FULL is asserted\n 0=Disable the interrupt."]
pub type TX_BUF_FULL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TX_BUF_EMPTY_EN` reader - 1=Enable an interrupt if TRANSMIT_BUFFER_EMPTY is asserted\n 0=Disable the interrupt."]
pub type TX_BUF_EMPTY_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_BUF_EMPTY_EN` writer - 1=Enable an interrupt if TRANSMIT_BUFFER_EMPTY is asserted\n 0=Disable the interrupt."]
pub type TX_BUF_EMPTY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TX_BUF_REQ_EN` reader - 1=Enable an interrupt if TRANSMIT_BUFFER_REQUEST is asserted\n 0=Disable the interrupt."]
pub type TX_BUF_REQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_BUF_REQ_EN` writer - 1=Enable an interrupt if TRANSMIT_BUFFER_REQUEST is asserted\n 0=Disable the interrupt."]
pub type TX_BUF_REQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `RX_BUF_FUL_EN` reader - 1=Enable an interrupt if RECEIVE_BUFFER_FULL is asserted\n 0=Disable the interrupt."]
pub type RX_BUF_FUL_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_BUF_FUL_EN` writer - 1=Enable an interrupt if RECEIVE_BUFFER_FULL is asserted\n 0=Disable the interrupt."]
pub type RX_BUF_FUL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `RX_BUF_EMPTY_EN` reader - 1=Enable an interrupt if RECEIVE_BUFFER_EMPTY is asserted\n 0=Disable the interrupt."]
pub type RX_BUF_EMPTY_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_BUF_EMPTY_EN` writer - 1=Enable an interrupt if RECEIVE_BUFFER_EMPTY is asserted\n 0=Disable the interrupt."]
pub type RX_BUF_EMPTY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `RX_BUF_REQ_EN` reader - 1=Enable an interrupt if RECEIVE_BUFFER_REQUEST is asserted\n 0=Disable the interrupt."]
pub type RX_BUF_REQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_BUF_REQ_EN` writer - 1=Enable an interrupt if RECEIVE_BUFFER_REQUEST is asserted\n 0=Disable the interrupt."]
pub type RX_BUF_REQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 1=Enable an interrupt if TRANSFER_COMPLETE is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn trans_compl_en(&self) -> TRANS_COMPL_EN_R {
        TRANS_COMPL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1=Enable an interrupt if DMA_COMPLETE is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn dma_compl_en(&self) -> DMA_COMPL_EN_R {
        DMA_COMPL_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1=Enable an interrupt if TRANSMIT_BUFFER_ERROR is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn tx_buf_err_en(&self) -> TX_BUF_ERR_EN_R {
        TX_BUF_ERR_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1=Enable an interrupt if RECEIVE_BUFFER_ERROR is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn rx_buf_err_en(&self) -> RX_BUF_ERR_EN_R {
        RX_BUF_ERR_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1=Enable an interrupt if PROGRAMMING_ERROR is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn prgm_err_en(&self) -> PRGM_ERR_EN_R {
        PRGM_ERR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 1=Enable an interrupt if TRANSMIT_BUFFER_FULL is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn tx_buf_full_en(&self) -> TX_BUF_FULL_EN_R {
        TX_BUF_FULL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1=Enable an interrupt if TRANSMIT_BUFFER_EMPTY is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn tx_buf_empty_en(&self) -> TX_BUF_EMPTY_EN_R {
        TX_BUF_EMPTY_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1=Enable an interrupt if TRANSMIT_BUFFER_REQUEST is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn tx_buf_req_en(&self) -> TX_BUF_REQ_EN_R {
        TX_BUF_REQ_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - 1=Enable an interrupt if RECEIVE_BUFFER_FULL is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn rx_buf_ful_en(&self) -> RX_BUF_FUL_EN_R {
        RX_BUF_FUL_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1=Enable an interrupt if RECEIVE_BUFFER_EMPTY is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn rx_buf_empty_en(&self) -> RX_BUF_EMPTY_EN_R {
        RX_BUF_EMPTY_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1=Enable an interrupt if RECEIVE_BUFFER_REQUEST is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn rx_buf_req_en(&self) -> RX_BUF_REQ_EN_R {
        RX_BUF_REQ_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1=Enable an interrupt if TRANSFER_COMPLETE is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn trans_compl_en(&mut self) -> TRANS_COMPL_EN_W<0> {
        TRANS_COMPL_EN_W::new(self)
    }
    #[doc = "Bit 1 - 1=Enable an interrupt if DMA_COMPLETE is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn dma_compl_en(&mut self) -> DMA_COMPL_EN_W<1> {
        DMA_COMPL_EN_W::new(self)
    }
    #[doc = "Bit 2 - 1=Enable an interrupt if TRANSMIT_BUFFER_ERROR is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn tx_buf_err_en(&mut self) -> TX_BUF_ERR_EN_W<2> {
        TX_BUF_ERR_EN_W::new(self)
    }
    #[doc = "Bit 3 - 1=Enable an interrupt if RECEIVE_BUFFER_ERROR is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn rx_buf_err_en(&mut self) -> RX_BUF_ERR_EN_W<3> {
        RX_BUF_ERR_EN_W::new(self)
    }
    #[doc = "Bit 4 - 1=Enable an interrupt if PROGRAMMING_ERROR is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn prgm_err_en(&mut self) -> PRGM_ERR_EN_W<4> {
        PRGM_ERR_EN_W::new(self)
    }
    #[doc = "Bit 8 - 1=Enable an interrupt if TRANSMIT_BUFFER_FULL is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn tx_buf_full_en(&mut self) -> TX_BUF_FULL_EN_W<8> {
        TX_BUF_FULL_EN_W::new(self)
    }
    #[doc = "Bit 9 - 1=Enable an interrupt if TRANSMIT_BUFFER_EMPTY is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn tx_buf_empty_en(&mut self) -> TX_BUF_EMPTY_EN_W<9> {
        TX_BUF_EMPTY_EN_W::new(self)
    }
    #[doc = "Bit 10 - 1=Enable an interrupt if TRANSMIT_BUFFER_REQUEST is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn tx_buf_req_en(&mut self) -> TX_BUF_REQ_EN_W<10> {
        TX_BUF_REQ_EN_W::new(self)
    }
    #[doc = "Bit 12 - 1=Enable an interrupt if RECEIVE_BUFFER_FULL is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn rx_buf_ful_en(&mut self) -> RX_BUF_FUL_EN_W<12> {
        RX_BUF_FUL_EN_W::new(self)
    }
    #[doc = "Bit 13 - 1=Enable an interrupt if RECEIVE_BUFFER_EMPTY is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn rx_buf_empty_en(&mut self) -> RX_BUF_EMPTY_EN_W<13> {
        RX_BUF_EMPTY_EN_W::new(self)
    }
    #[doc = "Bit 14 - 1=Enable an interrupt if RECEIVE_BUFFER_REQUEST is asserted\n 0=Disable the interrupt."]
    #[inline(always)]
    pub fn rx_buf_req_en(&mut self) -> RX_BUF_REQ_EN_W<14> {
        RX_BUF_REQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0x2000"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
