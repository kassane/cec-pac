#[doc = "Register `RX_DAT` reader"]
pub struct R(crate::R<RX_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_DAT` writer"]
pub struct W(crate::W<RX_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_DAT_SPEC>;
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
impl From<crate::W<RX_DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_DAT_SPEC>) -> Self {
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
#[doc = "7:0\\]
This register is used to read the value returned by the external SPI device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_dat](index.html) module"]
pub struct RX_DAT_SPEC;
impl crate::RegisterSpec for RX_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_dat::R](R) reader structure"]
impl crate::Readable for RX_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_dat::W](W) writer structure"]
impl crate::Writable for RX_DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_DAT to value 0"]
impl crate::Resettable for RX_DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
