#[doc = "Register `VLT_INTSTS` reader"]
pub struct R(crate::R<VLT_INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLT_INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLT_INTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLT_INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLT_INTSTS` writer"]
pub struct W(crate::W<VLT_INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLT_INTSTS_SPEC>;
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
impl From<crate::W<VLT_INTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLT_INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLTINTSTS` reader - Stores the status bits for voltage inputs"]
pub type VLTINTSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VLTINTSTS` writer - Stores the status bits for voltage inputs"]
pub type VLTINTSTS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, VLT_INTSTS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Stores the status bits for voltage inputs"]
    #[inline(always)]
    pub fn vltintsts(&self) -> VLTINTSTS_R {
        VLTINTSTS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Stores the status bits for voltage inputs"]
    #[inline(always)]
    pub fn vltintsts(&mut self) -> VLTINTSTS_W<0> {
        VLTINTSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Volt Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlt_intsts](index.html) module"]
pub struct VLT_INTSTS_SPEC;
impl crate::RegisterSpec for VLT_INTSTS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vlt_intsts::R](R) reader structure"]
impl crate::Readable for VLT_INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vlt_intsts::W](W) writer structure"]
impl crate::Writable for VLT_INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VLT_INTSTS to value 0"]
impl crate::Resettable for VLT_INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
