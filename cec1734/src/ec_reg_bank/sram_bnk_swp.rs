#[doc = "Register `SRAM_BNK_SWP` reader"]
pub struct R(crate::R<SRAM_BNK_SWP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_BNK_SWP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_BNK_SWP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_BNK_SWP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_BNK_SWP` writer"]
pub struct W(crate::W<SRAM_BNK_SWP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_BNK_SWP_SPEC>;
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
impl From<crate::W<SRAM_BNK_SWP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_BNK_SWP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNK_SWP` reader - SRAM bank Swap Register"]
pub type BNK_SWP_R = crate::BitReader<bool>;
#[doc = "Field `BNK_SWP` writer - SRAM bank Swap Register"]
pub type BNK_SWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_BNK_SWP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SRAM bank Swap Register"]
    #[inline(always)]
    pub fn bnk_swp(&self) -> BNK_SWP_R {
        BNK_SWP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM bank Swap Register"]
    #[inline(always)]
    pub fn bnk_swp(&mut self) -> BNK_SWP_W<0> {
        BNK_SWP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Monitor SRAM Bank Swap Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_bnk_swp](index.html) module"]
pub struct SRAM_BNK_SWP_SPEC;
impl crate::RegisterSpec for SRAM_BNK_SWP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_bnk_swp::R](R) reader structure"]
impl crate::Readable for SRAM_BNK_SWP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_bnk_swp::W](W) writer structure"]
impl crate::Writable for SRAM_BNK_SWP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM_BNK_SWP to value 0"]
impl crate::Resettable for SRAM_BNK_SWP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
