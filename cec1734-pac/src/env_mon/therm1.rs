#[doc = "Register `THERM1` reader"]
pub struct R(crate::R<THERM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THERM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THERM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THERM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `THERM1` reader - Stores the calculated ThermTrip temperature high limit derived from the voltage on VSET and compared against External Diode 1."]
pub type THERM1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Stores the calculated ThermTrip temperature high limit derived from the voltage on VSET and compared against External Diode 1."]
    #[inline(always)]
    pub fn therm1(&self) -> THERM1_R {
        THERM1_R::new(self.bits)
    }
}
#[doc = "Thermal Trip Temperature Diode 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [therm1](index.html) module"]
pub struct THERM1_SPEC;
impl crate::RegisterSpec for THERM1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [therm1::R](R) reader structure"]
impl crate::Readable for THERM1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets THERM1 to value 0x7f"]
impl crate::Resettable for THERM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
