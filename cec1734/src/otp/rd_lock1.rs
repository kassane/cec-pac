#[doc = "Register `RD_LOCK1` reader"]
pub struct R(crate::R<RD_LOCK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_LOCK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_LOCK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_LOCK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_LOCK1` writer"]
pub struct W(crate::W<RD_LOCK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_LOCK1_SPEC>;
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
impl From<crate::W<RD_LOCK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_LOCK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RL1` reader - When any of the bits are set, the corresponding 32byte range in the OTP is not readable."]
pub type RL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RL1` writer - When any of the bits are set, the corresponding 32byte range in the OTP is not readable."]
pub type RL1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RD_LOCK1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - When any of the bits are set, the corresponding 32byte range in the OTP is not readable."]
    #[inline(always)]
    pub fn rl1(&self) -> RL1_R {
        RL1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - When any of the bits are set, the corresponding 32byte range in the OTP is not readable."]
    #[inline(always)]
    pub fn rl1(&mut self) -> RL1_W<0> {
        RL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the Read Lock Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_lock1](index.html) module"]
pub struct RD_LOCK1_SPEC;
impl crate::RegisterSpec for RD_LOCK1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rd_lock1::R](R) reader structure"]
impl crate::Readable for RD_LOCK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_lock1::W](W) writer structure"]
impl crate::Writable for RD_LOCK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RD_LOCK1 to value 0"]
impl crate::Resettable for RD_LOCK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
