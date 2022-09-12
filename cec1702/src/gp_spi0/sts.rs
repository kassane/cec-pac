#[doc = "Register `STS` reader"]
pub struct R(crate::R<STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXBE` reader - 1=TX_Data buffer is empty, 0=TX_Data buffer is not empty"]
pub type TXBE_R = crate::BitReader<bool>;
#[doc = "Field `RXBF` reader - 1=RX_Data buffer is full, 0=RX_Data buffer is not full"]
pub type RXBF_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE` reader - ACTIVE status"]
pub type ACTIVE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - 1=TX_Data buffer is empty, 0=TX_Data buffer is not empty"]
    #[inline(always)]
    pub fn txbe(&self) -> TXBE_R {
        TXBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1=RX_Data buffer is full, 0=RX_Data buffer is not full"]
    #[inline(always)]
    pub fn rxbf(&self) -> RXBF_R {
        RXBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACTIVE status"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "SPI Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts::R](R) reader structure"]
impl crate::Readable for STS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
