#[doc = "Register `CLK32_EN` reader"]
pub struct R(crate::R<CLK32_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK32_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK32_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK32_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK32_EN` writer"]
pub struct W(crate::W<CLK32_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK32_EN_SPEC>;
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
impl From<crate::W<CLK32_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK32_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C32K_SUPPRESS` reader - 1=32KHz clock domain is off while VTR is off (i.e., while on VBAT only). The 32KHz domain is always on\n while VTR is on, so the PLL always has a reference.\n 0=32KHz clock domain is enabled while VTR is off (i.e., while on VBAT only). The clock source for the 32KHz domain is\n determined by the other bits in this register"]
pub type C32K_SUPPRESS_R = crate::BitReader<bool>;
#[doc = "Field `C32K_SUPPRESS` writer - 1=32KHz clock domain is off while VTR is off (i.e., while on VBAT only). The 32KHz domain is always on\n while VTR is on, so the PLL always has a reference.\n 0=32KHz clock domain is enabled while VTR is off (i.e., while on VBAT only). The clock source for the 32KHz domain is\n determined by the other bits in this register"]
pub type C32K_SUPPRESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK32_EN_SPEC, bool, O>;
#[doc = "Field `EXT_32K` reader - This bit selects the source for the 32KHz clock domain.\n 1=The 32KHZ_IN VTR-powered pin is used as a source for the 32KHz clock domain. If an activity detector does not detect a\n clock on the selected source, the always-on 32KHz internal clock source is automatically selected\n 0=The always-on32Khz clock source is used as the source for the 32KHz clock domain."]
pub type EXT_32K_R = crate::BitReader<bool>;
#[doc = "Field `EXT_32K` writer - This bit selects the source for the 32KHz clock domain.\n 1=The 32KHZ_IN VTR-powered pin is used as a source for the 32KHz clock domain. If an activity detector does not detect a\n clock on the selected source, the always-on 32KHz internal clock source is automatically selected\n 0=The always-on32Khz clock source is used as the source for the 32KHz clock domain."]
pub type EXT_32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK32_EN_SPEC, bool, O>;
#[doc = "Field `C32KHZ_SRC` reader - This field determines the source for the always-on 32KHz internal clock source. If set to '1b', this bit\n will only take effect if an active clock has been detected on the crystal pins. Once the 32KHz source has been switched,\n activity detection on the crystal no longer functions. Therefore, if the crystal oscillator uses a single-ended\n input, once started that input must not stop while this bit is '1b'.\n 1=Crystal Oscillator. The selection between a singled-ended input or a resonant crystal is determined by XOSEL in this register\n 0=Silicon Oscillator."]
pub type C32KHZ_SRC_R = crate::BitReader<bool>;
#[doc = "Field `C32KHZ_SRC` writer - This field determines the source for the always-on 32KHz internal clock source. If set to '1b', this bit\n will only take effect if an active clock has been detected on the crystal pins. Once the 32KHz source has been switched,\n activity detection on the crystal no longer functions. Therefore, if the crystal oscillator uses a single-ended\n input, once started that input must not stop while this bit is '1b'.\n 1=Crystal Oscillator. The selection between a singled-ended input or a resonant crystal is determined by XOSEL in this register\n 0=Silicon Oscillator."]
pub type C32KHZ_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK32_EN_SPEC, bool, O>;
#[doc = "Field `XOSEL` reader - This bit selects between a single-ended clock source for the crystal oscillator or an external parallel crystal.\n 1= the Crystal Oscillator is driven by a single-ended 32KHz clock source connected to the XTAL2 pin. \n 0= the Crystal Oscillator requires a 32KHz parallel resonant crystal connected between the XTAL1 and XTAL2 pins (default)."]
pub type XOSEL_R = crate::BitReader<bool>;
#[doc = "Field `XOSEL` writer - This bit selects between a single-ended clock source for the crystal oscillator or an external parallel crystal.\n 1= the Crystal Oscillator is driven by a single-ended 32KHz clock source connected to the XTAL2 pin. \n 0= the Crystal Oscillator requires a 32KHz parallel resonant crystal connected between the XTAL1 and XTAL2 pins (default)."]
pub type XOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK32_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 1=32KHz clock domain is off while VTR is off (i.e., while on VBAT only). The 32KHz domain is always on\n while VTR is on, so the PLL always has a reference.\n 0=32KHz clock domain is enabled while VTR is off (i.e., while on VBAT only). The clock source for the 32KHz domain is\n determined by the other bits in this register"]
    #[inline(always)]
    pub fn c32k_suppress(&self) -> C32K_SUPPRESS_R {
        C32K_SUPPRESS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit selects the source for the 32KHz clock domain.\n 1=The 32KHZ_IN VTR-powered pin is used as a source for the 32KHz clock domain. If an activity detector does not detect a\n clock on the selected source, the always-on 32KHz internal clock source is automatically selected\n 0=The always-on32Khz clock source is used as the source for the 32KHz clock domain."]
    #[inline(always)]
    pub fn ext_32k(&self) -> EXT_32K_R {
        EXT_32K_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This field determines the source for the always-on 32KHz internal clock source. If set to '1b', this bit\n will only take effect if an active clock has been detected on the crystal pins. Once the 32KHz source has been switched,\n activity detection on the crystal no longer functions. Therefore, if the crystal oscillator uses a single-ended\n input, once started that input must not stop while this bit is '1b'.\n 1=Crystal Oscillator. The selection between a singled-ended input or a resonant crystal is determined by XOSEL in this register\n 0=Silicon Oscillator."]
    #[inline(always)]
    pub fn c32khz_src(&self) -> C32KHZ_SRC_R {
        C32KHZ_SRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit selects between a single-ended clock source for the crystal oscillator or an external parallel crystal.\n 1= the Crystal Oscillator is driven by a single-ended 32KHz clock source connected to the XTAL2 pin. \n 0= the Crystal Oscillator requires a 32KHz parallel resonant crystal connected between the XTAL1 and XTAL2 pins (default)."]
    #[inline(always)]
    pub fn xosel(&self) -> XOSEL_R {
        XOSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1=32KHz clock domain is off while VTR is off (i.e., while on VBAT only). The 32KHz domain is always on\n while VTR is on, so the PLL always has a reference.\n 0=32KHz clock domain is enabled while VTR is off (i.e., while on VBAT only). The clock source for the 32KHz domain is\n determined by the other bits in this register"]
    #[inline(always)]
    pub fn c32k_suppress(&mut self) -> C32K_SUPPRESS_W<0> {
        C32K_SUPPRESS_W::new(self)
    }
    #[doc = "Bit 1 - This bit selects the source for the 32KHz clock domain.\n 1=The 32KHZ_IN VTR-powered pin is used as a source for the 32KHz clock domain. If an activity detector does not detect a\n clock on the selected source, the always-on 32KHz internal clock source is automatically selected\n 0=The always-on32Khz clock source is used as the source for the 32KHz clock domain."]
    #[inline(always)]
    pub fn ext_32k(&mut self) -> EXT_32K_W<1> {
        EXT_32K_W::new(self)
    }
    #[doc = "Bit 2 - This field determines the source for the always-on 32KHz internal clock source. If set to '1b', this bit\n will only take effect if an active clock has been detected on the crystal pins. Once the 32KHz source has been switched,\n activity detection on the crystal no longer functions. Therefore, if the crystal oscillator uses a single-ended\n input, once started that input must not stop while this bit is '1b'.\n 1=Crystal Oscillator. The selection between a singled-ended input or a resonant crystal is determined by XOSEL in this register\n 0=Silicon Oscillator."]
    #[inline(always)]
    pub fn c32khz_src(&mut self) -> C32KHZ_SRC_W<2> {
        C32KHZ_SRC_W::new(self)
    }
    #[doc = "Bit 3 - This bit selects between a single-ended clock source for the crystal oscillator or an external parallel crystal.\n 1= the Crystal Oscillator is driven by a single-ended 32KHz clock source connected to the XTAL2 pin. \n 0= the Crystal Oscillator requires a 32KHz parallel resonant crystal connected between the XTAL1 and XTAL2 pins (default)."]
    #[inline(always)]
    pub fn xosel(&mut self) -> XOSEL_W<3> {
        XOSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLOCK ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk32_en](index.html) module"]
pub struct CLK32_EN_SPEC;
impl crate::RegisterSpec for CLK32_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk32_en::R](R) reader structure"]
impl crate::Readable for CLK32_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk32_en::W](W) writer structure"]
impl crate::Writable for CLK32_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK32_EN to value 0"]
impl crate::Resettable for CLK32_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
