#[doc = "Register `LTMON_AGGR` reader"]
pub struct R(crate::R<LTMON_AGGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTMON_AGGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTMON_AGGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTMON_AGGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTMON_AGGR` writer"]
pub struct W(crate::W<LTMON_AGGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTMON_AGGR_SPEC>;
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
impl From<crate::W<LTMON_AGGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTMON_AGGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ0` reader - Load 0 Interrupt"]
pub type IRQ0_R = crate::BitReader<bool>;
#[doc = "Field `IRQ0` writer - Load 0 Interrupt"]
pub type IRQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `IRQ1` reader - Load 1 Interrupt"]
pub type IRQ1_R = crate::BitReader<bool>;
#[doc = "Field `IRQ1` writer - Load 1 Interrupt"]
pub type IRQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `IRQ2` reader - Load 2 Interrupt"]
pub type IRQ2_R = crate::BitReader<bool>;
#[doc = "Field `IRQ2` writer - Load 2 Interrupt"]
pub type IRQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `IRQ3` reader - Load 3 Interrupt"]
pub type IRQ3_R = crate::BitReader<bool>;
#[doc = "Field `IRQ3` writer - Load 3 Interrupt"]
pub type IRQ3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `IRQ4` reader - Load 4 Interrupt"]
pub type IRQ4_R = crate::BitReader<bool>;
#[doc = "Field `IRQ4` writer - Load 4 Interrupt"]
pub type IRQ4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `IRQ5` reader - Load 5 Interrupt"]
pub type IRQ5_R = crate::BitReader<bool>;
#[doc = "Field `IRQ5` writer - Load 5 Interrupt"]
pub type IRQ5_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `IRQ6` reader - Load 6 Interrupt"]
pub type IRQ6_R = crate::BitReader<bool>;
#[doc = "Field `IRQ6` writer - Load 6 Interrupt"]
pub type IRQ6_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `IRQ7` reader - Load 7 Interrupt"]
pub type IRQ7_R = crate::BitReader<bool>;
#[doc = "Field `IRQ7` writer - Load 7 Interrupt"]
pub type IRQ7_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `EN_IRQ0` reader - Enable Load 0 Interrupt"]
pub type EN_IRQ0_R = crate::BitReader<bool>;
#[doc = "Field `EN_IRQ0` writer - Enable Load 0 Interrupt"]
pub type EN_IRQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `EN_IRQ1` reader - Enable Load 1 Interrupt"]
pub type EN_IRQ1_R = crate::BitReader<bool>;
#[doc = "Field `EN_IRQ1` writer - Enable Load 1 Interrupt"]
pub type EN_IRQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `EN_IRQ2` reader - Enable Load 2 Interrupt"]
pub type EN_IRQ2_R = crate::BitReader<bool>;
#[doc = "Field `EN_IRQ2` writer - Enable Load 2 Interrupt"]
pub type EN_IRQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `EN_IRQ3` reader - Enable Load 3 Interrupt"]
pub type EN_IRQ3_R = crate::BitReader<bool>;
#[doc = "Field `EN_IRQ3` writer - Enable Load 3 Interrupt"]
pub type EN_IRQ3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `EN_IRQ4` reader - Enable Load 4 Interrupt"]
pub type EN_IRQ4_R = crate::BitReader<bool>;
#[doc = "Field `EN_IRQ4` writer - Enable Load 4 Interrupt"]
pub type EN_IRQ4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `EN_IRQ5` reader - Enable Load 5 Interrupt"]
pub type EN_IRQ5_R = crate::BitReader<bool>;
#[doc = "Field `EN_IRQ5` writer - Enable Load 5 Interrupt"]
pub type EN_IRQ5_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `EN_IRQ6` reader - Enable Load 6 Interrupt"]
pub type EN_IRQ6_R = crate::BitReader<bool>;
#[doc = "Field `EN_IRQ6` writer - Enable Load 6 Interrupt"]
pub type EN_IRQ6_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
#[doc = "Field `EN_IRQ7` reader - Enable Load 7 Interrupt"]
pub type EN_IRQ7_R = crate::BitReader<bool>;
#[doc = "Field `EN_IRQ7` writer - Enable Load 7 Interrupt"]
pub type EN_IRQ7_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_AGGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Load 0 Interrupt"]
    #[inline(always)]
    pub fn irq0(&self) -> IRQ0_R {
        IRQ0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load 1 Interrupt"]
    #[inline(always)]
    pub fn irq1(&self) -> IRQ1_R {
        IRQ1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Load 2 Interrupt"]
    #[inline(always)]
    pub fn irq2(&self) -> IRQ2_R {
        IRQ2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Load 3 Interrupt"]
    #[inline(always)]
    pub fn irq3(&self) -> IRQ3_R {
        IRQ3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Load 4 Interrupt"]
    #[inline(always)]
    pub fn irq4(&self) -> IRQ4_R {
        IRQ4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Load 5 Interrupt"]
    #[inline(always)]
    pub fn irq5(&self) -> IRQ5_R {
        IRQ5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Load 6 Interrupt"]
    #[inline(always)]
    pub fn irq6(&self) -> IRQ6_R {
        IRQ6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Load 7 Interrupt"]
    #[inline(always)]
    pub fn irq7(&self) -> IRQ7_R {
        IRQ7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Load 0 Interrupt"]
    #[inline(always)]
    pub fn en_irq0(&self) -> EN_IRQ0_R {
        EN_IRQ0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Load 1 Interrupt"]
    #[inline(always)]
    pub fn en_irq1(&self) -> EN_IRQ1_R {
        EN_IRQ1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Load 2 Interrupt"]
    #[inline(always)]
    pub fn en_irq2(&self) -> EN_IRQ2_R {
        EN_IRQ2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Load 3 Interrupt"]
    #[inline(always)]
    pub fn en_irq3(&self) -> EN_IRQ3_R {
        EN_IRQ3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Load 4 Interrupt"]
    #[inline(always)]
    pub fn en_irq4(&self) -> EN_IRQ4_R {
        EN_IRQ4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Load 5 Interrupt"]
    #[inline(always)]
    pub fn en_irq5(&self) -> EN_IRQ5_R {
        EN_IRQ5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Load 6 Interrupt"]
    #[inline(always)]
    pub fn en_irq6(&self) -> EN_IRQ6_R {
        EN_IRQ6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Load 7 Interrupt"]
    #[inline(always)]
    pub fn en_irq7(&self) -> EN_IRQ7_R {
        EN_IRQ7_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Load 0 Interrupt"]
    #[inline(always)]
    pub fn irq0(&mut self) -> IRQ0_W<0> {
        IRQ0_W::new(self)
    }
    #[doc = "Bit 1 - Load 1 Interrupt"]
    #[inline(always)]
    pub fn irq1(&mut self) -> IRQ1_W<1> {
        IRQ1_W::new(self)
    }
    #[doc = "Bit 2 - Load 2 Interrupt"]
    #[inline(always)]
    pub fn irq2(&mut self) -> IRQ2_W<2> {
        IRQ2_W::new(self)
    }
    #[doc = "Bit 3 - Load 3 Interrupt"]
    #[inline(always)]
    pub fn irq3(&mut self) -> IRQ3_W<3> {
        IRQ3_W::new(self)
    }
    #[doc = "Bit 4 - Load 4 Interrupt"]
    #[inline(always)]
    pub fn irq4(&mut self) -> IRQ4_W<4> {
        IRQ4_W::new(self)
    }
    #[doc = "Bit 5 - Load 5 Interrupt"]
    #[inline(always)]
    pub fn irq5(&mut self) -> IRQ5_W<5> {
        IRQ5_W::new(self)
    }
    #[doc = "Bit 6 - Load 6 Interrupt"]
    #[inline(always)]
    pub fn irq6(&mut self) -> IRQ6_W<6> {
        IRQ6_W::new(self)
    }
    #[doc = "Bit 7 - Load 7 Interrupt"]
    #[inline(always)]
    pub fn irq7(&mut self) -> IRQ7_W<7> {
        IRQ7_W::new(self)
    }
    #[doc = "Bit 8 - Enable Load 0 Interrupt"]
    #[inline(always)]
    pub fn en_irq0(&mut self) -> EN_IRQ0_W<8> {
        EN_IRQ0_W::new(self)
    }
    #[doc = "Bit 9 - Enable Load 1 Interrupt"]
    #[inline(always)]
    pub fn en_irq1(&mut self) -> EN_IRQ1_W<9> {
        EN_IRQ1_W::new(self)
    }
    #[doc = "Bit 10 - Enable Load 2 Interrupt"]
    #[inline(always)]
    pub fn en_irq2(&mut self) -> EN_IRQ2_W<10> {
        EN_IRQ2_W::new(self)
    }
    #[doc = "Bit 11 - Enable Load 3 Interrupt"]
    #[inline(always)]
    pub fn en_irq3(&mut self) -> EN_IRQ3_W<11> {
        EN_IRQ3_W::new(self)
    }
    #[doc = "Bit 12 - Enable Load 4 Interrupt"]
    #[inline(always)]
    pub fn en_irq4(&mut self) -> EN_IRQ4_W<12> {
        EN_IRQ4_W::new(self)
    }
    #[doc = "Bit 13 - Enable Load 5 Interrupt"]
    #[inline(always)]
    pub fn en_irq5(&mut self) -> EN_IRQ5_W<13> {
        EN_IRQ5_W::new(self)
    }
    #[doc = "Bit 14 - Enable Load 6 Interrupt"]
    #[inline(always)]
    pub fn en_irq6(&mut self) -> EN_IRQ6_W<14> {
        EN_IRQ6_W::new(self)
    }
    #[doc = "Bit 15 - Enable Load 7 Interrupt"]
    #[inline(always)]
    pub fn en_irq7(&mut self) -> EN_IRQ7_W<15> {
        EN_IRQ7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Loadtime (Hash) IRQ Aggregation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltmon_aggr](index.html) module"]
pub struct LTMON_AGGR_SPEC;
impl crate::RegisterSpec for LTMON_AGGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltmon_aggr::R](R) reader structure"]
impl crate::Readable for LTMON_AGGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltmon_aggr::W](W) writer structure"]
impl crate::Writable for LTMON_AGGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTMON_AGGR to value 0"]
impl crate::Resettable for LTMON_AGGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
