#[doc = "Register `RESULT14` reader"]
pub struct R(crate::R<RESULT14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_CH00` reader - DMA CH00"]
pub type DMA_CH00_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH01` reader - DMA CH01"]
pub type DMA_CH01_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH02` reader - DMA CH02"]
pub type DMA_CH02_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH03` reader - DMA CH03"]
pub type DMA_CH03_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH04` reader - DMA CH04"]
pub type DMA_CH04_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH05` reader - DMA CH05"]
pub type DMA_CH05_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH06` reader - DMA CH06"]
pub type DMA_CH06_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH07` reader - DMA CH07"]
pub type DMA_CH07_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH08` reader - DMA CH08"]
pub type DMA_CH08_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CH09` reader - DMA CH09"]
pub type DMA_CH09_R = crate::BitReader<bool>;
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
#[doc = "GIRQ14 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result14](index.html) module"]
pub struct RESULT14_SPEC;
impl crate::RegisterSpec for RESULT14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result14::R](R) reader structure"]
impl crate::Readable for RESULT14_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT14 to value 0"]
impl crate::Resettable for RESULT14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
