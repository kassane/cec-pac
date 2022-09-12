#[doc = "Register `LOCK_REG` reader"]
pub struct R(crate::R<LOCK_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCK_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCK_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCK_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCK_REG` writer"]
pub struct W(crate::W<LOCK_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCK_REG_SPEC>;
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
impl From<crate::W<LOCK_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCK_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCR_RST_EN_LOCK` reader - PCR Reset Enable Lock Register."]
pub type PCR_RST_EN_LOCK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PCR_RST_EN_LOCK` writer - PCR Reset Enable Lock Register."]
pub type PCR_RST_EN_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LOCK_REG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - PCR Reset Enable Lock Register."]
    #[inline(always)]
    pub fn pcr_rst_en_lock(&self) -> PCR_RST_EN_LOCK_R {
        PCR_RST_EN_LOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCR Reset Enable Lock Register."]
    #[inline(always)]
    pub fn pcr_rst_en_lock(&mut self) -> PCR_RST_EN_LOCK_W<0> {
        PCR_RST_EN_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LOCK Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock_reg](index.html) module"]
pub struct LOCK_REG_SPEC;
impl crate::RegisterSpec for LOCK_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lock_reg::R](R) reader structure"]
impl crate::Readable for LOCK_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lock_reg::W](W) writer structure"]
impl crate::Writable for LOCK_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCK_REG to value 0xa638_2d4d"]
impl crate::Resettable for LOCK_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa638_2d4d
    }
}
