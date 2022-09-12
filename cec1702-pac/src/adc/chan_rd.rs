#[doc = "Register `CHAN_RD[%s]` reader"]
pub struct R(crate::R<CHAN_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHAN_RD[%s]` writer"]
pub struct W(crate::W<CHAN_RD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHAN_RD_SPEC>;
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
impl From<crate::W<CHAN_RD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHAN_RD_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "All 16 ADC channels return their results into \n a 32-bit reading register. In each case the low 10 bits of the reading register\n return the result of the Analog to Digital conversion and the upper 22 bits return 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_rd](index.html) module"]
pub struct CHAN_RD_SPEC;
impl crate::RegisterSpec for CHAN_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_rd::R](R) reader structure"]
impl crate::Readable for CHAN_RD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chan_rd::W](W) writer structure"]
impl crate::Writable for CHAN_RD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHAN_RD[%s]
to value 0"]
impl crate::Resettable for CHAN_RD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
