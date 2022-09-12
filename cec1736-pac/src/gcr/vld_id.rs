#[doc = "Register `VLD_ID` reader"]
pub struct R(crate::R<VLD_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLD_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLD_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLD_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "A read-only register which provides Validation ID information.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vld_id](index.html) module"]
pub struct VLD_ID_SPEC;
impl crate::RegisterSpec for VLD_ID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vld_id::R](R) reader structure"]
impl crate::Readable for VLD_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VLD_ID to value 0"]
impl crate::Resettable for VLD_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
