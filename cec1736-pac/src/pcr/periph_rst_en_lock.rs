#[doc = "Register `PERIPH_RST_EN_LOCK` reader"]
pub struct R(crate::R<PERIPH_RST_EN_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_RST_EN_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_RST_EN_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_RST_EN_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIPH_RST_EN_LOCK` writer"]
pub struct W(crate::W<PERIPH_RST_EN_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIPH_RST_EN_LOCK_SPEC>;
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
impl From<crate::W<PERIPH_RST_EN_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIPH_RST_EN_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - PCR Reset Enable Lock Register."]
pub type EN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EN` writer - PCR Reset Enable Lock Register."]
pub type EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERIPH_RST_EN_LOCK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - PCR Reset Enable Lock Register."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PCR Reset Enable Lock Register."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_rst_en_lock](index.html) module"]
pub struct PERIPH_RST_EN_LOCK_SPEC;
impl crate::RegisterSpec for PERIPH_RST_EN_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periph_rst_en_lock::R](R) reader structure"]
impl crate::Readable for PERIPH_RST_EN_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [periph_rst_en_lock::W](W) writer structure"]
impl crate::Writable for PERIPH_RST_EN_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIPH_RST_EN_LOCK to value 0xa638_2d4d"]
impl crate::Resettable for PERIPH_RST_EN_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa638_2d4d
    }
}
