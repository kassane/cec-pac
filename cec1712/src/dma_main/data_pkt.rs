#[doc = "Register `DATA_PKT` reader"]
pub struct R(crate::R<DATA_PKT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_PKT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_PKT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_PKT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Debug register that has the data that is stored in the Data Packet. This data is read data from the currently active transfer source.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_pkt](index.html) module"]
pub struct DATA_PKT_SPEC;
impl crate::RegisterSpec for DATA_PKT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_pkt::R](R) reader structure"]
impl crate::Readable for DATA_PKT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATA_PKT to value 0"]
impl crate::Resettable for DATA_PKT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
