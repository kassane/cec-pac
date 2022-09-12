#[doc = "Register `STAP_TMIR` reader"]
pub struct R(crate::R<STAP_TMIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAP_TMIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAP_TMIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAP_TMIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QA_MODE` reader - This is the mirror of the QA_MODE (bit 0) of Boot Control Register.\n This register bit tells BOOT ROM to enter the QA mode.\n"]
pub type QA_MODE_R = crate::BitReader<bool>;
#[doc = "Field `VLD_MODE` reader - This is the mirror of the Validation MODE (bit 1) of Boot Control Register.\n This register bit tells BOOT ROM to enter the Validation mode.\n"]
pub type VLD_MODE_R = crate::BitReader<bool>;
#[doc = "Field `BS_STATUS` reader - This register bit tells BOOT ROM about the Boundary Scan Status.\n"]
pub type BS_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `INT_SPI_RECOV` reader - This register bit tells BOOT ROM that SPI FLASH Recovery Mode is entered.\n"]
pub type INT_SPI_RECOV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This is the mirror of the QA_MODE (bit 0) of Boot Control Register.\n This register bit tells BOOT ROM to enter the QA mode.\n"]
    #[inline(always)]
    pub fn qa_mode(&self) -> QA_MODE_R {
        QA_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the mirror of the Validation MODE (bit 1) of Boot Control Register.\n This register bit tells BOOT ROM to enter the Validation mode.\n"]
    #[inline(always)]
    pub fn vld_mode(&self) -> VLD_MODE_R {
        VLD_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This register bit tells BOOT ROM about the Boundary Scan Status.\n"]
    #[inline(always)]
    pub fn bs_status(&self) -> BS_STATUS_R {
        BS_STATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This register bit tells BOOT ROM that SPI FLASH Recovery Mode is entered.\n"]
    #[inline(always)]
    pub fn int_spi_recov(&self) -> INT_SPI_RECOV_R {
        INT_SPI_RECOV_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "This register is a mirror of the Boot Control Register.\n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stap_tmir](index.html) module"]
pub struct STAP_TMIR_SPEC;
impl crate::RegisterSpec for STAP_TMIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stap_tmir::R](R) reader structure"]
impl crate::Readable for STAP_TMIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAP_TMIR to value 0"]
impl crate::Resettable for STAP_TMIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
