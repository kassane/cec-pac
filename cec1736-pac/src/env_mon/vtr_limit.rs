#[doc = "Register `VTR_LIMIT` reader"]
pub struct R(crate::R<VTR_LIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VTR_LIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VTR_LIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VTR_LIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VTR_LIMIT` writer"]
pub struct W(crate::W<VTR_LIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VTR_LIMIT_SPEC>;
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
impl From<crate::W<VTR_LIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VTR_LIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTR_LIMIT` reader - Limit for VTR Voltage Monitor"]
pub type VTR_LIMIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VTR_LIMIT` writer - Limit for VTR Voltage Monitor"]
pub type VTR_LIMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, VTR_LIMIT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Limit for VTR Voltage Monitor"]
    #[inline(always)]
    pub fn vtr_limit(&self) -> VTR_LIMIT_R {
        VTR_LIMIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Limit for VTR Voltage Monitor"]
    #[inline(always)]
    pub fn vtr_limit(&mut self) -> VTR_LIMIT_W<0> {
        VTR_LIMIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VTR Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtr_limit](index.html) module"]
pub struct VTR_LIMIT_SPEC;
impl crate::RegisterSpec for VTR_LIMIT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [vtr_limit::R](R) reader structure"]
impl crate::Readable for VTR_LIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vtr_limit::W](W) writer structure"]
impl crate::Writable for VTR_LIMIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VTR_LIMIT to value 0xff00"]
impl crate::Resettable for VTR_LIMIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff00
    }
}
