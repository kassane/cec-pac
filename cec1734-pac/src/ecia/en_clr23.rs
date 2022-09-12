#[doc = "Register `EN_CLR23` reader"]
pub struct R(crate::R<EN_CLR23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_CLR23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_CLR23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_CLR23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN_CLR23` writer"]
pub struct W(crate::W<EN_CLR23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_CLR23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EN_CLR23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_CLR23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER32_0` reader - TIMER32_0"]
pub type TIMER32_0_R = crate::BitReader<bool>;
#[doc = "Field `TIMER32_0` writer - TIMER32_0"]
pub type TIMER32_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR23_SPEC, bool, O>;
#[doc = "Field `TIMER32_1` reader - TIMER32_1"]
pub type TIMER32_1_R = crate::BitReader<bool>;
#[doc = "Field `TIMER32_1` writer - TIMER32_1"]
pub type TIMER32_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR23_SPEC, bool, O>;
#[doc = "Field `RTMR` reader - RTMR"]
pub type RTMR_R = crate::BitReader<bool>;
#[doc = "Field `RTMR` writer - RTMR"]
pub type RTMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR23_SPEC, bool, O>;
#[doc = "Field `SWI0` reader - SWI0"]
pub type SWI0_R = crate::BitReader<bool>;
#[doc = "Field `SWI0` writer - SWI0"]
pub type SWI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR23_SPEC, bool, O>;
#[doc = "Field `SWI1` reader - SWI1"]
pub type SWI1_R = crate::BitReader<bool>;
#[doc = "Field `SWI1` writer - SWI1"]
pub type SWI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR23_SPEC, bool, O>;
#[doc = "Field `SWI2` reader - SWI2"]
pub type SWI2_R = crate::BitReader<bool>;
#[doc = "Field `SWI2` writer - SWI2"]
pub type SWI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR23_SPEC, bool, O>;
#[doc = "Field `SWI3` reader - SWI3"]
pub type SWI3_R = crate::BitReader<bool>;
#[doc = "Field `SWI3` writer - SWI3"]
pub type SWI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR23_SPEC, bool, O>;
#[doc = "Field `HTMR0` reader - HTMR0"]
pub type HTMR0_R = crate::BitReader<bool>;
#[doc = "Field `HTMR0` writer - HTMR0"]
pub type HTMR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR23_SPEC, bool, O>;
#[doc = "Field `HTMR1` reader - HTMR1"]
pub type HTMR1_R = crate::BitReader<bool>;
#[doc = "Field `HTMR1` writer - HTMR1"]
pub type HTMR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR23_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 4 - TIMER32_0"]
    #[inline(always)]
    pub fn timer32_0(&mut self) -> TIMER32_0_W<4> {
        TIMER32_0_W::new(self)
    }
    #[doc = "Bit 5 - TIMER32_1"]
    #[inline(always)]
    pub fn timer32_1(&mut self) -> TIMER32_1_W<5> {
        TIMER32_1_W::new(self)
    }
    #[doc = "Bit 10 - RTMR"]
    #[inline(always)]
    pub fn rtmr(&mut self) -> RTMR_W<10> {
        RTMR_W::new(self)
    }
    #[doc = "Bit 11 - SWI0"]
    #[inline(always)]
    pub fn swi0(&mut self) -> SWI0_W<11> {
        SWI0_W::new(self)
    }
    #[doc = "Bit 12 - SWI1"]
    #[inline(always)]
    pub fn swi1(&mut self) -> SWI1_W<12> {
        SWI1_W::new(self)
    }
    #[doc = "Bit 13 - SWI2"]
    #[inline(always)]
    pub fn swi2(&mut self) -> SWI2_W<13> {
        SWI2_W::new(self)
    }
    #[doc = "Bit 14 - SWI3"]
    #[inline(always)]
    pub fn swi3(&mut self) -> SWI3_W<14> {
        SWI3_W::new(self)
    }
    #[doc = "Bit 16 - HTMR0"]
    #[inline(always)]
    pub fn htmr0(&mut self) -> HTMR0_W<16> {
        HTMR0_W::new(self)
    }
    #[doc = "Bit 17 - HTMR1"]
    #[inline(always)]
    pub fn htmr1(&mut self) -> HTMR1_W<17> {
        HTMR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ23 ENABLE CLEAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_clr23](index.html) module"]
pub struct EN_CLR23_SPEC;
impl crate::RegisterSpec for EN_CLR23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en_clr23::R](R) reader structure"]
impl crate::Readable for EN_CLR23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en_clr23::W](W) writer structure"]
impl crate::Writable for EN_CLR23_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN_CLR23 to value 0"]
impl crate::Resettable for EN_CLR23_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
