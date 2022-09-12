#[doc = "Register `DEV_SUBID` reader"]
pub struct R(crate::R<DEV_SUBID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEV_SUBID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEV_SUBID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEV_SUBID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "A read-only register which provides device sub ID information.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_subid](index.html) module"]
pub struct DEV_SUBID_SPEC;
impl crate::RegisterSpec for DEV_SUBID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dev_subid::R](R) reader structure"]
impl crate::Readable for DEV_SUBID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEV_SUBID to value 0xa2"]
impl crate::Resettable for DEV_SUBID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa2
    }
}
