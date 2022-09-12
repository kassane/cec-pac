#[doc = "Register `REPT_EN` reader"]
pub struct R(crate::R<REPT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REPT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REPT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REPT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REPT_EN` writer"]
pub struct W(crate::W<REPT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REPT_EN_SPEC>;
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
impl From<crate::W<REPT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REPT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R_EN` reader - Each bit in this field enables the corresponding ADC channel for each pass of the Repeated ADC Conversion that is\n controlled by bit START_REPEAT in the ADC Control Register. 1=repeat conversions for this channel are enabled;\n 0=repeat conversions for this channel are disabled"]
pub type R_EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `R_EN` writer - Each bit in this field enables the corresponding ADC channel for each pass of the Repeated ADC Conversion that is\n controlled by bit START_REPEAT in the ADC Control Register. 1=repeat conversions for this channel are enabled;\n 0=repeat conversions for this channel are disabled"]
pub type R_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REPT_EN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Each bit in this field enables the corresponding ADC channel for each pass of the Repeated ADC Conversion that is\n controlled by bit START_REPEAT in the ADC Control Register. 1=repeat conversions for this channel are enabled;\n 0=repeat conversions for this channel are disabled"]
    #[inline(always)]
    pub fn r_en(&self) -> R_EN_R {
        R_EN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit in this field enables the corresponding ADC channel for each pass of the Repeated ADC Conversion that is\n controlled by bit START_REPEAT in the ADC Control Register. 1=repeat conversions for this channel are enabled;\n 0=repeat conversions for this channel are disabled"]
    #[inline(always)]
    pub fn r_en(&mut self) -> R_EN_W<0> {
        R_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The ADC Repeat Register is used to control which ADC channels \n are captured during a repeat conversion cycle initiated by the Start_Repeat bit in the ADC Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rept_en](index.html) module"]
pub struct REPT_EN_SPEC;
impl crate::RegisterSpec for REPT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rept_en::R](R) reader structure"]
impl crate::Readable for REPT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rept_en::W](W) writer structure"]
impl crate::Writable for REPT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REPT_EN to value 0"]
impl crate::Resettable for REPT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
