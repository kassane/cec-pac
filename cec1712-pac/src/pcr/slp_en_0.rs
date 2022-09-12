#[doc = "Register `SLP_EN_0` reader"]
pub struct R(crate::R<SLP_EN_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_EN_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_EN_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_EN_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_EN_0` writer"]
pub struct W(crate::W<SLP_EN_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_EN_0_SPEC>;
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
impl From<crate::W<SLP_EN_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_EN_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTP_SLP_EN` reader - OTP Enable"]
pub type OTP_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `OTP_SLP_EN` writer - OTP Enable"]
pub type OTP_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - OTP Enable"]
    #[inline(always)]
    pub fn otp_slp_en(&self) -> OTP_SLP_EN_R {
        OTP_SLP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - OTP Enable"]
    #[inline(always)]
    pub fn otp_slp_en(&mut self) -> OTP_SLP_EN_W<1> {
        OTP_SLP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Enable 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_en_0](index.html) module"]
pub struct SLP_EN_0_SPEC;
impl crate::RegisterSpec for SLP_EN_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_en_0::R](R) reader structure"]
impl crate::Readable for SLP_EN_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_en_0::W](W) writer structure"]
impl crate::Writable for SLP_EN_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_EN_0 to value 0"]
impl crate::Resettable for SLP_EN_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
