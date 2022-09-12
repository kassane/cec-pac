#[doc = "Register `AVG_EN` reader"]
pub struct R(crate::R<AVG_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AVG_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AVG_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AVG_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AVG_EN` writer"]
pub struct W(crate::W<AVG_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AVG_EN_SPEC>;
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
impl From<crate::W<AVG_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AVG_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVG_EN` reader - Software Averaging Enable"]
pub type AVG_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AVG_EN` writer - Software Averaging Enable"]
pub type AVG_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u8, AVG_EN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Software Averaging Enable"]
    #[inline(always)]
    pub fn avg_en(&self) -> AVG_EN_R {
        AVG_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software Averaging Enable"]
    #[inline(always)]
    pub fn avg_en(&mut self) -> AVG_EN_W<0> {
        AVG_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Averaging Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [avg_en](index.html) module"]
pub struct AVG_EN_SPEC;
impl crate::RegisterSpec for AVG_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [avg_en::R](R) reader structure"]
impl crate::Readable for AVG_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [avg_en::W](W) writer structure"]
impl crate::Writable for AVG_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AVG_EN to value 0"]
impl crate::Resettable for AVG_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
