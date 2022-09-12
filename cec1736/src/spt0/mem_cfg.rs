#[doc = "Register `MEM_CFG` reader"]
pub struct R(crate::R<MEM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_CFG` writer"]
pub struct W(crate::W<MEM_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_CFG_SPEC>;
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
impl From<crate::W<MEM_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAR_EN0_SEL` reader - Enables Region 0 operation. 0 = Disable Region 0. 1 = Enable Region 0."]
pub type BAR_EN0_SEL_R = crate::BitReader<bool>;
#[doc = "Field `BAR_EN0_SEL` writer - Enables Region 0 operation. 0 = Disable Region 0. 1 = Enable Region 0."]
pub type BAR_EN0_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEM_CFG_SPEC, bool, O>;
#[doc = "Field `BAR_EN1_SEL` reader - Enables Region 1 operation. 0 = Disable Region 1. 1 = Enable Region 1."]
pub type BAR_EN1_SEL_R = crate::BitReader<bool>;
#[doc = "Field `BAR_EN1_SEL` writer - Enables Region 1 operation. 0 = Disable Region 1. 1 = Enable Region 1."]
pub type BAR_EN1_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEM_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables Region 0 operation. 0 = Disable Region 0. 1 = Enable Region 0."]
    #[inline(always)]
    pub fn bar_en0_sel(&self) -> BAR_EN0_SEL_R {
        BAR_EN0_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables Region 1 operation. 0 = Disable Region 1. 1 = Enable Region 1."]
    #[inline(always)]
    pub fn bar_en1_sel(&self) -> BAR_EN1_SEL_R {
        BAR_EN1_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables Region 0 operation. 0 = Disable Region 0. 1 = Enable Region 0."]
    #[inline(always)]
    pub fn bar_en0_sel(&mut self) -> BAR_EN0_SEL_W<0> {
        BAR_EN0_SEL_W::new(self)
    }
    #[doc = "Bit 1 - Enables Region 1 operation. 0 = Disable Region 1. 1 = Enable Region 1."]
    #[inline(always)]
    pub fn bar_en1_sel(&mut self) -> BAR_EN1_SEL_W<1> {
        BAR_EN1_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Peripheral Target Memory Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_cfg](index.html) module"]
pub struct MEM_CFG_SPEC;
impl crate::RegisterSpec for MEM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_cfg::R](R) reader structure"]
impl crate::Readable for MEM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_cfg::W](W) writer structure"]
impl crate::Writable for MEM_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_CFG to value 0"]
impl crate::Resettable for MEM_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
