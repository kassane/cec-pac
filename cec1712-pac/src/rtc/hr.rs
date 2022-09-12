#[doc = "Register `HR` reader"]
pub struct R(crate::R<HR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HR` writer"]
pub struct W(crate::W<HR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HR_SPEC>;
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
impl From<crate::W<HR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hours Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hr](index.html) module"]
pub struct HR_SPEC;
impl crate::RegisterSpec for HR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hr::R](R) reader structure"]
impl crate::Readable for HR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hr::W](W) writer structure"]
impl crate::Writable for HR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HR to value 0"]
impl crate::Resettable for HR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
