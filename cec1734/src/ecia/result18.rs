#[doc = "Register `RESULT18` reader"]
pub struct R(crate::R<RESULT18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPT0` reader - SPT0"]
pub type SPT0_R = crate::BitReader<bool>;
#[doc = "Field `QMSPI0` reader - QMSPI0"]
pub type QMSPI0_R = crate::BitReader<bool>;
#[doc = "Field `QMSPI1` reader - QMSPI1"]
pub type QMSPI1_R = crate::BitReader<bool>;
#[doc = "Field `SPT1` reader - SPT1"]
pub type SPT1_R = crate::BitReader<bool>;
#[doc = "Field `CCT` reader - CCT"]
pub type CCT_R = crate::BitReader<bool>;
#[doc = "Field `CCT_CAP0` reader - CCT_CAP0"]
pub type CCT_CAP0_R = crate::BitReader<bool>;
#[doc = "Field `CCT_CAP1` reader - CCT_CAP1"]
pub type CCT_CAP1_R = crate::BitReader<bool>;
#[doc = "Field `CCT_CAP2` reader - CCT_CAP2"]
pub type CCT_CAP2_R = crate::BitReader<bool>;
#[doc = "Field `CCT_CAP3` reader - CCT_CAP3"]
pub type CCT_CAP3_R = crate::BitReader<bool>;
#[doc = "Field `CCT_CAP4` reader - CCT_CAP4"]
pub type CCT_CAP4_R = crate::BitReader<bool>;
#[doc = "Field `CCT_CAP5` reader - CCT_CAP5"]
pub type CCT_CAP5_R = crate::BitReader<bool>;
#[doc = "Field `CCT_CMP0` reader - CCT_CMP0"]
pub type CCT_CMP0_R = crate::BitReader<bool>;
#[doc = "Field `CCT_CMP1` reader - CCT_CMP1"]
pub type CCT_CMP1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SPT0"]
    #[inline(always)]
    pub fn spt0(&self) -> SPT0_R {
        SPT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QMSPI0"]
    #[inline(always)]
    pub fn qmspi0(&self) -> QMSPI0_R {
        QMSPI0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - QMSPI1"]
    #[inline(always)]
    pub fn qmspi1(&self) -> QMSPI1_R {
        QMSPI1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - SPT1"]
    #[inline(always)]
    pub fn spt1(&self) -> SPT1_R {
        SPT1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CCT"]
    #[inline(always)]
    pub fn cct(&self) -> CCT_R {
        CCT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CCT_CAP0"]
    #[inline(always)]
    pub fn cct_cap0(&self) -> CCT_CAP0_R {
        CCT_CAP0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CCT_CAP1"]
    #[inline(always)]
    pub fn cct_cap1(&self) -> CCT_CAP1_R {
        CCT_CAP1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CCT_CAP2"]
    #[inline(always)]
    pub fn cct_cap2(&self) -> CCT_CAP2_R {
        CCT_CAP2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CCT_CAP3"]
    #[inline(always)]
    pub fn cct_cap3(&self) -> CCT_CAP3_R {
        CCT_CAP3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CCT_CAP4"]
    #[inline(always)]
    pub fn cct_cap4(&self) -> CCT_CAP4_R {
        CCT_CAP4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CCT_CAP5"]
    #[inline(always)]
    pub fn cct_cap5(&self) -> CCT_CAP5_R {
        CCT_CAP5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CCT_CMP0"]
    #[inline(always)]
    pub fn cct_cmp0(&self) -> CCT_CMP0_R {
        CCT_CMP0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CCT_CMP1"]
    #[inline(always)]
    pub fn cct_cmp1(&self) -> CCT_CMP1_R {
        CCT_CMP1_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "GIRQ18 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result18](index.html) module"]
pub struct RESULT18_SPEC;
impl crate::RegisterSpec for RESULT18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result18::R](R) reader structure"]
impl crate::Readable for RESULT18_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT18 to value 0"]
impl crate::Resettable for RESULT18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
