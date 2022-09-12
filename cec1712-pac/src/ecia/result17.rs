#[doc = "Register `RESULT17` reader"]
pub struct R(crate::R<RESULT17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "GIRQ17 Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result17](index.html) module"]
pub struct RESULT17_SPEC;
impl crate::RegisterSpec for RESULT17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result17::R](R) reader structure"]
impl crate::Readable for RESULT17_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT17 to value 0"]
impl crate::Resettable for RESULT17_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
