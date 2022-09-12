#[doc = "Register `CONV_MOD` reader"]
pub struct R(crate::R<CONV_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONV_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONV_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONV_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONV_MOD` writer"]
pub struct W(crate::W<CONV_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONV_MOD_SPEC>;
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
impl From<crate::W<CONV_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONV_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONV_MOD` reader - Conversion Mode Register"]
pub type CONV_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONV_MOD` writer - Conversion Mode Register"]
pub type CONV_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CONV_MOD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Conversion Mode Register"]
    #[inline(always)]
    pub fn conv_mod(&self) -> CONV_MOD_R {
        CONV_MOD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Conversion Mode Register"]
    #[inline(always)]
    pub fn conv_mod(&mut self) -> CONV_MOD_W<0> {
        CONV_MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Conversion Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conv_mod](index.html) module"]
pub struct CONV_MOD_SPEC;
impl crate::RegisterSpec for CONV_MOD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [conv_mod::R](R) reader structure"]
impl crate::Readable for CONV_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conv_mod::W](W) writer structure"]
impl crate::Writable for CONV_MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONV_MOD to value 0"]
impl crate::Resettable for CONV_MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
