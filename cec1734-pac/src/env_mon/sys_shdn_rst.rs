#[doc = "Register `SYS_SHDN_RST` reader"]
pub struct R(crate::R<SYS_SHDN_RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SHDN_RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SHDN_RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SHDN_RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYS_SHDN_RST` reader - Used to de-assert the SYS_SHDN# signal Register"]
pub type SYS_SHDN_RST_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Used to de-assert the SYS_SHDN# signal Register"]
    #[inline(always)]
    pub fn sys_shdn_rst(&self) -> SYS_SHDN_RST_R {
        SYS_SHDN_RST_R::new(self.bits)
    }
}
#[doc = "System Shutdown Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_shdn_rst](index.html) module"]
pub struct SYS_SHDN_RST_SPEC;
impl crate::RegisterSpec for SYS_SHDN_RST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sys_shdn_rst::R](R) reader structure"]
impl crate::Readable for SYS_SHDN_RST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYS_SHDN_RST to value 0"]
impl crate::Resettable for SYS_SHDN_RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
