#[doc = "Register `EN_SET26` reader"]
pub struct R(crate::R<EN_SET26_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_SET26_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_SET26_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_SET26_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN_SET26` writer"]
pub struct W(crate::W<EN_SET26_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_SET26_SPEC>;
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
impl From<crate::W<EN_SET26_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_SET26_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO250` reader - GPIO250"]
pub type GPIO250_R = crate::BitReader<bool>;
#[doc = "Field `GPIO250` writer - GPIO250"]
pub type GPIO250_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET26_SPEC, bool, O>;
#[doc = "Field `GPIO260` reader - GPIO260"]
pub type GPIO260_R = crate::BitReader<bool>;
#[doc = "Field `GPIO260` writer - GPIO260"]
pub type GPIO260_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET26_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - GPIO250"]
    #[inline(always)]
    pub fn gpio250(&self) -> GPIO250_R {
        GPIO250_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO260"]
    #[inline(always)]
    pub fn gpio260(&self) -> GPIO260_R {
        GPIO260_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - GPIO250"]
    #[inline(always)]
    pub fn gpio250(&mut self) -> GPIO250_W<8> {
        GPIO250_W::new(self)
    }
    #[doc = "Bit 11 - GPIO260"]
    #[inline(always)]
    pub fn gpio260(&mut self) -> GPIO260_W<11> {
        GPIO260_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ26 ENABLE SET\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_set26](index.html) module"]
pub struct EN_SET26_SPEC;
impl crate::RegisterSpec for EN_SET26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en_set26::R](R) reader structure"]
impl crate::Readable for EN_SET26_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en_set26::W](W) writer structure"]
impl crate::Writable for EN_SET26_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN_SET26 to value 0"]
impl crate::Resettable for EN_SET26_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
