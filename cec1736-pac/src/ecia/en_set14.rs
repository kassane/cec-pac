#[doc = "Register `EN_SET14` reader"]
pub struct R(crate::R<EN_SET14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_SET14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_SET14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_SET14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN_SET14` writer"]
pub struct W(crate::W<EN_SET14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_SET14_SPEC>;
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
impl From<crate::W<EN_SET14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_SET14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_CH00` reader - DMA CH00"]
pub type DMA_CH00_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH00` writer - DMA CH00"]
pub type DMA_CH00_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET14_SPEC, bool, O>;
#[doc = "Field `DMA_CH01` reader - DMA CH01"]
pub type DMA_CH01_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH01` writer - DMA CH01"]
pub type DMA_CH01_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET14_SPEC, bool, O>;
#[doc = "Field `DMA_CH02` reader - DMA CH02"]
pub type DMA_CH02_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH02` writer - DMA CH02"]
pub type DMA_CH02_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET14_SPEC, bool, O>;
#[doc = "Field `DMA_CH03` reader - DMA CH03"]
pub type DMA_CH03_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH03` writer - DMA CH03"]
pub type DMA_CH03_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET14_SPEC, bool, O>;
#[doc = "Field `DMA_CH04` reader - DMA CH04"]
pub type DMA_CH04_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH04` writer - DMA CH04"]
pub type DMA_CH04_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET14_SPEC, bool, O>;
#[doc = "Field `DMA_CH05` reader - DMA CH05"]
pub type DMA_CH05_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH05` writer - DMA CH05"]
pub type DMA_CH05_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET14_SPEC, bool, O>;
#[doc = "Field `DMA_CH06` reader - DMA CH06"]
pub type DMA_CH06_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH06` writer - DMA CH06"]
pub type DMA_CH06_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET14_SPEC, bool, O>;
#[doc = "Field `DMA_CH07` reader - DMA CH07"]
pub type DMA_CH07_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH07` writer - DMA CH07"]
pub type DMA_CH07_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET14_SPEC, bool, O>;
#[doc = "Field `DMA_CH08` reader - DMA CH08"]
pub type DMA_CH08_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH08` writer - DMA CH08"]
pub type DMA_CH08_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET14_SPEC, bool, O>;
#[doc = "Field `DMA_CH09` reader - DMA CH09"]
pub type DMA_CH09_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH09` writer - DMA CH09"]
pub type DMA_CH09_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET14_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DMA CH00"]
    #[inline(always)]
    pub fn dma_ch00(&self) -> DMA_CH00_R {
        DMA_CH00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA CH01"]
    #[inline(always)]
    pub fn dma_ch01(&self) -> DMA_CH01_R {
        DMA_CH01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA CH02"]
    #[inline(always)]
    pub fn dma_ch02(&self) -> DMA_CH02_R {
        DMA_CH02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA CH03"]
    #[inline(always)]
    pub fn dma_ch03(&self) -> DMA_CH03_R {
        DMA_CH03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA CH04"]
    #[inline(always)]
    pub fn dma_ch04(&self) -> DMA_CH04_R {
        DMA_CH04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA CH05"]
    #[inline(always)]
    pub fn dma_ch05(&self) -> DMA_CH05_R {
        DMA_CH05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA CH06"]
    #[inline(always)]
    pub fn dma_ch06(&self) -> DMA_CH06_R {
        DMA_CH06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA CH07"]
    #[inline(always)]
    pub fn dma_ch07(&self) -> DMA_CH07_R {
        DMA_CH07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA CH08"]
    #[inline(always)]
    pub fn dma_ch08(&self) -> DMA_CH08_R {
        DMA_CH08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA CH09"]
    #[inline(always)]
    pub fn dma_ch09(&self) -> DMA_CH09_R {
        DMA_CH09_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA CH00"]
    #[inline(always)]
    pub fn dma_ch00(&mut self) -> DMA_CH00_W<0> {
        DMA_CH00_W::new(self)
    }
    #[doc = "Bit 1 - DMA CH01"]
    #[inline(always)]
    pub fn dma_ch01(&mut self) -> DMA_CH01_W<1> {
        DMA_CH01_W::new(self)
    }
    #[doc = "Bit 2 - DMA CH02"]
    #[inline(always)]
    pub fn dma_ch02(&mut self) -> DMA_CH02_W<2> {
        DMA_CH02_W::new(self)
    }
    #[doc = "Bit 3 - DMA CH03"]
    #[inline(always)]
    pub fn dma_ch03(&mut self) -> DMA_CH03_W<3> {
        DMA_CH03_W::new(self)
    }
    #[doc = "Bit 4 - DMA CH04"]
    #[inline(always)]
    pub fn dma_ch04(&mut self) -> DMA_CH04_W<4> {
        DMA_CH04_W::new(self)
    }
    #[doc = "Bit 5 - DMA CH05"]
    #[inline(always)]
    pub fn dma_ch05(&mut self) -> DMA_CH05_W<5> {
        DMA_CH05_W::new(self)
    }
    #[doc = "Bit 6 - DMA CH06"]
    #[inline(always)]
    pub fn dma_ch06(&mut self) -> DMA_CH06_W<6> {
        DMA_CH06_W::new(self)
    }
    #[doc = "Bit 7 - DMA CH07"]
    #[inline(always)]
    pub fn dma_ch07(&mut self) -> DMA_CH07_W<7> {
        DMA_CH07_W::new(self)
    }
    #[doc = "Bit 8 - DMA CH08"]
    #[inline(always)]
    pub fn dma_ch08(&mut self) -> DMA_CH08_W<8> {
        DMA_CH08_W::new(self)
    }
    #[doc = "Bit 9 - DMA CH09"]
    #[inline(always)]
    pub fn dma_ch09(&mut self) -> DMA_CH09_W<9> {
        DMA_CH09_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ14 ENABLE SET\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_set14](index.html) module"]
pub struct EN_SET14_SPEC;
impl crate::RegisterSpec for EN_SET14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en_set14::R](R) reader structure"]
impl crate::Readable for EN_SET14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en_set14::W](W) writer structure"]
impl crate::Writable for EN_SET14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN_SET14 to value 0"]
impl crate::Resettable for EN_SET14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
