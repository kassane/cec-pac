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
#[doc = "Field `TCLK` reader - Timer Clock Select. This field determines the clock source for the 16-bit counter in the timer."]
pub type TCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCLK` writer - Timer Clock Select. This field determines the clock source for the 16-bit counter in the timer."]
pub type TCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRLD_SPEC, u8, u8, 4, O>;
#[doc = "Field `EDGE` reader - This field selects which edge of the TINx input signal affects the timer in Event Mode, One-Shot Mode and Measurement Mode.\n Event Mode: 11b=No event selected; 10b=Counts rising and falling edges; 01b=Counts rising edges; 00b=Counts falling edges.\n One-Shot Mode: 11b=Start counting when the Enable bit is set; 10b=Starts counting on a rising or falling edge; 01b=Starts\n counting on a rising edge; 00b=Starts counting on a falling edge. Measurement Mode: 11b=No event selected; 10b=Measures\n the time between rising edges and falling edges and the time between falling edges and rising edges; 01b=Measures the\n time between rising edges; 00b=Measures the time between falling edges."]
pub type EDGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDGE` writer - This field selects which edge of the TINx input signal affects the timer in Event Mode, One-Shot Mode and Measurement Mode.\n Event Mode: 11b=No event selected; 10b=Counts rising and falling edges; 01b=Counts rising edges; 00b=Counts falling edges.\n One-Shot Mode: 11b=Start counting when the Enable bit is set; 10b=Starts counting on a rising or falling edge; 01b=Starts\n counting on a rising edge; 00b=Starts counting on a falling edge. Measurement Mode: 11b=No event selected; 10b=Measures\n the time between rising edges and falling edges and the time between falling edges and rising edges; 01b=Measures the\n time between rising edges; 00b=Measures the time between falling edges."]
pub type EDGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRLD_SPEC, u8, u8, 2, O>;
#[doc = "Field `EVENT` reader - Event Select. This bit is used to select the count source when the timer is operating in Event Mode.\n 1=TINx is count source; 0=Timer x-1 overflow is count source."]
pub type EVENT_R = crate::BitReader<bool>;
#[doc = "Field `EVENT` writer - Event Select. This bit is used to select the count source when the timer is operating in Event Mode.\n 1=TINx is count source; 0=Timer x-1 overflow is count source."]
pub type EVENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRLD_SPEC, bool, O>;
#[doc = "Field `FCLK` reader - Timer Clock Select. This field determines the clock source for the TINx noise filter. The available frequencies\n are the same as for TCLK."]
pub type FCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCLK` writer - Timer Clock Select. This field determines the clock source for the TINx noise filter. The available frequencies\n are the same as for TCLK."]
pub type FCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRLD_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Timer Clock Select. This field determines the clock source for the 16-bit counter in the timer."]
    #[inline(always)]
    pub fn tclk(&self) -> TCLK_R {
        TCLK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - This field selects which edge of the TINx input signal affects the timer in Event Mode, One-Shot Mode and Measurement Mode.\n Event Mode: 11b=No event selected; 10b=Counts rising and falling edges; 01b=Counts rising edges; 00b=Counts falling edges.\n One-Shot Mode: 11b=Start counting when the Enable bit is set; 10b=Starts counting on a rising or falling edge; 01b=Starts\n counting on a rising edge; 00b=Starts counting on a falling edge. Measurement Mode: 11b=No event selected; 10b=Measures\n the time between rising edges and falling edges and the time between falling edges and rising edges; 01b=Measures the\n time between rising edges; 00b=Measures the time between falling edges."]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Event Select. This bit is used to select the count source when the timer is operating in Event Mode.\n 1=TINx is count source; 0=Timer x-1 overflow is count source."]
    #[inline(always)]
    pub fn event(&self) -> EVENT_R {
        EVENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Timer Clock Select. This field determines the clock source for the TINx noise filter. The available frequencies\n are the same as for TCLK."]
    #[inline(always)]
    pub fn fclk(&self) -> FCLK_R {
        FCLK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timer Clock Select. This field determines the clock source for the 16-bit counter in the timer."]
    #[inline(always)]
    pub fn tclk(&mut self) -> TCLK_W<0> {
        TCLK_W::new(self)
    }
    #[doc = "Bits 5:6 - This field selects which edge of the TINx input signal affects the timer in Event Mode, One-Shot Mode and Measurement Mode.\n Event Mode: 11b=No event selected; 10b=Counts rising and falling edges; 01b=Counts rising edges; 00b=Counts falling edges.\n One-Shot Mode: 11b=Start counting when the Enable bit is set; 10b=Starts counting on a rising or falling edge; 01b=Starts\n counting on a rising edge; 00b=Starts counting on a falling edge. Measurement Mode: 11b=No event selected; 10b=Measures\n the time between rising edges and falling edges and the time between falling edges and rising edges; 01b=Measures the\n time between rising edges; 00b=Measures the time between falling edges."]
    #[inline(always)]
    pub fn edge(&mut self) -> EDGE_W<5> {
        EDGE_W::new(self)
    }
    #[doc = "Bit 7 - Event Select. This bit is used to select the count source when the timer is operating in Event Mode.\n 1=TINx is count source; 0=Timer x-1 overflow is count source."]
    #[inline(always)]
    pub fn event(&mut self) -> EVENT_W<7> {
        EVENT_W::new(self)
    }
    #[doc = "Bits 8:11 - Timer Clock Select. This field determines the clock source for the TINx noise filter. The available frequencies\n are the same as for TCLK."]
    #[inline(always)]
    pub fn fclk(&mut self) -> FCLK_W<8> {
        FCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the value of the Timer pre-load for the counter.\n This is used by H/W when the counter is to be restarted automatically; this will become the new value of the counter upon restart.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prld](index.html) module"]
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
