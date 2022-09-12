#[doc = "Register `PWM_DIV` reader"]
pub struct R(crate::R<PWM_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_DIV` writer"]
pub struct W(crate::W<PWM_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_DIV_SPEC>;
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
impl From<crate::W<PWM_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_DIV` reader - The PWM Divide value determines the final frequency of the PWM driver. The driver base frequency is divided by the\n PWM Divide value to determine the final frequency."]
pub type PWM_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWM_DIV` writer - The PWM Divide value determines the final frequency of the PWM driver. The driver base frequency is divided by the\n PWM Divide value to determine the final frequency."]
pub type PWM_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PWM_DIV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The PWM Divide value determines the final frequency of the PWM driver. The driver base frequency is divided by the\n PWM Divide value to determine the final frequency."]
    #[inline(always)]
    pub fn pwm_div(&self) -> PWM_DIV_R {
        PWM_DIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - The PWM Divide value determines the final frequency of the PWM driver. The driver base frequency is divided by the\n PWM Divide value to determine the final frequency."]
    #[inline(always)]
    pub fn pwm_div(&mut self) -> PWM_DIV_W<0> {
        PWM_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Divide\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_div](index.html) module"]
pub struct PWM_DIV_SPEC;
impl crate::RegisterSpec for PWM_DIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pwm_div::R](R) reader structure"]
impl crate::Readable for PWM_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_div::W](W) writer structure"]
impl crate::Writable for PWM_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_DIV to value 0x01"]
impl crate::Resettable for PWM_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
