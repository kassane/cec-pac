#[doc = "Register `PWR_RST_CTRL` reader"]
pub struct R(crate::R<PWR_RST_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_RST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_RST_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_RST_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_RST_CTRL` writer"]
pub struct W(crate::W<PWR_RST_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_RST_CTRL_SPEC>;
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
impl From<crate::W<PWR_RST_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_RST_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWR_INV` reader - Used by FW to control internal RESET_VCC signal function and external PWROK pin. This bit is read-only when VCC_PWRGD\n is de-asserted low."]
pub type PWR_INV_R = crate::BitReader<bool>;
#[doc = "Field `PWR_INV` writer - Used by FW to control internal RESET_VCC signal function and external PWROK pin. This bit is read-only when VCC_PWRGD\n is de-asserted low."]
pub type PWR_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_RST_CTRL_SPEC, bool, O>;
#[doc = "Field `H_RST_SEL` reader - Determines what generates the internal platform reset signal. 1=LRESET# pin; 0=eSPI PLTRST# VWire"]
pub type H_RST_SEL_R = crate::BitReader<bool>;
#[doc = "Field `H_RST_SEL` writer - Determines what generates the internal platform reset signal. 1=LRESET# pin; 0=eSPI PLTRST# VWire"]
pub type H_RST_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_RST_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Used by FW to control internal RESET_VCC signal function and external PWROK pin. This bit is read-only when VCC_PWRGD\n is de-asserted low."]
    #[inline(always)]
    pub fn pwr_inv(&self) -> PWR_INV_R {
        PWR_INV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Determines what generates the internal platform reset signal. 1=LRESET# pin; 0=eSPI PLTRST# VWire"]
    #[inline(always)]
    pub fn h_rst_sel(&self) -> H_RST_SEL_R {
        H_RST_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used by FW to control internal RESET_VCC signal function and external PWROK pin. This bit is read-only when VCC_PWRGD\n is de-asserted low."]
    #[inline(always)]
    pub fn pwr_inv(&mut self) -> PWR_INV_W<0> {
        PWR_INV_W::new(self)
    }
    #[doc = "Bit 8 - Determines what generates the internal platform reset signal. 1=LRESET# pin; 0=eSPI PLTRST# VWire"]
    #[inline(always)]
    pub fn h_rst_sel(&mut self) -> H_RST_SEL_W<8> {
        H_RST_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_rst_ctrl](index.html) module"]
pub struct PWR_RST_CTRL_SPEC;
impl crate::RegisterSpec for PWR_RST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_rst_ctrl::R](R) reader structure"]
impl crate::Readable for PWR_RST_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_rst_ctrl::W](W) writer structure"]
impl crate::Writable for PWR_RST_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_RST_CTRL to value 0x0101"]
impl crate::Resettable for PWR_RST_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101
    }
}
