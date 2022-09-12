#[doc = "Register `SHDW_DATA` reader"]
pub struct R(crate::R<SHDW_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHDW_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHDW_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHDW_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHDW_DATA` writer"]
pub struct W(crate::W<SHDW_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHDW_DATA_SPEC>;
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
impl From<crate::W<SHDW_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHDW_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHDW_DATA` reader - This is the I2C Shadow Data Register"]
pub type SHDW_DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHDW_DATA` writer - This is the I2C Shadow Data Register"]
pub type SHDW_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SHDW_DATA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This is the I2C Shadow Data Register"]
    #[inline(always)]
    pub fn shdw_data(&self) -> SHDW_DATA_R {
        SHDW_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - This is the I2C Shadow Data Register"]
    #[inline(always)]
    pub fn shdw_data(&mut self) -> SHDW_DATA_W<0> {
        SHDW_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the I2C Shadow Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_data](index.html) module"]
pub struct SHDW_DATA_SPEC;
impl crate::RegisterSpec for SHDW_DATA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [shdw_data::R](R) reader structure"]
impl crate::Readable for SHDW_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shdw_data::W](W) writer structure"]
impl crate::Writable for SHDW_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHDW_DATA to value 0"]
impl crate::Resettable for SHDW_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
