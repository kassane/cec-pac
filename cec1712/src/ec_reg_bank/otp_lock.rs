#[doc = "Register `OTP_LOCK` reader"]
pub struct R(crate::R<OTP_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTP_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTP_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTP_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTP_LOCK` writer"]
pub struct W(crate::W<OTP_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTP_LOCK_SPEC>;
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
impl From<crate::W<OTP_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTP_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEST` reader - Test"]
pub type TEST_R = crate::BitReader<bool>;
#[doc = "Field `TEST` writer - Test"]
pub type TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_LOCK_SPEC, bool, O>;
#[doc = "Field `VBAT_RAM_LOCK` reader - VBAT RAM LOCK bit.\n 0 = Not Locked.\n 1 = Locked.\n"]
pub type VBAT_RAM_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `VBAT_RAM_LOCK` writer - VBAT RAM LOCK bit.\n 0 = Not Locked.\n 1 = Locked.\n"]
pub type VBAT_RAM_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_LOCK_SPEC, bool, O>;
#[doc = "Field `VBAT_REG_LOCK` reader - VBAT REG LOCK.\n 0 = Not Locked.\n 1 = Locked.\n"]
pub type VBAT_REG_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `VBAT_REG_LOCK` writer - VBAT REG LOCK.\n 0 = Not Locked.\n 1 = Locked.\n"]
pub type VBAT_REG_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_LOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Test"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBAT RAM LOCK bit.\n 0 = Not Locked.\n 1 = Locked.\n"]
    #[inline(always)]
    pub fn vbat_ram_lock(&self) -> VBAT_RAM_LOCK_R {
        VBAT_RAM_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBAT REG LOCK.\n 0 = Not Locked.\n 1 = Locked.\n"]
    #[inline(always)]
    pub fn vbat_reg_lock(&self) -> VBAT_REG_LOCK_R {
        VBAT_REG_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W<0> {
        TEST_W::new(self)
    }
    #[doc = "Bit 1 - VBAT RAM LOCK bit.\n 0 = Not Locked.\n 1 = Locked.\n"]
    #[inline(always)]
    pub fn vbat_ram_lock(&mut self) -> VBAT_RAM_LOCK_W<1> {
        VBAT_RAM_LOCK_W::new(self)
    }
    #[doc = "Bit 2 - VBAT REG LOCK.\n 0 = Not Locked.\n 1 = Locked.\n"]
    #[inline(always)]
    pub fn vbat_reg_lock(&mut self) -> VBAT_REG_LOCK_W<2> {
        VBAT_REG_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otp_lock](index.html) module"]
pub struct OTP_LOCK_SPEC;
impl crate::RegisterSpec for OTP_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otp_lock::R](R) reader structure"]
impl crate::Readable for OTP_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otp_lock::W](W) writer structure"]
impl crate::Writable for OTP_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTP_LOCK to value 0"]
impl crate::Resettable for OTP_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
