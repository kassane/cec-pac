#[doc = "Register `LEG_DEV_REV` reader"]
pub struct R(crate::R<LEG_DEV_REV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEG_DEV_REV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEG_DEV_REV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEG_DEV_REV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "A read-only register which provides legacy device revision information.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leg_dev_rev](index.html) module"]
pub struct LEG_DEV_REV_SPEC;
impl crate::RegisterSpec for LEG_DEV_REV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [leg_dev_rev::R](R) reader structure"]
impl crate::Readable for LEG_DEV_REV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LEG_DEV_REV to value 0xa0"]
impl crate::Resettable for LEG_DEV_REV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa0
    }
}
