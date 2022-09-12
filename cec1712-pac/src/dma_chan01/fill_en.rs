#[doc = "Register `FILL_EN` reader"]
pub struct R(crate::R<FILL_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILL_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILL_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILL_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILL_EN` writer"]
pub struct W(crate::W<FILL_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILL_EN_SPEC>;
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
impl From<crate::W<FILL_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILL_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - 1=Enable the DMA Channel Fill Engine N\n 0=Disable the DMA Channel Fill Engine"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - 1=Enable the DMA Channel Fill Engine N\n 0=Disable the DMA Channel Fill Engine"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FILL_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 1=Enable the DMA Channel Fill Engine N\n 0=Disable the DMA Channel Fill Engine"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1=Enable the DMA Channel Fill Engine N\n 0=Disable the DMA Channel Fill Engine"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA CHANNEL N FILL ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fill_en](index.html) module"]
pub struct FILL_EN_SPEC;
impl crate::RegisterSpec for FILL_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fill_en::R](R) reader structure"]
impl crate::Readable for FILL_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fill_en::W](W) writer structure"]
impl crate::Writable for FILL_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILL_EN to value 0"]
impl crate::Resettable for FILL_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
