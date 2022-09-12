#[doc = "Register `RST_EN_3` reader"]
pub struct R(crate::R<RST_EN_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_EN_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_EN_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_EN_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST_EN_3` writer"]
pub struct W(crate::W<RST_EN_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_EN_3_SPEC>;
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
impl From<crate::W<RST_EN_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_EN_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_RST_EN` reader - ADC Reset Enable"]
pub type ADC_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC_RST_EN` writer - ADC Reset Enable"]
pub type ADC_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `GP_SPI0_RST_EN` reader - GP SPI0 Reset Enable"]
pub type GP_SPI0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `GP_SPI0_RST_EN` writer - GP SPI0 Reset Enable"]
pub type GP_SPI0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `HTMR_0_RST_EN` reader - HTIMER 0 Reset Enable"]
pub type HTMR_0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `HTMR_0_RST_EN` writer - HTIMER 0 Reset Enable"]
pub type HTMR_0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `KEYSCAN_RST_EN` reader - KEYSCAN Reset Enable"]
pub type KEYSCAN_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `KEYSCAN_RST_EN` writer - KEYSCAN Reset Enable"]
pub type KEYSCAN_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `RPMPWM_RST_EN` reader - RPM-PWM Reset Enable"]
pub type RPMPWM_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `RPMPWM_RST_EN` writer - RPM-PWM Reset Enable"]
pub type RPMPWM_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB1_RST_EN` reader - SMB1 Reset Enable"]
pub type SMB1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB1_RST_EN` writer - SMB1 Reset Enable"]
pub type SMB1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB2_RST_EN` reader - SMB2 Reset Enable"]
pub type SMB2_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB2_RST_EN` writer - SMB2 Reset Enable"]
pub type SMB2_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB3_RST_EN` reader - SMB3 Reset Enable"]
pub type SMB3_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB3_RST_EN` writer - SMB3 Reset Enable"]
pub type SMB3_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `LED0_RST_EN` reader - LED0 Reset Enable"]
pub type LED0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `LED0_RST_EN` writer - LED0 Reset Enable"]
pub type LED0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `LED1_RST_EN` reader - LED1 Reset Enable"]
pub type LED1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `LED1_RST_EN` writer - LED1 Reset Enable"]
pub type LED1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `TMR16_2_RST_EN` reader - TIMER16_2 Reset Enable"]
pub type TMR16_2_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR16_2_RST_EN` writer - TIMER16_2 Reset Enable"]
pub type TMR16_2_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `TMR16_3_RST_EN` reader - TIMER16_3 Reset Enable"]
pub type TMR16_3_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR16_3_RST_EN` writer - TIMER16_3 Reset Enable"]
pub type TMR16_3_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `TMR32_0_RST_EN` reader - TIMER32_0 Reset Enable"]
pub type TMR32_0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR32_0_RST_EN` writer - TIMER32_0 Reset Enable"]
pub type TMR32_0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `TMR32_1_RST_EN` reader - TIMER32_1 Reset Enable"]
pub type TMR32_1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR32_1_RST_EN` writer - TIMER32_1 Reset Enable"]
pub type TMR32_1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `HTMR_1_RST_EN` reader - HTIMER 1 Reset Enable"]
pub type HTMR_1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `HTMR_1_RST_EN` writer - HTIMER 1 Reset Enable"]
pub type HTMR_1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `CCTMR_RST_EN` reader - Capture Compare Timer Reset Enable"]
pub type CCTMR_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `CCTMR_RST_EN` writer - Capture Compare Timer Reset Enable"]
pub type CCTMR_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - ADC Reset Enable"]
    #[inline(always)]
    pub fn adc_rst_en(&self) -> ADC_RST_EN_R {
        ADC_RST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - GP SPI0 Reset Enable"]
    #[inline(always)]
    pub fn gp_spi0_rst_en(&self) -> GP_SPI0_RST_EN_R {
        GP_SPI0_RST_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HTIMER 0 Reset Enable"]
    #[inline(always)]
    pub fn htmr_0_rst_en(&self) -> HTMR_0_RST_EN_R {
        HTMR_0_RST_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - KEYSCAN Reset Enable"]
    #[inline(always)]
    pub fn keyscan_rst_en(&self) -> KEYSCAN_RST_EN_R {
        KEYSCAN_RST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RPM-PWM Reset Enable"]
    #[inline(always)]
    pub fn rpmpwm_rst_en(&self) -> RPMPWM_RST_EN_R {
        RPMPWM_RST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMB1 Reset Enable"]
    #[inline(always)]
    pub fn smb1_rst_en(&self) -> SMB1_RST_EN_R {
        SMB1_RST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SMB2 Reset Enable"]
    #[inline(always)]
    pub fn smb2_rst_en(&self) -> SMB2_RST_EN_R {
        SMB2_RST_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMB3 Reset Enable"]
    #[inline(always)]
    pub fn smb3_rst_en(&self) -> SMB3_RST_EN_R {
        SMB3_RST_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LED0 Reset Enable"]
    #[inline(always)]
    pub fn led0_rst_en(&self) -> LED0_RST_EN_R {
        LED0_RST_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LED1 Reset Enable"]
    #[inline(always)]
    pub fn led1_rst_en(&self) -> LED1_RST_EN_R {
        LED1_RST_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - TIMER16_2 Reset Enable"]
    #[inline(always)]
    pub fn tmr16_2_rst_en(&self) -> TMR16_2_RST_EN_R {
        TMR16_2_RST_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TIMER16_3 Reset Enable"]
    #[inline(always)]
    pub fn tmr16_3_rst_en(&self) -> TMR16_3_RST_EN_R {
        TMR16_3_RST_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TIMER32_0 Reset Enable"]
    #[inline(always)]
    pub fn tmr32_0_rst_en(&self) -> TMR32_0_RST_EN_R {
        TMR32_0_RST_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TIMER32_1 Reset Enable"]
    #[inline(always)]
    pub fn tmr32_1_rst_en(&self) -> TMR32_1_RST_EN_R {
        TMR32_1_RST_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - HTIMER 1 Reset Enable"]
    #[inline(always)]
    pub fn htmr_1_rst_en(&self) -> HTMR_1_RST_EN_R {
        HTMR_1_RST_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Capture Compare Timer Reset Enable"]
    #[inline(always)]
    pub fn cctmr_rst_en(&self) -> CCTMR_RST_EN_R {
        CCTMR_RST_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ADC Reset Enable"]
    #[inline(always)]
    pub fn adc_rst_en(&mut self) -> ADC_RST_EN_W<3> {
        ADC_RST_EN_W::new(self)
    }
    #[doc = "Bit 9 - GP SPI0 Reset Enable"]
    #[inline(always)]
    pub fn gp_spi0_rst_en(&mut self) -> GP_SPI0_RST_EN_W<9> {
        GP_SPI0_RST_EN_W::new(self)
    }
    #[doc = "Bit 10 - HTIMER 0 Reset Enable"]
    #[inline(always)]
    pub fn htmr_0_rst_en(&mut self) -> HTMR_0_RST_EN_W<10> {
        HTMR_0_RST_EN_W::new(self)
    }
    #[doc = "Bit 11 - KEYSCAN Reset Enable"]
    #[inline(always)]
    pub fn keyscan_rst_en(&mut self) -> KEYSCAN_RST_EN_W<11> {
        KEYSCAN_RST_EN_W::new(self)
    }
    #[doc = "Bit 12 - RPM-PWM Reset Enable"]
    #[inline(always)]
    pub fn rpmpwm_rst_en(&mut self) -> RPMPWM_RST_EN_W<12> {
        RPMPWM_RST_EN_W::new(self)
    }
    #[doc = "Bit 13 - SMB1 Reset Enable"]
    #[inline(always)]
    pub fn smb1_rst_en(&mut self) -> SMB1_RST_EN_W<13> {
        SMB1_RST_EN_W::new(self)
    }
    #[doc = "Bit 14 - SMB2 Reset Enable"]
    #[inline(always)]
    pub fn smb2_rst_en(&mut self) -> SMB2_RST_EN_W<14> {
        SMB2_RST_EN_W::new(self)
    }
    #[doc = "Bit 15 - SMB3 Reset Enable"]
    #[inline(always)]
    pub fn smb3_rst_en(&mut self) -> SMB3_RST_EN_W<15> {
        SMB3_RST_EN_W::new(self)
    }
    #[doc = "Bit 16 - LED0 Reset Enable"]
    #[inline(always)]
    pub fn led0_rst_en(&mut self) -> LED0_RST_EN_W<16> {
        LED0_RST_EN_W::new(self)
    }
    #[doc = "Bit 17 - LED1 Reset Enable"]
    #[inline(always)]
    pub fn led1_rst_en(&mut self) -> LED1_RST_EN_W<17> {
        LED1_RST_EN_W::new(self)
    }
    #[doc = "Bit 21 - TIMER16_2 Reset Enable"]
    #[inline(always)]
    pub fn tmr16_2_rst_en(&mut self) -> TMR16_2_RST_EN_W<21> {
        TMR16_2_RST_EN_W::new(self)
    }
    #[doc = "Bit 22 - TIMER16_3 Reset Enable"]
    #[inline(always)]
    pub fn tmr16_3_rst_en(&mut self) -> TMR16_3_RST_EN_W<22> {
        TMR16_3_RST_EN_W::new(self)
    }
    #[doc = "Bit 23 - TIMER32_0 Reset Enable"]
    #[inline(always)]
    pub fn tmr32_0_rst_en(&mut self) -> TMR32_0_RST_EN_W<23> {
        TMR32_0_RST_EN_W::new(self)
    }
    #[doc = "Bit 24 - TIMER32_1 Reset Enable"]
    #[inline(always)]
    pub fn tmr32_1_rst_en(&mut self) -> TMR32_1_RST_EN_W<24> {
        TMR32_1_RST_EN_W::new(self)
    }
    #[doc = "Bit 29 - HTIMER 1 Reset Enable"]
    #[inline(always)]
    pub fn htmr_1_rst_en(&mut self) -> HTMR_1_RST_EN_W<29> {
        HTMR_1_RST_EN_W::new(self)
    }
    #[doc = "Bit 30 - Capture Compare Timer Reset Enable"]
    #[inline(always)]
    pub fn cctmr_rst_en(&mut self) -> CCTMR_RST_EN_W<30> {
        CCTMR_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Enable 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_en_3](index.html) module"]
pub struct RST_EN_3_SPEC;
impl crate::RegisterSpec for RST_EN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst_en_3::R](R) reader structure"]
impl crate::Readable for RST_EN_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_en_3::W](W) writer structure"]
impl crate::Writable for RST_EN_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RST_EN_3 to value 0"]
impl crate::Resettable for RST_EN_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
