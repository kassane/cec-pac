#[doc = "Register `MNTR_CTRL` reader"]
pub struct R(crate::R<MNTR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MNTR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MNTR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MNTR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MNTR_CTRL` writer"]
pub struct W(crate::W<MNTR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MNTR_CTRL_SPEC>;
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
impl From<crate::W<MNTR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MNTR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT` reader - Activate 1= Activate, 0= De-activate"]
pub type ACT_R = crate::BitReader<bool>;
#[doc = "Field `ACT` writer - Activate 1= Activate, 0= De-activate"]
pub type ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MNTR_CTRL_SPEC, bool, O>;
#[doc = "Field `LCK_ACT` reader - Lock Activate 1= Activate field Locked, 0= Activate field unlocked"]
pub type LCK_ACT_R = crate::BitReader<bool>;
#[doc = "Field `LCK_ACT` writer - Lock Activate 1= Activate field Locked, 0= Activate field unlocked"]
pub type LCK_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MNTR_CTRL_SPEC, bool, O>;
#[doc = "Field `SFT_RST` reader - Soft Reset. This field is auto cleared by hardware. 1= Soft Reset SPI Monitor, 0 = No Effect"]
pub type SFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `SFT_RST` writer - Soft Reset. This field is auto cleared by hardware. 1= Soft Reset SPI Monitor, 0 = No Effect"]
pub type SFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MNTR_CTRL_SPEC, bool, O>;
#[doc = "Field `TAP_SEL` reader - CPTR_CLK_TAP_SEL selects which 0.5ns delay tap to have the capture clock on. 00h = 0 ns delay (or if TAP_EN = 0). 01h = 0.5ns nom delay; +/- 50% 02h = 1.0ns 03h = 1.5ns 04h = 2.0ns etc. 1Fh = 15.5ns"]
pub type TAP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAP_SEL` writer - CPTR_CLK_TAP_SEL selects which 0.5ns delay tap to have the capture clock on. 00h = 0 ns delay (or if TAP_EN = 0). 01h = 0.5ns nom delay; +/- 50% 02h = 1.0ns 03h = 1.5ns 04h = 2.0ns etc. 1Fh = 15.5ns"]
pub type TAP_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MNTR_CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `TAP_EN` reader - CPTR_CLK_TAP_EN enable the delay taps. It is recommended that a customer set the TAP_EN bit to 1, and the TAP_SEL field to 04h."]
pub type TAP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TAP_EN` writer - CPTR_CLK_TAP_EN enable the delay taps. It is recommended that a customer set the TAP_EN bit to 1, and the TAP_SEL field to 04h."]
pub type TAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MNTR_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Activate 1= Activate, 0= De-activate"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Activate 1= Activate field Locked, 0= Activate field unlocked"]
    #[inline(always)]
    pub fn lck_act(&self) -> LCK_ACT_R {
        LCK_ACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Soft Reset. This field is auto cleared by hardware. 1= Soft Reset SPI Monitor, 0 = No Effect"]
    #[inline(always)]
    pub fn sft_rst(&self) -> SFT_RST_R {
        SFT_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:20 - CPTR_CLK_TAP_SEL selects which 0.5ns delay tap to have the capture clock on. 00h = 0 ns delay (or if TAP_EN = 0). 01h = 0.5ns nom delay; +/- 50% 02h = 1.0ns 03h = 1.5ns 04h = 2.0ns etc. 1Fh = 15.5ns"]
    #[inline(always)]
    pub fn tap_sel(&self) -> TAP_SEL_R {
        TAP_SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - CPTR_CLK_TAP_EN enable the delay taps. It is recommended that a customer set the TAP_EN bit to 1, and the TAP_SEL field to 04h."]
    #[inline(always)]
    pub fn tap_en(&self) -> TAP_EN_R {
        TAP_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Activate 1= Activate, 0= De-activate"]
    #[inline(always)]
    pub fn act(&mut self) -> ACT_W<0> {
        ACT_W::new(self)
    }
    #[doc = "Bit 1 - Lock Activate 1= Activate field Locked, 0= Activate field unlocked"]
    #[inline(always)]
    pub fn lck_act(&mut self) -> LCK_ACT_W<1> {
        LCK_ACT_W::new(self)
    }
    #[doc = "Bit 8 - Soft Reset. This field is auto cleared by hardware. 1= Soft Reset SPI Monitor, 0 = No Effect"]
    #[inline(always)]
    pub fn sft_rst(&mut self) -> SFT_RST_W<8> {
        SFT_RST_W::new(self)
    }
    #[doc = "Bits 16:20 - CPTR_CLK_TAP_SEL selects which 0.5ns delay tap to have the capture clock on. 00h = 0 ns delay (or if TAP_EN = 0). 01h = 0.5ns nom delay; +/- 50% 02h = 1.0ns 03h = 1.5ns 04h = 2.0ns etc. 1Fh = 15.5ns"]
    #[inline(always)]
    pub fn tap_sel(&mut self) -> TAP_SEL_W<16> {
        TAP_SEL_W::new(self)
    }
    #[doc = "Bit 21 - CPTR_CLK_TAP_EN enable the delay taps. It is recommended that a customer set the TAP_EN bit to 1, and the TAP_SEL field to 04h."]
    #[inline(always)]
    pub fn tap_en(&mut self) -> TAP_EN_W<21> {
        TAP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Monitor Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mntr_ctrl](index.html) module"]
pub struct MNTR_CTRL_SPEC;
impl crate::RegisterSpec for MNTR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mntr_ctrl::R](R) reader structure"]
impl crate::Readable for MNTR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mntr_ctrl::W](W) writer structure"]
impl crate::Writable for MNTR_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MNTR_CTRL to value 0"]
impl crate::Resettable for MNTR_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
