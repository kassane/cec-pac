#[doc = "Register `RESULT18` reader"]
pub struct R(crate::R<RESULT18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "GIRQ18 Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result18](index.html) module"]
pub struct RESULT18_SPEC;
impl crate::RegisterSpec for RESULT18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result18::R](R) reader structure"]
impl crate::Readable for RESULT18_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT18 to value 0"]
impl crate::Resettable for RESULT18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
