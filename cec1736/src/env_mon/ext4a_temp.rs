#[doc = "Register `EXT4A_TEMP` reader"]
pub struct R(crate::R<EXT4A_TEMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT4A_TEMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT4A_TEMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT4A_TEMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIODE4A_TEMP` reader - Stores the fractional and integerdata for External Diode 4A Register"]
pub type DIODE4A_TEMP_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Stores the fractional and integerdata for External Diode 4A Register"]
    #[inline(always)]
    pub fn diode4a_temp(&self) -> DIODE4A_TEMP_R {
        DIODE4A_TEMP_R::new(self.bits)
    }
}
#[doc = "Stores the fractional and integer data for External Diode 4A Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext4a_temp](index.html) module"]
pub struct EXT4A_TEMP_SPEC;
impl crate::RegisterSpec for EXT4A_TEMP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ext4a_temp::R](R) reader structure"]
impl crate::Readable for EXT4A_TEMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXT4A_TEMP to value 0"]
impl crate::Resettable for EXT4A_TEMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
