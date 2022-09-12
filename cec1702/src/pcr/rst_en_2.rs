#[doc = "Register `RST_EN_2` reader"]
pub struct R(crate::R<RST_EN_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_EN_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_EN_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_EN_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST_EN_2` writer"]
pub struct W(crate::W<RST_EN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_EN_2_SPEC>;
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
impl From<crate::W<RST_EN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_EN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_0_RST_EN` reader - UART 0 Reset Enable"]
pub type UART_0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART_0_RST_EN` writer - UART 0 Reset Enable"]
pub type UART_0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `UART_1_RST_EN` reader - UART 1 Reset Enable"]
pub type UART_1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART_1_RST_EN` writer - UART 1 Reset Enable"]
pub type UART_1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `GLBL_CFG_RST_EN` reader - GLBL_CFG Reset Enable"]
pub type GLBL_CFG_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `GLBL_CFG_RST_EN` writer - GLBL_CFG Reset Enable"]
pub type GLBL_CFG_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `RTC_RST_EN` reader - RTC Reset Enable"]
pub type RTC_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `RTC_RST_EN` writer - RTC Reset Enable"]
pub type RTC_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - UART 0 Reset Enable"]
    #[inline(always)]
    pub fn uart_0_rst_en(&self) -> UART_0_RST_EN_R {
        UART_0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART 1 Reset Enable"]
    #[inline(always)]
    pub fn uart_1_rst_en(&self) -> UART_1_RST_EN_R {
        UART_1_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 12 - GLBL_CFG Reset Enable"]
    #[inline(always)]
    pub fn glbl_cfg_rst_en(&self) -> GLBL_CFG_RST_EN_R {
        GLBL_CFG_RST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - RTC Reset Enable"]
    #[inline(always)]
    pub fn rtc_rst_en(&self) -> RTC_RST_EN_R {
        RTC_RST_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - UART 0 Reset Enable"]
    #[inline(always)]
    pub fn uart_0_rst_en(&mut self) -> UART_0_RST_EN_W<1> {
        UART_0_RST_EN_W::new(self)
    }
    #[doc = "Bit 2 - UART 1 Reset Enable"]
    #[inline(always)]
    pub fn uart_1_rst_en(&mut self) -> UART_1_RST_EN_W<2> {
        UART_1_RST_EN_W::new(self)
    }
    #[doc = "Bit 12 - GLBL_CFG Reset Enable"]
    #[inline(always)]
    pub fn glbl_cfg_rst_en(&mut self) -> GLBL_CFG_RST_EN_W<12> {
        GLBL_CFG_RST_EN_W::new(self)
    }
    #[doc = "Bit 18 - RTC Reset Enable"]
    #[inline(always)]
    pub fn rtc_rst_en(&mut self) -> RTC_RST_EN_W<18> {
        RTC_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Enable 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_en_2](index.html) module"]
pub struct RST_EN_2_SPEC;
impl crate::RegisterSpec for RST_EN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst_en_2::R](R) reader structure"]
impl crate::Readable for RST_EN_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_en_2::W](W) writer structure"]
impl crate::Writable for RST_EN_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RST_EN_2 to value 0"]
impl crate::Resettable for RST_EN_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
