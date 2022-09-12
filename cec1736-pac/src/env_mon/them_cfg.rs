#[doc = "Register `THEM_CFG` reader"]
pub struct R(crate::R<THEM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THEM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THEM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THEM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THEM_CFG` writer"]
pub struct W(crate::W<THEM_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THEM_CFG_SPEC>;
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
impl From<crate::W<THEM_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THEM_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THEM_CFG` reader - Controls Thermistor or diodes Configuration"]
pub type THEM_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THEM_CFG` writer - Controls Thermistor or diodes Configuration"]
pub type THEM_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u8, THEM_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Controls Thermistor or diodes Configuration"]
    #[inline(always)]
    pub fn them_cfg(&self) -> THEM_CFG_R {
        THEM_CFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls Thermistor or diodes Configuration"]
    #[inline(always)]
    pub fn them_cfg(&mut self) -> THEM_CFG_W<0> {
        THEM_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls Thermistor or diodes Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [them_cfg](index.html) module"]
pub struct THEM_CFG_SPEC;
impl crate::RegisterSpec for THEM_CFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [them_cfg::R](R) reader structure"]
impl crate::Readable for THEM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [them_cfg::W](W) writer structure"]
impl crate::Writable for THEM_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THEM_CFG to value 0"]
impl crate::Resettable for THEM_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
