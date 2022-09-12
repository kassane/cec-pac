#[doc = "Register `MSTART` reader"]
pub struct R(crate::R<MSTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTART` writer"]
pub struct W(crate::W<MSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTART_SPEC>;
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
impl From<crate::W<MSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTART_SPEC>) -> Self {
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
#[doc = "This is the starting address for the Memory device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstart](index.html) module"]
pub struct MSTART_SPEC;
impl crate::RegisterSpec for MSTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstart::R](R) reader structure"]
impl crate::Readable for MSTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstart::W](W) writer structure"]
impl crate::Writable for MSTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSTART to value 0"]
impl crate::Resettable for MSTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
