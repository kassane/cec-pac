#[doc = "Register `LM_DIGEST` reader"]
pub struct R(crate::R<LM_DIGEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LM_DIGEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LM_DIGEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LM_DIGEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DGST` reader - A Read-Only FIFO Portal to Hash digest result. 12 or 8 Dwords depending on algorithm."]
pub type DGST_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - A Read-Only FIFO Portal to Hash digest result. 12 or 8 Dwords depending on algorithm."]
    #[inline(always)]
    pub fn dgst(&self) -> DGST_R {
        DGST_R::new(self.bits)
    }
}
#[doc = "Loadtime Monitor Channel Digest Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lm_digest](index.html) module"]
pub struct LM_DIGEST_SPEC;
impl crate::RegisterSpec for LM_DIGEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lm_digest::R](R) reader structure"]
impl crate::Readable for LM_DIGEST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LM_DIGEST to value 0"]
impl crate::Resettable for LM_DIGEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
