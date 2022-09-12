#[doc = "Register `LDMA_TXSTRT_ADDR` reader"]
pub struct R(crate::R<LDMA_TXSTRT_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDMA_TXSTRT_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDMA_TXSTRT_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDMA_TXSTRT_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDMA_TXSTRT_ADDR` writer"]
pub struct W(crate::W<LDMA_TXSTRT_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDMA_TXSTRT_ADDR_SPEC>;
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
impl From<crate::W<LDMA_TXSTRT_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDMA_TXSTRT_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRT_ADDR` reader - This is the Starting Address for the DMA access into the memory space (Read from this address on Tx). This address is updated by the transfer size based on the Local DMA Access Size after every access."]
pub type STRT_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STRT_ADDR` writer - This is the Starting Address for the DMA access into the memory space (Read from this address on Tx). This address is updated by the transfer size based on the Local DMA Access Size after every access."]
pub type STRT_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDMA_TXSTRT_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This is the Starting Address for the DMA access into the memory space (Read from this address on Tx). This address is updated by the transfer size based on the Local DMA Access Size after every access."]
    #[inline(always)]
    pub fn strt_addr(&self) -> STRT_ADDR_R {
        STRT_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the Starting Address for the DMA access into the memory space (Read from this address on Tx). This address is updated by the transfer size based on the Local DMA Access Size after every access."]
    #[inline(always)]
    pub fn strt_addr(&mut self) -> STRT_ADDR_W<0> {
        STRT_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Local DMA Tx Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldma_txstrt_addr](index.html) module"]
pub struct LDMA_TXSTRT_ADDR_SPEC;
impl crate::RegisterSpec for LDMA_TXSTRT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldma_txstrt_addr::R](R) reader structure"]
impl crate::Readable for LDMA_TXSTRT_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldma_txstrt_addr::W](W) writer structure"]
impl crate::Writable for LDMA_TXSTRT_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDMA_TXSTRT_ADDR to value 0"]
impl crate::Resettable for LDMA_TXSTRT_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
