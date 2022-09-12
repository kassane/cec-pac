#[doc = "Register `PD_MON_STS` reader"]
pub struct R(crate::R<PD_MON_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_MON_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_MON_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_MON_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD_MON_STS` writer"]
pub struct W(crate::W<PD_MON_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_MON_STS_SPEC>;
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
impl From<crate::W<PD_MON_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_MON_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTR1_PD_STS` reader - Pad Monitor VTR1 Power Down Status"]
pub type VTR1_PD_STS_R = crate::BitReader<bool>;
#[doc = "Field `VTR1_PD_STS` writer - Pad Monitor VTR1 Power Down Status"]
pub type VTR1_PD_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_STS_SPEC, bool, O>;
#[doc = "Field `VTR1_PU_STS` reader - Pad Monitor VTR1 Power Up Status"]
pub type VTR1_PU_STS_R = crate::BitReader<bool>;
#[doc = "Field `VTR1_PU_STS` writer - Pad Monitor VTR1 Power Up Status"]
pub type VTR1_PU_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_STS_SPEC, bool, O>;
#[doc = "Field `VTR2_PD_STS` reader - Pad Monitor VTR2 Power Down Status"]
pub type VTR2_PD_STS_R = crate::BitReader<bool>;
#[doc = "Field `VTR2_PD_STS` writer - Pad Monitor VTR2 Power Down Status"]
pub type VTR2_PD_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_STS_SPEC, bool, O>;
#[doc = "Field `VTR2_PU_STS` reader - Pad Monitor VTR2 Power Up Status"]
pub type VTR2_PU_STS_R = crate::BitReader<bool>;
#[doc = "Field `VTR2_PU_STS` writer - Pad Monitor VTR2 Power Up Status"]
pub type VTR2_PU_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pad Monitor VTR1 Power Down Status"]
    #[inline(always)]
    pub fn vtr1_pd_sts(&self) -> VTR1_PD_STS_R {
        VTR1_PD_STS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pad Monitor VTR1 Power Up Status"]
    #[inline(always)]
    pub fn vtr1_pu_sts(&self) -> VTR1_PU_STS_R {
        VTR1_PU_STS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Pad Monitor VTR2 Power Down Status"]
    #[inline(always)]
    pub fn vtr2_pd_sts(&self) -> VTR2_PD_STS_R {
        VTR2_PD_STS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pad Monitor VTR2 Power Up Status"]
    #[inline(always)]
    pub fn vtr2_pu_sts(&self) -> VTR2_PU_STS_R {
        VTR2_PU_STS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pad Monitor VTR1 Power Down Status"]
    #[inline(always)]
    pub fn vtr1_pd_sts(&mut self) -> VTR1_PD_STS_W<0> {
        VTR1_PD_STS_W::new(self)
    }
    #[doc = "Bit 1 - Pad Monitor VTR1 Power Up Status"]
    #[inline(always)]
    pub fn vtr1_pu_sts(&mut self) -> VTR1_PU_STS_W<1> {
        VTR1_PU_STS_W::new(self)
    }
    #[doc = "Bit 8 - Pad Monitor VTR2 Power Down Status"]
    #[inline(always)]
    pub fn vtr2_pd_sts(&mut self) -> VTR2_PD_STS_W<8> {
        VTR2_PD_STS_W::new(self)
    }
    #[doc = "Bit 9 - Pad Monitor VTR2 Power Up Status"]
    #[inline(always)]
    pub fn vtr2_pu_sts(&mut self) -> VTR2_PU_STS_W<9> {
        VTR2_PU_STS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAD Monitor Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_mon_sts](index.html) module"]
pub struct PD_MON_STS_SPEC;
impl crate::RegisterSpec for PD_MON_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_mon_sts::R](R) reader structure"]
impl crate::Readable for PD_MON_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_mon_sts::W](W) writer structure"]
impl crate::Writable for PD_MON_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD_MON_STS to value 0"]
impl crate::Resettable for PD_MON_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
