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
#[doc = "Field `EVT_INT` reader - This is the interrupt status that fires when the timer reaches its limit. This is the interrupt status that fires when\n the timer reaches its limit. This may be level or a self clearing signal cycle pulse, based on the AUTO_RESTART bit in the Timer\n Control Register. If the timer is set to automatically restart, it will provide a pulse, otherwise a level is provided.(R/WC)"]
pub type EVT_INT_R = crate::BitReader<bool>;
#[doc = "Field `EVT_INT` writer - This is the interrupt status that fires when the timer reaches its limit. This is the interrupt status that fires when\n the timer reaches its limit. This may be level or a self clearing signal cycle pulse, based on the AUTO_RESTART bit in the Timer\n Control Register. If the timer is set to automatically restart, it will provide a pulse, otherwise a level is provided.(R/WC)"]
pub type EVT_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This is the interrupt status that fires when the timer reaches its limit. This is the interrupt status that fires when\n the timer reaches its limit. This may be level or a self clearing signal cycle pulse, based on the AUTO_RESTART bit in the Timer\n Control Register. If the timer is set to automatically restart, it will provide a pulse, otherwise a level is provided.(R/WC)"]
    #[inline(always)]
    pub fn evt_int(&self) -> EVT_INT_R {
        EVT_INT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is the interrupt status that fires when the timer reaches its limit. This is the interrupt status that fires when\n the timer reaches its limit. This may be level or a self clearing signal cycle pulse, based on the AUTO_RESTART bit in the Timer\n Control Register. If the timer is set to automatically restart, it will provide a pulse, otherwise a level is provided.(R/WC)"]
    #[inline(always)]
    pub fn evt_int(&mut self) -> EVT_INT_W<0> {
        EVT_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the interrupt status that fires when the timer reaches its limit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
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
