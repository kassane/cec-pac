#[doc = "Register `KICK` writer"]
pub struct W(crate::W<KICK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KICK_SPEC>;
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
impl From<crate::W<KICK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KICK_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The WDT Kick Register is a strobe. Reads of this register return 0. Writes to this register cause the WDT to reload\n the WDT Load Register value and start decrementing when the WDT_ENABLE bit in the WDT Control Register is set to '1'. When the WDT_ENABLE\n bit in the WDT Control Register is cleared to '0', writes to the WDT Kick Register have no effect.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kick](index.html) module"]
pub struct KICK_SPEC;
impl crate::RegisterSpec for KICK_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [kick::W](W) writer structure"]
impl crate::Writable for KICK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KICK to value 0"]
impl crate::Resettable for KICK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
