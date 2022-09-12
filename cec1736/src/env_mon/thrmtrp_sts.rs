#[doc = "Register `THRMTRP_STS` reader"]
pub struct R(crate::R<THRMTRP_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRMTRP_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRMTRP_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRMTRP_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `THRMTRP_STS` reader - Stores the pin state of the signals that affect the SYS_SHDN_n signal"]
pub type THRMTRP_STS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Stores the pin state of the signals that affect the SYS_SHDN_n signal"]
    #[inline(always)]
    pub fn thrmtrp_sts(&self) -> THRMTRP_STS_R {
        THRMTRP_STS_R::new(self.bits)
    }
}
#[doc = "ThermTrip Pin Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thrmtrp_sts](index.html) module"]
pub struct THRMTRP_STS_SPEC;
impl crate::RegisterSpec for THRMTRP_STS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [thrmtrp_sts::R](R) reader structure"]
impl crate::Readable for THRMTRP_STS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets THRMTRP_STS to value 0"]
impl crate::Resettable for THRMTRP_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
