#[doc = "Register `FLT_TEMPSTS` reader"]
pub struct R(crate::R<FLT_TEMPSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT_TEMPSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT_TEMPSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT_TEMPSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLT_TEMPSTS` writer"]
pub struct W(crate::W<FLT_TEMPSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT_TEMPSTS_SPEC>;
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
impl From<crate::W<FLT_TEMPSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT_TEMPSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT_TEMPSTS` reader - Stores the status of the External Diode Faults"]
pub type FLT_TEMPSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT_TEMPSTS` writer - Stores the status of the External Diode Faults"]
pub type FLT_TEMPSTS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, FLT_TEMPSTS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Stores the status of the External Diode Faults"]
    #[inline(always)]
    pub fn flt_tempsts(&self) -> FLT_TEMPSTS_R {
        FLT_TEMPSTS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Stores the status of the External Diode Faults"]
    #[inline(always)]
    pub fn flt_tempsts(&mut self) -> FLT_TEMPSTS_W<0> {
        FLT_TEMPSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault temperature Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt_tempsts](index.html) module"]
pub struct FLT_TEMPSTS_SPEC;
impl crate::RegisterSpec for FLT_TEMPSTS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [flt_tempsts::R](R) reader structure"]
impl crate::Readable for FLT_TEMPSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flt_tempsts::W](W) writer structure"]
impl crate::Writable for FLT_TEMPSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLT_TEMPSTS to value 0"]
impl crate::Resettable for FLT_TEMPSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
