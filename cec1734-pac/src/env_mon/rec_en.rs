#[doc = "Register `REC_EN` reader"]
pub struct R(crate::R<REC_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REC_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REC_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REC_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REC_EN` writer"]
pub struct W(crate::W<REC_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REC_EN_SPEC>;
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
impl From<crate::W<REC_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REC_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REC_EN` reader - Enables REC for all external diode channels"]
pub type REC_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REC_EN` writer - Enables REC for all external diode channels"]
pub type REC_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u8, REC_EN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Enables REC for all external diode channels"]
    #[inline(always)]
    pub fn rec_en(&self) -> REC_EN_R {
        REC_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enables REC for all external diode channels"]
    #[inline(always)]
    pub fn rec_en(&mut self) -> REC_EN_W<0> {
        REC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "REC Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rec_en](index.html) module"]
pub struct REC_EN_SPEC;
impl crate::RegisterSpec for REC_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rec_en::R](R) reader structure"]
impl crate::Readable for REC_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rec_en::W](W) writer structure"]
impl crate::Writable for REC_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REC_EN to value 0xff"]
impl crate::Resettable for REC_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
