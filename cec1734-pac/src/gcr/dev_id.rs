#[doc = "Register `DEV_ID` reader"]
pub struct R(crate::R<DEV_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEV_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEV_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEV_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "A read-only register which provides device identification LSB.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_id](index.html) module"]
pub struct DEV_ID_SPEC;
impl crate::RegisterSpec for DEV_ID_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dev_id::R](R) reader structure"]
impl crate::Readable for DEV_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEV_ID to value 0x23"]
impl crate::Resettable for DEV_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x23
    }
}
