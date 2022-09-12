#[doc = "Register `CHAN_STS` reader"]
pub struct R(crate::R<CHAN_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHAN_STS` writer"]
pub struct W(crate::W<CHAN_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHAN_STS_SPEC>;
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
impl From<crate::W<CHAN_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHAN_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STS` reader - All bits are cleared by being written with a '1'. 1=conversion of the corresponding ADC channel is complete;\n 0=conversion of the corresponding ADC channel is not complete. For enabled single cycles, the SINGLE_DONE_STATUS bit in the\n ADC Control Register is also set after all enabled channel conversion are done; for enabled repeat cycles, the REPEAT_DONE_STATUS\n in the ADC Control Register is also set after all enabled channel conversion are done."]
pub type STS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STS` writer - All bits are cleared by being written with a '1'. 1=conversion of the corresponding ADC channel is complete;\n 0=conversion of the corresponding ADC channel is not complete. For enabled single cycles, the SINGLE_DONE_STATUS bit in the\n ADC Control Register is also set after all enabled channel conversion are done; for enabled repeat cycles, the REPEAT_DONE_STATUS\n in the ADC Control Register is also set after all enabled channel conversion are done."]
pub type STS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHAN_STS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - All bits are cleared by being written with a '1'. 1=conversion of the corresponding ADC channel is complete;\n 0=conversion of the corresponding ADC channel is not complete. For enabled single cycles, the SINGLE_DONE_STATUS bit in the\n ADC Control Register is also set after all enabled channel conversion are done; for enabled repeat cycles, the REPEAT_DONE_STATUS\n in the ADC Control Register is also set after all enabled channel conversion are done."]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - All bits are cleared by being written with a '1'. 1=conversion of the corresponding ADC channel is complete;\n 0=conversion of the corresponding ADC channel is not complete. For enabled single cycles, the SINGLE_DONE_STATUS bit in the\n ADC Control Register is also set after all enabled channel conversion are done; for enabled repeat cycles, the REPEAT_DONE_STATUS\n in the ADC Control Register is also set after all enabled channel conversion are done."]
    #[inline(always)]
    pub fn sts(&mut self) -> STS_W<0> {
        STS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The ADC Status Register indicates whether the ADC has completed a conversion cycle. All bits are cleared by being written with a 1. \n 0: conversion of the corresponding ADC channel is not complete\n 1: conversion of the corresponding ADC channel is complete\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_sts](index.html) module"]
pub struct CHAN_STS_SPEC;
impl crate::RegisterSpec for CHAN_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_sts::R](R) reader structure"]
impl crate::Readable for CHAN_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chan_sts::W](W) writer structure"]
impl crate::Writable for CHAN_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHAN_STS to value 0"]
impl crate::Resettable for CHAN_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
