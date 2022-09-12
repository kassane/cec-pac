#[doc = "Register `DRIV_BASE_FREQ` reader"]
pub struct R(crate::R<DRIV_BASE_FREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRIV_BASE_FREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRIV_BASE_FREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRIV_BASE_FREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRIV_BASE_FREQ` writer"]
pub struct W(crate::W<DRIV_BASE_FREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRIV_BASE_FREQ_SPEC>;
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
impl From<crate::W<DRIV_BASE_FREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRIV_BASE_FREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_BASE` reader - Determines the frequency range of the PWM fan driver (when enabled). PWM resolution is 10-bit, except when this field\n is set to '0b', when it is 8-bit.\n 3=2.34KHz\n 2=4.67KHz\n 1=23.4KHz\n 0=26.8KHz"]
pub type PWM_BASE_R = crate::FieldReader<u8, PWM_BASESELECT_A>;
#[doc = "Determines the frequency range of the PWM fan driver (when enabled). PWM resolution is 10-bit, except when this field\n is set to '0b', when it is 8-bit.\n 3=2.34KHz\n 2=4.67KHz\n 1=23.4KHz\n 0=26.8KHz\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_BASESELECT_A {
    #[doc = "3: 3=2.34KHz"]
    PWM_FAN_FREQ_2KHZ = 3,
    #[doc = "2: 2=4.67KHz"]
    PWM_FAN_FREQ_4KHZ = 2,
    #[doc = "1: 1=23.4KHz"]
    PWM_FAN_FREQ_23KHZ = 1,
    #[doc = "0: 0=26.8KHz"]
    PWM_FAN_FREQ_26KHZ = 0,
}
impl From<PWM_BASESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_BASESELECT_A) -> Self {
        variant as _
    }
}
impl PWM_BASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_BASESELECT_A {
        match self.bits {
            3 => PWM_BASESELECT_A::PWM_FAN_FREQ_2KHZ,
            2 => PWM_BASESELECT_A::PWM_FAN_FREQ_4KHZ,
            1 => PWM_BASESELECT_A::PWM_FAN_FREQ_23KHZ,
            0 => PWM_BASESELECT_A::PWM_FAN_FREQ_26KHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_FAN_FREQ_2KHZ`"]
    #[inline(always)]
    pub fn is_pwm_fan_freq_2khz(&self) -> bool {
        *self == PWM_BASESELECT_A::PWM_FAN_FREQ_2KHZ
    }
    #[doc = "Checks if the value of the field is `PWM_FAN_FREQ_4KHZ`"]
    #[inline(always)]
    pub fn is_pwm_fan_freq_4khz(&self) -> bool {
        *self == PWM_BASESELECT_A::PWM_FAN_FREQ_4KHZ
    }
    #[doc = "Checks if the value of the field is `PWM_FAN_FREQ_23KHZ`"]
    #[inline(always)]
    pub fn is_pwm_fan_freq_23khz(&self) -> bool {
        *self == PWM_BASESELECT_A::PWM_FAN_FREQ_23KHZ
    }
    #[doc = "Checks if the value of the field is `PWM_FAN_FREQ_26KHZ`"]
    #[inline(always)]
    pub fn is_pwm_fan_freq_26khz(&self) -> bool {
        *self == PWM_BASESELECT_A::PWM_FAN_FREQ_26KHZ
    }
}
#[doc = "Field `PWM_BASE` writer - Determines the frequency range of the PWM fan driver (when enabled). PWM resolution is 10-bit, except when this field\n is set to '0b', when it is 8-bit.\n 3=2.34KHz\n 2=4.67KHz\n 1=23.4KHz\n 0=26.8KHz"]
pub type PWM_BASE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, DRIV_BASE_FREQ_SPEC, u8, PWM_BASESELECT_A, 2, O>;
impl<'a, const O: u8> PWM_BASE_W<'a, O> {
    #[doc = "3=2.34KHz"]
    #[inline(always)]
    pub fn pwm_fan_freq_2khz(self) -> &'a mut W {
        self.variant(PWM_BASESELECT_A::PWM_FAN_FREQ_2KHZ)
    }
    #[doc = "2=4.67KHz"]
    #[inline(always)]
    pub fn pwm_fan_freq_4khz(self) -> &'a mut W {
        self.variant(PWM_BASESELECT_A::PWM_FAN_FREQ_4KHZ)
    }
    #[doc = "1=23.4KHz"]
    #[inline(always)]
    pub fn pwm_fan_freq_23khz(self) -> &'a mut W {
        self.variant(PWM_BASESELECT_A::PWM_FAN_FREQ_23KHZ)
    }
    #[doc = "0=26.8KHz"]
    #[inline(always)]
    pub fn pwm_fan_freq_26khz(self) -> &'a mut W {
        self.variant(PWM_BASESELECT_A::PWM_FAN_FREQ_26KHZ)
    }
}
impl R {
    #[doc = "Bits 0:1 - Determines the frequency range of the PWM fan driver (when enabled). PWM resolution is 10-bit, except when this field\n is set to '0b', when it is 8-bit.\n 3=2.34KHz\n 2=4.67KHz\n 1=23.4KHz\n 0=26.8KHz"]
    #[inline(always)]
    pub fn pwm_base(&self) -> PWM_BASE_R {
        PWM_BASE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Determines the frequency range of the PWM fan driver (when enabled). PWM resolution is 10-bit, except when this field\n is set to '0b', when it is 8-bit.\n 3=2.34KHz\n 2=4.67KHz\n 1=23.4KHz\n 0=26.8KHz"]
    #[inline(always)]
    pub fn pwm_base(&mut self) -> PWM_BASE_W<0> {
        PWM_BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1:0\\]
Determines the frequency range of the PWM fan driver\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [driv_base_freq](index.html) module"]
pub struct DRIV_BASE_FREQ_SPEC;
impl crate::RegisterSpec for DRIV_BASE_FREQ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [driv_base_freq::R](R) reader structure"]
impl crate::Readable for DRIV_BASE_FREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [driv_base_freq::W](W) writer structure"]
impl crate::Writable for DRIV_BASE_FREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DRIV_BASE_FREQ to value 0"]
impl crate::Resettable for DRIV_BASE_FREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
