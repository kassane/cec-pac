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
#[doc = "Field `OUTOF_LIM_STS` reader - TACH_OUT_OF_LIMIT_STATUS 1=Tach is outside of limits, 0=Tach is within limits (R/WC)"]
pub type OUTOF_LIM_STS_R = crate::BitReader<bool>;
#[doc = "Field `OUTOF_LIM_STS` writer - TACH_OUT_OF_LIMIT_STATUS 1=Tach is outside of limits, 0=Tach is within limits (R/WC)"]
pub type OUTOF_LIM_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `PIN_STS` reader - TACH_PIN_STATUS 1= Tach Input is high, 0= Tach Input is low"]
pub type PIN_STS_R = crate::BitReader<bool>;
#[doc = "Field `PIN_STS` writer - TACH_PIN_STATUS 1= Tach Input is high, 0= Tach Input is low"]
pub type PIN_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `TOG_STS` reader - TOGGLE_STATUS 1=Tach Input changed state (this bit is set on a low-to-high or high-tolow transition), 0=Tach stable (R/WC)"]
pub type TOG_STS_R = crate::BitReader<bool>;
#[doc = "Field `TOG_STS` writer - TOGGLE_STATUS 1=Tach Input changed state (this bit is set on a low-to-high or high-tolow transition), 0=Tach stable (R/WC)"]
pub type TOG_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `CNT_RDY_STS` reader - COUNT_READY_STATUS 1=Reading ready, 0=Reading not ready"]
pub type CNT_RDY_STS_R = crate::BitReader<bool>;
#[doc = "Field `CNT_RDY_STS` writer - COUNT_READY_STATUS 1=Reading ready, 0=Reading not ready"]
pub type CNT_RDY_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TACH_OUT_OF_LIMIT_STATUS 1=Tach is outside of limits, 0=Tach is within limits (R/WC)"]
    #[inline(always)]
    pub fn outof_lim_sts(&self) -> OUTOF_LIM_STS_R {
        OUTOF_LIM_STS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TACH_PIN_STATUS 1= Tach Input is high, 0= Tach Input is low"]
    #[inline(always)]
    pub fn pin_sts(&self) -> PIN_STS_R {
        PIN_STS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TOGGLE_STATUS 1=Tach Input changed state (this bit is set on a low-to-high or high-tolow transition), 0=Tach stable (R/WC)"]
    #[inline(always)]
    pub fn tog_sts(&self) -> TOG_STS_R {
        TOG_STS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COUNT_READY_STATUS 1=Reading ready, 0=Reading not ready"]
    #[inline(always)]
    pub fn cnt_rdy_sts(&self) -> CNT_RDY_STS_R {
        CNT_RDY_STS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TACH_OUT_OF_LIMIT_STATUS 1=Tach is outside of limits, 0=Tach is within limits (R/WC)"]
    #[inline(always)]
    pub fn outof_lim_sts(&mut self) -> OUTOF_LIM_STS_W<0> {
        OUTOF_LIM_STS_W::new(self)
    }
    #[doc = "Bit 1 - TACH_PIN_STATUS 1= Tach Input is high, 0= Tach Input is low"]
    #[inline(always)]
    pub fn pin_sts(&mut self) -> PIN_STS_W<1> {
        PIN_STS_W::new(self)
    }
    #[doc = "Bit 2 - TOGGLE_STATUS 1=Tach Input changed state (this bit is set on a low-to-high or high-tolow transition), 0=Tach stable (R/WC)"]
    #[inline(always)]
    pub fn tog_sts(&mut self) -> TOG_STS_W<2> {
        TOG_STS_W::new(self)
    }
    #[doc = "Bit 3 - COUNT_READY_STATUS 1=Reading ready, 0=Reading not ready"]
    #[inline(always)]
    pub fn cnt_rdy_sts(&mut self) -> CNT_RDY_STS_W<3> {
        CNT_RDY_STS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TACHx Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
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
