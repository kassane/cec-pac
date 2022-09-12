#[doc = "Register `STS` reader"]
pub struct R(crate::R<STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STS` writer"]
pub struct W(crate::W<STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STS_SPEC>;
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
impl From<crate::W<STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAN_STALL` reader - The bit Indicates that the tachometer measurement on the Fan detects a stalled fan. (R/WC)\n 0 - Stalled fan not detected.\n 1 - Stalled fan detected."]
pub type FAN_STALL_R = crate::BitReader<bool>;
#[doc = "Field `FAN_STALL` writer - The bit Indicates that the tachometer measurement on the Fan detects a stalled fan. (R/WC)\n 0 - Stalled fan not detected.\n 1 - Stalled fan detected."]
pub type FAN_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u8, STS_SPEC, bool, O>;
#[doc = "Field `FAN_SPIN` reader - The bit Indicates that the Spin up Routine for the Fan could not detect a valid tachometer reading within its maximum\n time window. (R/WC)\n 1 - The Spin up Routine for the Fan could not detect a valid tachometer reading within its maximum time window.\n 0 - The Spin up Routine for the Fan detected a valid tachometer reading within its maximum time window."]
pub type FAN_SPIN_R = crate::BitReader<bool>;
#[doc = "Field `FAN_SPIN` writer - The bit Indicates that the Spin up Routine for the Fan could not detect a valid tachometer reading within its maximum\n time window. (R/WC)\n 1 - The Spin up Routine for the Fan could not detect a valid tachometer reading within its maximum time window.\n 0 - The Spin up Routine for the Fan detected a valid tachometer reading within its maximum time window."]
pub type FAN_SPIN_W<'a, const O: u8> = crate::BitWriter<'a, u8, STS_SPEC, bool, O>;
#[doc = "Field `DRIVE_FAIL` reader - The bit Indicates that the RPM-based Fan Speed Control Algorithm cannot drive the Fan to the desired target setting at\n maximum drive. (R/WC)\n 1- The RPM-based Fan Speed Control Algorithm cannot drive Fan to the desired target setting at maximum drive.\n 0- The RPM-based Fan Speed Control Algorithm can drive Fan to the desired target setting."]
pub type DRIVE_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `DRIVE_FAIL` writer - The bit Indicates that the RPM-based Fan Speed Control Algorithm cannot drive the Fan to the desired target setting at\n maximum drive. (R/WC)\n 1- The RPM-based Fan Speed Control Algorithm cannot drive Fan to the desired target setting at maximum drive.\n 0- The RPM-based Fan Speed Control Algorithm can drive Fan to the desired target setting."]
pub type DRIVE_FAIL_W<'a, const O: u8> = crate::BitWriter<'a, u8, STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The bit Indicates that the tachometer measurement on the Fan detects a stalled fan. (R/WC)\n 0 - Stalled fan not detected.\n 1 - Stalled fan detected."]
    #[inline(always)]
    pub fn fan_stall(&self) -> FAN_STALL_R {
        FAN_STALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit Indicates that the Spin up Routine for the Fan could not detect a valid tachometer reading within its maximum\n time window. (R/WC)\n 1 - The Spin up Routine for the Fan could not detect a valid tachometer reading within its maximum time window.\n 0 - The Spin up Routine for the Fan detected a valid tachometer reading within its maximum time window."]
    #[inline(always)]
    pub fn fan_spin(&self) -> FAN_SPIN_R {
        FAN_SPIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit Indicates that the RPM-based Fan Speed Control Algorithm cannot drive the Fan to the desired target setting at\n maximum drive. (R/WC)\n 1- The RPM-based Fan Speed Control Algorithm cannot drive Fan to the desired target setting at maximum drive.\n 0- The RPM-based Fan Speed Control Algorithm can drive Fan to the desired target setting."]
    #[inline(always)]
    pub fn drive_fail(&self) -> DRIVE_FAIL_R {
        DRIVE_FAIL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit Indicates that the tachometer measurement on the Fan detects a stalled fan. (R/WC)\n 0 - Stalled fan not detected.\n 1 - Stalled fan detected."]
    #[inline(always)]
    pub fn fan_stall(&mut self) -> FAN_STALL_W<0> {
        FAN_STALL_W::new(self)
    }
    #[doc = "Bit 1 - The bit Indicates that the Spin up Routine for the Fan could not detect a valid tachometer reading within its maximum\n time window. (R/WC)\n 1 - The Spin up Routine for the Fan could not detect a valid tachometer reading within its maximum time window.\n 0 - The Spin up Routine for the Fan detected a valid tachometer reading within its maximum time window."]
    #[inline(always)]
    pub fn fan_spin(&mut self) -> FAN_SPIN_W<1> {
        FAN_SPIN_W::new(self)
    }
    #[doc = "Bit 5 - The bit Indicates that the RPM-based Fan Speed Control Algorithm cannot drive the Fan to the desired target setting at\n maximum drive. (R/WC)\n 1- The RPM-based Fan Speed Control Algorithm cannot drive Fan to the desired target setting at maximum drive.\n 0- The RPM-based Fan Speed Control Algorithm can drive Fan to the desired target setting."]
    #[inline(always)]
    pub fn drive_fail(&mut self) -> DRIVE_FAIL_W<5> {
        DRIVE_FAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The bits in this register are routed to interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sts::R](R) reader structure"]
impl crate::Readable for STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sts::W](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
