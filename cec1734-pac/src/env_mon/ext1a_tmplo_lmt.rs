#[doc = "Register `EXT1A_TMPLO_LMT` reader"]
pub struct R(crate::R<EXT1A_TMPLO_LMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT1A_TMPLO_LMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT1A_TMPLO_LMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT1A_TMPLO_LMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT1A_TMPLO_LMT` writer"]
pub struct W(crate::W<EXT1A_TMPLO_LMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT1A_TMPLO_LMT_SPEC>;
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
impl From<crate::W<EXT1A_TMPLO_LMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT1A_TMPLO_LMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMPLO1A_LIMIT` reader - Low limit for External Diode 1A Register"]
pub type TMPLO1A_LIMIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMPLO1A_LIMIT` writer - Low limit for External Diode 1A Register"]
pub type TMPLO1A_LIMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, EXT1A_TMPLO_LMT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Low limit for External Diode 1A Register"]
    #[inline(always)]
    pub fn tmplo1a_limit(&self) -> TMPLO1A_LIMIT_R {
        TMPLO1A_LIMIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low limit for External Diode 1A Register"]
    #[inline(always)]
    pub fn tmplo1a_limit(&mut self) -> TMPLO1A_LIMIT_W<0> {
        TMPLO1A_LIMIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low limit for External Diode 1A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext1a_tmplo_lmt](index.html) module"]
pub struct EXT1A_TMPLO_LMT_SPEC;
impl crate::RegisterSpec for EXT1A_TMPLO_LMT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ext1a_tmplo_lmt::R](R) reader structure"]
impl crate::Readable for EXT1A_TMPLO_LMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext1a_tmplo_lmt::W](W) writer structure"]
impl crate::Writable for EXT1A_TMPLO_LMT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT1A_TMPLO_LMT to value 0x81"]
impl crate::Resettable for EXT1A_TMPLO_LMT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x81
    }
}
