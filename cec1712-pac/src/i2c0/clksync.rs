#[doc = "Register `CLKSYNC` reader"]
pub struct R(crate::R<CLKSYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLK_SYNC` reader - This register must not be written, or undesirable results may occur."]
pub type CLK_SYNC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - This register must not be written, or undesirable results may occur."]
    #[inline(always)]
    pub fn clk_sync(&self) -> CLK_SYNC_R {
        CLK_SYNC_R::new(self.bits)
    }
}
#[doc = "This is Clock Sync Register. This register must not be written, or undesirable results may occur.\n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksync](index.html) module"]
pub struct CLKSYNC_SPEC;
impl crate::RegisterSpec for CLKSYNC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clksync::R](R) reader structure"]
impl crate::Readable for CLKSYNC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLKSYNC to value 0"]
impl crate::Resettable for CLKSYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
