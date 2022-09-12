#[doc = "Register `THRMTRP_TMP3` reader"]
pub struct R(crate::R<THRMTRP_TMP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRMTRP_TMP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRMTRP_TMP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRMTRP_TMP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRMTRP_TMP3` writer"]
pub struct W(crate::W<THRMTRP_TMP3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRMTRP_TMP3_SPEC>;
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
impl From<crate::W<THRMTRP_TMP3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRMTRP_TMP3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRMTRP_TMP3` reader - ThermTrip temperature high limit compared against External Diode 3"]
pub type THRMTRP_TMP3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRMTRP_TMP3` writer - ThermTrip temperature high limit compared against External Diode 3"]
pub type THRMTRP_TMP3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, THRMTRP_TMP3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ThermTrip temperature high limit compared against External Diode 3"]
    #[inline(always)]
    pub fn thrmtrp_tmp3(&self) -> THRMTRP_TMP3_R {
        THRMTRP_TMP3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - ThermTrip temperature high limit compared against External Diode 3"]
    #[inline(always)]
    pub fn thrmtrp_tmp3(&mut self) -> THRMTRP_TMP3_W<0> {
        THRMTRP_TMP3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Thermal Trip Temperature Diode 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thrmtrp_tmp3](index.html) module"]
pub struct THRMTRP_TMP3_SPEC;
impl crate::RegisterSpec for THRMTRP_TMP3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [thrmtrp_tmp3::R](R) reader structure"]
impl crate::Readable for THRMTRP_TMP3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thrmtrp_tmp3::W](W) writer structure"]
impl crate::Writable for THRMTRP_TMP3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THRMTRP_TMP3 to value 0x7f"]
impl crate::Resettable for THRMTRP_TMP3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
