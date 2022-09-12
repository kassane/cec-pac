#[doc = "Register `LM_COUNT` reader"]
pub struct R(crate::R<LM_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LM_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LM_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LM_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - A Read-Only count of bytes processed."]
pub type CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - A Read-Only count of bytes processed."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
#[doc = "Loadtime Monitor Channel Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lm_count](index.html) module"]
pub struct LM_COUNT_SPEC;
impl crate::RegisterSpec for LM_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lm_count::R](R) reader structure"]
impl crate::Readable for LM_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LM_COUNT to value 0"]
impl crate::Resettable for LM_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
