#[doc = "Register `VTT_LIMIT` reader"]
pub struct R(crate::R<VTT_LIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VTT_LIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VTT_LIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VTT_LIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VTT_LIMIT` writer"]
pub struct W(crate::W<VTT_LIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VTT_LIMIT_SPEC>;
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
impl From<crate::W<VTT_LIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VTT_LIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTT_LIMIT` reader - Limit for VTT Voltage Monitor"]
pub type VTT_LIMIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VTT_LIMIT` writer - Limit for VTT Voltage Monitor"]
pub type VTT_LIMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, VTT_LIMIT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Limit for VTT Voltage Monitor"]
    #[inline(always)]
    pub fn vtt_limit(&self) -> VTT_LIMIT_R {
        VTT_LIMIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Limit for VTT Voltage Monitor"]
    #[inline(always)]
    pub fn vtt_limit(&mut self) -> VTT_LIMIT_W<0> {
        VTT_LIMIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VTT Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtt_limit](index.html) module"]
pub struct VTT_LIMIT_SPEC;
impl crate::RegisterSpec for VTT_LIMIT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [vtt_limit::R](R) reader structure"]
impl crate::Readable for VTT_LIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vtt_limit::W](W) writer structure"]
impl crate::Writable for VTT_LIMIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VTT_LIMIT to value 0xff00"]
impl crate::Resettable for VTT_LIMIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff00
    }
}
