#[doc = "Register `SRC24` reader"]
pub struct R(crate::R<SRC24_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC24_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC24_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC24_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC24` writer"]
pub struct W(crate::W<SRC24_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC24_SPEC>;
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
impl From<crate::W<SRC24_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC24_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIMON0_VLTN` reader - SPIMON0_VLTN"]
pub type SPIMON0_VLTN_R = crate::BitReader<bool>;
#[doc = "Field `SPIMON0_VLTN` writer - SPIMON0_VLTN"]
pub type SPIMON0_VLTN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC24_SPEC, bool, O>;
#[doc = "Field `SPIMON0_MTMON` reader - SPIMON0_MTMON"]
pub type SPIMON0_MTMON_R = crate::BitReader<bool>;
#[doc = "Field `SPIMON0_MTMON` writer - SPIMON0_MTMON"]
pub type SPIMON0_MTMON_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC24_SPEC, bool, O>;
#[doc = "Field `SPIMON0_LTMON` reader - SPIMON0_LTMON"]
pub type SPIMON0_LTMON_R = crate::BitReader<bool>;
#[doc = "Field `SPIMON0_LTMON` writer - SPIMON0_LTMON"]
pub type SPIMON0_LTMON_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC24_SPEC, bool, O>;
#[doc = "Field `SPIMON1_VLTN` reader - SPIMON1_VLTN"]
pub type SPIMON1_VLTN_R = crate::BitReader<bool>;
#[doc = "Field `SPIMON1_VLTN` writer - SPIMON1_VLTN"]
pub type SPIMON1_VLTN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC24_SPEC, bool, O>;
#[doc = "Field `SPIMON1_MTMON` reader - SPIMON1_MTMON"]
pub type SPIMON1_MTMON_R = crate::BitReader<bool>;
#[doc = "Field `SPIMON1_MTMON` writer - SPIMON1_MTMON"]
pub type SPIMON1_MTMON_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC24_SPEC, bool, O>;
#[doc = "Field `SPIMON1_LTMON` reader - SPIMON1_LTMON"]
pub type SPIMON1_LTMON_R = crate::BitReader<bool>;
#[doc = "Field `SPIMON1_LTMON` writer - SPIMON1_LTMON"]
pub type SPIMON1_LTMON_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC24_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SPIMON0_VLTN"]
    #[inline(always)]
    pub fn spimon0_vltn(&self) -> SPIMON0_VLTN_R {
        SPIMON0_VLTN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPIMON0_MTMON"]
    #[inline(always)]
    pub fn spimon0_mtmon(&self) -> SPIMON0_MTMON_R {
        SPIMON0_MTMON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPIMON0_LTMON"]
    #[inline(always)]
    pub fn spimon0_ltmon(&self) -> SPIMON0_LTMON_R {
        SPIMON0_LTMON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPIMON1_VLTN"]
    #[inline(always)]
    pub fn spimon1_vltn(&self) -> SPIMON1_VLTN_R {
        SPIMON1_VLTN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPIMON1_MTMON"]
    #[inline(always)]
    pub fn spimon1_mtmon(&self) -> SPIMON1_MTMON_R {
        SPIMON1_MTMON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPIMON1_LTMON"]
    #[inline(always)]
    pub fn spimon1_ltmon(&self) -> SPIMON1_LTMON_R {
        SPIMON1_LTMON_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPIMON0_VLTN"]
    #[inline(always)]
    pub fn spimon0_vltn(&mut self) -> SPIMON0_VLTN_W<0> {
        SPIMON0_VLTN_W::new(self)
    }
    #[doc = "Bit 1 - SPIMON0_MTMON"]
    #[inline(always)]
    pub fn spimon0_mtmon(&mut self) -> SPIMON0_MTMON_W<1> {
        SPIMON0_MTMON_W::new(self)
    }
    #[doc = "Bit 2 - SPIMON0_LTMON"]
    #[inline(always)]
    pub fn spimon0_ltmon(&mut self) -> SPIMON0_LTMON_W<2> {
        SPIMON0_LTMON_W::new(self)
    }
    #[doc = "Bit 3 - SPIMON1_VLTN"]
    #[inline(always)]
    pub fn spimon1_vltn(&mut self) -> SPIMON1_VLTN_W<3> {
        SPIMON1_VLTN_W::new(self)
    }
    #[doc = "Bit 4 - SPIMON1_MTMON"]
    #[inline(always)]
    pub fn spimon1_mtmon(&mut self) -> SPIMON1_MTMON_W<4> {
        SPIMON1_MTMON_W::new(self)
    }
    #[doc = "Bit 5 - SPIMON1_LTMON"]
    #[inline(always)]
    pub fn spimon1_ltmon(&mut self) -> SPIMON1_LTMON_W<5> {
        SPIMON1_LTMON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ24 SOURCE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src24](index.html) module"]
pub struct SRC24_SPEC;
impl crate::RegisterSpec for SRC24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src24::R](R) reader structure"]
impl crate::Readable for SRC24_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src24::W](W) writer structure"]
impl crate::Writable for SRC24_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRC24 to value 0"]
impl crate::Resettable for SRC24_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
