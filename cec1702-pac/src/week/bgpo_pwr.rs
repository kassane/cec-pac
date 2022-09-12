#[doc = "Register `BGPO_PWR` reader"]
pub struct R(crate::R<BGPO_PWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGPO_PWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGPO_PWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGPO_PWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGPO_PWR` writer"]
pub struct W(crate::W<BGPO_PWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGPO_PWR_SPEC>;
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
impl From<crate::W<BGPO_PWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGPO_PWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BGPO_POWER` reader - Battery powered General Purpose Output power source. For each bit \\[i\\]
in the field:\n 1=BGPO\\[i\\]
is powered by VBAT. The BGPO\\[i\\]
pin is always determined by the corresponding bit in the BGPO Data Register. The GPIO Input register\n for the GPIO that is multiplexed with the BGPO always returns a '1b'.\n 0=The pin for BGPO\\[i\\]
functions as a GPIO. When VTR is powered, the pin associated with BGPO\\[i\\]
is determined by the GPIO associated with the pin.\n When VTR is unpowered, the pin is tristated."]
pub type BGPO_POWER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BGPO_POWER` writer - Battery powered General Purpose Output power source. For each bit \\[i\\]
in the field:\n 1=BGPO\\[i\\]
is powered by VBAT. The BGPO\\[i\\]
pin is always determined by the corresponding bit in the BGPO Data Register. The GPIO Input register\n for the GPIO that is multiplexed with the BGPO always returns a '1b'.\n 0=The pin for BGPO\\[i\\]
functions as a GPIO. When VTR is powered, the pin associated with BGPO\\[i\\]
is determined by the GPIO associated with the pin.\n When VTR is unpowered, the pin is tristated."]
pub type BGPO_POWER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGPO_PWR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 1:5 - Battery powered General Purpose Output power source. For each bit \\[i\\]
in the field:\n 1=BGPO\\[i\\]
is powered by VBAT. The BGPO\\[i\\]
pin is always determined by the corresponding bit in the BGPO Data Register. The GPIO Input register\n for the GPIO that is multiplexed with the BGPO always returns a '1b'.\n 0=The pin for BGPO\\[i\\]
functions as a GPIO. When VTR is powered, the pin associated with BGPO\\[i\\]
is determined by the GPIO associated with the pin.\n When VTR is unpowered, the pin is tristated."]
    #[inline(always)]
    pub fn bgpo_power(&self) -> BGPO_POWER_R {
        BGPO_POWER_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:5 - Battery powered General Purpose Output power source. For each bit \\[i\\]
in the field:\n 1=BGPO\\[i\\]
is powered by VBAT. The BGPO\\[i\\]
pin is always determined by the corresponding bit in the BGPO Data Register. The GPIO Input register\n for the GPIO that is multiplexed with the BGPO always returns a '1b'.\n 0=The pin for BGPO\\[i\\]
functions as a GPIO. When VTR is powered, the pin associated with BGPO\\[i\\]
is determined by the GPIO associated with the pin.\n When VTR is unpowered, the pin is tristated."]
    #[inline(always)]
    pub fn bgpo_power(&mut self) -> BGPO_POWER_W<1> {
        BGPO_POWER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BGPO Power Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgpo_pwr](index.html) module"]
pub struct BGPO_PWR_SPEC;
impl crate::RegisterSpec for BGPO_PWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgpo_pwr::R](R) reader structure"]
impl crate::Readable for BGPO_PWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgpo_pwr::W](W) writer structure"]
impl crate::Writable for BGPO_PWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGPO_PWR to value 0x3e"]
impl crate::Resettable for BGPO_PWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3e
    }
}
