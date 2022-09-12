#[doc = "Register `CLK_REQ_3` reader"]
pub struct R(crate::R<CLK_REQ_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_REQ_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_REQ_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_REQ_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_REQ_3` writer"]
pub struct W(crate::W<CLK_REQ_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_REQ_3_SPEC>;
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
impl From<crate::W<CLK_REQ_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_REQ_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_CLK_REQ` reader - ADC Clock Reuqired"]
pub type ADC_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `ADC_CLK_REQ` writer - ADC Clock Reuqired"]
pub type ADC_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `GP_SPI0_CLK_REQ` reader - GP SPI0 Clock Reuqired"]
pub type GP_SPI0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `GP_SPI0_CLK_REQ` writer - GP SPI0 Clock Reuqired"]
pub type GP_SPI0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `HTMR_0_CLK_REQ` reader - HTIMER 0 Clock Reuqired"]
pub type HTMR_0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `HTMR_0_CLK_REQ` writer - HTIMER 0 Clock Reuqired"]
pub type HTMR_0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `KEYSCAN_CLK_REQ` reader - KEYSCAN Clock Reuqired"]
pub type KEYSCAN_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `KEYSCAN_CLK_REQ` writer - KEYSCAN Clock Reuqired"]
pub type KEYSCAN_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `RPMPWM_CLK_REQ` reader - RPM-PWM Clock Reuqired"]
pub type RPMPWM_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `RPMPWM_CLK_REQ` writer - RPM-PWM Clock Reuqired"]
pub type RPMPWM_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `SMB1_CLK_REQ` reader - SMB1 Clock Reuqired"]
pub type SMB1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SMB1_CLK_REQ` writer - SMB1 Clock Reuqired"]
pub type SMB1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `SMB2_CLK_REQ` reader - SMB2 Clock Reuqired"]
pub type SMB2_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SMB2_CLK_REQ` writer - SMB2 Clock Reuqired"]
pub type SMB2_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `SMB3_CLK_REQ` reader - SMB3 Clock Reuqired"]
pub type SMB3_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SMB3_CLK_REQ` writer - SMB3 Clock Reuqired"]
pub type SMB3_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `LED0_CLK_REQ` reader - LED0 Clock Reuqired"]
pub type LED0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `LED0_CLK_REQ` writer - LED0 Clock Reuqired"]
pub type LED0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `LED1_CLK_REQ` reader - LED1 Clock Reuqired"]
pub type LED1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `LED1_CLK_REQ` writer - LED1 Clock Reuqired"]
pub type LED1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `TMR16_2_CLK_REQ` reader - TIMER16_2 Clock Reuqired"]
pub type TMR16_2_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TMR16_2_CLK_REQ` writer - TIMER16_2 Clock Reuqired"]
pub type TMR16_2_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `TMR16_3_CLK_REQ` reader - TIMER16_3 Clock Reuqired"]
pub type TMR16_3_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TMR16_3_CLK_REQ` writer - TIMER16_3 Clock Reuqired"]
pub type TMR16_3_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `TMR32_0_CLK_REQ` reader - TIMER32_0 Clock Reuqired"]
pub type TMR32_0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TMR32_0_CLK_REQ` writer - TIMER32_0 Clock Reuqired"]
pub type TMR32_0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `TMR32_1_CLK_REQ` reader - TIMER32_1 Clock Reuqired"]
pub type TMR32_1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TMR32_1_CLK_REQ` writer - TIMER32_1 Clock Reuqired"]
pub type TMR32_1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `HTMR_1_CLK_REQ` reader - HTIMER 1 Clock Reuqired"]
pub type HTMR_1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `HTMR_1_CLK_REQ` writer - HTIMER 1 Clock Reuqired"]
pub type HTMR_1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `CCTMR_CLK_REQ` reader - Capture Compare Timer Clock Reuqired"]
pub type CCTMR_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `CCTMR_CLK_REQ` writer - Capture Compare Timer Clock Reuqired"]
pub type CCTMR_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - ADC Clock Reuqired"]
    #[inline(always)]
    pub fn adc_clk_req(&self) -> ADC_CLK_REQ_R {
        ADC_CLK_REQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - GP SPI0 Clock Reuqired"]
    #[inline(always)]
    pub fn gp_spi0_clk_req(&self) -> GP_SPI0_CLK_REQ_R {
        GP_SPI0_CLK_REQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HTIMER 0 Clock Reuqired"]
    #[inline(always)]
    pub fn htmr_0_clk_req(&self) -> HTMR_0_CLK_REQ_R {
        HTMR_0_CLK_REQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - KEYSCAN Clock Reuqired"]
    #[inline(always)]
    pub fn keyscan_clk_req(&self) -> KEYSCAN_CLK_REQ_R {
        KEYSCAN_CLK_REQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RPM-PWM Clock Reuqired"]
    #[inline(always)]
    pub fn rpmpwm_clk_req(&self) -> RPMPWM_CLK_REQ_R {
        RPMPWM_CLK_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMB1 Clock Reuqired"]
    #[inline(always)]
    pub fn smb1_clk_req(&self) -> SMB1_CLK_REQ_R {
        SMB1_CLK_REQ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SMB2 Clock Reuqired"]
    #[inline(always)]
    pub fn smb2_clk_req(&self) -> SMB2_CLK_REQ_R {
        SMB2_CLK_REQ_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMB3 Clock Reuqired"]
    #[inline(always)]
    pub fn smb3_clk_req(&self) -> SMB3_CLK_REQ_R {
        SMB3_CLK_REQ_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LED0 Clock Reuqired"]
    #[inline(always)]
    pub fn led0_clk_req(&self) -> LED0_CLK_REQ_R {
        LED0_CLK_REQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LED1 Clock Reuqired"]
    #[inline(always)]
    pub fn led1_clk_req(&self) -> LED1_CLK_REQ_R {
        LED1_CLK_REQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - TIMER16_2 Clock Reuqired"]
    #[inline(always)]
    pub fn tmr16_2_clk_req(&self) -> TMR16_2_CLK_REQ_R {
        TMR16_2_CLK_REQ_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TIMER16_3 Clock Reuqired"]
    #[inline(always)]
    pub fn tmr16_3_clk_req(&self) -> TMR16_3_CLK_REQ_R {
        TMR16_3_CLK_REQ_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TIMER32_0 Clock Reuqired"]
    #[inline(always)]
    pub fn tmr32_0_clk_req(&self) -> TMR32_0_CLK_REQ_R {
        TMR32_0_CLK_REQ_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TIMER32_1 Clock Reuqired"]
    #[inline(always)]
    pub fn tmr32_1_clk_req(&self) -> TMR32_1_CLK_REQ_R {
        TMR32_1_CLK_REQ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - HTIMER 1 Clock Reuqired"]
    #[inline(always)]
    pub fn htmr_1_clk_req(&self) -> HTMR_1_CLK_REQ_R {
        HTMR_1_CLK_REQ_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Capture Compare Timer Clock Reuqired"]
    #[inline(always)]
    pub fn cctmr_clk_req(&self) -> CCTMR_CLK_REQ_R {
        CCTMR_CLK_REQ_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ADC Clock Reuqired"]
    #[inline(always)]
    pub fn adc_clk_req(&mut self) -> ADC_CLK_REQ_W<3> {
        ADC_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 9 - GP SPI0 Clock Reuqired"]
    #[inline(always)]
    pub fn gp_spi0_clk_req(&mut self) -> GP_SPI0_CLK_REQ_W<9> {
        GP_SPI0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 10 - HTIMER 0 Clock Reuqired"]
    #[inline(always)]
    pub fn htmr_0_clk_req(&mut self) -> HTMR_0_CLK_REQ_W<10> {
        HTMR_0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 11 - KEYSCAN Clock Reuqired"]
    #[inline(always)]
    pub fn keyscan_clk_req(&mut self) -> KEYSCAN_CLK_REQ_W<11> {
        KEYSCAN_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 12 - RPM-PWM Clock Reuqired"]
    #[inline(always)]
    pub fn rpmpwm_clk_req(&mut self) -> RPMPWM_CLK_REQ_W<12> {
        RPMPWM_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 13 - SMB1 Clock Reuqired"]
    #[inline(always)]
    pub fn smb1_clk_req(&mut self) -> SMB1_CLK_REQ_W<13> {
        SMB1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 14 - SMB2 Clock Reuqired"]
    #[inline(always)]
    pub fn smb2_clk_req(&mut self) -> SMB2_CLK_REQ_W<14> {
        SMB2_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 15 - SMB3 Clock Reuqired"]
    #[inline(always)]
    pub fn smb3_clk_req(&mut self) -> SMB3_CLK_REQ_W<15> {
        SMB3_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 16 - LED0 Clock Reuqired"]
    #[inline(always)]
    pub fn led0_clk_req(&mut self) -> LED0_CLK_REQ_W<16> {
        LED0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 17 - LED1 Clock Reuqired"]
    #[inline(always)]
    pub fn led1_clk_req(&mut self) -> LED1_CLK_REQ_W<17> {
        LED1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 21 - TIMER16_2 Clock Reuqired"]
    #[inline(always)]
    pub fn tmr16_2_clk_req(&mut self) -> TMR16_2_CLK_REQ_W<21> {
        TMR16_2_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 22 - TIMER16_3 Clock Reuqired"]
    #[inline(always)]
    pub fn tmr16_3_clk_req(&mut self) -> TMR16_3_CLK_REQ_W<22> {
        TMR16_3_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 23 - TIMER32_0 Clock Reuqired"]
    #[inline(always)]
    pub fn tmr32_0_clk_req(&mut self) -> TMR32_0_CLK_REQ_W<23> {
        TMR32_0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 24 - TIMER32_1 Clock Reuqired"]
    #[inline(always)]
    pub fn tmr32_1_clk_req(&mut self) -> TMR32_1_CLK_REQ_W<24> {
        TMR32_1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 29 - HTIMER 1 Clock Reuqired"]
    #[inline(always)]
    pub fn htmr_1_clk_req(&mut self) -> HTMR_1_CLK_REQ_W<29> {
        HTMR_1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 30 - Capture Compare Timer Clock Reuqired"]
    #[inline(always)]
    pub fn cctmr_clk_req(&mut self) -> CCTMR_CLK_REQ_W<30> {
        CCTMR_CLK_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Required 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_req_3](index.html) module"]
pub struct CLK_REQ_3_SPEC;
impl crate::RegisterSpec for CLK_REQ_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_req_3::R](R) reader structure"]
impl crate::Readable for CLK_REQ_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_req_3::W](W) writer structure"]
impl crate::Writable for CLK_REQ_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_REQ_3 to value 0"]
impl crate::Resettable for CLK_REQ_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
