#[doc = "Register `SPIMON_IB_CNGF` reader"]
pub struct R(crate::R<SPIMON_IB_CNGF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIMON_IB_CNGF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIMON_IB_CNGF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIMON_IB_CNGF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIMON_IB_CNGF` writer"]
pub struct W(crate::W<SPIMON_IB_CNGF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIMON_IB_CNGF_SPEC>;
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
impl From<crate::W<SPIMON_IB_CNGF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIMON_IB_CNGF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MON0` reader - QSPI0 Monitor 0. 1 = Route QMSPI0 Traffic to Monitor. 0 = Route Host AP0 Traffic to Monitor"]
pub type MON0_R = crate::BitReader<bool>;
#[doc = "Field `MON0` writer - QSPI0 Monitor 0. 1 = Route QMSPI0 Traffic to Monitor. 0 = Route Host AP0 Traffic to Monitor"]
pub type MON0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIMON_IB_CNGF_SPEC, bool, O>;
#[doc = "Field `MON1` reader - QSPI1 Monitor 1. 1 = Route QMSPI1 Traffic to Monitor. 0 = Route Host AP1 Traffic to Monitor"]
pub type MON1_R = crate::BitReader<bool>;
#[doc = "Field `MON1` writer - QSPI1 Monitor 1. 1 = Route QMSPI1 Traffic to Monitor. 0 = Route Host AP1 Traffic to Monitor"]
pub type MON1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIMON_IB_CNGF_SPEC, bool, O>;
#[doc = "Field `IDL` reader - Lock bit for IDE"]
pub type IDL_R = crate::BitReader<bool>;
#[doc = "Field `IDL` writer - Lock bit for IDE"]
pub type IDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIMON_IB_CNGF_SPEC, bool, O>;
#[doc = "Field `IDV` reader - Delay Value"]
pub type IDV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDV` writer - Delay Value"]
pub type IDV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPIMON_IB_CNGF_SPEC, u8, u8, 5, O>;
#[doc = "Field `IDU` reader - Delay Units"]
pub type IDU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDU` writer - Delay Units"]
pub type IDU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPIMON_IB_CNGF_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDE` reader - Inter-Bus Intervention Enable"]
pub type IDE_R = crate::BitReader<bool>;
#[doc = "Field `IDE` writer - Inter-Bus Intervention Enable"]
pub type IDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIMON_IB_CNGF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - QSPI0 Monitor 0. 1 = Route QMSPI0 Traffic to Monitor. 0 = Route Host AP0 Traffic to Monitor"]
    #[inline(always)]
    pub fn mon0(&self) -> MON0_R {
        MON0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - QSPI1 Monitor 1. 1 = Route QMSPI1 Traffic to Monitor. 0 = Route Host AP1 Traffic to Monitor"]
    #[inline(always)]
    pub fn mon1(&self) -> MON1_R {
        MON1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Lock bit for IDE"]
    #[inline(always)]
    pub fn idl(&self) -> IDL_R {
        IDL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 20:24 - Delay Value"]
    #[inline(always)]
    pub fn idv(&self) -> IDV_R {
        IDV_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:26 - Delay Units"]
    #[inline(always)]
    pub fn idu(&self) -> IDU_R {
        IDU_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Inter-Bus Intervention Enable"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - QSPI0 Monitor 0. 1 = Route QMSPI0 Traffic to Monitor. 0 = Route Host AP0 Traffic to Monitor"]
    #[inline(always)]
    pub fn mon0(&mut self) -> MON0_W<8> {
        MON0_W::new(self)
    }
    #[doc = "Bit 9 - QSPI1 Monitor 1. 1 = Route QMSPI1 Traffic to Monitor. 0 = Route Host AP1 Traffic to Monitor"]
    #[inline(always)]
    pub fn mon1(&mut self) -> MON1_W<9> {
        MON1_W::new(self)
    }
    #[doc = "Bit 12 - Lock bit for IDE"]
    #[inline(always)]
    pub fn idl(&mut self) -> IDL_W<12> {
        IDL_W::new(self)
    }
    #[doc = "Bits 20:24 - Delay Value"]
    #[inline(always)]
    pub fn idv(&mut self) -> IDV_W<20> {
        IDV_W::new(self)
    }
    #[doc = "Bits 25:26 - Delay Units"]
    #[inline(always)]
    pub fn idu(&mut self) -> IDU_W<25> {
        IDU_W::new(self)
    }
    #[doc = "Bit 27 - Inter-Bus Intervention Enable"]
    #[inline(always)]
    pub fn ide(&mut self) -> IDE_W<27> {
        IDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Monitor's Inter-Bus Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spimon_ib_cngf](index.html) module"]
pub struct SPIMON_IB_CNGF_SPEC;
impl crate::RegisterSpec for SPIMON_IB_CNGF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spimon_ib_cngf::R](R) reader structure"]
impl crate::Readable for SPIMON_IB_CNGF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spimon_ib_cngf::W](W) writer structure"]
impl crate::Writable for SPIMON_IB_CNGF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPIMON_IB_CNGF to value 0"]
impl crate::Resettable for SPIMON_IB_CNGF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
