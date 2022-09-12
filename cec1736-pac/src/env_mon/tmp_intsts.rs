#[doc = "Register `TMP_INTSTS` reader"]
pub struct R(crate::R<TMP_INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMP_INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMP_INTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMP_INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMP_INTSTS` writer"]
pub struct W(crate::W<TMP_INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMP_INTSTS_SPEC>;
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
impl From<crate::W<TMP_INTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMP_INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMP_INTSTS` reader - Controls whether the External Diode events generate an interrupt if the associated status bit is set."]
pub type TMP_INTSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMP_INTSTS` writer - Controls whether the External Diode events generate an interrupt if the associated status bit is set."]
pub type TMP_INTSTS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TMP_INTSTS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Controls whether the External Diode events generate an interrupt if the associated status bit is set."]
    #[inline(always)]
    pub fn tmp_intsts(&self) -> TMP_INTSTS_R {
        TMP_INTSTS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls whether the External Diode events generate an interrupt if the associated status bit is set."]
    #[inline(always)]
    pub fn tmp_intsts(&mut self) -> TMP_INTSTS_W<0> {
        TMP_INTSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temp Interrupt Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmp_intsts](index.html) module"]
pub struct TMP_INTSTS_SPEC;
impl crate::RegisterSpec for TMP_INTSTS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tmp_intsts::R](R) reader structure"]
impl crate::Readable for TMP_INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmp_intsts::W](W) writer structure"]
impl crate::Writable for TMP_INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMP_INTSTS to value 0"]
impl crate::Resettable for TMP_INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
