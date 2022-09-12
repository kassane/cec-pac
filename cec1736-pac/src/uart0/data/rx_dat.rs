#[doc = "Register `RX_DAT` reader"]
pub struct R(crate::R<RX_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "UART Receive (Read) Buffer Register (DLAB=0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_dat](index.html) module"]
pub struct RX_DAT_SPEC;
impl crate::RegisterSpec for RX_DAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rx_dat::R](R) reader structure"]
impl crate::Readable for RX_DAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_DAT to value 0"]
impl crate::Resettable for RX_DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
