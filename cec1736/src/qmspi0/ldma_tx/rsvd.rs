#[doc = "Register `RSVD` reader"]
pub struct R(crate::R<RSVD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSVD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSVD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSVD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsvd](index.html) module"]
pub struct RSVD_SPEC;
impl crate::RegisterSpec for RSVD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsvd::R](R) reader structure"]
impl crate::Readable for RSVD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSVD to value 0"]
impl crate::Resettable for RSVD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
