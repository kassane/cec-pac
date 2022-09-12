#[doc = "Register `SRC26` reader"]
pub struct R(crate::R<SRC26_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC26_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC26_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC26_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC26` writer"]
pub struct W(crate::W<SRC26_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC26_SPEC>;
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
impl From<crate::W<SRC26_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC26_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO250` reader - GPIO250"]
pub type GPIO250_R = crate::BitReader<bool>;
#[doc = "Field `GPIO250` writer - GPIO250"]
pub type GPIO250_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC26_SPEC, bool, O>;
#[doc = "Field `GPIO253` reader - GPIO253"]
pub type GPIO253_R = crate::BitReader<bool>;
#[doc = "Field `GPIO253` writer - GPIO253"]
pub type GPIO253_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC26_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - GPIO250"]
    #[inline(always)]
    pub fn gpio250(&self) -> GPIO250_R {
        GPIO250_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO253"]
    #[inline(always)]
    pub fn gpio253(&self) -> GPIO253_R {
        GPIO253_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - GPIO250"]
    #[inline(always)]
    pub fn gpio250(&mut self) -> GPIO250_W<8> {
        GPIO250_W::new(self)
    }
    #[doc = "Bit 11 - GPIO253"]
    #[inline(always)]
    pub fn gpio253(&mut self) -> GPIO253_W<11> {
        GPIO253_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ26 SOURCE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src26](index.html) module"]
pub struct SRC26_SPEC;
impl crate::RegisterSpec for SRC26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src26::R](R) reader structure"]
impl crate::Readable for SRC26_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src26::W](W) writer structure"]
impl crate::Writable for SRC26_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRC26 to value 0"]
impl crate::Resettable for SRC26_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
