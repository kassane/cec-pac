#[doc = "Register `LTMON_CTRLSTS` reader"]
pub struct R(crate::R<LTMON_CTRLSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTMON_CTRLSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTMON_CTRLSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTMON_CTRLSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTMON_CTRLSTS` writer"]
pub struct W(crate::W<LTMON_CTRLSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTMON_CTRLSTS_SPEC>;
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
impl From<crate::W<LTMON_CTRLSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTMON_CTRLSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_MTY` reader - FIFO Empty"]
pub type FIFO_MTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_MTY` writer - FIFO Empty"]
pub type FIFO_MTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `FIFO_FUL` reader - FIFO Full"]
pub type FIFO_FUL_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_FUL` writer - FIFO Full"]
pub type FIFO_FUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `FIFO_OVRF` reader - FIFO Overflow"]
pub type FIFO_OVRF_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_OVRF` writer - FIFO Overflow"]
pub type FIFO_OVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `FIFO_UDRF` reader - FIFO Underflow"]
pub type FIFO_UDRF_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_UDRF` writer - FIFO Underflow"]
pub type FIFO_UDRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `CLR_FIFO` reader - Clear FIFO"]
pub type CLR_FIFO_R = crate::BitReader<bool>;
#[doc = "Field `CLR_FIFO` writer - Clear FIFO"]
pub type CLR_FIFO_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTMON_CTRLSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FIFO Empty"]
    #[inline(always)]
    pub fn fifo_mty(&self) -> FIFO_MTY_R {
        FIFO_MTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Full"]
    #[inline(always)]
    pub fn fifo_ful(&self) -> FIFO_FUL_R {
        FIFO_FUL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Overflow"]
    #[inline(always)]
    pub fn fifo_ovrf(&self) -> FIFO_OVRF_R {
        FIFO_OVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO Underflow"]
    #[inline(always)]
    pub fn fifo_udrf(&self) -> FIFO_UDRF_R {
        FIFO_UDRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear FIFO"]
    #[inline(always)]
    pub fn clr_fifo(&self) -> CLR_FIFO_R {
        CLR_FIFO_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Empty"]
    #[inline(always)]
    pub fn fifo_mty(&mut self) -> FIFO_MTY_W<0> {
        FIFO_MTY_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Full"]
    #[inline(always)]
    pub fn fifo_ful(&mut self) -> FIFO_FUL_W<1> {
        FIFO_FUL_W::new(self)
    }
    #[doc = "Bit 2 - FIFO Overflow"]
    #[inline(always)]
    pub fn fifo_ovrf(&mut self) -> FIFO_OVRF_W<2> {
        FIFO_OVRF_W::new(self)
    }
    #[doc = "Bit 3 - FIFO Underflow"]
    #[inline(always)]
    pub fn fifo_udrf(&mut self) -> FIFO_UDRF_W<3> {
        FIFO_UDRF_W::new(self)
    }
    #[doc = "Bit 8 - Clear FIFO"]
    #[inline(always)]
    pub fn clr_fifo(&mut self) -> CLR_FIFO_W<8> {
        CLR_FIFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Load Monitor Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltmon_ctrlsts](index.html) module"]
pub struct LTMON_CTRLSTS_SPEC;
impl crate::RegisterSpec for LTMON_CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltmon_ctrlsts::R](R) reader structure"]
impl crate::Readable for LTMON_CTRLSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltmon_ctrlsts::W](W) writer structure"]
impl crate::Writable for LTMON_CTRLSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTMON_CTRLSTS to value 0"]
impl crate::Resettable for LTMON_CTRLSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
