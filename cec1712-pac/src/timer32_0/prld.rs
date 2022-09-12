#[doc = "Register `PRLD` reader"]
pub struct R(crate::R<PRLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRLD` writer"]
pub struct W(crate::W<PRLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRLD_SPEC>;
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
impl From<crate::W<PRLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRLD_SPEC>) -> Self {
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
#[doc = "This is the value of the Timer pre-load for the counter. This is used by H/W when the counter is to be restarted\n automatically; this will become the new value of the counter upon restart.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prld](index.html) module"]
pub struct PRLD_SPEC;
impl crate::RegisterSpec for PRLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prld::R](R) reader structure"]
impl crate::Readable for PRLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prld::W](W) writer structure"]
impl crate::Writable for PRLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRLD to value 0"]
impl crate::Resettable for PRLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
