#[doc = "Register `RT_START` reader"]
pub struct R(crate::R<RT_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RT_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RT_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RT_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RT_START` writer"]
pub struct W(crate::W<RT_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RT_START_SPEC>;
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
impl From<crate::W<RT_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RT_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRT` reader - A Flash address shifted by 12 (4K byte units). Bottom 12 address bits are 000h"]
pub type STRT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STRT` writer - A Flash address shifted by 12 (4K byte units). Bottom 12 address bits are 000h"]
pub type STRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RT_START_SPEC, u32, u32, 20, O>;
#[doc = "Field `E32` reader - Enable 32KByte Erase opcode for this region. 0 = Disable, 1 = Enable"]
pub type E32_R = crate::BitReader<bool>;
#[doc = "Field `E32` writer - Enable 32KByte Erase opcode for this region. 0 = Disable, 1 = Enable"]
pub type E32_W<'a, const O: u8> = crate::BitWriter<'a, u32, RT_START_SPEC, bool, O>;
#[doc = "Field `E64` reader - Enable 64KByte Erase opcode for this region. 0 = Disable, 1 = Enable"]
pub type E64_R = crate::BitReader<bool>;
#[doc = "Field `E64` writer - Enable 64KByte Erase opcode for this region. 0 = Disable, 1 = Enable"]
pub type E64_W<'a, const O: u8> = crate::BitWriter<'a, u32, RT_START_SPEC, bool, O>;
#[doc = "Field `DV` reader - Flash Device 0 = CS0#, 1 = CS1#."]
pub type DV_R = crate::BitReader<bool>;
#[doc = "Field `DV` writer - Flash Device 0 = CS0#, 1 = CS1#."]
pub type DV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RT_START_SPEC, bool, O>;
#[doc = "Field `RD` reader - Read Allowed for Region. 0 = No, 1 = Yes"]
pub type RD_R = crate::BitReader<bool>;
#[doc = "Field `RD` writer - Read Allowed for Region. 0 = No, 1 = Yes"]
pub type RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RT_START_SPEC, bool, O>;
#[doc = "Field `WR` reader - Write Allowed for Region. 0 = No, 1 = Yes"]
pub type WR_R = crate::BitReader<bool>;
#[doc = "Field `WR` writer - Write Allowed for Region. 0 = No, 1 = Yes"]
pub type WR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RT_START_SPEC, bool, O>;
#[doc = "Field `EN` reader - Enable Register Pair for Monitoring. 0 = No, 1 = Yes"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable Register Pair for Monitoring. 0 = No, 1 = Yes"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RT_START_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:19 - A Flash address shifted by 12 (4K byte units). Bottom 12 address bits are 000h"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 24 - Enable 32KByte Erase opcode for this region. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn e32(&self) -> E32_R {
        E32_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable 64KByte Erase opcode for this region. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn e64(&self) -> E64_R {
        E64_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Flash Device 0 = CS0#, 1 = CS1#."]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Read Allowed for Region. 0 = No, 1 = Yes"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Write Allowed for Region. 0 = No, 1 = Yes"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Register Pair for Monitoring. 0 = No, 1 = Yes"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - A Flash address shifted by 12 (4K byte units). Bottom 12 address bits are 000h"]
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<0> {
        STRT_W::new(self)
    }
    #[doc = "Bit 24 - Enable 32KByte Erase opcode for this region. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn e32(&mut self) -> E32_W<24> {
        E32_W::new(self)
    }
    #[doc = "Bit 25 - Enable 64KByte Erase opcode for this region. 0 = Disable, 1 = Enable"]
    #[inline(always)]
    pub fn e64(&mut self) -> E64_W<25> {
        E64_W::new(self)
    }
    #[doc = "Bit 27 - Flash Device 0 = CS0#, 1 = CS1#."]
    #[inline(always)]
    pub fn dv(&mut self) -> DV_W<27> {
        DV_W::new(self)
    }
    #[doc = "Bit 28 - Read Allowed for Region. 0 = No, 1 = Yes"]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W<28> {
        RD_W::new(self)
    }
    #[doc = "Bit 29 - Write Allowed for Region. 0 = No, 1 = Yes"]
    #[inline(always)]
    pub fn wr(&mut self) -> WR_W<29> {
        WR_W::new(self)
    }
    #[doc = "Bit 31 - Enable Register Pair for Monitoring. 0 = No, 1 = Yes"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<31> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Runtime Monitoring Start Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rt_start](index.html) module"]
pub struct RT_START_SPEC;
impl crate::RegisterSpec for RT_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rt_start::R](R) reader structure"]
impl crate::Readable for RT_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rt_start::W](W) writer structure"]
impl crate::Writable for RT_START_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RT_START to value 0"]
impl crate::Resettable for RT_START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
