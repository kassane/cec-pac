#[doc = "Register `INTTMP_INTEN` reader"]
pub struct R(crate::R<INTTMP_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTTMP_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTTMP_INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTTMP_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTTMP_INTEN` writer"]
pub struct W(crate::W<INTTMP_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTTMP_INTEN_SPEC>;
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
impl From<crate::W<INTTMP_INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTTMP_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTTMP_INTEN` reader - Controls whether the Internal Diode event generate an interrupt if the associated status bit is set."]
pub type INTTMP_INTEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTTMP_INTEN` writer - Controls whether the Internal Diode event generate an interrupt if the associated status bit is set."]
pub type INTTMP_INTEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, INTTMP_INTEN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Controls whether the Internal Diode event generate an interrupt if the associated status bit is set."]
    #[inline(always)]
    pub fn inttmp_inten(&self) -> INTTMP_INTEN_R {
        INTTMP_INTEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls whether the Internal Diode event generate an interrupt if the associated status bit is set."]
    #[inline(always)]
    pub fn inttmp_inten(&mut self) -> INTTMP_INTEN_W<0> {
        INTTMP_INTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Int Temp Interrupt Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inttmp_inten](index.html) module"]
pub struct INTTMP_INTEN_SPEC;
impl crate::RegisterSpec for INTTMP_INTEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [inttmp_inten::R](R) reader structure"]
impl crate::Readable for INTTMP_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inttmp_inten::W](W) writer structure"]
impl crate::Writable for INTTMP_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTTMP_INTEN to value 0"]
impl crate::Resettable for INTTMP_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
