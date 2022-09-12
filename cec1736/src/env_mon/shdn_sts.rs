#[doc = "Register `SHDN_STS` reader"]
pub struct R(crate::R<SHDN_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHDN_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHDN_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHDN_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SHDN_STS` reader - Stores the status bits that indicate which diode caused the SYS_SHDN# output to assert."]
pub type SHDN_STS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Stores the status bits that indicate which diode caused the SYS_SHDN# output to assert."]
    #[inline(always)]
    pub fn shdn_sts(&self) -> SHDN_STS_R {
        SHDN_STS_R::new(self.bits)
    }
}
#[doc = "Shutdown Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdn_sts](index.html) module"]
pub struct SHDN_STS_SPEC;
impl crate::RegisterSpec for SHDN_STS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [shdn_sts::R](R) reader structure"]
impl crate::Readable for SHDN_STS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SHDN_STS to value 0"]
impl crate::Resettable for SHDN_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
