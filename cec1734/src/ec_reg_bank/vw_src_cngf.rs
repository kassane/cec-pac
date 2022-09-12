#[doc = "Register `VW_SRC_CNGF` reader"]
pub struct R(crate::R<VW_SRC_CNGF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VW_SRC_CNGF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VW_SRC_CNGF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VW_SRC_CNGF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VW_SRC_CNGF` writer"]
pub struct W(crate::W<VW_SRC_CNGF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VW_SRC_CNGF_SPEC>;
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
impl From<crate::W<VW_SRC_CNGF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VW_SRC_CNGF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VW_SRC` reader - VWIRE_SOURCE \\[2\\], VWIRE_SOURCE \\[1\\], VWIRE_SOURCE \\[0\\]"]
pub type VW_SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VW_SRC` writer - VWIRE_SOURCE \\[2\\], VWIRE_SOURCE \\[1\\], VWIRE_SOURCE \\[0\\]"]
pub type VW_SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VW_SRC_CNGF_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - VWIRE_SOURCE \\[2\\], VWIRE_SOURCE \\[1\\], VWIRE_SOURCE \\[0\\]"]
    #[inline(always)]
    pub fn vw_src(&self) -> VW_SRC_R {
        VW_SRC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - VWIRE_SOURCE \\[2\\], VWIRE_SOURCE \\[1\\], VWIRE_SOURCE \\[0\\]"]
    #[inline(always)]
    pub fn vw_src(&mut self) -> VW_SRC_W<0> {
        VW_SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Virtual Wire Source Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vw_src_cngf](index.html) module"]
pub struct VW_SRC_CNGF_SPEC;
impl crate::RegisterSpec for VW_SRC_CNGF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vw_src_cngf::R](R) reader structure"]
impl crate::Readable for VW_SRC_CNGF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vw_src_cngf::W](W) writer structure"]
impl crate::Writable for VW_SRC_CNGF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VW_SRC_CNGF to value 0x07"]
impl crate::Resettable for VW_SRC_CNGF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
