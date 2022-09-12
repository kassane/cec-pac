#[doc = "Register `TACH_RD` reader"]
pub struct R(crate::R<TACH_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TACH_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TACH_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TACH_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TACH_RD` writer"]
pub struct W(crate::W<TACH_RD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TACH_RD_SPEC>;
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
impl From<crate::W<TACH_RD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TACH_RD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TACH_RD` reader - The current tachometer reading value."]
pub type TACH_RD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TACH_RD` writer - The current tachometer reading value."]
pub type TACH_RD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TACH_RD_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 3:15 - The current tachometer reading value."]
    #[inline(always)]
    pub fn tach_rd(&self) -> TACH_RD_R {
        TACH_RD_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - The current tachometer reading value."]
    #[inline(always)]
    pub fn tach_rd(&mut self) -> TACH_RD_W<3> {
        TACH_RD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "15:3\\]
The current tachometer reading value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tach_rd](index.html) module"]
pub struct TACH_RD_SPEC;
impl crate::RegisterSpec for TACH_RD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tach_rd::R](R) reader structure"]
impl crate::Readable for TACH_RD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tach_rd::W](W) writer structure"]
impl crate::Writable for TACH_RD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TACH_RD to value 0"]
impl crate::Resettable for TACH_RD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
