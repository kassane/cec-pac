#[doc = "Register `EN_SET` reader"]
pub struct R(crate::R<EN_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN_SET` writer"]
pub struct W(crate::W<EN_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_SET_SPEC>;
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
impl From<crate::W<EN_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_SET_SPEC>) -> Self {
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
#[doc = "Write to set source enables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_set](index.html) module"]
pub struct EN_SET_SPEC;
impl crate::RegisterSpec for EN_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en_set::R](R) reader structure"]
impl crate::Readable for EN_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en_set::W](W) writer structure"]
impl crate::Writable for EN_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN_SET to value 0"]
impl crate::Resettable for EN_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
