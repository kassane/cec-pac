#[doc = "Register `STEP` reader"]
pub struct R(crate::R<STEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STEP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STEP` writer"]
pub struct W(crate::W<STEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STEP_SPEC>;
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
impl From<crate::W<STEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAN_STEP` reader - The Fan Step value represents the maximum step size the fan driver will take between update times.\n When the PWM_BASE frequency range field in the PWM Driver Base Frequency Register is set to the value 1, 2 or 3, this 8-bit field\n is added to the 10-bit PWM duty cycle, for a maximum step size of 25%. When the PWM_BASE field is set to 0, the PWM operates in\n an 8-bit mode. In 8-bit mode, this 8-bit field is added to the 8-bit duty cycle, for a maximum step size of 100%."]
pub type FAN_STEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAN_STEP` writer - The Fan Step value represents the maximum step size the fan driver will take between update times.\n When the PWM_BASE frequency range field in the PWM Driver Base Frequency Register is set to the value 1, 2 or 3, this 8-bit field\n is added to the 10-bit PWM duty cycle, for a maximum step size of 25%. When the PWM_BASE field is set to 0, the PWM operates in\n an 8-bit mode. In 8-bit mode, this 8-bit field is added to the 8-bit duty cycle, for a maximum step size of 100%."]
pub type FAN_STEP_W<'a, const O: u8> = crate::FieldWriter<'a, u8, STEP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The Fan Step value represents the maximum step size the fan driver will take between update times.\n When the PWM_BASE frequency range field in the PWM Driver Base Frequency Register is set to the value 1, 2 or 3, this 8-bit field\n is added to the 10-bit PWM duty cycle, for a maximum step size of 25%. When the PWM_BASE field is set to 0, the PWM operates in\n an 8-bit mode. In 8-bit mode, this 8-bit field is added to the 8-bit duty cycle, for a maximum step size of 100%."]
    #[inline(always)]
    pub fn fan_step(&self) -> FAN_STEP_R {
        FAN_STEP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - The Fan Step value represents the maximum step size the fan driver will take between update times.\n When the PWM_BASE frequency range field in the PWM Driver Base Frequency Register is set to the value 1, 2 or 3, this 8-bit field\n is added to the 10-bit PWM duty cycle, for a maximum step size of 25%. When the PWM_BASE field is set to 0, the PWM operates in\n an 8-bit mode. In 8-bit mode, this 8-bit field is added to the 8-bit duty cycle, for a maximum step size of 100%."]
    #[inline(always)]
    pub fn fan_step(&mut self) -> FAN_STEP_W<0> {
        FAN_STEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FAN_STEP The Fan Step value represents the maximum step size the fan driver will take between update times\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [step](index.html) module"]
pub struct STEP_SPEC;
impl crate::RegisterSpec for STEP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [step::R](R) reader structure"]
impl crate::Readable for STEP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [step::W](W) writer structure"]
impl crate::Writable for STEP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STEP to value 0x10"]
impl crate::Resettable for STEP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
