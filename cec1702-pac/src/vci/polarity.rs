#[doc = "Register `POLARITY` reader"]
pub struct R(crate::R<POLARITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POLARITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POLARITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POLARITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POLARITY` writer"]
pub struct W(crate::W<POLARITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POLARITY_SPEC>;
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
impl From<crate::W<POLARITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POLARITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCI_IN` reader - These bits determine the polarity of the VCI_IN input signals: For each bit in the field:\n 1=Active High. The value on the pins is inverted before use\n 0=Active Low (default)."]
pub type VCI_IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCI_IN` writer - These bits determine the polarity of the VCI_IN input signals: For each bit in the field:\n 1=Active High. The value on the pins is inverted before use\n 0=Active Low (default)."]
pub type VCI_IN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POLARITY_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - These bits determine the polarity of the VCI_IN input signals: For each bit in the field:\n 1=Active High. The value on the pins is inverted before use\n 0=Active Low (default)."]
    #[inline(always)]
    pub fn vci_in(&self) -> VCI_IN_R {
        VCI_IN_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - These bits determine the polarity of the VCI_IN input signals: For each bit in the field:\n 1=Active High. The value on the pins is inverted before use\n 0=Active Low (default)."]
    #[inline(always)]
    pub fn vci_in(&mut self) -> VCI_IN_W<0> {
        VCI_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VCI Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [polarity](index.html) module"]
pub struct POLARITY_SPEC;
impl crate::RegisterSpec for POLARITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [polarity::R](R) reader structure"]
impl crate::Readable for POLARITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [polarity::W](W) writer structure"]
impl crate::Writable for POLARITY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POLARITY to value 0"]
impl crate::Resettable for POLARITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
