#[doc = "Register `WDT_CNT` reader"]
pub struct R(crate::R<WDT_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_CNT` writer"]
pub struct W(crate::W<WDT_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_CNT_SPEC>;
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
impl From<crate::W<WDT_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_CNT_SPEC>) -> Self {
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
#[doc = "WDT Event Count \\[3:0\\]
WDT_COUNT (WDT_COUNT) These EC R/W bits are cleared to 0 on VCC1 POR,\n but not on a WDT Note: This field is written by Boot ROM firmware to indicate the number of times a WDT fired before loading a good EC code image.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_cnt](index.html) module"]
pub struct WDT_CNT_SPEC;
impl crate::RegisterSpec for WDT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_cnt::R](R) reader structure"]
impl crate::Readable for WDT_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_cnt::W](W) writer structure"]
impl crate::Writable for WDT_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDT_CNT to value 0"]
impl crate::Resettable for WDT_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
