#[doc = "Register `RD_LOCK0` reader"]
pub struct R(crate::R<RD_LOCK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_LOCK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_LOCK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_LOCK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_LOCK0` writer"]
pub struct W(crate::W<RD_LOCK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_LOCK0_SPEC>;
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
impl From<crate::W<RD_LOCK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_LOCK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RL0` reader - When any of the bits are set, the corresponding 32byte range in the OTP is not readable."]
pub type RL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RL0` writer - When any of the bits are set, the corresponding 32byte range in the OTP is not readable."]
pub type RL0_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RD_LOCK0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - When any of the bits are set, the corresponding 32byte range in the OTP is not readable."]
    #[inline(always)]
    pub fn rl0(&self) -> RL0_R {
        RL0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - When any of the bits are set, the corresponding 32byte range in the OTP is not readable."]
    #[inline(always)]
    pub fn rl0(&mut self) -> RL0_W<0> {
        RL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the Read Lock Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_lock0](index.html) module"]
pub struct RD_LOCK0_SPEC;
impl crate::RegisterSpec for RD_LOCK0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rd_lock0::R](R) reader structure"]
impl crate::Readable for RD_LOCK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_lock0::W](W) writer structure"]
impl crate::Writable for RD_LOCK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RD_LOCK0 to value 0"]
impl crate::Resettable for RD_LOCK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
