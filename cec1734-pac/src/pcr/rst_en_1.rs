#[doc = "Register `RST_EN_1` reader"]
pub struct R(crate::R<RST_EN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_EN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_EN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_EN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST_EN_1` writer"]
pub struct W(crate::W<RST_EN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_EN_1_SPEC>;
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
impl From<crate::W<RST_EN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_EN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_RST_EN` reader - Interrupt Reset Enable"]
pub type INT_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `INT_RST_EN` writer - Interrupt Reset Enable"]
pub type INT_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_1_SPEC, bool, O>;
#[doc = "Field `PWM0_RST_EN` reader - PWM0 Reset Enable (PWM0_RST_EN)"]
pub type PWM0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM0_RST_EN` writer - PWM0 Reset Enable (PWM0_RST_EN)"]
pub type PWM0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_1_SPEC, bool, O>;
#[doc = "Field `DMA_RST_EN` reader - DMA Reset Enable (DMA_RST_EN)"]
pub type DMA_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RST_EN` writer - DMA Reset Enable (DMA_RST_EN)"]
pub type DMA_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_1_SPEC, bool, O>;
#[doc = "Field `TFDP_RST_EN` reader - TFDP Reset Enable (TFDP_RST_EN)"]
pub type TFDP_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `TFDP_RST_EN` writer - TFDP Reset Enable (TFDP_RST_EN)"]
pub type TFDP_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_1_SPEC, bool, O>;
#[doc = "Field `WDT_RST_EN` reader - WDT Reset Enable (WDT_RST_EN)"]
pub type WDT_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_RST_EN` writer - WDT Reset Enable (WDT_RST_EN)"]
pub type WDT_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_1_SPEC, bool, O>;
#[doc = "Field `SMB0_RST_EN` reader - SMB0 Reset Enable (SMB0_RST_EN)"]
pub type SMB0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB0_RST_EN` writer - SMB0 Reset Enable (SMB0_RST_EN)"]
pub type SMB0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_1_SPEC, bool, O>;
#[doc = "Field `TMR32_0_RST_EN` reader - TIMER32_0 Reset Enable (TIMER32_0_RST_EN)"]
pub type TMR32_0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR32_0_RST_EN` writer - TIMER32_0 Reset Enable (TIMER32_0_RST_EN)"]
pub type TMR32_0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_1_SPEC, bool, O>;
#[doc = "Field `TMR32_1_RST_EN` reader - TIMER32_1 Reset Enable (TIMER32_1_RST_EN)"]
pub type TMR32_1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR32_1_RST_EN` writer - TIMER32_1 Reset Enable (TIMER32_1_RST_EN)"]
pub type TMR32_1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Reset Enable"]
    #[inline(always)]
    pub fn int_rst_en(&self) -> INT_RST_EN_R {
        INT_RST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - PWM0 Reset Enable (PWM0_RST_EN)"]
    #[inline(always)]
    pub fn pwm0_rst_en(&self) -> PWM0_RST_EN_R {
        PWM0_RST_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Reset Enable (DMA_RST_EN)"]
    #[inline(always)]
    pub fn dma_rst_en(&self) -> DMA_RST_EN_R {
        DMA_RST_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TFDP Reset Enable (TFDP_RST_EN)"]
    #[inline(always)]
    pub fn tfdp_rst_en(&self) -> TFDP_RST_EN_R {
        TFDP_RST_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - WDT Reset Enable (WDT_RST_EN)"]
    #[inline(always)]
    pub fn wdt_rst_en(&self) -> WDT_RST_EN_R {
        WDT_RST_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SMB0 Reset Enable (SMB0_RST_EN)"]
    #[inline(always)]
    pub fn smb0_rst_en(&self) -> SMB0_RST_EN_R {
        SMB0_RST_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 30 - TIMER32_0 Reset Enable (TIMER32_0_RST_EN)"]
    #[inline(always)]
    pub fn tmr32_0_rst_en(&self) -> TMR32_0_RST_EN_R {
        TMR32_0_RST_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TIMER32_1 Reset Enable (TIMER32_1_RST_EN)"]
    #[inline(always)]
    pub fn tmr32_1_rst_en(&self) -> TMR32_1_RST_EN_R {
        TMR32_1_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Reset Enable"]
    #[inline(always)]
    pub fn int_rst_en(&mut self) -> INT_RST_EN_W<0> {
        INT_RST_EN_W::new(self)
    }
    #[doc = "Bit 4 - PWM0 Reset Enable (PWM0_RST_EN)"]
    #[inline(always)]
    pub fn pwm0_rst_en(&mut self) -> PWM0_RST_EN_W<4> {
        PWM0_RST_EN_W::new(self)
    }
    #[doc = "Bit 6 - DMA Reset Enable (DMA_RST_EN)"]
    #[inline(always)]
    pub fn dma_rst_en(&mut self) -> DMA_RST_EN_W<6> {
        DMA_RST_EN_W::new(self)
    }
    #[doc = "Bit 7 - TFDP Reset Enable (TFDP_RST_EN)"]
    #[inline(always)]
    pub fn tfdp_rst_en(&mut self) -> TFDP_RST_EN_W<7> {
        TFDP_RST_EN_W::new(self)
    }
    #[doc = "Bit 9 - WDT Reset Enable (WDT_RST_EN)"]
    #[inline(always)]
    pub fn wdt_rst_en(&mut self) -> WDT_RST_EN_W<9> {
        WDT_RST_EN_W::new(self)
    }
    #[doc = "Bit 10 - SMB0 Reset Enable (SMB0_RST_EN)"]
    #[inline(always)]
    pub fn smb0_rst_en(&mut self) -> SMB0_RST_EN_W<10> {
        SMB0_RST_EN_W::new(self)
    }
    #[doc = "Bit 30 - TIMER32_0 Reset Enable (TIMER32_0_RST_EN)"]
    #[inline(always)]
    pub fn tmr32_0_rst_en(&mut self) -> TMR32_0_RST_EN_W<30> {
        TMR32_0_RST_EN_W::new(self)
    }
    #[doc = "Bit 31 - TIMER32_1 Reset Enable (TIMER32_1_RST_EN)"]
    #[inline(always)]
    pub fn tmr32_1_rst_en(&mut self) -> TMR32_1_RST_EN_W<31> {
        TMR32_1_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Enable 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_en_1](index.html) module"]
pub struct RST_EN_1_SPEC;
impl crate::RegisterSpec for RST_EN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst_en_1::R](R) reader structure"]
impl crate::Readable for RST_EN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_en_1::W](W) writer structure"]
impl crate::Writable for RST_EN_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RST_EN_1 to value 0"]
impl crate::Resettable for RST_EN_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
