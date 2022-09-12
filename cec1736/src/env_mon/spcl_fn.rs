#[doc = "Register `SPCL_FN` reader"]
pub struct R(crate::R<SPCL_FN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPCL_FN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPCL_FN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPCL_FN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPCL_FN` writer"]
pub struct W(crate::W<SPCL_FN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPCL_FN_SPEC>;
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
impl From<crate::W<SPCL_FN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPCL_FN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMP_INTSTS` reader - Controls the bit that resets the FailSafe Status Register"]
pub type TMP_INTSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMP_INTSTS` writer - Controls the bit that resets the FailSafe Status Register"]
pub type TMP_INTSTS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SPCL_FN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Controls the bit that resets the FailSafe Status Register"]
    #[inline(always)]
    pub fn tmp_intsts(&self) -> TMP_INTSTS_R {
        TMP_INTSTS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls the bit that resets the FailSafe Status Register"]
    #[inline(always)]
    pub fn tmp_intsts(&mut self) -> TMP_INTSTS_W<0> {
        TMP_INTSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Special Function Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spcl_fn](index.html) module"]
pub struct SPCL_FN_SPEC;
impl crate::RegisterSpec for SPCL_FN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spcl_fn::R](R) reader structure"]
impl crate::Readable for SPCL_FN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spcl_fn::W](W) writer structure"]
impl crate::Writable for SPCL_FN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPCL_FN to value 0"]
impl crate::Resettable for SPCL_FN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
