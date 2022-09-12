#[doc = "Register `SHDN_CFG` reader"]
pub struct R(crate::R<SHDN_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHDN_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHDN_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHDN_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHDN_CFG` writer"]
pub struct W(crate::W<SHDN_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHDN_CFG_SPEC>;
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
impl From<crate::W<SHDN_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHDN_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHDN_CFG` reader - Stores configuration bits that are retained over all power modes"]
pub type SHDN_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHDN_CFG` writer - Stores configuration bits that are retained over all power modes"]
pub type SHDN_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SHDN_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Stores configuration bits that are retained over all power modes"]
    #[inline(always)]
    pub fn shdn_cfg(&self) -> SHDN_CFG_R {
        SHDN_CFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Stores configuration bits that are retained over all power modes"]
    #[inline(always)]
    pub fn shdn_cfg(&mut self) -> SHDN_CFG_W<0> {
        SHDN_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shutdown Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdn_cfg](index.html) module"]
pub struct SHDN_CFG_SPEC;
impl crate::RegisterSpec for SHDN_CFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [shdn_cfg::R](R) reader structure"]
impl crate::Readable for SHDN_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shdn_cfg::W](W) writer structure"]
impl crate::Writable for SHDN_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHDN_CFG to value 0x01"]
impl crate::Resettable for SHDN_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
