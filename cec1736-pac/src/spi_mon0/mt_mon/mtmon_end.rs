#[doc = "Register `MTMON_END` reader"]
pub struct R(crate::R<MTMON_END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTMON_END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTMON_END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTMON_END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTMON_END` writer"]
pub struct W(crate::W<MTMON_END_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTMON_END_SPEC>;
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
impl From<crate::W<MTMON_END_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTMON_END_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `END` reader - The 19 bits of base address within the designated SPI Flash, specifying the last aligned 8K block."]
pub type END_R = crate::FieldReader<u32, u32>;
#[doc = "Field `END` writer - The 19 bits of base address within the designated SPI Flash, specifying the last aligned 8K block."]
pub type END_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTMON_END_SPEC, u32, u32, 19, O>;
impl R {
    #[doc = "Bits 0:18 - The 19 bits of base address within the designated SPI Flash, specifying the last aligned 8K block."]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18 - The 19 bits of base address within the designated SPI Flash, specifying the last aligned 8K block."]
    #[inline(always)]
    pub fn end(&mut self) -> END_W<0> {
        END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Match Monitor Region End Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtmon_end](index.html) module"]
pub struct MTMON_END_SPEC;
impl crate::RegisterSpec for MTMON_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtmon_end::R](R) reader structure"]
impl crate::Readable for MTMON_END_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtmon_end::W](W) writer structure"]
impl crate::Writable for MTMON_END_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTMON_END to value 0"]
impl crate::Resettable for MTMON_END_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
