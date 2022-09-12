#[doc = "Register `SS_INTR_SEL` reader"]
pub struct R(crate::R<SS_INTR_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SS_INTR_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SS_INTR_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SS_INTR_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SS_INTR_SEL` writer"]
pub struct W(crate::W<SS_INTR_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SS_INTR_SEL_SPEC>;
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
impl From<crate::W<SS_INTR_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SS_INTR_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPISR` reader - This field determines the rate at which Sub-Second interrupt events are generated."]
pub type SPISR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPISR` writer - This field determines the rate at which Sub-Second interrupt events are generated."]
pub type SPISR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SS_INTR_SEL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - This field determines the rate at which Sub-Second interrupt events are generated."]
    #[inline(always)]
    pub fn spisr(&self) -> SPISR_R {
        SPISR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This field determines the rate at which Sub-Second interrupt events are generated."]
    #[inline(always)]
    pub fn spisr(&mut self) -> SPISR_W<0> {
        SPISR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub-Second Programmable Interrupt Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ss_intr_sel](index.html) module"]
pub struct SS_INTR_SEL_SPEC;
impl crate::RegisterSpec for SS_INTR_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ss_intr_sel::R](R) reader structure"]
impl crate::Readable for SS_INTR_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ss_intr_sel::W](W) writer structure"]
impl crate::Writable for SS_INTR_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SS_INTR_SEL to value 0"]
impl crate::Resettable for SS_INTR_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
