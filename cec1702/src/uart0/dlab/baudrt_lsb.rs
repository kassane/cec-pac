#[doc = "Register `BAUDRT_LSB` reader"]
pub struct R(crate::R<BAUDRT_LSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUDRT_LSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUDRT_LSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUDRT_LSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUDRT_LSB` writer"]
pub struct W(crate::W<BAUDRT_LSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUDRT_LSB_SPEC>;
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
impl From<crate::W<BAUDRT_LSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUDRT_LSB_SPEC>) -> Self {
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
#[doc = "UART Programmable BAUD Rate Generator (LSB) Register (DLAB=1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baudrt_lsb](index.html) module"]
pub struct BAUDRT_LSB_SPEC;
impl crate::RegisterSpec for BAUDRT_LSB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [baudrt_lsb::R](R) reader structure"]
impl crate::Readable for BAUDRT_LSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baudrt_lsb::W](W) writer structure"]
impl crate::Writable for BAUDRT_LSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUDRT_LSB to value 0"]
impl crate::Resettable for BAUDRT_LSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
