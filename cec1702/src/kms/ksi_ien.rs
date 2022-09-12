#[doc = "Register `KSI_IEN` reader"]
pub struct R(crate::R<KSI_IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KSI_IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KSI_IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KSI_IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KSI_IEN` writer"]
pub struct W(crate::W<KSI_IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KSI_IEN_SPEC>;
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
impl From<crate::W<KSI_IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KSI_IEN_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "7:0\\]
Each bit in KSI_IEN enables interrupt generation due to highto-low transition on a KSI input.\n An interrupt is generated when the corresponding bits in KSI_STATUS and KSI_INT_EN are both set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ksi_ien](index.html) module"]
pub struct KSI_IEN_SPEC;
impl crate::RegisterSpec for KSI_IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ksi_ien::R](R) reader structure"]
impl crate::Readable for KSI_IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ksi_ien::W](W) writer structure"]
impl crate::Writable for KSI_IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KSI_IEN to value 0"]
impl crate::Resettable for KSI_IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
