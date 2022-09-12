#[doc = "Register `SNG_EN` reader"]
pub struct R(crate::R<SNG_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNG_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNG_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNG_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNG_EN` writer"]
pub struct W(crate::W<SNG_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNG_EN_SPEC>;
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
impl From<crate::W<SNG_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNG_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S_EN` reader - Each bit in this field enables the corresponding ADC channel when a single cycle of conversions is started when the\n START_SINGLE bit in the ADC Control Register is written with a 1. 1=single cycle conversions for this channel are enabled\n 0=single cycle conversions for this channel are disabled. Note: If this register is changed while a conversion."]
pub type S_EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `S_EN` writer - Each bit in this field enables the corresponding ADC channel when a single cycle of conversions is started when the\n START_SINGLE bit in the ADC Control Register is written with a 1. 1=single cycle conversions for this channel are enabled\n 0=single cycle conversions for this channel are disabled. Note: If this register is changed while a conversion."]
pub type S_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SNG_EN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Each bit in this field enables the corresponding ADC channel when a single cycle of conversions is started when the\n START_SINGLE bit in the ADC Control Register is written with a 1. 1=single cycle conversions for this channel are enabled\n 0=single cycle conversions for this channel are disabled. Note: If this register is changed while a conversion."]
    #[inline(always)]
    pub fn s_en(&self) -> S_EN_R {
        S_EN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit in this field enables the corresponding ADC channel when a single cycle of conversions is started when the\n START_SINGLE bit in the ADC Control Register is written with a 1. 1=single cycle conversions for this channel are enabled\n 0=single cycle conversions for this channel are disabled. Note: If this register is changed while a conversion."]
    #[inline(always)]
    pub fn s_en(&mut self) -> S_EN_W<0> {
        S_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The ADC Single Register is used to control which ADC channel \n is captured during a Single-Sample conversion cycle initiated by the Start_Single bit in the ADC Control Register. \n APPLICATION NOTE: Do not change the bits in this register in the middle of a conversion cycle to insure proper operation.\n 0: single cycle conversions for this channel are disabled\n 1: single cycle conversions for this channel are enabled\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sng_en](index.html) module"]
pub struct SNG_EN_SPEC;
impl crate::RegisterSpec for SNG_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sng_en::R](R) reader structure"]
impl crate::Readable for SNG_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sng_en::W](W) writer structure"]
impl crate::Writable for SNG_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SNG_EN to value 0"]
impl crate::Resettable for SNG_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
