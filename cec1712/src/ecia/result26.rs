#[doc = "Register `RESULT26` reader"]
pub struct R(crate::R<RESULT26_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT26_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT26_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT26_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "GIRQ26 Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result26](index.html) module"]
pub struct RESULT26_SPEC;
impl crate::RegisterSpec for RESULT26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result26::R](R) reader structure"]
impl crate::Readable for RESULT26_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT26 to value 0"]
impl crate::Resettable for RESULT26_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
