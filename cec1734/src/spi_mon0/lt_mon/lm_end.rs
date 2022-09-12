#[doc = "Register `LM_END` reader"]
pub struct R(crate::R<LM_END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LM_END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LM_END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LM_END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LM_END` writer"]
pub struct W(crate::W<LM_END_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LM_END_SPEC>;
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
impl From<crate::W<LM_END_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LM_END_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EADDR` reader - A byte address within the designated Flash, specifying the last byte of the load image."]
pub type EADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EADDR` writer - A byte address within the designated Flash, specifying the last byte of the load image."]
pub type EADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LM_END_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - A byte address within the designated Flash, specifying the last byte of the load image."]
    #[inline(always)]
    pub fn eaddr(&self) -> EADDR_R {
        EADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A byte address within the designated Flash, specifying the last byte of the load image."]
    #[inline(always)]
    pub fn eaddr(&mut self) -> EADDR_W<0> {
        EADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Loadtime Monitor Channel End Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lm_end](index.html) module"]
pub struct LM_END_SPEC;
impl crate::RegisterSpec for LM_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lm_end::R](R) reader structure"]
impl crate::Readable for LM_END_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lm_end::W](W) writer structure"]
impl crate::Writable for LM_END_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LM_END to value 0"]
impl crate::Resettable for LM_END_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
