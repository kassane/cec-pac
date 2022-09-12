#[doc = "Register `PRLD` reader"]
pub struct R(crate::R<PRLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRLD` writer"]
pub struct W(crate::W<PRLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRLD_SPEC>;
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
impl From<crate::W<PRLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRELOAD` reader - The this register is loaded into the RTOS Timer counter either when the TIMER_START bit is written with a 1, or when the timer counter counts down to 0 and the AUTO_RELOAD bit is 1. This register must be programmed with a new count value before the TIMER_START bit is set to 1. If this register is updated while the counter is operating, the new count value will only take effect if the counter transitions form 1 to 0 while the AUTO_RELOAD bit is set."]
pub type PRELOAD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRELOAD` writer - The this register is loaded into the RTOS Timer counter either when the TIMER_START bit is written with a 1, or when the timer counter counts down to 0 and the AUTO_RELOAD bit is 1. This register must be programmed with a new count value before the TIMER_START bit is set to 1. If this register is updated while the counter is operating, the new count value will only take effect if the counter transitions form 1 to 0 while the AUTO_RELOAD bit is set."]
pub type PRELOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRLD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The this register is loaded into the RTOS Timer counter either when the TIMER_START bit is written with a 1, or when the timer counter counts down to 0 and the AUTO_RELOAD bit is 1. This register must be programmed with a new count value before the TIMER_START bit is set to 1. If this register is updated while the counter is operating, the new count value will only take effect if the counter transitions form 1 to 0 while the AUTO_RELOAD bit is set."]
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The this register is loaded into the RTOS Timer counter either when the TIMER_START bit is written with a 1, or when the timer counter counts down to 0 and the AUTO_RELOAD bit is 1. This register must be programmed with a new count value before the TIMER_START bit is set to 1. If this register is updated while the counter is operating, the new count value will only take effect if the counter transitions form 1 to 0 while the AUTO_RELOAD bit is set."]
    #[inline(always)]
    pub fn preload(&mut self) -> PRELOAD_W<0> {
        PRELOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTOS Timer Preload Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prld](index.html) module"]
pub struct PRLD_SPEC;
impl crate::RegisterSpec for PRLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prld::R](R) reader structure"]
impl crate::Readable for PRLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prld::W](W) writer structure"]
impl crate::Writable for PRLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRLD to value 0"]
impl crate::Resettable for PRLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
