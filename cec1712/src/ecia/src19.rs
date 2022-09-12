#[doc = "Register `SRC19` reader"]
pub struct R(crate::R<SRC19_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC19_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC19_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC19_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC19` writer"]
pub struct W(crate::W<SRC19_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC19_SPEC>;
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
impl From<crate::W<SRC19_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC19_SPEC>) -> Self {
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
#[doc = "GIRQ19 Source Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src19](index.html) module"]
pub struct SRC19_SPEC;
impl crate::RegisterSpec for SRC19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src19::R](R) reader structure"]
impl crate::Readable for SRC19_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src19::W](W) writer structure"]
impl crate::Writable for SRC19_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRC19 to value 0"]
impl crate::Resettable for SRC19_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
