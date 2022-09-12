#[doc = "Register `RESULT16` reader"]
pub struct R(crate::R<RESULT16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "GIRQ16 Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result16](index.html) module"]
pub struct RESULT16_SPEC;
impl crate::RegisterSpec for RESULT16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result16::R](R) reader structure"]
impl crate::Readable for RESULT16_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT16 to value 0"]
impl crate::Resettable for RESULT16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
