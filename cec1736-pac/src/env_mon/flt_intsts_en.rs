#[doc = "Register `FLT_INTSTS_EN` reader"]
pub struct R(crate::R<FLT_INTSTS_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT_INTSTS_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT_INTSTS_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT_INTSTS_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLT_INTSTS_EN` writer"]
pub struct W(crate::W<FLT_INTSTS_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT_INTSTS_EN_SPEC>;
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
impl From<crate::W<FLT_INTSTS_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT_INTSTS_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT_INTSTS_EN` reader - Controls whether the External Diode Fault events generates interrupt if the associated status bit is 1."]
pub type FLT_INTSTS_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT_INTSTS_EN` writer - Controls whether the External Diode Fault events generates interrupt if the associated status bit is 1."]
pub type FLT_INTSTS_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, FLT_INTSTS_EN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Controls whether the External Diode Fault events generates interrupt if the associated status bit is 1."]
    #[inline(always)]
    pub fn flt_intsts_en(&self) -> FLT_INTSTS_EN_R {
        FLT_INTSTS_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls whether the External Diode Fault events generates interrupt if the associated status bit is 1."]
    #[inline(always)]
    pub fn flt_intsts_en(&mut self) -> FLT_INTSTS_EN_W<0> {
        FLT_INTSTS_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Interrupt Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt_intsts_en](index.html) module"]
pub struct FLT_INTSTS_EN_SPEC;
impl crate::RegisterSpec for FLT_INTSTS_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [flt_intsts_en::R](R) reader structure"]
impl crate::Readable for FLT_INTSTS_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flt_intsts_en::W](W) writer structure"]
impl crate::Writable for FLT_INTSTS_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLT_INTSTS_EN to value 0x01"]
impl crate::Resettable for FLT_INTSTS_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
