#[doc = "Register `SWK_ALARM` reader"]
pub struct R(crate::R<SWK_ALARM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWK_ALARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWK_ALARM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWK_ALARM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNTR_LOAD` reader - Writes with a non-zero value to this field reload the 9-bit Sub-Week Alarm counter. Writes of 0 disable the counter.\n If the Sub-Week Alarm counter decrements to 0 and the AUTO_RELOAD bit is set, the value in this field is automatically loaded into the Sub-Week Alarm counter."]
pub type CNTR_LOAD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNTR_STS` reader - Reads of this register return the current state of the 9-bit Sub-Week Alarm counter."]
pub type CNTR_STS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:8 - Writes with a non-zero value to this field reload the 9-bit Sub-Week Alarm counter. Writes of 0 disable the counter.\n If the Sub-Week Alarm counter decrements to 0 and the AUTO_RELOAD bit is set, the value in this field is automatically loaded into the Sub-Week Alarm counter."]
    #[inline(always)]
    pub fn cntr_load(&self) -> CNTR_LOAD_R {
        CNTR_LOAD_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Reads of this register return the current state of the 9-bit Sub-Week Alarm counter."]
    #[inline(always)]
    pub fn cntr_sts(&self) -> CNTR_STS_R {
        CNTR_STS_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
#[doc = "Sub-Week Alarm Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swk_alarm](index.html) module"]
pub struct SWK_ALARM_SPEC;
impl crate::RegisterSpec for SWK_ALARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swk_alarm::R](R) reader structure"]
impl crate::Readable for SWK_ALARM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SWK_ALARM to value 0"]
impl crate::Resettable for SWK_ALARM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
