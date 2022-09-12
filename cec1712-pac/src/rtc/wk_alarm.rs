#[doc = "Register `WK_ALARM` reader"]
pub struct R(crate::R<WK_ALARM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WK_ALARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WK_ALARM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WK_ALARM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WK_ALARM` writer"]
pub struct W(crate::W<WK_ALARM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WK_ALARM_SPEC>;
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
impl From<crate::W<WK_ALARM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WK_ALARM_SPEC>) -> Self {
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
#[doc = "Week Alarm Register\\[7:0\\]
- ALARM_DAY_OF_WEEK This register, if written to a value in the range 1- -7, will inhibit the Alarm\n interrupt unless this field matches the contents of the Day of Week Register also.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wk_alarm](index.html) module"]
pub struct WK_ALARM_SPEC;
impl crate::RegisterSpec for WK_ALARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wk_alarm::R](R) reader structure"]
impl crate::Readable for WK_ALARM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wk_alarm::W](W) writer structure"]
impl crate::Writable for WK_ALARM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WK_ALARM to value 0xff"]
impl crate::Resettable for WK_ALARM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
