#[doc = "Register `ACTIVATE` reader"]
pub struct R(crate::R<ACTIVATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTIVATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTIVATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTIVATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTIVATE` writer"]
pub struct W(crate::W<ACTIVATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTIVATE_SPEC>;
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
impl From<crate::W<ACTIVATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTIVATE_SPEC>) -> Self {
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
#[doc = "UART Activate Register. \\[0:0\\]
ACTIVATE When this bit is 1, the UART logical device is powered and functional. When this bit is 0, the UART logical device is powered down and inactive.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [activate](index.html) module"]
pub struct ACTIVATE_SPEC;
impl crate::RegisterSpec for ACTIVATE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [activate::R](R) reader structure"]
impl crate::Readable for ACTIVATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [activate::W](W) writer structure"]
impl crate::Writable for ACTIVATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACTIVATE to value 0"]
impl crate::Resettable for ACTIVATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
