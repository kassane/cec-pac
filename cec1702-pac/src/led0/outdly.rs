#[doc = "Register `OUTDLY` reader"]
pub struct R(crate::R<OUTDLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTDLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTDLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTDLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTDLY` writer"]
pub struct W(crate::W<OUTDLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTDLY_SPEC>;
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
impl From<crate::W<OUTDLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTDLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELAY` reader - The delay, in counts of the clock defined in Clock Source (CLKSRC), in which output transitions are delayed.\n When this field is 0, there is no added transition delay. When the LED is programmed to be Always On or Always Off, the\n Output Delay field has no effect."]
pub type DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELAY` writer - The delay, in counts of the clock defined in Clock Source (CLKSRC), in which output transitions are delayed.\n When this field is 0, there is no added transition delay. When the LED is programmed to be Always On or Always Off, the\n Output Delay field has no effect."]
pub type DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTDLY_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The delay, in counts of the clock defined in Clock Source (CLKSRC), in which output transitions are delayed.\n When this field is 0, there is no added transition delay. When the LED is programmed to be Always On or Always Off, the\n Output Delay field has no effect."]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The delay, in counts of the clock defined in Clock Source (CLKSRC), in which output transitions are delayed.\n When this field is 0, there is no added transition delay. When the LED is programmed to be Always On or Always Off, the\n Output Delay field has no effect."]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W<0> {
        DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LED Output Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outdly](index.html) module"]
pub struct OUTDLY_SPEC;
impl crate::RegisterSpec for OUTDLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outdly::R](R) reader structure"]
impl crate::Readable for OUTDLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outdly::W](W) writer structure"]
impl crate::Writable for OUTDLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTDLY to value 0"]
impl crate::Resettable for OUTDLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
