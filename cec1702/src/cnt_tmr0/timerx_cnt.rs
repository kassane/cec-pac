#[doc = "Register `TIMERX_CNT` reader"]
pub struct R(crate::R<TIMERX_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMERX_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMERX_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMERX_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMERX_CNT` writer"]
pub struct W(crate::W<TIMERX_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMERX_CNT_SPEC>;
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
impl From<crate::W<TIMERX_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMERX_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMR_CNT` reader - This is the current value of the timer in all modes."]
pub type TMR_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TMR_CNT` writer - This is the current value of the timer in all modes."]
pub type TMR_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMERX_CNT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This is the current value of the timer in all modes."]
    #[inline(always)]
    pub fn tmr_cnt(&self) -> TMR_CNT_R {
        TMR_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This is the current value of the timer in all modes."]
    #[inline(always)]
    pub fn tmr_cnt(&mut self) -> TMR_CNT_W<0> {
        TMR_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register returns the current value of the timer in all modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timerx_cnt](index.html) module"]
pub struct TIMERX_CNT_SPEC;
impl crate::RegisterSpec for TIMERX_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timerx_cnt::R](R) reader structure"]
impl crate::Readable for TIMERX_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timerx_cnt::W](W) writer structure"]
impl crate::Writable for TIMERX_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMERX_CNT to value 0xffff"]
impl crate::Resettable for TIMERX_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
