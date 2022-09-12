#[doc = "Register `LIM_LO` reader"]
pub struct R(crate::R<LIM_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIM_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIM_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIM_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIM_LO` writer"]
pub struct W(crate::W<LIM_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIM_LO_SPEC>;
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
impl From<crate::W<LIM_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIM_LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_LOW` reader - This value is compared with the value in the TACHX_COUNTER field of the TACHx Control Register. If the value in the counter\n is less than the value programmed in this register, the TACH_OUT_OF_LIMIT_STATUS bit will be set. The TACH_OUT_OF_LIMIT_STATUS\n status event may be enabled to generate an interrupt to the embedded controller via the TACH_OUT_OF_LIMIT_ENABLE bit in the TACHx\n Control Register To disable the TACH_OUT_OF_LIMIT_STATUS low event, program 0000h into this register."]
pub type T_LOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_LOW` writer - This value is compared with the value in the TACHX_COUNTER field of the TACHx Control Register. If the value in the counter\n is less than the value programmed in this register, the TACH_OUT_OF_LIMIT_STATUS bit will be set. The TACH_OUT_OF_LIMIT_STATUS\n status event may be enabled to generate an interrupt to the embedded controller via the TACH_OUT_OF_LIMIT_ENABLE bit in the TACHx\n Control Register To disable the TACH_OUT_OF_LIMIT_STATUS low event, program 0000h into this register."]
pub type T_LOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LIM_LO_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This value is compared with the value in the TACHX_COUNTER field of the TACHx Control Register. If the value in the counter\n is less than the value programmed in this register, the TACH_OUT_OF_LIMIT_STATUS bit will be set. The TACH_OUT_OF_LIMIT_STATUS\n status event may be enabled to generate an interrupt to the embedded controller via the TACH_OUT_OF_LIMIT_ENABLE bit in the TACHx\n Control Register To disable the TACH_OUT_OF_LIMIT_STATUS low event, program 0000h into this register."]
    #[inline(always)]
    pub fn t_low(&self) -> T_LOW_R {
        T_LOW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This value is compared with the value in the TACHX_COUNTER field of the TACHx Control Register. If the value in the counter\n is less than the value programmed in this register, the TACH_OUT_OF_LIMIT_STATUS bit will be set. The TACH_OUT_OF_LIMIT_STATUS\n status event may be enabled to generate an interrupt to the embedded controller via the TACH_OUT_OF_LIMIT_ENABLE bit in the TACHx\n Control Register To disable the TACH_OUT_OF_LIMIT_STATUS low event, program 0000h into this register."]
    #[inline(always)]
    pub fn t_low(&mut self) -> T_LOW_W<0> {
        T_LOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TACHx Low Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lim_lo](index.html) module"]
pub struct LIM_LO_SPEC;
impl crate::RegisterSpec for LIM_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lim_lo::R](R) reader structure"]
impl crate::Readable for LIM_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lim_lo::W](W) writer structure"]
impl crate::Writable for LIM_LO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIM_LO to value 0"]
impl crate::Resettable for LIM_LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
