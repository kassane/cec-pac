#[doc = "Register `VCP_LIMIT` reader"]
pub struct R(crate::R<VCP_LIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCP_LIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCP_LIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCP_LIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VCP_LIMIT` writer"]
pub struct W(crate::W<VCP_LIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCP_LIMIT_SPEC>;
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
impl From<crate::W<VCP_LIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCP_LIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCP_LIMIT` reader - Limit for VCP Voltage Monitor"]
pub type VCP_LIMIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VCP_LIMIT` writer - Limit for VCP Voltage Monitor"]
pub type VCP_LIMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, VCP_LIMIT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Limit for VCP Voltage Monitor"]
    #[inline(always)]
    pub fn vcp_limit(&self) -> VCP_LIMIT_R {
        VCP_LIMIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Limit for VCP Voltage Monitor"]
    #[inline(always)]
    pub fn vcp_limit(&mut self) -> VCP_LIMIT_W<0> {
        VCP_LIMIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VCP Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vcp_limit](index.html) module"]
pub struct VCP_LIMIT_SPEC;
impl crate::RegisterSpec for VCP_LIMIT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [vcp_limit::R](R) reader structure"]
impl crate::Readable for VCP_LIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vcp_limit::W](W) writer structure"]
impl crate::Writable for VCP_LIMIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VCP_LIMIT to value 0"]
impl crate::Resettable for VCP_LIMIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
