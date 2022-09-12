#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_EN` reader - WDT Block enabled"]
pub type WDT_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_EN` writer - WDT Block enabled"]
pub type WDT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `WDT_STS` reader - WDT_STATUS is set by hardware if the last reset of the device was caused by an underflow of the WDT. This bit must be cleared by the EC firmware writing a '1' to this bit. Writing a '0' to this bit has no effect."]
pub type WDT_STS_R = crate::BitReader<bool>;
#[doc = "Field `WDT_STS` writer - WDT_STATUS is set by hardware if the last reset of the device was caused by an underflow of the WDT. This bit must be cleared by the EC firmware writing a '1' to this bit. Writing a '0' to this bit has no effect."]
pub type WDT_STS_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `HIB_TMR0_STL` reader - This bit enables the WDT Stall function if the Hibernation Timer 0 is active. 1=The WDT is stalled while the Hibernation Timer 0 is active 0=The WDT is not affected by Hibernation Timer 0."]
pub type HIB_TMR0_STL_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TMR0_STL` writer - This bit enables the WDT Stall function if the Hibernation Timer 0 is active. 1=The WDT is stalled while the Hibernation Timer 0 is active 0=The WDT is not affected by Hibernation Timer 0."]
pub type HIB_TMR0_STL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `WK_TMR_STL` reader - This bit enables the WDT Stall function if the Week Timer is active. 1=The WDT is stalled while the Week Timer is active 0=The WDT is not affected by the Week Timer."]
pub type WK_TMR_STL_R = crate::BitReader<bool>;
#[doc = "Field `WK_TMR_STL` writer - This bit enables the WDT Stall function if the Week Timer is active. 1=The WDT is stalled while the Week Timer is active 0=The WDT is not affected by the Week Timer."]
pub type WK_TMR_STL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `JTAG_STL` reader - This bit enables the WDT Stall function if JTAG or SWD debug functions are active 1=The WDT is stalled while either JTAG or SWD is active 0=The WDT is not affected by the JTAG debug interface."]
pub type JTAG_STL_R = crate::BitReader<bool>;
#[doc = "Field `JTAG_STL` writer - This bit enables the WDT Stall function if JTAG or SWD debug functions are active 1=The WDT is stalled while either JTAG or SWD is active 0=The WDT is not affected by the JTAG debug interface."]
pub type JTAG_STL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `WDT_RST` reader - If the WDT_RESET bit is set and the watch dog timer expires, the Watch dog module will generate interrupt and clear the WDT_RESET to 0b."]
pub type WDT_RST_R = crate::BitReader<bool>;
#[doc = "Field `WDT_RST` writer - If the WDT_RESET bit is set and the watch dog timer expires, the Watch dog module will generate interrupt and clear the WDT_RESET to 0b."]
pub type WDT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - WDT Block enabled"]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT_STATUS is set by hardware if the last reset of the device was caused by an underflow of the WDT. This bit must be cleared by the EC firmware writing a '1' to this bit. Writing a '0' to this bit has no effect."]
    #[inline(always)]
    pub fn wdt_sts(&self) -> WDT_STS_R {
        WDT_STS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit enables the WDT Stall function if the Hibernation Timer 0 is active. 1=The WDT is stalled while the Hibernation Timer 0 is active 0=The WDT is not affected by Hibernation Timer 0."]
    #[inline(always)]
    pub fn hib_tmr0_stl(&self) -> HIB_TMR0_STL_R {
        HIB_TMR0_STL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit enables the WDT Stall function if the Week Timer is active. 1=The WDT is stalled while the Week Timer is active 0=The WDT is not affected by the Week Timer."]
    #[inline(always)]
    pub fn wk_tmr_stl(&self) -> WK_TMR_STL_R {
        WK_TMR_STL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit enables the WDT Stall function if JTAG or SWD debug functions are active 1=The WDT is stalled while either JTAG or SWD is active 0=The WDT is not affected by the JTAG debug interface."]
    #[inline(always)]
    pub fn jtag_stl(&self) -> JTAG_STL_R {
        JTAG_STL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - If the WDT_RESET bit is set and the watch dog timer expires, the Watch dog module will generate interrupt and clear the WDT_RESET to 0b."]
    #[inline(always)]
    pub fn wdt_rst(&self) -> WDT_RST_R {
        WDT_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Block enabled"]
    #[inline(always)]
    pub fn wdt_en(&mut self) -> WDT_EN_W<0> {
        WDT_EN_W::new(self)
    }
    #[doc = "Bit 1 - WDT_STATUS is set by hardware if the last reset of the device was caused by an underflow of the WDT. This bit must be cleared by the EC firmware writing a '1' to this bit. Writing a '0' to this bit has no effect."]
    #[inline(always)]
    pub fn wdt_sts(&mut self) -> WDT_STS_W<1> {
        WDT_STS_W::new(self)
    }
    #[doc = "Bit 2 - This bit enables the WDT Stall function if the Hibernation Timer 0 is active. 1=The WDT is stalled while the Hibernation Timer 0 is active 0=The WDT is not affected by Hibernation Timer 0."]
    #[inline(always)]
    pub fn hib_tmr0_stl(&mut self) -> HIB_TMR0_STL_W<2> {
        HIB_TMR0_STL_W::new(self)
    }
    #[doc = "Bit 3 - This bit enables the WDT Stall function if the Week Timer is active. 1=The WDT is stalled while the Week Timer is active 0=The WDT is not affected by the Week Timer."]
    #[inline(always)]
    pub fn wk_tmr_stl(&mut self) -> WK_TMR_STL_W<3> {
        WK_TMR_STL_W::new(self)
    }
    #[doc = "Bit 4 - This bit enables the WDT Stall function if JTAG or SWD debug functions are active 1=The WDT is stalled while either JTAG or SWD is active 0=The WDT is not affected by the JTAG debug interface."]
    #[inline(always)]
    pub fn jtag_stl(&mut self) -> JTAG_STL_W<4> {
        JTAG_STL_W::new(self)
    }
    #[doc = "Bit 9 - If the WDT_RESET bit is set and the watch dog timer expires, the Watch dog module will generate interrupt and clear the WDT_RESET to 0b."]
    #[inline(always)]
    pub fn wdt_rst(&mut self) -> WDT_RST_W<9> {
        WDT_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
