#[doc = "Register `EN_CLR12` reader"]
pub struct R(crate::R<EN_CLR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_CLR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_CLR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_CLR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN_CLR12` writer"]
pub struct W(crate::W<EN_CLR12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_CLR12_SPEC>;
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
impl From<crate::W<EN_CLR12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_CLR12_SPEC>) -> Self {
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
#[doc = "GIRQ12 Enable Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_clr12](index.html) module"]
pub struct EN_CLR12_SPEC;
impl crate::RegisterSpec for EN_CLR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en_clr12::R](R) reader structure"]
impl crate::Readable for EN_CLR12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en_clr12::W](W) writer structure"]
impl crate::Writable for EN_CLR12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN_CLR12 to value 0"]
impl crate::Resettable for EN_CLR12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
