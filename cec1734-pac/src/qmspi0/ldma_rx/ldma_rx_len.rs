#[doc = "Register `LDMA_RX_LEN` reader"]
pub struct R(crate::R<LDMA_RX_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDMA_RX_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDMA_RX_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDMA_RX_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDMA_RX_LEN` writer"]
pub struct W(crate::W<LDMA_RX_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDMA_RX_LEN_SPEC>;
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
impl From<crate::W<LDMA_RX_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDMA_RX_LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_LEN` reader - This is the maximum Length of the transfer in Bytes that the DMA Channel will allow access to."]
pub type RX_LEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RX_LEN` writer - This is the maximum Length of the transfer in Bytes that the DMA Channel will allow access to."]
pub type RX_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDMA_RX_LEN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This is the maximum Length of the transfer in Bytes that the DMA Channel will allow access to."]
    #[inline(always)]
    pub fn rx_len(&self) -> RX_LEN_R {
        RX_LEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the maximum Length of the transfer in Bytes that the DMA Channel will allow access to."]
    #[inline(always)]
    pub fn rx_len(&mut self) -> RX_LEN_W<0> {
        RX_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Local DMA Rx Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldma_rx_len](index.html) module"]
pub struct LDMA_RX_LEN_SPEC;
impl crate::RegisterSpec for LDMA_RX_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldma_rx_len::R](R) reader structure"]
impl crate::Readable for LDMA_RX_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldma_rx_len::W](W) writer structure"]
impl crate::Writable for LDMA_RX_LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDMA_RX_LEN to value 0"]
impl crate::Resettable for LDMA_RX_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
