#[doc = "Register `KSI_STS` reader"]
pub struct R(crate::R<KSI_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KSI_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KSI_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KSI_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KSI_STS` writer"]
pub struct W(crate::W<KSI_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KSI_STS_SPEC>;
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
impl From<crate::W<KSI_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KSI_STS_SPEC>) -> Self {
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
Each bit in this field is set on the falling edge of the corresponding KSI input pin.\n A KSI interrupt is generated when its corresponding status bit and interrupt enable bit are both set. KSI interrupts are logically ORed together to produce KSC_INT and KSC_INT_WAKE.\n Writing a '1' to a bit will clear it. Writing a '0' to a bit has no effect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ksi_sts](index.html) module"]
pub struct KSI_STS_SPEC;
impl crate::RegisterSpec for KSI_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ksi_sts::R](R) reader structure"]
impl crate::Readable for KSI_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ksi_sts::W](W) writer structure"]
impl crate::Writable for KSI_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KSI_STS to value 0"]
impl crate::Resettable for KSI_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
