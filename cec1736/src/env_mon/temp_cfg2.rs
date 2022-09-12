#[doc = "Register `TEMP_CFG2` reader"]
pub struct R(crate::R<TEMP_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMP_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMP_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMP_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMP_CFG2` writer"]
pub struct W(crate::W<TEMP_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMP_CFG2_SPEC>;
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
impl From<crate::W<TEMP_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMP_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEMP_CFG2` reader - Controls temp sensing for external diodes"]
pub type TEMP_CFG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEMP_CFG2` writer - Controls temp sensing for external diodes"]
pub type TEMP_CFG2_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TEMP_CFG2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Controls temp sensing for external diodes"]
    #[inline(always)]
    pub fn temp_cfg2(&self) -> TEMP_CFG2_R {
        TEMP_CFG2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls temp sensing for external diodes"]
    #[inline(always)]
    pub fn temp_cfg2(&mut self) -> TEMP_CFG2_W<0> {
        TEMP_CFG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls temp sensing for external diodes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_cfg2](index.html) module"]
pub struct TEMP_CFG2_SPEC;
impl crate::RegisterSpec for TEMP_CFG2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [temp_cfg2::R](R) reader structure"]
impl crate::Readable for TEMP_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [temp_cfg2::W](W) writer structure"]
impl crate::Writable for TEMP_CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEMP_CFG2 to value 0"]
impl crate::Resettable for TEMP_CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
