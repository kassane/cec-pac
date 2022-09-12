#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTS` reader - CTS Delta Clear To Send (DCTS)."]
pub type CTS_R = crate::BitReader<bool>;
#[doc = "Field `DSR` reader - DSR Delta Data Set Ready (DDSR)."]
pub type DSR_R = crate::BitReader<bool>;
#[doc = "Field `RI` reader - RI Trailing Edge of Ring Indicator (TERI)."]
pub type RI_R = crate::BitReader<bool>;
#[doc = "Field `DCD` reader - DCD Delta Data Carrier Detect (DDCD)."]
pub type DCD_R = crate::BitReader<bool>;
#[doc = "Field `nCTS` reader - nCTS This bit is the complement of the Clear To Send (nCTS) input."]
pub type N_CTS_R = crate::BitReader<bool>;
#[doc = "Field `nDSR` reader - This bit is the complement of the Data Set Ready (nDSR) input."]
pub type N_DSR_R = crate::BitReader<bool>;
#[doc = "Field `nRI` reader - nRI This bit is the complement of the Ring Indicator (nRI) input."]
pub type N_RI_R = crate::BitReader<bool>;
#[doc = "Field `nDCD` reader - nDCD This bit is the complement of the Data Carrier Detect (nDCD) input."]
pub type N_DCD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - CTS Delta Clear To Send (DCTS)."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSR Delta Data Set Ready (DDSR)."]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RI Trailing Edge of Ring Indicator (TERI)."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DCD Delta Data Carrier Detect (DDCD)."]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - nCTS This bit is the complement of the Clear To Send (nCTS) input."]
    #[inline(always)]
    pub fn n_cts(&self) -> N_CTS_R {
        N_CTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is the complement of the Data Set Ready (nDSR) input."]
    #[inline(always)]
    pub fn n_dsr(&self) -> N_DSR_R {
        N_DSR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - nRI This bit is the complement of the Ring Indicator (nRI) input."]
    #[inline(always)]
    pub fn n_ri(&self) -> N_RI_R {
        N_RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - nDCD This bit is the complement of the Data Carrier Detect (nDCD) input."]
    #[inline(always)]
    pub fn n_dcd(&self) -> N_DCD_R {
        N_DCD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART Modem Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module"]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
