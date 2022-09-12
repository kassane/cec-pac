#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEOUT` reader - This flags when a transfer has terminated due to timeout on the response phase. 1=A transfer error occurred due to an invalid response; 0=No error occurred. (R/WC)"]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` writer - This flags when a transfer has terminated due to timeout on the response phase. 1=A transfer error occurred due to an invalid response; 0=No error occurred. (R/WC)"]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `INVALID_RESPONSE` reader - The IMSPI has detected an invalid response field and therefore is aborting the transfer in failure. 1=A transfer error occurred due to an invalid response; 0=No error occurred. (R/WC)"]
pub type INVALID_RESPONSE_R = crate::BitReader<bool>;
#[doc = "Field `INVALID_RESPONSE` writer - The IMSPI has detected an invalid response field and therefore is aborting the transfer in failure. 1=A transfer error occurred due to an invalid response; 0=No error occurred. (R/WC)"]
pub type INVALID_RESPONSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This flags when a transfer has terminated due to timeout on the response phase. 1=A transfer error occurred due to an invalid response; 0=No error occurred. (R/WC)"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The IMSPI has detected an invalid response field and therefore is aborting the transfer in failure. 1=A transfer error occurred due to an invalid response; 0=No error occurred. (R/WC)"]
    #[inline(always)]
    pub fn invalid_response(&self) -> INVALID_RESPONSE_R {
        INVALID_RESPONSE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This flags when a transfer has terminated due to timeout on the response phase. 1=A transfer error occurred due to an invalid response; 0=No error occurred. (R/WC)"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<0> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 1 - The IMSPI has detected an invalid response field and therefore is aborting the transfer in failure. 1=A transfer error occurred due to an invalid response; 0=No error occurred. (R/WC)"]
    #[inline(always)]
    pub fn invalid_response(&mut self) -> INVALID_RESPONSE_W<1> {
        INVALID_RESPONSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IMSPI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
