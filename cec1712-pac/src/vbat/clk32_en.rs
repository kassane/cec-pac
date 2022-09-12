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
#[doc = "Field `EXT_32K` reader - This bit selects the source for the 32KHz clock domain.\n 1=The 32KHZ_IN VTR-powered pin is used as a source for the 32KHz clock domain. If an activity detector does not detect a\n clock on the selected source, the always-on 32KHz internal clock source is automatically selected\n 0=The always-on32Khz clock source is used as the source for the 32KHz clock domain."]
pub type EXT_32K_R = crate::BitReader<bool>;
#[doc = "Field `EXT_32K` writer - This bit selects the source for the 32KHz clock domain.\n 1=The 32KHZ_IN VTR-powered pin is used as a source for the 32KHz clock domain. If an activity detector does not detect a\n clock on the selected source, the always-on 32KHz internal clock source is automatically selected\n 0=The always-on32Khz clock source is used as the source for the 32KHz clock domain."]
pub type EXT_32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK32_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - This bit selects the source for the 32KHz clock domain.\n 1=The 32KHZ_IN VTR-powered pin is used as a source for the 32KHz clock domain. If an activity detector does not detect a\n clock on the selected source, the always-on 32KHz internal clock source is automatically selected\n 0=The always-on32Khz clock source is used as the source for the 32KHz clock domain."]
    #[inline(always)]
    pub fn ext_32k(&self) -> EXT_32K_R {
        EXT_32K_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit selects the source for the 32KHz clock domain.\n 1=The 32KHZ_IN VTR-powered pin is used as a source for the 32KHz clock domain. If an activity detector does not detect a\n clock on the selected source, the always-on 32KHz internal clock source is automatically selected\n 0=The always-on32Khz clock source is used as the source for the 32KHz clock domain."]
    #[inline(always)]
    pub fn ext_32k(&mut self) -> EXT_32K_W<1> {
        EXT_32K_W::new(self)
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
