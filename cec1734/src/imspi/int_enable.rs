#[doc = "Register `INT_ENABLE` reader"]
pub struct R(crate::R<INT_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENABLE` writer"]
pub struct W(crate::W<INT_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENABLE_SPEC>;
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
impl From<crate::W<INT_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEOUT_LE` reader - Assert an IMSPI interrupt when the TIMEOUT status is asserted. 1=Enable Interrupt; 0=Disable Interrupt"]
pub type TIMEOUT_LE_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT_LE` writer - Assert an IMSPI interrupt when the TIMEOUT status is asserted. 1=Enable Interrupt; 0=Disable Interrupt"]
pub type TIMEOUT_LE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `INVALID_RESPONSE_LE` reader - Assert an EEPROM interrupt when the INVALID_RESPONSE status is asserted. 1=Enable Interrupt; 0=Disable Interrupt"]
pub type INVALID_RESPONSE_LE_R = crate::BitReader<bool>;
#[doc = "Field `INVALID_RESPONSE_LE` writer - Assert an EEPROM interrupt when the INVALID_RESPONSE status is asserted. 1=Enable Interrupt; 0=Disable Interrupt"]
pub type INVALID_RESPONSE_LE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Assert an IMSPI interrupt when the TIMEOUT status is asserted. 1=Enable Interrupt; 0=Disable Interrupt"]
    #[inline(always)]
    pub fn timeout_le(&self) -> TIMEOUT_LE_R {
        TIMEOUT_LE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Assert an EEPROM interrupt when the INVALID_RESPONSE status is asserted. 1=Enable Interrupt; 0=Disable Interrupt"]
    #[inline(always)]
    pub fn invalid_response_le(&self) -> INVALID_RESPONSE_LE_R {
        INVALID_RESPONSE_LE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Assert an IMSPI interrupt when the TIMEOUT status is asserted. 1=Enable Interrupt; 0=Disable Interrupt"]
    #[inline(always)]
    pub fn timeout_le(&mut self) -> TIMEOUT_LE_W<0> {
        TIMEOUT_LE_W::new(self)
    }
    #[doc = "Bit 1 - Assert an EEPROM interrupt when the INVALID_RESPONSE status is asserted. 1=Enable Interrupt; 0=Disable Interrupt"]
    #[inline(always)]
    pub fn invalid_response_le(&mut self) -> INVALID_RESPONSE_LE_W<1> {
        INVALID_RESPONSE_LE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IMSPI Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_enable](index.html) module"]
pub struct INT_ENABLE_SPEC;
impl crate::RegisterSpec for INT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_enable::R](R) reader structure"]
impl crate::Readable for INT_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_enable::W](W) writer structure"]
impl crate::Writable for INT_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENABLE to value 0"]
impl crate::Resettable for INT_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
