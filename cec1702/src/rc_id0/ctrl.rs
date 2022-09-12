#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - This bit is cleared to 0 when the RC_ID interface is in the Reset phase, and set to 1 when the interface completes an RC_ID measurement."]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - This bit is cleared to 0 when the RC_ID interface is in the Reset phase, and set to 1 when the interface completes an RC_ID measurement."]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TC` reader - This bit is cleared to 0 when the RC_ID interface is in the Reset phase, and set to 1 when the interface completes the Discharged phase of an RC_ID measurement."]
pub type TC_R = crate::BitReader<bool>;
#[doc = "Field `TC` writer - This bit is cleared to 0 when the RC_ID interface is in the Reset phase, and set to 1 when the interface completes the Discharged phase of an RC_ID measurement."]
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CY_ER` reader - This bit is 1 if an RC_ID measurement encountered an error and the reading in the RC_ID Data Register is invalid. This bit is cleared to 0 when the RC_ID interface is in the Reset phase."]
pub type CY_ER_R = crate::BitReader<bool>;
#[doc = "Field `CY_ER` writer - This bit is 1 if an RC_ID measurement encountered an error and the reading in the RC_ID Data Register is invalid. This bit is cleared to 0 when the RC_ID interface is in the Reset phase."]
pub type CY_ER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `START` reader - Setting this bit to 1 initiates the Discharged phase of an RC_ID measurement."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Setting this bit to 1 initiates the Discharged phase of an RC_ID measurement."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Clearing the bit to 0 causes the RC_ID interface to enter the Reset state, gating its clocks, clearing the status bits in this register and entering \n into its lowest power state. Setting this bit to 1 causes the RC_ID interface to enter the Armed phase of an RC_ID measurement."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Clearing the bit to 0 causes the RC_ID interface to enter the Reset state, gating its clocks, clearing the status bits in this register and entering \n into its lowest power state. Setting this bit to 1 causes the RC_ID interface to enter the Armed phase of an RC_ID measurement."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CLOCK_SET` reader - This field selects the frequency of the Counter circuit clock. This field must retain the same value as long as the ENABLE bit in this register is 1."]
pub type CLOCK_SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_SET` writer - This field selects the frequency of the Counter circuit clock. This field must retain the same value as long as the ENABLE bit in this register is 1."]
pub type CLOCK_SET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - This bit is cleared to 0 when the RC_ID interface is in the Reset phase, and set to 1 when the interface completes an RC_ID measurement."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is cleared to 0 when the RC_ID interface is in the Reset phase, and set to 1 when the interface completes the Discharged phase of an RC_ID measurement."]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is 1 if an RC_ID measurement encountered an error and the reading in the RC_ID Data Register is invalid. This bit is cleared to 0 when the RC_ID interface is in the Reset phase."]
    #[inline(always)]
    pub fn cy_er(&self) -> CY_ER_R {
        CY_ER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Setting this bit to 1 initiates the Discharged phase of an RC_ID measurement."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clearing the bit to 0 causes the RC_ID interface to enter the Reset state, gating its clocks, clearing the status bits in this register and entering \n into its lowest power state. Setting this bit to 1 causes the RC_ID interface to enter the Armed phase of an RC_ID measurement."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - This field selects the frequency of the Counter circuit clock. This field must retain the same value as long as the ENABLE bit in this register is 1."]
    #[inline(always)]
    pub fn clock_set(&self) -> CLOCK_SET_R {
        CLOCK_SET_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is cleared to 0 when the RC_ID interface is in the Reset phase, and set to 1 when the interface completes an RC_ID measurement."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<0> {
        DONE_W::new(self)
    }
    #[doc = "Bit 1 - This bit is cleared to 0 when the RC_ID interface is in the Reset phase, and set to 1 when the interface completes the Discharged phase of an RC_ID measurement."]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<1> {
        TC_W::new(self)
    }
    #[doc = "Bit 2 - This bit is 1 if an RC_ID measurement encountered an error and the reading in the RC_ID Data Register is invalid. This bit is cleared to 0 when the RC_ID interface is in the Reset phase."]
    #[inline(always)]
    pub fn cy_er(&mut self) -> CY_ER_W<2> {
        CY_ER_W::new(self)
    }
    #[doc = "Bit 6 - Setting this bit to 1 initiates the Discharged phase of an RC_ID measurement."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<6> {
        START_W::new(self)
    }
    #[doc = "Bit 7 - Clearing the bit to 0 causes the RC_ID interface to enter the Reset state, gating its clocks, clearing the status bits in this register and entering \n into its lowest power state. Setting this bit to 1 causes the RC_ID interface to enter the Armed phase of an RC_ID measurement."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<7> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 8:9 - This field selects the frequency of the Counter circuit clock. This field must retain the same value as long as the ENABLE bit in this register is 1."]
    #[inline(always)]
    pub fn clock_set(&mut self) -> CLOCK_SET_W<8> {
        CLOCK_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RC_ID Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
