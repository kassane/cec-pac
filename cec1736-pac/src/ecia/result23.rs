#[doc = "Register `RESULT23` reader"]
pub struct R(crate::R<RESULT23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER32_0` reader - TIMER32_0"]
pub type TIMER32_0_R = crate::BitReader<bool>;
#[doc = "Field `TIMER32_1` reader - TIMER32_1"]
pub type TIMER32_1_R = crate::BitReader<bool>;
#[doc = "Field `RTMR` reader - RTMR"]
pub type RTMR_R = crate::BitReader<bool>;
#[doc = "Field `SWI0` reader - SWI0"]
pub type SWI0_R = crate::BitReader<bool>;
#[doc = "Field `SWI1` reader - SWI1"]
pub type SWI1_R = crate::BitReader<bool>;
#[doc = "Field `SWI2` reader - SWI2"]
pub type SWI2_R = crate::BitReader<bool>;
#[doc = "Field `SWI3` reader - SWI3"]
pub type SWI3_R = crate::BitReader<bool>;
#[doc = "Field `HTMR0` reader - HTMR0"]
pub type HTMR0_R = crate::BitReader<bool>;
#[doc = "Field `HTMR1` reader - HTMR1"]
pub type HTMR1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 4 - TIMER32_0"]
    #[inline(always)]
    pub fn timer32_0(&self) -> TIMER32_0_R {
        TIMER32_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIMER32_1"]
    #[inline(always)]
    pub fn timer32_1(&self) -> TIMER32_1_R {
        TIMER32_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - RTMR"]
    #[inline(always)]
    pub fn rtmr(&self) -> RTMR_R {
        RTMR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SWI0"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SWI1"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SWI2"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SWI3"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - HTMR0"]
    #[inline(always)]
    pub fn htmr0(&self) -> HTMR0_R {
        HTMR0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HTMR1"]
    #[inline(always)]
    pub fn htmr1(&self) -> HTMR1_R {
        HTMR1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "GIRQ23 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result23](index.html) module"]
pub struct RESULT23_SPEC;
impl crate::RegisterSpec for RESULT23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result23::R](R) reader structure"]
impl crate::Readable for RESULT23_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT23 to value 0"]
impl crate::Resettable for RESULT23_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
