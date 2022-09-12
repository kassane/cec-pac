#[doc = "Register `PEC` reader"]
pub struct R(crate::R<PEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEC` writer"]
pub struct W(crate::W<PEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEC_SPEC>;
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
impl From<crate::W<PEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEC` reader - The SMBus Packet Error Check (PEC) byte."]
pub type PEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PEC` writer - The SMBus Packet Error Check (PEC) byte."]
pub type PEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PEC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The SMBus Packet Error Check (PEC) byte."]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The SMBus Packet Error Check (PEC) byte."]
    #[inline(always)]
    pub fn pec(&mut self) -> PEC_W<0> {
        PEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet Error Check (PEC) Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pec](index.html) module"]
pub struct PEC_SPEC;
impl crate::RegisterSpec for PEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pec::R](R) reader structure"]
impl crate::Readable for PEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pec::W](W) writer structure"]
impl crate::Writable for PEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEC to value 0"]
impl crate::Resettable for PEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
