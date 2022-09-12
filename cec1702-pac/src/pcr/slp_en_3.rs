#[doc = "Register `SLP_EN_3` reader"]
pub struct R(crate::R<SLP_EN_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_EN_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_EN_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_EN_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_EN_3` writer"]
pub struct W(crate::W<SLP_EN_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_EN_3_SPEC>;
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
impl From<crate::W<SLP_EN_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_EN_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SLP_EN` reader - ADC Sleep Enable"]
pub type ADC_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SLP_EN` writer - ADC Sleep Enable"]
pub type ADC_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `GP_SPI0_SLP_EN` reader - GP SPI0 Sleep Enable"]
pub type GP_SPI0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `GP_SPI0_SLP_EN` writer - GP SPI0 Sleep Enable"]
pub type GP_SPI0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `HTMR_0_SLP_EN` reader - HTIMER 0 Sleep Enable"]
pub type HTMR_0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `HTMR_0_SLP_EN` writer - HTIMER 0 Sleep Enable"]
pub type HTMR_0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `KEYSCAN_SLP_EN` reader - KEYSCAN Sleep Enable"]
pub type KEYSCAN_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `KEYSCAN_SLP_EN` writer - KEYSCAN Sleep Enable"]
pub type KEYSCAN_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `RPMPWM_SLP_EN` reader - RPM-PWM Sleep Enable"]
pub type RPMPWM_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `RPMPWM_SLP_EN` writer - RPM-PWM Sleep Enable"]
pub type RPMPWM_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB1_SLP_EN` reader - SMB1 Sleep Enable"]
pub type SMB1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB1_SLP_EN` writer - SMB1 Sleep Enable"]
pub type SMB1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB2_SLP_EN` reader - SMB2 Sleep Enable"]
pub type SMB2_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB2_SLP_EN` writer - SMB2 Sleep Enable"]
pub type SMB2_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB3_SLP_EN` reader - SMB3 Sleep Enable"]
pub type SMB3_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB3_SLP_EN` writer - SMB3 Sleep Enable"]
pub type SMB3_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `LED0_SLP_EN` reader - LED0 Sleep Enable"]
pub type LED0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `LED0_SLP_EN` writer - LED0 Sleep Enable"]
pub type LED0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `LED1_SLP_EN` reader - LED1 Sleep Enable"]
pub type LED1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `LED1_SLP_EN` writer - LED1 Sleep Enable"]
pub type LED1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `TMR16_2_SLP_EN` reader - TIMER16_2_Sleep Enable"]
pub type TMR16_2_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR16_2_SLP_EN` writer - TIMER16_2_Sleep Enable"]
pub type TMR16_2_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `TMR16_3_SLP_EN` reader - TIMER16_3 Sleep Enable"]
pub type TMR16_3_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR16_3_SLP_EN` writer - TIMER16_3 Sleep Enable"]
pub type TMR16_3_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `TMR32_0_SLP_EN` reader - TIMER32_0 Sleep Enable"]
pub type TMR32_0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR32_0_SLP_EN` writer - TIMER32_0 Sleep Enable"]
pub type TMR32_0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `TMR32_1_SLP_EN` reader - TIMER32_1 Sleep Enable"]
pub type TMR32_1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR32_1_SLP_EN` writer - TIMER32_1 Sleep Enable"]
pub type TMR32_1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `HTMR_1_SLP_EN` reader - HTIMER 1 Sleep Enable"]
pub type HTMR_1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `HTMR_1_SLP_EN` writer - HTIMER 1 Sleep Enable"]
pub type HTMR_1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `CCTMR_SLP_EN` reader - Capture Compare Timer Sleep Enable"]
pub type CCTMR_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `CCTMR_SLP_EN` writer - Capture Compare Timer Sleep Enable"]
pub type CCTMR_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - ADC Sleep Enable"]
    #[inline(always)]
    pub fn adc_slp_en(&self) -> ADC_SLP_EN_R {
        ADC_SLP_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - GP SPI0 Sleep Enable"]
    #[inline(always)]
    pub fn gp_spi0_slp_en(&self) -> GP_SPI0_SLP_EN_R {
        GP_SPI0_SLP_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HTIMER 0 Sleep Enable"]
    #[inline(always)]
    pub fn htmr_0_slp_en(&self) -> HTMR_0_SLP_EN_R {
        HTMR_0_SLP_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - KEYSCAN Sleep Enable"]
    #[inline(always)]
    pub fn keyscan_slp_en(&self) -> KEYSCAN_SLP_EN_R {
        KEYSCAN_SLP_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RPM-PWM Sleep Enable"]
    #[inline(always)]
    pub fn rpmpwm_slp_en(&self) -> RPMPWM_SLP_EN_R {
        RPMPWM_SLP_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMB1 Sleep Enable"]
    #[inline(always)]
    pub fn smb1_slp_en(&self) -> SMB1_SLP_EN_R {
        SMB1_SLP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SMB2 Sleep Enable"]
    #[inline(always)]
    pub fn smb2_slp_en(&self) -> SMB2_SLP_EN_R {
        SMB2_SLP_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMB3 Sleep Enable"]
    #[inline(always)]
    pub fn smb3_slp_en(&self) -> SMB3_SLP_EN_R {
        SMB3_SLP_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LED0 Sleep Enable"]
    #[inline(always)]
    pub fn led0_slp_en(&self) -> LED0_SLP_EN_R {
        LED0_SLP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LED1 Sleep Enable"]
    #[inline(always)]
    pub fn led1_slp_en(&self) -> LED1_SLP_EN_R {
        LED1_SLP_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - TIMER16_2_Sleep Enable"]
    #[inline(always)]
    pub fn tmr16_2_slp_en(&self) -> TMR16_2_SLP_EN_R {
        TMR16_2_SLP_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TIMER16_3 Sleep Enable"]
    #[inline(always)]
    pub fn tmr16_3_slp_en(&self) -> TMR16_3_SLP_EN_R {
        TMR16_3_SLP_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TIMER32_0 Sleep Enable"]
    #[inline(always)]
    pub fn tmr32_0_slp_en(&self) -> TMR32_0_SLP_EN_R {
        TMR32_0_SLP_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TIMER32_1 Sleep Enable"]
    #[inline(always)]
    pub fn tmr32_1_slp_en(&self) -> TMR32_1_SLP_EN_R {
        TMR32_1_SLP_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - HTIMER 1 Sleep Enable"]
    #[inline(always)]
    pub fn htmr_1_slp_en(&self) -> HTMR_1_SLP_EN_R {
        HTMR_1_SLP_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Capture Compare Timer Sleep Enable"]
    #[inline(always)]
    pub fn cctmr_slp_en(&self) -> CCTMR_SLP_EN_R {
        CCTMR_SLP_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ADC Sleep Enable"]
    #[inline(always)]
    pub fn adc_slp_en(&mut self) -> ADC_SLP_EN_W<3> {
        ADC_SLP_EN_W::new(self)
    }
    #[doc = "Bit 9 - GP SPI0 Sleep Enable"]
    #[inline(always)]
    pub fn gp_spi0_slp_en(&mut self) -> GP_SPI0_SLP_EN_W<9> {
        GP_SPI0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 10 - HTIMER 0 Sleep Enable"]
    #[inline(always)]
    pub fn htmr_0_slp_en(&mut self) -> HTMR_0_SLP_EN_W<10> {
        HTMR_0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 11 - KEYSCAN Sleep Enable"]
    #[inline(always)]
    pub fn keyscan_slp_en(&mut self) -> KEYSCAN_SLP_EN_W<11> {
        KEYSCAN_SLP_EN_W::new(self)
    }
    #[doc = "Bit 12 - RPM-PWM Sleep Enable"]
    #[inline(always)]
    pub fn rpmpwm_slp_en(&mut self) -> RPMPWM_SLP_EN_W<12> {
        RPMPWM_SLP_EN_W::new(self)
    }
    #[doc = "Bit 13 - SMB1 Sleep Enable"]
    #[inline(always)]
    pub fn smb1_slp_en(&mut self) -> SMB1_SLP_EN_W<13> {
        SMB1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 14 - SMB2 Sleep Enable"]
    #[inline(always)]
    pub fn smb2_slp_en(&mut self) -> SMB2_SLP_EN_W<14> {
        SMB2_SLP_EN_W::new(self)
    }
    #[doc = "Bit 15 - SMB3 Sleep Enable"]
    #[inline(always)]
    pub fn smb3_slp_en(&mut self) -> SMB3_SLP_EN_W<15> {
        SMB3_SLP_EN_W::new(self)
    }
    #[doc = "Bit 16 - LED0 Sleep Enable"]
    #[inline(always)]
    pub fn led0_slp_en(&mut self) -> LED0_SLP_EN_W<16> {
        LED0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 17 - LED1 Sleep Enable"]
    #[inline(always)]
    pub fn led1_slp_en(&mut self) -> LED1_SLP_EN_W<17> {
        LED1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 21 - TIMER16_2_Sleep Enable"]
    #[inline(always)]
    pub fn tmr16_2_slp_en(&mut self) -> TMR16_2_SLP_EN_W<21> {
        TMR16_2_SLP_EN_W::new(self)
    }
    #[doc = "Bit 22 - TIMER16_3 Sleep Enable"]
    #[inline(always)]
    pub fn tmr16_3_slp_en(&mut self) -> TMR16_3_SLP_EN_W<22> {
        TMR16_3_SLP_EN_W::new(self)
    }
    #[doc = "Bit 23 - TIMER32_0 Sleep Enable"]
    #[inline(always)]
    pub fn tmr32_0_slp_en(&mut self) -> TMR32_0_SLP_EN_W<23> {
        TMR32_0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 24 - TIMER32_1 Sleep Enable"]
    #[inline(always)]
    pub fn tmr32_1_slp_en(&mut self) -> TMR32_1_SLP_EN_W<24> {
        TMR32_1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 29 - HTIMER 1 Sleep Enable"]
    #[inline(always)]
    pub fn htmr_1_slp_en(&mut self) -> HTMR_1_SLP_EN_W<29> {
        HTMR_1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 30 - Capture Compare Timer Sleep Enable"]
    #[inline(always)]
    pub fn cctmr_slp_en(&mut self) -> CCTMR_SLP_EN_W<30> {
        CCTMR_SLP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Enable 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_en_3](index.html) module"]
pub struct SLP_EN_3_SPEC;
impl crate::RegisterSpec for SLP_EN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_en_3::R](R) reader structure"]
impl crate::Readable for SLP_EN_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_en_3::W](W) writer structure"]
impl crate::Writable for SLP_EN_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_EN_3 to value 0"]
impl crate::Resettable for SLP_EN_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
