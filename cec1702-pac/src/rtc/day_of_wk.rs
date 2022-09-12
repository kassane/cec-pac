#[doc = "Register `DAY_OF_WK` reader"]
pub struct R(crate::R<DAY_OF_WK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAY_OF_WK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAY_OF_WK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAY_OF_WK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAY_OF_WK` writer"]
pub struct W(crate::W<DAY_OF_WK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAY_OF_WK_SPEC>;
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
impl From<crate::W<DAY_OF_WK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAY_OF_WK_SPEC>) -> Self {
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
#[doc = "Day of Week Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [day_of_wk](index.html) module"]
pub struct DAY_OF_WK_SPEC;
impl crate::RegisterSpec for DAY_OF_WK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [day_of_wk::R](R) reader structure"]
impl crate::Readable for DAY_OF_WK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [day_of_wk::W](W) writer structure"]
impl crate::Writable for DAY_OF_WK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAY_OF_WK to value 0"]
impl crate::Resettable for DAY_OF_WK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
