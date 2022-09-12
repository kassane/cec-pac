#[doc = "Register `VCP_VOLT` reader"]
pub struct R(crate::R<VCP_VOLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCP_VOLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCP_VOLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCP_VOLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VCP_VOLT` reader - Stores the VCP Voltage Monitor data"]
pub type VCP_VOLT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Stores the VCP Voltage Monitor data"]
    #[inline(always)]
    pub fn vcp_volt(&self) -> VCP_VOLT_R {
        VCP_VOLT_R::new(self.bits)
    }
}
#[doc = "Stores the VCP Voltage Monitor data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vcp_volt](index.html) module"]
pub struct VCP_VOLT_SPEC;
impl crate::RegisterSpec for VCP_VOLT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vcp_volt::R](R) reader structure"]
impl crate::Readable for VCP_VOLT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VCP_VOLT to value 0xff"]
impl crate::Resettable for VCP_VOLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
