#[doc = "Register `MCNT_HI` reader"]
pub struct R(crate::R<MCNT_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCNT_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCNT_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCNT_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCNT_HI` writer"]
pub struct W(crate::W<MCNT_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCNT_HI_SPEC>;
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
impl From<crate::W<MCNT_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCNT_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTR` reader - Thirty-two bit read/write register. If software sets this register to an incrementing value, based on an external\n non-volatile store, this register may be combined with the Monotonic Counter Register to form a 64-bit monotonic counter."]
pub type CNTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNTR` writer - Thirty-two bit read/write register. If software sets this register to an incrementing value, based on an external\n non-volatile store, this register may be combined with the Monotonic Counter Register to form a 64-bit monotonic counter."]
pub type CNTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCNT_HI_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Thirty-two bit read/write register. If software sets this register to an incrementing value, based on an external\n non-volatile store, this register may be combined with the Monotonic Counter Register to form a 64-bit monotonic counter."]
    #[inline(always)]
    pub fn cntr(&self) -> CNTR_R {
        CNTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Thirty-two bit read/write register. If software sets this register to an incrementing value, based on an external\n non-volatile store, this register may be combined with the Monotonic Counter Register to form a 64-bit monotonic counter."]
    #[inline(always)]
    pub fn cntr(&mut self) -> CNTR_W<0> {
        CNTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COUNTER HIWORD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcnt_hi](index.html) module"]
pub struct MCNT_HI_SPEC;
impl crate::RegisterSpec for MCNT_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcnt_hi::R](R) reader structure"]
impl crate::Readable for MCNT_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcnt_hi::W](W) writer structure"]
impl crate::Writable for MCNT_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCNT_HI to value 0"]
impl crate::Resettable for MCNT_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
