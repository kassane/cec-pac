#[doc = "Register `CNT_ON` reader"]
pub struct R(crate::R<CNT_ON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_ON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_ON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_ON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT_ON` writer"]
pub struct W(crate::W<CNT_ON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_ON_SPEC>;
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
impl From<crate::W<CNT_ON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_ON_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This field determines both the frequency and duty cycle of the PWM signal. Setting this field to a value of n will cause the On time of the PWM to be n+1 cycles of the PWM Clock Source. When this field is set to zero and the PWMX_COUNTER_OFF_TIME is not set to zero, the PWM_OUTPUT is held low (Full Off).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt_on](index.html) module"]
pub struct CNT_ON_SPEC;
impl crate::RegisterSpec for CNT_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt_on::R](R) reader structure"]
impl crate::Readable for CNT_ON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt_on::W](W) writer structure"]
impl crate::Writable for CNT_ON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNT_ON to value 0"]
impl crate::Resettable for CNT_ON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
