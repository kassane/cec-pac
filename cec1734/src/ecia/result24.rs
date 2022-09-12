#[doc = "Register `RESULT24` reader"]
pub struct R(crate::R<RESULT24_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT24_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT24_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT24_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPIMON0_VLTN` reader - SPIMON0_VLTN"]
pub type SPIMON0_VLTN_R = crate::BitReader<bool>;
#[doc = "Field `SPIMON0_MTMON` reader - SPIMON0_MTMON"]
pub type SPIMON0_MTMON_R = crate::BitReader<bool>;
#[doc = "Field `SPIMON0_LTMON` reader - SPIMON0_LTMON"]
pub type SPIMON0_LTMON_R = crate::BitReader<bool>;
#[doc = "Field `SPIMON1_VLTN` reader - SPIMON1_VLTN"]
pub type SPIMON1_VLTN_R = crate::BitReader<bool>;
#[doc = "Field `SPIMON1_MTMON` reader - SPIMON1_MTMON"]
pub type SPIMON1_MTMON_R = crate::BitReader<bool>;
#[doc = "Field `SPIMON1_LTMON` reader - SPIMON1_LTMON"]
pub type SPIMON1_LTMON_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SPIMON0_VLTN"]
    #[inline(always)]
    pub fn spimon0_vltn(&self) -> SPIMON0_VLTN_R {
        SPIMON0_VLTN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPIMON0_MTMON"]
    #[inline(always)]
    pub fn spimon0_mtmon(&self) -> SPIMON0_MTMON_R {
        SPIMON0_MTMON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPIMON0_LTMON"]
    #[inline(always)]
    pub fn spimon0_ltmon(&self) -> SPIMON0_LTMON_R {
        SPIMON0_LTMON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPIMON1_VLTN"]
    #[inline(always)]
    pub fn spimon1_vltn(&self) -> SPIMON1_VLTN_R {
        SPIMON1_VLTN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPIMON1_MTMON"]
    #[inline(always)]
    pub fn spimon1_mtmon(&self) -> SPIMON1_MTMON_R {
        SPIMON1_MTMON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPIMON1_LTMON"]
    #[inline(always)]
    pub fn spimon1_ltmon(&self) -> SPIMON1_LTMON_R {
        SPIMON1_LTMON_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "GIRQ24 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result24](index.html) module"]
pub struct RESULT24_SPEC;
impl crate::RegisterSpec for RESULT24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result24::R](R) reader structure"]
impl crate::Readable for RESULT24_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT24 to value 0"]
impl crate::Resettable for RESULT24_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
