#[doc = "Register `LIMIT` reader"]
pub struct R(crate::R<LIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIMIT` writer"]
pub struct W(crate::W<LIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIMIT_SPEC>;
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
impl From<crate::W<LIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIN` reader - In breathing mode, when the current duty cycle is less than or equal to this value the breathing apparatus holds \n the current duty cycle for the period specified by the field LD in register LED_DELAY, then starts incrementing the current duty cycle In blinking mode,\n this field defines the duty cycle of the blink function."]
pub type MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN` writer - In breathing mode, when the current duty cycle is less than or equal to this value the breathing apparatus holds \n the current duty cycle for the period specified by the field LD in register LED_DELAY, then starts incrementing the current duty cycle In blinking mode,\n this field defines the duty cycle of the blink function."]
pub type MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LIMIT_SPEC, u8, u8, 8, O>;
#[doc = "Field `MAX` reader - In breathing mode, when the current duty cycle is greater than or equal to this value the breathing apparatus holds \n the current duty cycle for the period specified by the field HD in register LED_DELAY, then starts decrementing the current duty cycle"]
pub type MAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX` writer - In breathing mode, when the current duty cycle is greater than or equal to this value the breathing apparatus holds \n the current duty cycle for the period specified by the field HD in register LED_DELAY, then starts decrementing the current duty cycle"]
pub type MAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LIMIT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In breathing mode, when the current duty cycle is less than or equal to this value the breathing apparatus holds \n the current duty cycle for the period specified by the field LD in register LED_DELAY, then starts incrementing the current duty cycle In blinking mode,\n this field defines the duty cycle of the blink function."]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In breathing mode, when the current duty cycle is greater than or equal to this value the breathing apparatus holds \n the current duty cycle for the period specified by the field HD in register LED_DELAY, then starts decrementing the current duty cycle"]
    #[inline(always)]
    pub fn max(&self) -> MAX_R {
        MAX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - In breathing mode, when the current duty cycle is less than or equal to this value the breathing apparatus holds \n the current duty cycle for the period specified by the field LD in register LED_DELAY, then starts incrementing the current duty cycle In blinking mode,\n this field defines the duty cycle of the blink function."]
    #[inline(always)]
    pub fn min(&mut self) -> MIN_W<0> {
        MIN_W::new(self)
    }
    #[doc = "Bits 8:15 - In breathing mode, when the current duty cycle is greater than or equal to this value the breathing apparatus holds \n the current duty cycle for the period specified by the field HD in register LED_DELAY, then starts decrementing the current duty cycle"]
    #[inline(always)]
    pub fn max(&mut self) -> MAX_W<8> {
        MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LED Limits This register may be written at any time.\n Values written into the register are held in an holding register, which is transferred into the actual register at the end of a PWM period.\n The two byte fields may be written independently. Reads of this register return the current contents and not the value of the holding register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limit](index.html) module"]
pub struct LIMIT_SPEC;
impl crate::RegisterSpec for LIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [limit::R](R) reader structure"]
impl crate::Readable for LIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [limit::W](W) writer structure"]
impl crate::Writable for LIMIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIMIT to value 0"]
impl crate::Resettable for LIMIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
