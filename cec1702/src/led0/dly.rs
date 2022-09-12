#[doc = "Register `DLY` reader"]
pub struct R(crate::R<DLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLY` writer"]
pub struct W(crate::W<DLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLY_SPEC>;
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
impl From<crate::W<DLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOW_PULSE` reader - The number of PWM periods to wait before updating the current duty cycle when the current duty cycle is greater than or equal \n to the value MIN in register LED_LIMIT."]
pub type LOW_PULSE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LOW_PULSE` writer - The number of PWM periods to wait before updating the current duty cycle when the current duty cycle is greater than or equal \n to the value MIN in register LED_LIMIT."]
pub type LOW_PULSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLY_SPEC, u16, u16, 12, O>;
#[doc = "Field `HIGH_PULSE` reader - In breathing mode, the number of PWM periods to wait before updating the current duty cycle when the current duty cycle is greater\n than or equal to the value MAX in register LED_LIMIT."]
pub type HIGH_PULSE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HIGH_PULSE` writer - In breathing mode, the number of PWM periods to wait before updating the current duty cycle when the current duty cycle is greater\n than or equal to the value MAX in register LED_LIMIT."]
pub type HIGH_PULSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLY_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - The number of PWM periods to wait before updating the current duty cycle when the current duty cycle is greater than or equal \n to the value MIN in register LED_LIMIT."]
    #[inline(always)]
    pub fn low_pulse(&self) -> LOW_PULSE_R {
        LOW_PULSE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - In breathing mode, the number of PWM periods to wait before updating the current duty cycle when the current duty cycle is greater\n than or equal to the value MAX in register LED_LIMIT."]
    #[inline(always)]
    pub fn high_pulse(&self) -> HIGH_PULSE_R {
        HIGH_PULSE_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The number of PWM periods to wait before updating the current duty cycle when the current duty cycle is greater than or equal \n to the value MIN in register LED_LIMIT."]
    #[inline(always)]
    pub fn low_pulse(&mut self) -> LOW_PULSE_W<0> {
        LOW_PULSE_W::new(self)
    }
    #[doc = "Bits 12:23 - In breathing mode, the number of PWM periods to wait before updating the current duty cycle when the current duty cycle is greater\n than or equal to the value MAX in register LED_LIMIT."]
    #[inline(always)]
    pub fn high_pulse(&mut self) -> HIGH_PULSE_W<12> {
        HIGH_PULSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LED Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dly](index.html) module"]
pub struct DLY_SPEC;
impl crate::RegisterSpec for DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dly::R](R) reader structure"]
impl crate::Readable for DLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dly::W](W) writer structure"]
impl crate::Writable for DLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLY to value 0"]
impl crate::Resettable for DLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
