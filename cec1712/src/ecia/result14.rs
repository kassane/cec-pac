#[doc = "Register `RESULT14` reader"]
pub struct R(crate::R<RESULT14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "GIRQ14 Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result14](index.html) module"]
pub struct RESULT14_SPEC;
impl crate::RegisterSpec for RESULT14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result14::R](R) reader structure"]
impl crate::Readable for RESULT14_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT14 to value 0"]
impl crate::Resettable for RESULT14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
