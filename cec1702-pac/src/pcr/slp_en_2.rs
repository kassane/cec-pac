#[doc = "Register `SLP_EN_2` reader"]
pub struct R(crate::R<SLP_EN_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_EN_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_EN_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_EN_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_EN_2` writer"]
pub struct W(crate::W<SLP_EN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_EN_2_SPEC>;
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
impl From<crate::W<SLP_EN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_EN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART0_SLP_EN` reader - UART 0 Sleep Enable"]
pub type UART0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART0_SLP_EN` writer - UART 0 Sleep Enable"]
pub type UART0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_2_SPEC, bool, O>;
#[doc = "Field `UART1_SLP_EN` reader - UART 1 Sleep Enable"]
pub type UART1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART1_SLP_EN` writer - UART 1 Sleep Enable"]
pub type UART1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_2_SPEC, bool, O>;
#[doc = "Field `GLBL_CFG_SLP_EN` reader - GLBL_CFG"]
pub type GLBL_CFG_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `GLBL_CFG_SLP_EN` writer - GLBL_CFG"]
pub type GLBL_CFG_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_2_SPEC, bool, O>;
#[doc = "Field `RTC_SLP_EN` reader - RTC Sleep Enable"]
pub type RTC_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SLP_EN` writer - RTC Sleep Enable"]
pub type RTC_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - UART 0 Sleep Enable"]
    #[inline(always)]
    pub fn uart0_slp_en(&self) -> UART0_SLP_EN_R {
        UART0_SLP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART 1 Sleep Enable"]
    #[inline(always)]
    pub fn uart1_slp_en(&self) -> UART1_SLP_EN_R {
        UART1_SLP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 12 - GLBL_CFG"]
    #[inline(always)]
    pub fn glbl_cfg_slp_en(&self) -> GLBL_CFG_SLP_EN_R {
        GLBL_CFG_SLP_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - RTC Sleep Enable"]
    #[inline(always)]
    pub fn rtc_slp_en(&self) -> RTC_SLP_EN_R {
        RTC_SLP_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - UART 0 Sleep Enable"]
    #[inline(always)]
    pub fn uart0_slp_en(&mut self) -> UART0_SLP_EN_W<1> {
        UART0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 2 - UART 1 Sleep Enable"]
    #[inline(always)]
    pub fn uart1_slp_en(&mut self) -> UART1_SLP_EN_W<2> {
        UART1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 12 - GLBL_CFG"]
    #[inline(always)]
    pub fn glbl_cfg_slp_en(&mut self) -> GLBL_CFG_SLP_EN_W<12> {
        GLBL_CFG_SLP_EN_W::new(self)
    }
    #[doc = "Bit 18 - RTC Sleep Enable"]
    #[inline(always)]
    pub fn rtc_slp_en(&mut self) -> RTC_SLP_EN_W<18> {
        RTC_SLP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Enable 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_en_2](index.html) module"]
pub struct SLP_EN_2_SPEC;
impl crate::RegisterSpec for SLP_EN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_en_2::R](R) reader structure"]
impl crate::Readable for SLP_EN_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_en_2::W](W) writer structure"]
impl crate::Writable for SLP_EN_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_EN_2 to value 0"]
impl crate::Resettable for SLP_EN_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
