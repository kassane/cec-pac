#[doc = "Register `HR_ALARM` reader"]
pub struct R(crate::R<HR_ALARM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR_ALARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR_ALARM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR_ALARM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HR_ALARM` writer"]
pub struct W(crate::W<HR_ALARM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HR_ALARM_SPEC>;
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
impl From<crate::W<HR_ALARM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HR_ALARM_SPEC>) -> Self {
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
#[doc = "Hours Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hr_alarm](index.html) module"]
pub struct HR_ALARM_SPEC;
impl crate::RegisterSpec for HR_ALARM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hr_alarm::R](R) reader structure"]
impl crate::Readable for HR_ALARM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hr_alarm::W](W) writer structure"]
impl crate::Writable for HR_ALARM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HR_ALARM to value 0"]
impl crate::Resettable for HR_ALARM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
