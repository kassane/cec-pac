#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_INTEN` reader - WDT_Int_Enable: This is the interrupt enables bit for WDT_INT interrupt.\n 1= WDT_INT Interrupt Enable 0= WDT_INT Interrupt Disabled"]
pub type WDT_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_INTEN` writer - WDT_Int_Enable: This is the interrupt enables bit for WDT_INT interrupt.\n 1= WDT_INT Interrupt Enable 0= WDT_INT Interrupt Disabled"]
pub type WDT_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - WDT_Int_Enable: This is the interrupt enables bit for WDT_INT interrupt.\n 1= WDT_INT Interrupt Enable 0= WDT_INT Interrupt Disabled"]
    #[inline(always)]
    pub fn wdt_inten(&self) -> WDT_INTEN_R {
        WDT_INTEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT_Int_Enable: This is the interrupt enables bit for WDT_INT interrupt.\n 1= WDT_INT Interrupt Enable 0= WDT_INT Interrupt Disabled"]
    #[inline(always)]
    pub fn wdt_inten(&mut self) -> WDT_INTEN_W<0> {
        WDT_INTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watch Dog Interrupt Enable Register.\n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
