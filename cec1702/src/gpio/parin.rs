#[doc = "Register `PARIN[%s]` reader"]
pub struct R(crate::R<PARIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PARIN[%s]` writer"]
pub struct W(crate::W<PARIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PARIN_SPEC>;
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
impl From<crate::W<PARIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PARIN_SPEC>) -> Self {
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
#[doc = "The GPIO Input Registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parin](index.html) module"]
pub struct PARIN_SPEC;
impl crate::RegisterSpec for PARIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [parin::R](R) reader structure"]
impl crate::Readable for PARIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [parin::W](W) writer structure"]
impl crate::Writable for PARIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PARIN[%s]
to value 0"]
impl crate::Resettable for PARIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
