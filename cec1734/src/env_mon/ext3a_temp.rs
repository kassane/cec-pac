#[doc = "Register `EXT3A_TEMP` reader"]
pub struct R(crate::R<EXT3A_TEMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT3A_TEMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT3A_TEMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT3A_TEMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIODE3A_TEMP` reader - Stores the fractional and integer data for External Diode 3A Register"]
pub type DIODE3A_TEMP_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Stores the fractional and integer data for External Diode 3A Register"]
    #[inline(always)]
    pub fn diode3a_temp(&self) -> DIODE3A_TEMP_R {
        DIODE3A_TEMP_R::new(self.bits)
    }
}
#[doc = "Stores the fractional and integer data for External Diode 3A Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext3a_temp](index.html) module"]
pub struct EXT3A_TEMP_SPEC;
impl crate::RegisterSpec for EXT3A_TEMP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ext3a_temp::R](R) reader structure"]
impl crate::Readable for EXT3A_TEMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXT3A_TEMP to value 0"]
impl crate::Resettable for EXT3A_TEMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
