#[doc = "Register `RXF_HOST_BAR` reader"]
pub struct R(crate::R<RXF_HOST_BAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF_HOST_BAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF_HOST_BAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF_HOST_BAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BAR` reader - RX FIFO Host Bar Register."]
pub type BAR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - RX FIFO Host Bar Register."]
    #[inline(always)]
    pub fn bar(&self) -> BAR_R {
        BAR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "SPI Peripheral Target RX FIFO Host Bar Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf_host_bar](index.html) module"]
pub struct RXF_HOST_BAR_SPEC;
impl crate::RegisterSpec for RXF_HOST_BAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf_host_bar::R](R) reader structure"]
impl crate::Readable for RXF_HOST_BAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXF_HOST_BAR to value 0"]
impl crate::Resettable for RXF_HOST_BAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
