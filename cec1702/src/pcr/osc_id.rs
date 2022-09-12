#[doc = "Register `OSC_ID` reader"]
pub struct R(crate::R<OSC_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_ID` writer"]
pub struct W(crate::W<OSC_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_ID_SPEC>;
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
impl From<crate::W<OSC_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEST` reader - Test bits"]
pub type TEST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEST` writer - Test bits"]
pub type TEST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSC_ID_SPEC, u8, u8, 8, O>;
#[doc = "Field `PLL_LOCK` reader - PLL Lock Status"]
pub type PLL_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `PLL_LOCK` writer - PLL Lock Status"]
pub type PLL_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_ID_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Test bits"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PLL Lock Status"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PLL_LOCK_R {
        PLL_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Test bits"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W<0> {
        TEST_W::new(self)
    }
    #[doc = "Bit 8 - PLL Lock Status"]
    #[inline(always)]
    pub fn pll_lock(&mut self) -> PLL_LOCK_W<8> {
        PLL_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_id](index.html) module"]
pub struct OSC_ID_SPEC;
impl crate::RegisterSpec for OSC_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_id::R](R) reader structure"]
impl crate::Readable for OSC_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_id::W](W) writer structure"]
impl crate::Writable for OSC_ID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC_ID to value 0"]
impl crate::Resettable for OSC_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
