#[doc = "Register `VIN_VOLT` reader"]
pub struct R(crate::R<VIN_VOLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VIN_VOLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VIN_VOLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VIN_VOLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VIN_VOLT` reader - Stores the voltage Measured on VIN channel"]
pub type VIN_VOLT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Stores the voltage Measured on VIN channel"]
    #[inline(always)]
    pub fn vin_volt(&self) -> VIN_VOLT_R {
        VIN_VOLT_R::new(self.bits)
    }
}
#[doc = "Stores the voltage Measured on VIN channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vin_volt](index.html) module"]
pub struct VIN_VOLT_SPEC;
impl crate::RegisterSpec for VIN_VOLT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vin_volt::R](R) reader structure"]
impl crate::Readable for VIN_VOLT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VIN_VOLT to value 0xff"]
impl crate::Resettable for VIN_VOLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
