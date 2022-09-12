#[doc = "Register `EXT3A_TMPLO_LMT` reader"]
pub struct R(crate::R<EXT3A_TMPLO_LMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT3A_TMPLO_LMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT3A_TMPLO_LMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT3A_TMPLO_LMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT3A_TMPLO_LMT` writer"]
pub struct W(crate::W<EXT3A_TMPLO_LMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT3A_TMPLO_LMT_SPEC>;
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
impl From<crate::W<EXT3A_TMPLO_LMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT3A_TMPLO_LMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMPLO3A_LIMIT` reader - Low limit for External Diode 3A Register"]
pub type TMPLO3A_LIMIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMPLO3A_LIMIT` writer - Low limit for External Diode 3A Register"]
pub type TMPLO3A_LIMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, EXT3A_TMPLO_LMT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Low limit for External Diode 3A Register"]
    #[inline(always)]
    pub fn tmplo3a_limit(&self) -> TMPLO3A_LIMIT_R {
        TMPLO3A_LIMIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low limit for External Diode 3A Register"]
    #[inline(always)]
    pub fn tmplo3a_limit(&mut self) -> TMPLO3A_LIMIT_W<0> {
        TMPLO3A_LIMIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low limit for External Diode 3A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext3a_tmplo_lmt](index.html) module"]
pub struct EXT3A_TMPLO_LMT_SPEC;
impl crate::RegisterSpec for EXT3A_TMPLO_LMT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ext3a_tmplo_lmt::R](R) reader structure"]
impl crate::Readable for EXT3A_TMPLO_LMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext3a_tmplo_lmt::W](W) writer structure"]
impl crate::Writable for EXT3A_TMPLO_LMT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT3A_TMPLO_LMT to value 0x81"]
impl crate::Resettable for EXT3A_TMPLO_LMT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x81
    }
}
