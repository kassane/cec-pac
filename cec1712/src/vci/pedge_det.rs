#[doc = "Register `PEDGE_DET` reader"]
pub struct R(crate::R<PEDGE_DET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEDGE_DET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEDGE_DET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEDGE_DET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEDGE_DET` writer"]
pub struct W(crate::W<PEDGE_DET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEDGE_DET_SPEC>;
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
impl From<crate::W<PEDGE_DET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEDGE_DET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCI_IN` reader - These bits record a low to high transition on the VCI_IN# pins. A 1 indicates a transition occurred. For each bit in the field:\n 1=Positive Edge Detected; 0=No edge detected."]
pub type VCI_IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCI_IN` writer - These bits record a low to high transition on the VCI_IN# pins. A 1 indicates a transition occurred. For each bit in the field:\n 1=Positive Edge Detected; 0=No edge detected."]
pub type VCI_IN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PEDGE_DET_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - These bits record a low to high transition on the VCI_IN# pins. A 1 indicates a transition occurred. For each bit in the field:\n 1=Positive Edge Detected; 0=No edge detected."]
    #[inline(always)]
    pub fn vci_in(&self) -> VCI_IN_R {
        VCI_IN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - These bits record a low to high transition on the VCI_IN# pins. A 1 indicates a transition occurred. For each bit in the field:\n 1=Positive Edge Detected; 0=No edge detected."]
    #[inline(always)]
    pub fn vci_in(&mut self) -> VCI_IN_W<0> {
        VCI_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VCI Posedge Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pedge_det](index.html) module"]
pub struct PEDGE_DET_SPEC;
impl crate::RegisterSpec for PEDGE_DET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pedge_det::R](R) reader structure"]
impl crate::Readable for PEDGE_DET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pedge_det::W](W) writer structure"]
impl crate::Writable for PEDGE_DET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEDGE_DET to value 0"]
impl crate::Resettable for PEDGE_DET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
