#[doc = "Register `RESULT10` reader"]
pub struct R(crate::R<RESULT10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "GIRQ10 Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result10](index.html) module"]
pub struct RESULT10_SPEC;
impl crate::RegisterSpec for RESULT10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result10::R](R) reader structure"]
impl crate::Readable for RESULT10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT10 to value 0"]
impl crate::Resettable for RESULT10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
