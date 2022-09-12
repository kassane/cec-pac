#[doc = "Register `MIN_ALARM` reader"]
pub struct R(crate::R<MIN_ALARM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIN_ALARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIN_ALARM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIN_ALARM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIN_ALARM` writer"]
pub struct W(crate::W<MIN_ALARM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIN_ALARM_SPEC>;
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
impl From<crate::W<MIN_ALARM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIN_ALARM_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Minutes Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [min_alarm](index.html) module"]
pub struct MIN_ALARM_SPEC;
impl crate::RegisterSpec for MIN_ALARM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [min_alarm::R](R) reader structure"]
impl crate::Readable for MIN_ALARM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [min_alarm::W](W) writer structure"]
impl crate::Writable for MIN_ALARM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIN_ALARM to value 0"]
impl crate::Resettable for MIN_ALARM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
