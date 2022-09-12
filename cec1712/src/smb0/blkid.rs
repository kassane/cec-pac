#[doc = "Register `BLKID` reader"]
pub struct R(crate::R<BLKID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLKID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLKID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLKID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - Block ID."]
pub type ID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Block ID."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "Block ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blkid](index.html) module"]
pub struct BLKID_SPEC;
impl crate::RegisterSpec for BLKID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [blkid::R](R) reader structure"]
impl crate::Readable for BLKID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLKID to value 0x11"]
impl crate::Resettable for BLKID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x11
    }
}
