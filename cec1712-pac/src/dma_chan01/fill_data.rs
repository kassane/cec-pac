#[doc = "Register `FILL_DATA` reader"]
pub struct R(crate::R<FILL_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILL_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILL_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILL_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILL_DATA` writer"]
pub struct W(crate::W<FILL_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILL_DATA_SPEC>;
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
impl From<crate::W<FILL_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILL_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - This is the data pattern used to fill memory."]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - This is the data pattern used to fill memory."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILL_DATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This is the data pattern used to fill memory."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the data pattern used to fill memory."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA CHANNEL N FILL DATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fill_data](index.html) module"]
pub struct FILL_DATA_SPEC;
impl crate::RegisterSpec for FILL_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fill_data::R](R) reader structure"]
impl crate::Readable for FILL_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fill_data::W](W) writer structure"]
impl crate::Writable for FILL_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILL_DATA to value 0"]
impl crate::Resettable for FILL_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
