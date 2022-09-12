#[doc = "Register `MTMON_CTRLSTS` reader"]
pub struct R(crate::R<MTMON_CTRLSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTMON_CTRLSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTMON_CTRLSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTMON_CTRLSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTMON_CTRLSTS` writer"]
pub struct W(crate::W<MTMON_CTRLSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTMON_CTRLSTS_SPEC>;
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
impl From<crate::W<MTMON_CTRLSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTMON_CTRLSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T` reader - Timeout"]
pub type T_R = crate::BitReader<bool>;
#[doc = "Field `T` writer - Timeout"]
pub type T_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `F` reader - First Fetch in any Match region"]
pub type F_R = crate::BitReader<bool>;
#[doc = "Field `F` writer - First Fetch in any Match region"]
pub type F_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `AM` reader - Set to 1 on a 3B/4B Address Mode switch on either Flash."]
pub type AM_R = crate::BitReader<bool>;
#[doc = "Field `AM` writer - Set to 1 on a 3B/4B Address Mode switch on either Flash."]
pub type AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `ET_IRQ` reader - Enable Timeout Interrupt"]
pub type ET_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `ET_IRQ` writer - Enable Timeout Interrupt"]
pub type ET_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `EF_IRQ` reader - Enable First Fetch in any Match region Interrupt"]
pub type EF_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `EF_IRQ` writer - Enable First Fetch in any Match region Interrupt"]
pub type EF_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `AM_IRQ` reader - Enable 3B/4B Address Mode switch on either Flash Interrupt"]
pub type AM_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `AM_IRQ` writer - Enable 3B/4B Address Mode switch on either Flash Interrupt"]
pub type AM_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `FIFO_MTY` reader - FIFO Empty"]
pub type FIFO_MTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_MTY` writer - FIFO Empty"]
pub type FIFO_MTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `FIFO_FUL` reader - FIFO Full"]
pub type FIFO_FUL_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_FUL` writer - FIFO Full"]
pub type FIFO_FUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `FIFO_OVRF` reader - FIFO Overflow"]
pub type FIFO_OVRF_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_OVRF` writer - FIFO Overflow"]
pub type FIFO_OVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `FIFO_UDRF` reader - FIFO Underflow"]
pub type FIFO_UDRF_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_UDRF` writer - FIFO Underflow"]
pub type FIFO_UDRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `CLR_FIFO` reader - Clear FIFO. This field is autocleared by hardware"]
pub type CLR_FIFO_R = crate::BitReader<bool>;
#[doc = "Field `CLR_FIFO` writer - Clear FIFO. This field is autocleared by hardware"]
pub type CLR_FIFO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_CTRLSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timeout"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - First Fetch in any Match region"]
    #[inline(always)]
    pub fn f(&self) -> F_R {
        F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set to 1 on a 3B/4B Address Mode switch on either Flash."]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Timeout Interrupt"]
    #[inline(always)]
    pub fn et_irq(&self) -> ET_IRQ_R {
        ET_IRQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable First Fetch in any Match region Interrupt"]
    #[inline(always)]
    pub fn ef_irq(&self) -> EF_IRQ_R {
        EF_IRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable 3B/4B Address Mode switch on either Flash Interrupt"]
    #[inline(always)]
    pub fn am_irq(&self) -> AM_IRQ_R {
        AM_IRQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - FIFO Empty"]
    #[inline(always)]
    pub fn fifo_mty(&self) -> FIFO_MTY_R {
        FIFO_MTY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FIFO Full"]
    #[inline(always)]
    pub fn fifo_ful(&self) -> FIFO_FUL_R {
        FIFO_FUL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FIFO Overflow"]
    #[inline(always)]
    pub fn fifo_ovrf(&self) -> FIFO_OVRF_R {
        FIFO_OVRF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - FIFO Underflow"]
    #[inline(always)]
    pub fn fifo_udrf(&self) -> FIFO_UDRF_R {
        FIFO_UDRF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Clear FIFO. This field is autocleared by hardware"]
    #[inline(always)]
    pub fn clr_fifo(&self) -> CLR_FIFO_R {
        CLR_FIFO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout"]
    #[inline(always)]
    pub fn t(&mut self) -> T_W<0> {
        T_W::new(self)
    }
    #[doc = "Bit 1 - First Fetch in any Match region"]
    #[inline(always)]
    pub fn f(&mut self) -> F_W<1> {
        F_W::new(self)
    }
    #[doc = "Bit 2 - Set to 1 on a 3B/4B Address Mode switch on either Flash."]
    #[inline(always)]
    pub fn am(&mut self) -> AM_W<2> {
        AM_W::new(self)
    }
    #[doc = "Bit 8 - Enable Timeout Interrupt"]
    #[inline(always)]
    pub fn et_irq(&mut self) -> ET_IRQ_W<8> {
        ET_IRQ_W::new(self)
    }
    #[doc = "Bit 9 - Enable First Fetch in any Match region Interrupt"]
    #[inline(always)]
    pub fn ef_irq(&mut self) -> EF_IRQ_W<9> {
        EF_IRQ_W::new(self)
    }
    #[doc = "Bit 10 - Enable 3B/4B Address Mode switch on either Flash Interrupt"]
    #[inline(always)]
    pub fn am_irq(&mut self) -> AM_IRQ_W<10> {
        AM_IRQ_W::new(self)
    }
    #[doc = "Bit 16 - FIFO Empty"]
    #[inline(always)]
    pub fn fifo_mty(&mut self) -> FIFO_MTY_W<16> {
        FIFO_MTY_W::new(self)
    }
    #[doc = "Bit 17 - FIFO Full"]
    #[inline(always)]
    pub fn fifo_ful(&mut self) -> FIFO_FUL_W<17> {
        FIFO_FUL_W::new(self)
    }
    #[doc = "Bit 18 - FIFO Overflow"]
    #[inline(always)]
    pub fn fifo_ovrf(&mut self) -> FIFO_OVRF_W<18> {
        FIFO_OVRF_W::new(self)
    }
    #[doc = "Bit 19 - FIFO Underflow"]
    #[inline(always)]
    pub fn fifo_udrf(&mut self) -> FIFO_UDRF_W<19> {
        FIFO_UDRF_W::new(self)
    }
    #[doc = "Bit 24 - Clear FIFO. This field is autocleared by hardware"]
    #[inline(always)]
    pub fn clr_fifo(&mut self) -> CLR_FIFO_W<24> {
        CLR_FIFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Match Monitor Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtmon_ctrlsts](index.html) module"]
pub struct MTMON_CTRLSTS_SPEC;
impl crate::RegisterSpec for MTMON_CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtmon_ctrlsts::R](R) reader structure"]
impl crate::Readable for MTMON_CTRLSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtmon_ctrlsts::W](W) writer structure"]
impl crate::Writable for MTMON_CTRLSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTMON_CTRLSTS to value 0"]
impl crate::Resettable for MTMON_CTRLSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
