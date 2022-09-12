#[doc = "Register `VIOCTRLSTS` reader"]
pub struct R(crate::R<VIOCTRLSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VIOCTRLSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VIOCTRLSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VIOCTRLSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VIOCTRLSTS` writer"]
pub struct W(crate::W<VIOCTRLSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VIOCTRLSTS_SPEC>;
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
impl From<crate::W<VIOCTRLSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VIOCTRLSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OP` reader - Opcode Violation"]
pub type OP_R = crate::BitReader<bool>;
#[doc = "Field `OP` writer - Opcode Violation"]
pub type OP_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIOCTRLSTS_SPEC, bool, O>;
#[doc = "Field `MC` reader - Data Mismatch Violation"]
pub type MC_R = crate::BitReader<bool>;
#[doc = "Field `MC` writer - Data Mismatch Violation"]
pub type MC_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIOCTRLSTS_SPEC, bool, O>;
#[doc = "Field `OB` reader - Out of Bounds. Outside all Runtime Regions"]
pub type OB_R = crate::BitReader<bool>;
#[doc = "Field `OB` writer - Out of Bounds. Outside all Runtime Regions"]
pub type OB_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIOCTRLSTS_SPEC, bool, O>;
#[doc = "Field `RG` reader - Runtime Region R/W Permission Violation"]
pub type RG_R = crate::BitReader<bool>;
#[doc = "Field `RG` writer - Runtime Region R/W Permission Violation"]
pub type RG_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIOCTRLSTS_SPEC, bool, O>;
#[doc = "Field `MT` reader - Timeout in Match Phase"]
pub type MT_R = crate::BitReader<bool>;
#[doc = "Field `MT` writer - Timeout in Match Phase"]
pub type MT_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIOCTRLSTS_SPEC, bool, O>;
#[doc = "Field `AW` reader - Address Wrap within a Flash device."]
pub type AW_R = crate::BitReader<bool>;
#[doc = "Field `AW` writer - Address Wrap within a Flash device."]
pub type AW_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIOCTRLSTS_SPEC, bool, O>;
#[doc = "Field `EOP` reader - Enable Opcode Violation Interrupt. 0 = Disable, 1 = Enable"]
pub type EOP_R = crate::BitReader<bool>;
#[doc = "Field `EOP` writer - Enable Opcode Violation Interrupt. 0 = Disable, 1 = Enable"]
pub type EOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIOCTRLSTS_SPEC, bool, O>;
#[doc = "Field `EMC` reader - Enable Data Mismatch Violation Interrupt. 0 = Disable, 1 = Enable"]
pub type EMC_R = crate::BitReader<bool>;
#[doc = "Field `EMC` writer - Enable Data Mismatch Violation Interrupt. 0 = Disable, 1 = Enable"]
pub type EMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIOCTRLSTS_SPEC, bool, O>;
#[doc = "Field `EOB` reader - Enable Out of Bounds Interrupt. 0 = Disable, 1 = Enable"]
pub type EOB_R = crate::BitReader<bool>;
#[doc = "Field `EOB` writer - Enable Out of Bounds Interrupt. 0 = Disable, 1 = Enable"]
pub type EOB_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIOCTRLSTS_SPEC, bool, O>;
#[doc = "Field `ERG` reader - Enable Runtime Region R/W Permission Violation Interrupt. 0 = Disable, 1 = Enable"]
pub type ERG_R = crate::BitReader<bool>;
#[doc = "Field `ERG` writer - Enable Runtime Region R/W Permission Violation Interrupt. 0 = Disable, 1 = Enable"]
pub type ERG_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIOCTRLSTS_SPEC, bool, O>;
#[doc = "Field `EMT` reader - Enable Timeout in Match Phase Interrupt. 0 = Disable, 1 = Enable"]
pub type EMT_R = crate::BitReader<bool>;
#[doc = "Field `EMT` writer - Enable Timeout in Match Phase Interrupt. 0 = Disable, 1 = Enable"]
pub type EMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIOCTRLSTS_SPEC, bool, O>;
#[doc = "Field `EAW` reader - Enable Address Wrap within a Flash device Interrupt. 0 = Disable, 1 = Enable"]
pub type EAW_R = crate::BitReader<bool>;
#[doc = "Field `EAW` writer - Enable Address Wrap within a Flash device Interrupt. 0 = Disable, 1 = Enable"]
pub type EAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIOCTRLSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Opcode Violation"]
    #[inline(always)]
    pub fn op(&self) -> OP_R {
        OP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Mismatch Violation"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Out of Bounds. Outside all Runtime Regions"]
    #[inline(always)]
    pub fn ob(&self) -> OB_R {
        OB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Runtime Region R/W Permission Violation"]
    #[inline(always)]
    pub fn rg(&self) -> RG_R {
        RG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timeout in Match Phase"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Address Wrap within a Flash device."]
    #[inline(always)]
    pub fn aw(&self) -> AW_R {
        AW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Opcode Violation Interrupt. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Data Mismatch Violation Interrupt. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn emc(&self) -> EMC_R {
        EMC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Out of Bounds Interrupt. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn eob(&self) -> EOB_R {
        EOB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Runtime Region R/W Permission Violation Interrupt. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn erg(&self) -> ERG_R {
        ERG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Timeout in Match Phase Interrupt. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn emt(&self) -> EMT_R {
        EMT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Address Wrap within a Flash device Interrupt. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn eaw(&self) -> EAW_R {
        EAW_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Opcode Violation"]
    #[inline(always)]
    pub fn op(&mut self) -> OP_W<0> {
        OP_W::new(self)
    }
    #[doc = "Bit 1 - Data Mismatch Violation"]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W<1> {
        MC_W::new(self)
    }
    #[doc = "Bit 2 - Out of Bounds. Outside all Runtime Regions"]
    #[inline(always)]
    pub fn ob(&mut self) -> OB_W<2> {
        OB_W::new(self)
    }
    #[doc = "Bit 3 - Runtime Region R/W Permission Violation"]
    #[inline(always)]
    pub fn rg(&mut self) -> RG_W<3> {
        RG_W::new(self)
    }
    #[doc = "Bit 4 - Timeout in Match Phase"]
    #[inline(always)]
    pub fn mt(&mut self) -> MT_W<4> {
        MT_W::new(self)
    }
    #[doc = "Bit 5 - Address Wrap within a Flash device."]
    #[inline(always)]
    pub fn aw(&mut self) -> AW_W<5> {
        AW_W::new(self)
    }
    #[doc = "Bit 8 - Enable Opcode Violation Interrupt. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<8> {
        EOP_W::new(self)
    }
    #[doc = "Bit 9 - Enable Data Mismatch Violation Interrupt. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn emc(&mut self) -> EMC_W<9> {
        EMC_W::new(self)
    }
    #[doc = "Bit 10 - Enable Out of Bounds Interrupt. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn eob(&mut self) -> EOB_W<10> {
        EOB_W::new(self)
    }
    #[doc = "Bit 11 - Enable Runtime Region R/W Permission Violation Interrupt. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn erg(&mut self) -> ERG_W<11> {
        ERG_W::new(self)
    }
    #[doc = "Bit 12 - Enable Timeout in Match Phase Interrupt. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn emt(&mut self) -> EMT_W<12> {
        EMT_W::new(self)
    }
    #[doc = "Bit 13 - Enable Address Wrap within a Flash device Interrupt. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn eaw(&mut self) -> EAW_W<13> {
        EAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Violation IRQ Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vioctrlsts](index.html) module"]
pub struct VIOCTRLSTS_SPEC;
impl crate::RegisterSpec for VIOCTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vioctrlsts::R](R) reader structure"]
impl crate::Readable for VIOCTRLSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vioctrlsts::W](W) writer structure"]
impl crate::Writable for VIOCTRLSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VIOCTRLSTS to value 0"]
impl crate::Resettable for VIOCTRLSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
