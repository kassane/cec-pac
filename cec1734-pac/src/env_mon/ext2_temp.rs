#[doc = "Register `EXT2_TEMP` reader"]
pub struct R(crate::R<EXT2_TEMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT2_TEMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT2_TEMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT2_TEMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIODE2_TEMP` reader - External Diode 2 Temp Byte Register"]
pub type DIODE2_TEMP_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - External Diode 2 Temp Byte Register"]
    #[inline(always)]
    pub fn diode2_temp(&self) -> DIODE2_TEMP_R {
        DIODE2_TEMP_R::new(self.bits)
    }
}
#[doc = "External Diode 2 Temp Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext2_temp](index.html) module"]
pub struct EXT2_TEMP_SPEC;
impl crate::RegisterSpec for EXT2_TEMP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ext2_temp::R](R) reader structure"]
impl crate::Readable for EXT2_TEMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXT2_TEMP to value 0"]
impl crate::Resettable for EXT2_TEMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
