#[doc = "Register `KSI` reader"]
pub struct R(crate::R<KSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "7:0\\]
This field returns the current state of the KSI pins.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ksi](index.html) module"]
pub struct KSI_SPEC;
impl crate::RegisterSpec for KSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ksi::R](R) reader structure"]
impl crate::Readable for KSI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets KSI to value 0"]
impl crate::Resettable for KSI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
