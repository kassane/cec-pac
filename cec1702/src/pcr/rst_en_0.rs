#[doc = "Register `RST_EN_0` reader"]
pub struct R(crate::R<RST_EN_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_EN_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_EN_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_EN_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST_EN_0` writer"]
pub struct W(crate::W<RST_EN_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_EN_0_SPEC>;
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
impl From<crate::W<RST_EN_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_EN_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JTAG_STAP_RST_EN` reader - JTAG STAP Reset Enable"]
pub type JTAG_STAP_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `JTAG_STAP_RST_EN` writer - JTAG STAP Reset Enable"]
pub type JTAG_STAP_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_0_SPEC, bool, O>;
#[doc = "Field `EFUSE_RST_EN` reader - eFuse Reset Enable"]
pub type EFUSE_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_RST_EN` writer - eFuse Reset Enable"]
pub type EFUSE_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - JTAG STAP Reset Enable"]
    #[inline(always)]
    pub fn jtag_stap_rst_en(&self) -> JTAG_STAP_RST_EN_R {
        JTAG_STAP_RST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - eFuse Reset Enable"]
    #[inline(always)]
    pub fn efuse_rst_en(&self) -> EFUSE_RST_EN_R {
        EFUSE_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - JTAG STAP Reset Enable"]
    #[inline(always)]
    pub fn jtag_stap_rst_en(&mut self) -> JTAG_STAP_RST_EN_W<0> {
        JTAG_STAP_RST_EN_W::new(self)
    }
    #[doc = "Bit 1 - eFuse Reset Enable"]
    #[inline(always)]
    pub fn efuse_rst_en(&mut self) -> EFUSE_RST_EN_W<1> {
        EFUSE_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Enable 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_en_0](index.html) module"]
pub struct RST_EN_0_SPEC;
impl crate::RegisterSpec for RST_EN_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst_en_0::R](R) reader structure"]
impl crate::Readable for RST_EN_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_en_0::W](W) writer structure"]
impl crate::Writable for RST_EN_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RST_EN_0 to value 0"]
impl crate::Resettable for RST_EN_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
