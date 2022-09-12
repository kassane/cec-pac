#[doc = "Register `EXT4_TEMP` reader"]
pub struct R(crate::R<EXT4_TEMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT4_TEMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT4_TEMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT4_TEMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIODE4_TEMP` reader - External Diode 4 Temp Byte Register"]
pub type DIODE4_TEMP_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - External Diode 4 Temp Byte Register"]
    #[inline(always)]
    pub fn diode4_temp(&self) -> DIODE4_TEMP_R {
        DIODE4_TEMP_R::new(self.bits)
    }
}
#[doc = "External Diode 4 Temp Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext4_temp](index.html) module"]
pub struct EXT4_TEMP_SPEC;
impl crate::RegisterSpec for EXT4_TEMP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ext4_temp::R](R) reader structure"]
impl crate::Readable for EXT4_TEMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXT4_TEMP to value 0"]
impl crate::Resettable for EXT4_TEMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
