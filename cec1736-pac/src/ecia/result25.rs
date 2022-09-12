#[doc = "Register `RESULT25` reader"]
pub struct R(crate::R<RESULT25_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT25_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT25_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT25_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "GIRQ25 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result25](index.html) module"]
pub struct RESULT25_SPEC;
impl crate::RegisterSpec for RESULT25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result25::R](R) reader structure"]
impl crate::Readable for RESULT25_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT25 to value 0"]
impl crate::Resettable for RESULT25_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
