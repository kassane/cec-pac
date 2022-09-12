#[doc = "Register `UNLCK` reader"]
pub struct R(crate::R<UNLCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNLCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNLCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNLCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UNLCK` reader - Unlock Register"]
pub type UNLCK_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Unlock Register"]
    #[inline(always)]
    pub fn unlck(&self) -> UNLCK_R {
        UNLCK_R::new(self.bits)
    }
}
#[doc = "Unlock Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unlck](index.html) module"]
pub struct UNLCK_SPEC;
impl crate::RegisterSpec for UNLCK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [unlck::R](R) reader structure"]
impl crate::Readable for UNLCK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UNLCK to value 0"]
impl crate::Resettable for UNLCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
