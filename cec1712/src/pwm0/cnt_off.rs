#[doc = "Register `CNT_OFF` reader"]
pub struct R(crate::R<CNT_OFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_OFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_OFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_OFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT_OFF` writer"]
pub struct W(crate::W<CNT_OFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_OFF_SPEC>;
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
impl From<crate::W<CNT_OFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_OFF_SPEC>) -> Self {
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
#[doc = "This field determine both the frequency and duty cycle of the PWM signal. Setting this field to a value of n will\n cause the Off time of the PWM to be n+1 cycles of the PWM Clock Source.\n When this field is set to zero, the PWM_OUTPUT is held high (Full On).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt_off](index.html) module"]
pub struct CNT_OFF_SPEC;
impl crate::RegisterSpec for CNT_OFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt_off::R](R) reader structure"]
impl crate::Readable for CNT_OFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt_off::W](W) writer structure"]
impl crate::Writable for CNT_OFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNT_OFF to value 0xffff"]
impl crate::Resettable for CNT_OFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
