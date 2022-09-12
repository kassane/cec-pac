#[doc = "Register `BCOMP4_EN` reader"]
pub struct R(crate::R<BCOMP4_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCOMP4_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCOMP4_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCOMP4_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCOMP4_EN` writer"]
pub struct W(crate::W<BCOMP4_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCOMP4_EN_SPEC>;
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
impl From<crate::W<BCOMP4_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCOMP4_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCOMP4_EN` reader - Beta compensation settings for External Diode4 Enable"]
pub type BCOMP4_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCOMP4_EN` writer - Beta compensation settings for External Diode4 Enable"]
pub type BCOMP4_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BCOMP4_EN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Beta compensation settings for External Diode4 Enable"]
    #[inline(always)]
    pub fn bcomp4_en(&self) -> BCOMP4_EN_R {
        BCOMP4_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Beta compensation settings for External Diode4 Enable"]
    #[inline(always)]
    pub fn bcomp4_en(&mut self) -> BCOMP4_EN_W<0> {
        BCOMP4_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Diode4 Beta compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcomp4_en](index.html) module"]
pub struct BCOMP4_EN_SPEC;
impl crate::RegisterSpec for BCOMP4_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcomp4_en::R](R) reader structure"]
impl crate::Readable for BCOMP4_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcomp4_en::W](W) writer structure"]
impl crate::Writable for BCOMP4_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCOMP4_EN to value 0x1f"]
impl crate::Resettable for BCOMP4_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
