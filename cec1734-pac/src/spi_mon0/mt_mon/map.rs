#[doc = "Register `MAP` reader"]
pub struct R(crate::R<MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAP` writer"]
pub struct W(crate::W<MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAP_SPEC>;
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
impl From<crate::W<MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAP` reader - The 19 bits of base address within the designated SPI Flash, specifying the last aligned 8K block."]
pub type MAP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MAP` writer - The 19 bits of base address within the designated SPI Flash, specifying the last aligned 8K block."]
pub type MAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAP_SPEC, u32, u32, 19, O>;
#[doc = "Field `ME` reader - Match Enable for individual Region R"]
pub type ME_R = crate::BitReader<bool>;
#[doc = "Field `ME` writer - Match Enable for individual Region R"]
pub type ME_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAP_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:18 - The 19 bits of base address within the designated SPI Flash, specifying the last aligned 8K block."]
    #[inline(always)]
    pub fn map(&self) -> MAP_R {
        MAP_R::new((self.bits & 0x0007_ffff) as u32)
    }
    #[doc = "Bit 31 - Match Enable for individual Region R"]
    #[inline(always)]
    pub fn me(&self) -> ME_R {
        ME_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18 - The 19 bits of base address within the designated SPI Flash, specifying the last aligned 8K block."]
    #[inline(always)]
    pub fn map(&mut self) -> MAP_W<0> {
        MAP_W::new(self)
    }
    #[doc = "Bit 31 - Match Enable for individual Region R"]
    #[inline(always)]
    pub fn me(&mut self) -> ME_W<31> {
        ME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Match Monitor Region Map Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [map](index.html) module"]
pub struct MAP_SPEC;
impl crate::RegisterSpec for MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [map::R](R) reader structure"]
impl crate::Readable for MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [map::W](W) writer structure"]
impl crate::Writable for MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAP to value 0"]
impl crate::Resettable for MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
