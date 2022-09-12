#[doc = "Register `EN_CLR21` reader"]
pub struct R(crate::R<EN_CLR21_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_CLR21_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_CLR21_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_CLR21_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN_CLR21` writer"]
pub struct W(crate::W<EN_CLR21_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_CLR21_SPEC>;
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
impl From<crate::W<EN_CLR21_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_CLR21_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT` reader - WDT"]
pub type WDT_R = crate::BitReader<bool>;
#[doc = "Field `WDT` writer - WDT"]
pub type WDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR21_SPEC, bool, O>;
#[doc = "Field `EMC` reader - EMC"]
pub type EMC_R = crate::BitReader<bool>;
#[doc = "Field `EMC` writer - EMC"]
pub type EMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR21_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - WDT"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 24 - EMC"]
    #[inline(always)]
    pub fn emc(&self) -> EMC_R {
        EMC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - WDT"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<2> {
        WDT_W::new(self)
    }
    #[doc = "Bit 24 - EMC"]
    #[inline(always)]
    pub fn emc(&mut self) -> EMC_W<24> {
        EMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ21 ENABLE CLEAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_clr21](index.html) module"]
pub struct EN_CLR21_SPEC;
impl crate::RegisterSpec for EN_CLR21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en_clr21::R](R) reader structure"]
impl crate::Readable for EN_CLR21_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en_clr21::W](W) writer structure"]
impl crate::Writable for EN_CLR21_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN_CLR21 to value 0"]
impl crate::Resettable for EN_CLR21_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
