#[doc = "Register `TACH_TGT` reader"]
pub struct R(crate::R<TACH_TGT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TACH_TGT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TACH_TGT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TACH_TGT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TACH_TGT` writer"]
pub struct W(crate::W<TACH_TGT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TACH_TGT_SPEC>;
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
impl From<crate::W<TACH_TGT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TACH_TGT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TACH_TGT` reader - The target tachometer value."]
pub type TACH_TGT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TACH_TGT` writer - The target tachometer value."]
pub type TACH_TGT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TACH_TGT_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 3:15 - The target tachometer value."]
    #[inline(always)]
    pub fn tach_tgt(&self) -> TACH_TGT_R {
        TACH_TGT_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - The target tachometer value."]
    #[inline(always)]
    pub fn tach_tgt(&mut self) -> TACH_TGT_W<3> {
        TACH_TGT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The target tachometer value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tach_tgt](index.html) module"]
pub struct TACH_TGT_SPEC;
impl crate::RegisterSpec for TACH_TGT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tach_tgt::R](R) reader structure"]
impl crate::Readable for TACH_TGT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tach_tgt::W](W) writer structure"]
impl crate::Writable for TACH_TGT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TACH_TGT to value 0"]
impl crate::Resettable for TACH_TGT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
