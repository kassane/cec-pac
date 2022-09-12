#[doc = "Register `FLSF_STS` reader"]
pub struct R(crate::R<FLSF_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLSF_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLSF_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLSF_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLSF_STS` reader - Stores the status indicate which ThermTrip input condition caused the SYS_SHDN# pin to be asserted."]
pub type FLSF_STS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Stores the status indicate which ThermTrip input condition caused the SYS_SHDN# pin to be asserted."]
    #[inline(always)]
    pub fn flsf_sts(&self) -> FLSF_STS_R {
        FLSF_STS_R::new(self.bits)
    }
}
#[doc = "FailSafe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flsf_sts](index.html) module"]
pub struct FLSF_STS_SPEC;
impl crate::RegisterSpec for FLSF_STS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [flsf_sts::R](R) reader structure"]
impl crate::Readable for FLSF_STS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLSF_STS to value 0"]
impl crate::Resettable for FLSF_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
