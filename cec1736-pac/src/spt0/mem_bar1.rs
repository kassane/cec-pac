#[doc = "Register `MEM_BAR1` reader"]
pub struct R(crate::R<MEM_BAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_BAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_BAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_BAR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_BAR1` writer"]
pub struct W(crate::W<MEM_BAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_BAR1_SPEC>;
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
impl From<crate::W<MEM_BAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_BAR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD1` reader - Base Address for Region 1."]
pub type ADD1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADD1` writer - Base Address for Region 1."]
pub type ADD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEM_BAR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Base Address for Region 1."]
    #[inline(always)]
    pub fn add1(&self) -> ADD1_R {
        ADD1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base Address for Region 1."]
    #[inline(always)]
    pub fn add1(&mut self) -> ADD1_W<0> {
        ADD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Peripheral Target Memory Base Address1 Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_bar1](index.html) module"]
pub struct MEM_BAR1_SPEC;
impl crate::RegisterSpec for MEM_BAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_bar1::R](R) reader structure"]
impl crate::Readable for MEM_BAR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_bar1::W](W) writer structure"]
impl crate::Writable for MEM_BAR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_BAR1 to value 0"]
impl crate::Resettable for MEM_BAR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
