#[doc = "Register `LIM_HI` reader"]
pub struct R(crate::R<LIM_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIM_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIM_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIM_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIM_HI` writer"]
pub struct W(crate::W<LIM_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIM_HI_SPEC>;
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
impl From<crate::W<LIM_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIM_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_HIGH` reader - This value is compared with the value in the TACHX_COUNTER field. If the value in the counter is greater than the value\n programmed in this register, the TACH_OUT_OF_LIMIT_STATUS bit will be set. The TACH_OUT_OF_LIMIT_STATUS status event may be enabled\n to generate an interrupt to the embedded controller via the TACH_OUT_OF_LIMIT_ENABLE bit in the TACHx Control Register."]
pub type T_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_HIGH` writer - This value is compared with the value in the TACHX_COUNTER field. If the value in the counter is greater than the value\n programmed in this register, the TACH_OUT_OF_LIMIT_STATUS bit will be set. The TACH_OUT_OF_LIMIT_STATUS status event may be enabled\n to generate an interrupt to the embedded controller via the TACH_OUT_OF_LIMIT_ENABLE bit in the TACHx Control Register."]
pub type T_HIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LIM_HI_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This value is compared with the value in the TACHX_COUNTER field. If the value in the counter is greater than the value\n programmed in this register, the TACH_OUT_OF_LIMIT_STATUS bit will be set. The TACH_OUT_OF_LIMIT_STATUS status event may be enabled\n to generate an interrupt to the embedded controller via the TACH_OUT_OF_LIMIT_ENABLE bit in the TACHx Control Register."]
    #[inline(always)]
    pub fn t_high(&self) -> T_HIGH_R {
        T_HIGH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This value is compared with the value in the TACHX_COUNTER field. If the value in the counter is greater than the value\n programmed in this register, the TACH_OUT_OF_LIMIT_STATUS bit will be set. The TACH_OUT_OF_LIMIT_STATUS status event may be enabled\n to generate an interrupt to the embedded controller via the TACH_OUT_OF_LIMIT_ENABLE bit in the TACHx Control Register."]
    #[inline(always)]
    pub fn t_high(&mut self) -> T_HIGH_W<0> {
        T_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TACH HIGH LIMIT Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lim_hi](index.html) module"]
pub struct LIM_HI_SPEC;
impl crate::RegisterSpec for LIM_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lim_hi::R](R) reader structure"]
impl crate::Readable for LIM_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lim_hi::W](W) writer structure"]
impl crate::Writable for LIM_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIM_HI to value 0xffff"]
impl crate::Resettable for LIM_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
