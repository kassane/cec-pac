#[doc = "Register `MODE_ALT1` reader"]
pub struct R(crate::R<MODE_ALT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_ALT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_ALT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_ALT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE_ALT1` writer"]
pub struct W(crate::W<MODE_ALT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_ALT1_SPEC>;
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
impl From<crate::W<MODE_ALT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_ALT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS1_ALTMOD_EN` reader - Enable the CS1 Clock Divide to be active if CS1 is the interface in use."]
pub type CS1_ALTMOD_EN_R = crate::BitReader<bool>;
#[doc = "Field `CS1_ALTMOD_EN` writer - Enable the CS1 Clock Divide to be active if CS1 is the interface in use."]
pub type CS1_ALTMOD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_ALT1_SPEC, bool, O>;
#[doc = "Field `CS1_ALTCLK_DIV` reader - The SPI clock divide in number of system clocks when CS1 is in use and CS1 Alt Mode Enable is set."]
pub type CS1_ALTCLK_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CS1_ALTCLK_DIV` writer - The SPI clock divide in number of system clocks when CS1 is in use and CS1 Alt Mode Enable is set."]
pub type CS1_ALTCLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MODE_ALT1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Enable the CS1 Clock Divide to be active if CS1 is the interface in use."]
    #[inline(always)]
    pub fn cs1_altmod_en(&self) -> CS1_ALTMOD_EN_R {
        CS1_ALTMOD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - The SPI clock divide in number of system clocks when CS1 is in use and CS1 Alt Mode Enable is set."]
    #[inline(always)]
    pub fn cs1_altclk_div(&self) -> CS1_ALTCLK_DIV_R {
        CS1_ALTCLK_DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the CS1 Clock Divide to be active if CS1 is the interface in use."]
    #[inline(always)]
    pub fn cs1_altmod_en(&mut self) -> CS1_ALTMOD_EN_W<0> {
        CS1_ALTMOD_EN_W::new(self)
    }
    #[doc = "Bits 16:31 - The SPI clock divide in number of system clocks when CS1 is in use and CS1 Alt Mode Enable is set."]
    #[inline(always)]
    pub fn cs1_altclk_div(&mut self) -> CS1_ALTCLK_DIV_W<16> {
        CS1_ALTCLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Mode Alternate 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode_alt1](index.html) module"]
pub struct MODE_ALT1_SPEC;
impl crate::RegisterSpec for MODE_ALT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode_alt1::R](R) reader structure"]
impl crate::Readable for MODE_ALT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode_alt1::W](W) writer structure"]
impl crate::Writable for MODE_ALT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE_ALT1 to value 0"]
impl crate::Resettable for MODE_ALT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
