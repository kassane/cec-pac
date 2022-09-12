#[doc = "Register `FLSF_CFG` reader"]
pub struct R(crate::R<FLSF_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLSF_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLSF_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLSF_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLSF_CFG` writer"]
pub struct W(crate::W<FLSF_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLSF_CFG_SPEC>;
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
impl From<crate::W<FLSF_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLSF_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLSF_CFG` reader - Stores configuration bits that are retained over all power modes"]
pub type FLSF_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLSF_CFG` writer - Stores configuration bits that are retained over all power modes"]
pub type FLSF_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FLSF_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Stores configuration bits that are retained over all power modes"]
    #[inline(always)]
    pub fn flsf_cfg(&self) -> FLSF_CFG_R {
        FLSF_CFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Stores configuration bits that are retained over all power modes"]
    #[inline(always)]
    pub fn flsf_cfg(&mut self) -> FLSF_CFG_W<0> {
        FLSF_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FailSafe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flsf_cfg](index.html) module"]
pub struct FLSF_CFG_SPEC;
impl crate::RegisterSpec for FLSF_CFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [flsf_cfg::R](R) reader structure"]
impl crate::Readable for FLSF_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flsf_cfg::W](W) writer structure"]
impl crate::Writable for FLSF_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLSF_CFG to value 0"]
impl crate::Resettable for FLSF_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
