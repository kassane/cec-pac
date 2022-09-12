#[doc = "Register `CLK_REQ_2` reader"]
pub struct R(crate::R<CLK_REQ_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_REQ_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_REQ_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_REQ_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_REQ_2` writer"]
pub struct W(crate::W<CLK_REQ_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_REQ_2_SPEC>;
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
impl From<crate::W<CLK_REQ_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_REQ_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMAP_CLK_REQ` reader - IMAP Clock Required (IMAP_CLK_REQ)"]
pub type IMAP_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `IMAP_CLK_REQ` writer - IMAP Clock Required (IMAP_CLK_REQ)"]
pub type IMAP_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_2_SPEC, bool, O>;
#[doc = "Field `UART0_CLK_REQ` reader - UART 0 Clock Required"]
pub type UART0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `UART0_CLK_REQ` writer - UART 0 Clock Required"]
pub type UART0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_2_SPEC, bool, O>;
#[doc = "Field `UART1_CLK_REQ` reader - UART 1 Clock Required"]
pub type UART1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `UART1_CLK_REQ` writer - UART 1 Clock Required"]
pub type UART1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_2_SPEC, bool, O>;
#[doc = "Field `GLBL_CFG_CLK_REQ` reader - GLBL_CFG (GLBL_CFG_CLK_REQ)"]
pub type GLBL_CFG_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `GLBL_CFG_CLK_REQ` writer - GLBL_CFG (GLBL_CFG_CLK_REQ)"]
pub type GLBL_CFG_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_2_SPEC, bool, O>;
#[doc = "Field `RTC_CLK_REQ` reader - RTC Clock Required (RTC_CLK_REQ)"]
pub type RTC_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `RTC_CLK_REQ` writer - RTC Clock Required (RTC_CLK_REQ)"]
pub type RTC_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IMAP Clock Required (IMAP_CLK_REQ)"]
    #[inline(always)]
    pub fn imap_clk_req(&self) -> IMAP_CLK_REQ_R {
        IMAP_CLK_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART 0 Clock Required"]
    #[inline(always)]
    pub fn uart0_clk_req(&self) -> UART0_CLK_REQ_R {
        UART0_CLK_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART 1 Clock Required"]
    #[inline(always)]
    pub fn uart1_clk_req(&self) -> UART1_CLK_REQ_R {
        UART1_CLK_REQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 12 - GLBL_CFG (GLBL_CFG_CLK_REQ)"]
    #[inline(always)]
    pub fn glbl_cfg_clk_req(&self) -> GLBL_CFG_CLK_REQ_R {
        GLBL_CFG_CLK_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - RTC Clock Required (RTC_CLK_REQ)"]
    #[inline(always)]
    pub fn rtc_clk_req(&self) -> RTC_CLK_REQ_R {
        RTC_CLK_REQ_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IMAP Clock Required (IMAP_CLK_REQ)"]
    #[inline(always)]
    pub fn imap_clk_req(&mut self) -> IMAP_CLK_REQ_W<0> {
        IMAP_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 1 - UART 0 Clock Required"]
    #[inline(always)]
    pub fn uart0_clk_req(&mut self) -> UART0_CLK_REQ_W<1> {
        UART0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 2 - UART 1 Clock Required"]
    #[inline(always)]
    pub fn uart1_clk_req(&mut self) -> UART1_CLK_REQ_W<2> {
        UART1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 12 - GLBL_CFG (GLBL_CFG_CLK_REQ)"]
    #[inline(always)]
    pub fn glbl_cfg_clk_req(&mut self) -> GLBL_CFG_CLK_REQ_W<12> {
        GLBL_CFG_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 18 - RTC Clock Required (RTC_CLK_REQ)"]
    #[inline(always)]
    pub fn rtc_clk_req(&mut self) -> RTC_CLK_REQ_W<18> {
        RTC_CLK_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Required 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_req_2](index.html) module"]
pub struct CLK_REQ_2_SPEC;
impl crate::RegisterSpec for CLK_REQ_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_req_2::R](R) reader structure"]
impl crate::Readable for CLK_REQ_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_req_2::W](W) writer structure"]
impl crate::Writable for CLK_REQ_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_REQ_2 to value 0"]
impl crate::Resettable for CLK_REQ_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
