#[doc = "Register `MEM_u32[%s]` reader"]
pub struct R(crate::R<MEM_U32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_U32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_U32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_U32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_u32[%s]` writer"]
pub struct W(crate::W<MEM_U32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_U32_SPEC>;
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
impl From<crate::W<MEM_U32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_U32_SPEC>) -> Self {
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
#[doc = "32-bits of VBAT powered RAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_u32](index.html) module"]
pub struct MEM_U32_SPEC;
impl crate::RegisterSpec for MEM_U32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_u32::R](R) reader structure"]
impl crate::Readable for MEM_U32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_u32::W](W) writer structure"]
impl crate::Writable for MEM_U32_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_u32[%s]
to value 0"]
impl crate::Resettable for MEM_U32_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
