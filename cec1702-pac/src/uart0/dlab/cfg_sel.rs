#[doc = "Register `CFG_SEL` reader"]
pub struct R(crate::R<CFG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_SEL` writer"]
pub struct W(crate::W<CFG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SEL_SPEC>;
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
impl From<crate::W<CFG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_SRC` reader - CLK_SRC 1=The UART Baud Clock is derived from an external clock source, 0=The UART Baud Clock is derived from one of the two internal clock sources"]
pub type CLK_SRC_R = crate::BitReader<bool>;
#[doc = "Field `CLK_SRC` writer - CLK_SRC 1=The UART Baud Clock is derived from an external clock source, 0=The UART Baud Clock is derived from one of the two internal clock sources"]
pub type CLK_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFG_SEL_SPEC, bool, O>;
#[doc = "Field `PWR` reader - POWER 1=The RESET reset signal is derived from nSIO_RESET, 0=The RESET reset signal is derived from VCC1_RESET"]
pub type PWR_R = crate::BitReader<bool>;
#[doc = "Field `PWR` writer - POWER 1=The RESET reset signal is derived from nSIO_RESET, 0=The RESET reset signal is derived from VCC1_RESET"]
pub type PWR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFG_SEL_SPEC, bool, O>;
#[doc = "Field `POLAR` reader - POLARITY 1=The UART_TX and UART_RX pins functions are inverted, 0=The UART_TX and UART_RX pins functions are not inverted"]
pub type POLAR_R = crate::BitReader<bool>;
#[doc = "Field `POLAR` writer - POLARITY 1=The UART_TX and UART_RX pins functions are inverted, 0=The UART_TX and UART_RX pins functions are not inverted"]
pub type POLAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFG_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CLK_SRC 1=The UART Baud Clock is derived from an external clock source, 0=The UART Baud Clock is derived from one of the two internal clock sources"]
    #[inline(always)]
    pub fn clk_src(&self) -> CLK_SRC_R {
        CLK_SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - POWER 1=The RESET reset signal is derived from nSIO_RESET, 0=The RESET reset signal is derived from VCC1_RESET"]
    #[inline(always)]
    pub fn pwr(&self) -> PWR_R {
        PWR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - POLARITY 1=The UART_TX and UART_RX pins functions are inverted, 0=The UART_TX and UART_RX pins functions are not inverted"]
    #[inline(always)]
    pub fn polar(&self) -> POLAR_R {
        POLAR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLK_SRC 1=The UART Baud Clock is derived from an external clock source, 0=The UART Baud Clock is derived from one of the two internal clock sources"]
    #[inline(always)]
    pub fn clk_src(&mut self) -> CLK_SRC_W<0> {
        CLK_SRC_W::new(self)
    }
    #[doc = "Bit 1 - POWER 1=The RESET reset signal is derived from nSIO_RESET, 0=The RESET reset signal is derived from VCC1_RESET"]
    #[inline(always)]
    pub fn pwr(&mut self) -> PWR_W<1> {
        PWR_W::new(self)
    }
    #[doc = "Bit 2 - POLARITY 1=The UART_TX and UART_RX pins functions are inverted, 0=The UART_TX and UART_RX pins functions are not inverted"]
    #[inline(always)]
    pub fn polar(&mut self) -> POLAR_W<2> {
        POLAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Config Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_sel](index.html) module"]
pub struct CFG_SEL_SPEC;
impl crate::RegisterSpec for CFG_SEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cfg_sel::R](R) reader structure"]
impl crate::Readable for CFG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_sel::W](W) writer structure"]
impl crate::Writable for CFG_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_SEL to value 0"]
impl crate::Resettable for CFG_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
