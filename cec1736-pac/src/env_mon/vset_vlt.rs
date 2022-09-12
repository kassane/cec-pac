#[doc = "Register `VSET_VLT` reader"]
pub struct R(crate::R<VSET_VLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VSET_VLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VSET_VLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VSET_VLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VSET_VLT` reader - Stores the VSET Voltage Monitor reading"]
pub type VSET_VLT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Stores the VSET Voltage Monitor reading"]
    #[inline(always)]
    pub fn vset_vlt(&self) -> VSET_VLT_R {
        VSET_VLT_R::new(self.bits)
    }
}
#[doc = "VSET Voltage Reading Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vset_vlt](index.html) module"]
pub struct VSET_VLT_SPEC;
impl crate::RegisterSpec for VSET_VLT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vset_vlt::R](R) reader structure"]
impl crate::Readable for VSET_VLT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VSET_VLT to value 0xff"]
impl crate::Resettable for VSET_VLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
