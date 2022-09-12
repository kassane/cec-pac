#[doc = "Register `DESC_LDMA_RXEN` reader"]
pub struct R(crate::R<DESC_LDMA_RXEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESC_LDMA_RXEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESC_LDMA_RXEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESC_LDMA_RXEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESC_LDMA_RXEN` writer"]
pub struct W(crate::W<DESC_LDMA_RXEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESC_LDMA_RXEN_SPEC>;
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
impl From<crate::W<DESC_LDMA_RXEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESC_LDMA_RXEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DESC_LDMA_RXEN` reader - This enables the Local RX DMA usage (instead of the Central DMA) when the Descriptor Buffer register enables the DMA."]
pub type DESC_LDMA_RXEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DESC_LDMA_RXEN` writer - This enables the Local RX DMA usage (instead of the Central DMA) when the Descriptor Buffer register enables the DMA."]
pub type DESC_LDMA_RXEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESC_LDMA_RXEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This enables the Local RX DMA usage (instead of the Central DMA) when the Descriptor Buffer register enables the DMA."]
    #[inline(always)]
    pub fn desc_ldma_rxen(&self) -> DESC_LDMA_RXEN_R {
        DESC_LDMA_RXEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This enables the Local RX DMA usage (instead of the Central DMA) when the Descriptor Buffer register enables the DMA."]
    #[inline(always)]
    pub fn desc_ldma_rxen(&mut self) -> DESC_LDMA_RXEN_W<0> {
        DESC_LDMA_RXEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Descriptor Local DMA Rx Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [desc_ldma_rxen](index.html) module"]
pub struct DESC_LDMA_RXEN_SPEC;
impl crate::RegisterSpec for DESC_LDMA_RXEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [desc_ldma_rxen::R](R) reader structure"]
impl crate::Readable for DESC_LDMA_RXEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [desc_ldma_rxen::W](W) writer structure"]
impl crate::Writable for DESC_LDMA_RXEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DESC_LDMA_RXEN to value 0"]
impl crate::Resettable for DESC_LDMA_RXEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
