#[doc = "Register `TIMERX_RLD` reader"]
pub struct R(crate::R<TIMERX_RLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMERX_RLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMERX_RLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMERX_RLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMERX_RLD` writer"]
pub struct W(crate::W<TIMERX_RLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMERX_RLD_SPEC>;
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
impl From<crate::W<TIMERX_RLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMERX_RLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMR_RLD` reader - The Timer Reload register is used in Timer and One-Shot modes to set the lower limit of the timer.\n In Event mode the Timer Reload register sets either the upper or lower limit of the timer depending on if the\n timer is counting up or down. Valid Timer Reload values are 0001h - FFFFh. If the timer is running, the\n reload value will not be updated until the timer overflows or underflows. Programming a 0000h as a preload\n value is not a valid count value. Using a value of 0000h will cause unpredictable behavior."]
pub type TMR_RLD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TMR_RLD` writer - The Timer Reload register is used in Timer and One-Shot modes to set the lower limit of the timer.\n In Event mode the Timer Reload register sets either the upper or lower limit of the timer depending on if the\n timer is counting up or down. Valid Timer Reload values are 0001h - FFFFh. If the timer is running, the\n reload value will not be updated until the timer overflows or underflows. Programming a 0000h as a preload\n value is not a valid count value. Using a value of 0000h will cause unpredictable behavior."]
pub type TMR_RLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMERX_RLD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The Timer Reload register is used in Timer and One-Shot modes to set the lower limit of the timer.\n In Event mode the Timer Reload register sets either the upper or lower limit of the timer depending on if the\n timer is counting up or down. Valid Timer Reload values are 0001h - FFFFh. If the timer is running, the\n reload value will not be updated until the timer overflows or underflows. Programming a 0000h as a preload\n value is not a valid count value. Using a value of 0000h will cause unpredictable behavior."]
    #[inline(always)]
    pub fn tmr_rld(&self) -> TMR_RLD_R {
        TMR_RLD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The Timer Reload register is used in Timer and One-Shot modes to set the lower limit of the timer.\n In Event mode the Timer Reload register sets either the upper or lower limit of the timer depending on if the\n timer is counting up or down. Valid Timer Reload values are 0001h - FFFFh. If the timer is running, the\n reload value will not be updated until the timer overflows or underflows. Programming a 0000h as a preload\n value is not a valid count value. Using a value of 0000h will cause unpredictable behavior."]
    #[inline(always)]
    pub fn tmr_rld(&mut self) -> TMR_RLD_W<0> {
        TMR_RLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used in Timer and One-Shot modes to set the lower limit of the timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timerx_rld](index.html) module"]
pub struct TIMERX_RLD_SPEC;
impl crate::RegisterSpec for TIMERX_RLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timerx_rld::R](R) reader structure"]
impl crate::Readable for TIMERX_RLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timerx_rld::W](W) writer structure"]
impl crate::Writable for TIMERX_RLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMERX_RLD to value 0xffff"]
impl crate::Resettable for TIMERX_RLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
