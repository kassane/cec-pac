#[doc = "Register `CNVR_CFG` reader"]
pub struct R(crate::R<CNVR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNVR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNVR_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNVR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNVR_CFG` writer"]
pub struct W(crate::W<CNVR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNVR_CFG_SPEC>;
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
impl From<crate::W<CNVR_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNVR_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNVR_CFG` reader - Controls Temperature Conversion for the temperature channels"]
pub type CNVR_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNVR_CFG` writer - Controls Temperature Conversion for the temperature channels"]
pub type CNVR_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CNVR_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Controls Temperature Conversion for the temperature channels"]
    #[inline(always)]
    pub fn cnvr_cfg(&self) -> CNVR_CFG_R {
        CNVR_CFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls Temperature Conversion for the temperature channels"]
    #[inline(always)]
    pub fn cnvr_cfg(&mut self) -> CNVR_CFG_W<0> {
        CNVR_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls Temperature Conversion for the temperature channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnvr_cfg](index.html) module"]
pub struct CNVR_CFG_SPEC;
impl crate::RegisterSpec for CNVR_CFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cnvr_cfg::R](R) reader structure"]
impl crate::Readable for CNVR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnvr_cfg::W](W) writer structure"]
impl crate::Writable for CNVR_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNVR_CFG to value 0"]
impl crate::Resettable for CNVR_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
