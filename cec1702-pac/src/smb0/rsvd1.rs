#[doc = "Register `RSVD1[%s]` reader"]
pub struct R(crate::R<RSVD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSVD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSVD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSVD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsvd1](index.html) module"]
pub struct RSVD1_SPEC;
impl crate::RegisterSpec for RSVD1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rsvd1::R](R) reader structure"]
impl crate::Readable for RSVD1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSVD1[%s]
to value 0"]
impl crate::Resettable for RSVD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
