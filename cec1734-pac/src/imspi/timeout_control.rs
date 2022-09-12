#[doc = "Register `TIMEOUT_CONTROL` reader"]
pub struct R(crate::R<TIMEOUT_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEOUT_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEOUT_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEOUT_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMEOUT_CONTROL` writer"]
pub struct W(crate::W<TIMEOUT_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEOUT_CONTROL_SPEC>;
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
impl From<crate::W<TIMEOUT_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEOUT_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESPONSE_TIMEOUT` reader - This field is the maximum number of response cycles the IMSPI will wait until flagging a timeout. A setting of 0 will disable the timeout feature."]
pub type RESPONSE_TIMEOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESPONSE_TIMEOUT` writer - This field is the maximum number of response cycles the IMSPI will wait until flagging a timeout. A setting of 0 will disable the timeout feature."]
pub type RESPONSE_TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMEOUT_CONTROL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - This field is the maximum number of response cycles the IMSPI will wait until flagging a timeout. A setting of 0 will disable the timeout feature."]
    #[inline(always)]
    pub fn response_timeout(&self) -> RESPONSE_TIMEOUT_R {
        RESPONSE_TIMEOUT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This field is the maximum number of response cycles the IMSPI will wait until flagging a timeout. A setting of 0 will disable the timeout feature."]
    #[inline(always)]
    pub fn response_timeout(&mut self) -> RESPONSE_TIMEOUT_W<0> {
        RESPONSE_TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IMSPI Timeout Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeout_control](index.html) module"]
pub struct TIMEOUT_CONTROL_SPEC;
impl crate::RegisterSpec for TIMEOUT_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timeout_control::R](R) reader structure"]
impl crate::Readable for TIMEOUT_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timeout_control::W](W) writer structure"]
impl crate::Writable for TIMEOUT_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMEOUT_CONTROL to value 0"]
impl crate::Resettable for TIMEOUT_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
