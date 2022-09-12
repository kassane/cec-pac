#[doc = "Register `AHB_ERR_ADDR` reader"]
pub struct R(crate::R<AHB_ERR_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_ERR_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_ERR_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_ERR_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_ERR_ADDR` writer"]
pub struct W(crate::W<AHB_ERR_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_ERR_ADDR_SPEC>;
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
impl From<crate::W<AHB_ERR_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_ERR_ADDR_SPEC>) -> Self {
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
#[doc = "AHB Error Address \\[0:0\\]
AHB_ERR_ADDR, In priority order:\n 1. AHB address is registered when an AHB error occurs on the processor's AHB master port and the register value was\n already 0. This way only the first address to generate an exception is captured.\n 2. The processor can clear this register by writing any 32-bit value to this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_err_addr](index.html) module"]
pub struct AHB_ERR_ADDR_SPEC;
impl crate::RegisterSpec for AHB_ERR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_err_addr::R](R) reader structure"]
impl crate::Readable for AHB_ERR_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_err_addr::W](W) writer structure"]
impl crate::Writable for AHB_ERR_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB_ERR_ADDR to value 0"]
impl crate::Resettable for AHB_ERR_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
