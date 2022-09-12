#[doc = "Register `PD_MON_INT_EN` reader"]
pub struct R(crate::R<PD_MON_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_MON_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_MON_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_MON_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD_MON_INT_EN` writer"]
pub struct W(crate::W<PD_MON_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_MON_INT_EN_SPEC>;
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
impl From<crate::W<PD_MON_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_MON_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTR1_PD_INTEN` reader - Pad Monitor VTR1 Power Down Interrupt Enable."]
pub type VTR1_PD_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `VTR1_PD_INTEN` writer - Pad Monitor VTR1 Power Down Interrupt Enable."]
pub type VTR1_PD_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_INT_EN_SPEC, bool, O>;
#[doc = "Field `VTR1_PU_INTEN` reader - Pad Monitor VTR1 Power Up Interrupt Enable"]
pub type VTR1_PU_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `VTR1_PU_INTEN` writer - Pad Monitor VTR1 Power Up Interrupt Enable"]
pub type VTR1_PU_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_INT_EN_SPEC, bool, O>;
#[doc = "Field `VTR2_PD_INTEN` reader - Pad Monitor VTR2 Power Down Interrupt Enable."]
pub type VTR2_PD_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `VTR2_PD_INTEN` writer - Pad Monitor VTR2 Power Down Interrupt Enable."]
pub type VTR2_PD_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_INT_EN_SPEC, bool, O>;
#[doc = "Field `VTR2_PU_INTEN` reader - Pad Monitor VTR2 Power Up Interrupt Enable"]
pub type VTR2_PU_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `VTR2_PU_INTEN` writer - Pad Monitor VTR2 Power Up Interrupt Enable"]
pub type VTR2_PU_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pad Monitor VTR1 Power Down Interrupt Enable."]
    #[inline(always)]
    pub fn vtr1_pd_inten(&self) -> VTR1_PD_INTEN_R {
        VTR1_PD_INTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pad Monitor VTR1 Power Up Interrupt Enable"]
    #[inline(always)]
    pub fn vtr1_pu_inten(&self) -> VTR1_PU_INTEN_R {
        VTR1_PU_INTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Pad Monitor VTR2 Power Down Interrupt Enable."]
    #[inline(always)]
    pub fn vtr2_pd_inten(&self) -> VTR2_PD_INTEN_R {
        VTR2_PD_INTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pad Monitor VTR2 Power Up Interrupt Enable"]
    #[inline(always)]
    pub fn vtr2_pu_inten(&self) -> VTR2_PU_INTEN_R {
        VTR2_PU_INTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pad Monitor VTR1 Power Down Interrupt Enable."]
    #[inline(always)]
    pub fn vtr1_pd_inten(&mut self) -> VTR1_PD_INTEN_W<0> {
        VTR1_PD_INTEN_W::new(self)
    }
    #[doc = "Bit 1 - Pad Monitor VTR1 Power Up Interrupt Enable"]
    #[inline(always)]
    pub fn vtr1_pu_inten(&mut self) -> VTR1_PU_INTEN_W<1> {
        VTR1_PU_INTEN_W::new(self)
    }
    #[doc = "Bit 8 - Pad Monitor VTR2 Power Down Interrupt Enable."]
    #[inline(always)]
    pub fn vtr2_pd_inten(&mut self) -> VTR2_PD_INTEN_W<8> {
        VTR2_PD_INTEN_W::new(self)
    }
    #[doc = "Bit 9 - Pad Monitor VTR2 Power Up Interrupt Enable"]
    #[inline(always)]
    pub fn vtr2_pu_inten(&mut self) -> VTR2_PU_INTEN_W<9> {
        VTR2_PU_INTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAD Monitor Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_mon_int_en](index.html) module"]
pub struct PD_MON_INT_EN_SPEC;
impl crate::RegisterSpec for PD_MON_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_mon_int_en::R](R) reader structure"]
impl crate::Readable for PD_MON_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_mon_int_en::W](W) writer structure"]
impl crate::Writable for PD_MON_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD_MON_INT_EN to value 0"]
impl crate::Resettable for PD_MON_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
