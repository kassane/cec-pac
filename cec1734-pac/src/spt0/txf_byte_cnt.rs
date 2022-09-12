#[doc = "Register `TXF_BYTE_CNT` reader"]
pub struct R(crate::R<TXF_BYTE_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXF_BYTE_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXF_BYTE_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXF_BYTE_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BCNT` reader - TX FIFO Byte Count Register."]
pub type BCNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - TX FIFO Byte Count Register."]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "SPI Peripheral Target TX FIFO Byte Counter Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txf_byte_cnt](index.html) module"]
pub struct TXF_BYTE_CNT_SPEC;
impl crate::RegisterSpec for TXF_BYTE_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txf_byte_cnt::R](R) reader structure"]
impl crate::Readable for TXF_BYTE_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXF_BYTE_CNT to value 0"]
impl crate::Resettable for TXF_BYTE_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
