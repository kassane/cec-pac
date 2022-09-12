#[doc = "Register `BLKREV` reader"]
pub struct R(crate::R<BLKREV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLKREV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLKREV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLKREV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REV` reader - Block Revision Number"]
pub type REV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Block Revision Number"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new(self.bits)
    }
}
#[doc = "Revision Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blkrev](index.html) module"]
pub struct BLKREV_SPEC;
impl crate::RegisterSpec for BLKREV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [blkrev::R](R) reader structure"]
impl crate::Readable for BLKREV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLKREV to value 0"]
impl crate::Resettable for BLKREV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
