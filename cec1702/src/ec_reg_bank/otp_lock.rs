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
#[doc = "Field `MCHIP_LOCK` reader - This bit controls access to Microchip region of the eFuse memory, bytes 32 to 127. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the Private Key is inaccessible, independent of the state of this bit.\n 1=The Microchip region is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The Microchip region is accessible."]
pub type MCHIP_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `MCHIP_LOCK` writer - This bit controls access to Microchip region of the eFuse memory, bytes 32 to 127. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the Private Key is inaccessible, independent of the state of this bit.\n 1=The Microchip region is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The Microchip region is accessible."]
pub type MCHIP_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_LOCK_SPEC, bool, O>;
#[doc = "Field `PRIV_KEY_LOCK` reader - This bit controls access to Private Key region of the eFuse memory, bytes 0 to 31. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the Private Key is inaccessible, independent of the state of this bit.\n 1=The Private Key is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The Private Key is accessible."]
pub type PRIV_KEY_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `PRIV_KEY_LOCK` writer - This bit controls access to Private Key region of the eFuse memory, bytes 0 to 31. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the Private Key is inaccessible, independent of the state of this bit.\n 1=The Private Key is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The Private Key is accessible."]
pub type PRIV_KEY_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_LOCK_SPEC, bool, O>;
#[doc = "Field `USER_OTP_LOCK` reader - This bit controls access to the User region of the eFuse memory, bytes 192 to 511. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the User region is inaccessible, independent of the state of this bit.\n 1=The User region is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The User region is accessible"]
pub type USER_OTP_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `USER_OTP_LOCK` writer - This bit controls access to the User region of the eFuse memory, bytes 192 to 511. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the User region is inaccessible, independent of the state of this bit.\n 1=The User region is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The User region is accessible"]
pub type USER_OTP_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_LOCK_SPEC, bool, O>;
#[doc = "Field `PUB_KEY_LOCK` reader - This bit controls access to the Public Key region of the eFuse memory, bytes 128 to 191. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the Public Key is inaccessible, independent of the state of this bit.\n 1=The Public Key is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The Public Key is accessible"]
pub type PUB_KEY_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `PUB_KEY_LOCK` writer - This bit controls access to the Public Key region of the eFuse memory, bytes 128 to 191. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the Public Key is inaccessible, independent of the state of this bit.\n 1=The Public Key is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The Public Key is accessible"]
pub type PUB_KEY_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_LOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Test"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit controls access to Microchip region of the eFuse memory, bytes 32 to 127. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the Private Key is inaccessible, independent of the state of this bit.\n 1=The Microchip region is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The Microchip region is accessible."]
    #[inline(always)]
    pub fn mchip_lock(&self) -> MCHIP_LOCK_R {
        MCHIP_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit controls access to Private Key region of the eFuse memory, bytes 0 to 31. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the Private Key is inaccessible, independent of the state of this bit.\n 1=The Private Key is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The Private Key is accessible."]
    #[inline(always)]
    pub fn priv_key_lock(&self) -> PRIV_KEY_LOCK_R {
        PRIV_KEY_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit controls access to the User region of the eFuse memory, bytes 192 to 511. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the User region is inaccessible, independent of the state of this bit.\n 1=The User region is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The User region is accessible"]
    #[inline(always)]
    pub fn user_otp_lock(&self) -> USER_OTP_LOCK_R {
        USER_OTP_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit controls access to the Public Key region of the eFuse memory, bytes 128 to 191. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the Public Key is inaccessible, independent of the state of this bit.\n 1=The Public Key is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The Public Key is accessible"]
    #[inline(always)]
    pub fn pub_key_lock(&self) -> PUB_KEY_LOCK_R {
        PUB_KEY_LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W<0> {
        TEST_W::new(self)
    }
    #[doc = "Bit 1 - This bit controls access to Microchip region of the eFuse memory, bytes 32 to 127. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the Private Key is inaccessible, independent of the state of this bit.\n 1=The Microchip region is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The Microchip region is accessible."]
    #[inline(always)]
    pub fn mchip_lock(&mut self) -> MCHIP_LOCK_W<1> {
        MCHIP_LOCK_W::new(self)
    }
    #[doc = "Bit 2 - This bit controls access to Private Key region of the eFuse memory, bytes 0 to 31. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the Private Key is inaccessible, independent of the state of this bit.\n 1=The Private Key is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The Private Key is accessible."]
    #[inline(always)]
    pub fn priv_key_lock(&mut self) -> PRIV_KEY_LOCK_W<2> {
        PRIV_KEY_LOCK_W::new(self)
    }
    #[doc = "Bit 3 - This bit controls access to the User region of the eFuse memory, bytes 192 to 511. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the User region is inaccessible, independent of the state of this bit.\n 1=The User region is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The User region is accessible"]
    #[inline(always)]
    pub fn user_otp_lock(&mut self) -> USER_OTP_LOCK_W<3> {
        USER_OTP_LOCK_W::new(self)
    }
    #[doc = "Bit 4 - This bit controls access to the Public Key region of the eFuse memory, bytes 128 to 191. Once written, this bit becomes Read Only.\n If the JTAG_EN bit is 1 (enabled), the Public Key is inaccessible, independent of the state of this bit.\n 1=The Public Key is inaccessible (i.e, always returns 0 or 1 for every bit); 0=The Public Key is accessible"]
    #[inline(always)]
    pub fn pub_key_lock(&mut self) -> PUB_KEY_LOCK_W<4> {
        PUB_KEY_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTP Lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otp_lock](index.html) module"]
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
