#[doc = "Register `VTT_VOLT` reader"]
pub struct R(crate::R<VTT_VOLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VTT_VOLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VTT_VOLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VTT_VOLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VTT_VOLT` reader - Stores the VTT Voltage Monitor data"]
pub type VTT_VOLT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Stores the VTT Voltage Monitor data"]
    #[inline(always)]
    pub fn vtt_volt(&self) -> VTT_VOLT_R {
        VTT_VOLT_R::new(self.bits)
    }
}
#[doc = "Stores the VTT Voltage Monitor data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtt_volt](index.html) module"]
pub struct VTT_VOLT_SPEC;
impl crate::RegisterSpec for VTT_VOLT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vtt_volt::R](R) reader structure"]
impl crate::Readable for VTT_VOLT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VTT_VOLT to value 0xff"]
impl crate::Resettable for VTT_VOLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
