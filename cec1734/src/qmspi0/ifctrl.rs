#[doc = "Register `IFCTRL` reader"]
pub struct R(crate::R<IFCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFCTRL` writer"]
pub struct W(crate::W<IFCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCTRL_SPEC>;
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
impl From<crate::W<IFCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_PRCT_OUT_VAL` reader - This bit sets the value on the WRITE PROTECT SPI Output Port if it is driven. 1=WRITE PROTECT is driven to 1; 0=WRITE PROTECT is driven to 0"]
pub type WR_PRCT_OUT_VAL_R = crate::BitReader<bool>;
#[doc = "Field `WR_PRCT_OUT_VAL` writer - This bit sets the value on the WRITE PROTECT SPI Output Port if it is driven. 1=WRITE PROTECT is driven to 1; 0=WRITE PROTECT is driven to 0"]
pub type WR_PRCT_OUT_VAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCTRL_SPEC, bool, O>;
#[doc = "Field `WR_PRCT_OUT_EN` reader - 1=WRITE PROTECT SPI Output Port is driven 0=WRITE PROTECT SPI Output Port is not driven"]
pub type WR_PRCT_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `WR_PRCT_OUT_EN` writer - 1=WRITE PROTECT SPI Output Port is driven 0=WRITE PROTECT SPI Output Port is not driven"]
pub type WR_PRCT_OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCTRL_SPEC, bool, O>;
#[doc = "Field `HLD_OUT_VAL` reader - This bit sets the value on the HOLD SPI Output Port if it is driven. 1=HOLD is driven to 1; 0=HOLD is driven to 0."]
pub type HLD_OUT_VAL_R = crate::BitReader<bool>;
#[doc = "Field `HLD_OUT_VAL` writer - This bit sets the value on the HOLD SPI Output Port if it is driven. 1=HOLD is driven to 1; 0=HOLD is driven to 0."]
pub type HLD_OUT_VAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCTRL_SPEC, bool, O>;
#[doc = "Field `HLD_OUT_EN` reader - 1=HOLD SPI Output Port is driven 0=HOLD SPI Output Port is not driven."]
pub type HLD_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `HLD_OUT_EN` writer - 1=HOLD SPI Output Port is driven 0=HOLD SPI Output Port is not driven."]
pub type HLD_OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCTRL_SPEC, bool, O>;
#[doc = "Field `PD_ON_NOT_SEL` reader - 1=Enable pull-down resistors on Receive pins while the SPI Chip Select signal is not asserted 0=No pull-down resistors enabled on Receive pins"]
pub type PD_ON_NOT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PD_ON_NOT_SEL` writer - 1=Enable pull-down resistors on Receive pins while the SPI Chip Select signal is not asserted 0=No pull-down resistors enabled on Receive pins"]
pub type PD_ON_NOT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCTRL_SPEC, bool, O>;
#[doc = "Field `PU_ON_NOTSEL` reader - 1=Enable pull-up resistors on Receive pins while the SPI Chip Select signal is not asserted 0=No pull-up resistors enabled on Receive pins."]
pub type PU_ON_NOTSEL_R = crate::BitReader<bool>;
#[doc = "Field `PU_ON_NOTSEL` writer - 1=Enable pull-up resistors on Receive pins while the SPI Chip Select signal is not asserted 0=No pull-up resistors enabled on Receive pins."]
pub type PU_ON_NOTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCTRL_SPEC, bool, O>;
#[doc = "Field `PD_ON_NOTDRIVEN` reader - 1=Enable pull-down resistors on Transmit pins while the pins are not driven 0=No pull-down resistors enabled ion Transmit pins."]
pub type PD_ON_NOTDRIVEN_R = crate::BitReader<bool>;
#[doc = "Field `PD_ON_NOTDRIVEN` writer - 1=Enable pull-down resistors on Transmit pins while the pins are not driven 0=No pull-down resistors enabled ion Transmit pins."]
pub type PD_ON_NOTDRIVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCTRL_SPEC, bool, O>;
#[doc = "Field `PU_ON_NOTDRIVEN` reader - 1=Enable pull-up resistors on Transmit pins while the pins are not driven 0=No pull-up resistors enabled ion Transmit pins."]
pub type PU_ON_NOTDRIVEN_R = crate::BitReader<bool>;
#[doc = "Field `PU_ON_NOTDRIVEN` writer - 1=Enable pull-up resistors on Transmit pins while the pins are not driven 0=No pull-up resistors enabled ion Transmit pins."]
pub type PU_ON_NOTDRIVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit sets the value on the WRITE PROTECT SPI Output Port if it is driven. 1=WRITE PROTECT is driven to 1; 0=WRITE PROTECT is driven to 0"]
    #[inline(always)]
    pub fn wr_prct_out_val(&self) -> WR_PRCT_OUT_VAL_R {
        WR_PRCT_OUT_VAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1=WRITE PROTECT SPI Output Port is driven 0=WRITE PROTECT SPI Output Port is not driven"]
    #[inline(always)]
    pub fn wr_prct_out_en(&self) -> WR_PRCT_OUT_EN_R {
        WR_PRCT_OUT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit sets the value on the HOLD SPI Output Port if it is driven. 1=HOLD is driven to 1; 0=HOLD is driven to 0."]
    #[inline(always)]
    pub fn hld_out_val(&self) -> HLD_OUT_VAL_R {
        HLD_OUT_VAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1=HOLD SPI Output Port is driven 0=HOLD SPI Output Port is not driven."]
    #[inline(always)]
    pub fn hld_out_en(&self) -> HLD_OUT_EN_R {
        HLD_OUT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1=Enable pull-down resistors on Receive pins while the SPI Chip Select signal is not asserted 0=No pull-down resistors enabled on Receive pins"]
    #[inline(always)]
    pub fn pd_on_not_sel(&self) -> PD_ON_NOT_SEL_R {
        PD_ON_NOT_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1=Enable pull-up resistors on Receive pins while the SPI Chip Select signal is not asserted 0=No pull-up resistors enabled on Receive pins."]
    #[inline(always)]
    pub fn pu_on_notsel(&self) -> PU_ON_NOTSEL_R {
        PU_ON_NOTSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1=Enable pull-down resistors on Transmit pins while the pins are not driven 0=No pull-down resistors enabled ion Transmit pins."]
    #[inline(always)]
    pub fn pd_on_notdriven(&self) -> PD_ON_NOTDRIVEN_R {
        PD_ON_NOTDRIVEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1=Enable pull-up resistors on Transmit pins while the pins are not driven 0=No pull-up resistors enabled ion Transmit pins."]
    #[inline(always)]
    pub fn pu_on_notdriven(&self) -> PU_ON_NOTDRIVEN_R {
        PU_ON_NOTDRIVEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value on the WRITE PROTECT SPI Output Port if it is driven. 1=WRITE PROTECT is driven to 1; 0=WRITE PROTECT is driven to 0"]
    #[inline(always)]
    pub fn wr_prct_out_val(&mut self) -> WR_PRCT_OUT_VAL_W<0> {
        WR_PRCT_OUT_VAL_W::new(self)
    }
    #[doc = "Bit 1 - 1=WRITE PROTECT SPI Output Port is driven 0=WRITE PROTECT SPI Output Port is not driven"]
    #[inline(always)]
    pub fn wr_prct_out_en(&mut self) -> WR_PRCT_OUT_EN_W<1> {
        WR_PRCT_OUT_EN_W::new(self)
    }
    #[doc = "Bit 2 - This bit sets the value on the HOLD SPI Output Port if it is driven. 1=HOLD is driven to 1; 0=HOLD is driven to 0."]
    #[inline(always)]
    pub fn hld_out_val(&mut self) -> HLD_OUT_VAL_W<2> {
        HLD_OUT_VAL_W::new(self)
    }
    #[doc = "Bit 3 - 1=HOLD SPI Output Port is driven 0=HOLD SPI Output Port is not driven."]
    #[inline(always)]
    pub fn hld_out_en(&mut self) -> HLD_OUT_EN_W<3> {
        HLD_OUT_EN_W::new(self)
    }
    #[doc = "Bit 4 - 1=Enable pull-down resistors on Receive pins while the SPI Chip Select signal is not asserted 0=No pull-down resistors enabled on Receive pins"]
    #[inline(always)]
    pub fn pd_on_not_sel(&mut self) -> PD_ON_NOT_SEL_W<4> {
        PD_ON_NOT_SEL_W::new(self)
    }
    #[doc = "Bit 5 - 1=Enable pull-up resistors on Receive pins while the SPI Chip Select signal is not asserted 0=No pull-up resistors enabled on Receive pins."]
    #[inline(always)]
    pub fn pu_on_notsel(&mut self) -> PU_ON_NOTSEL_W<5> {
        PU_ON_NOTSEL_W::new(self)
    }
    #[doc = "Bit 6 - 1=Enable pull-down resistors on Transmit pins while the pins are not driven 0=No pull-down resistors enabled ion Transmit pins."]
    #[inline(always)]
    pub fn pd_on_notdriven(&mut self) -> PD_ON_NOTDRIVEN_W<6> {
        PD_ON_NOTDRIVEN_W::new(self)
    }
    #[doc = "Bit 7 - 1=Enable pull-up resistors on Transmit pins while the pins are not driven 0=No pull-up resistors enabled ion Transmit pins."]
    #[inline(always)]
    pub fn pu_on_notdriven(&mut self) -> PU_ON_NOTDRIVEN_W<7> {
        PU_ON_NOTDRIVEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Interface Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifctrl](index.html) module"]
pub struct IFCTRL_SPEC;
impl crate::RegisterSpec for IFCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifctrl::R](R) reader structure"]
impl crate::Readable for IFCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifctrl::W](W) writer structure"]
impl crate::Writable for IFCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFCTRL to value 0"]
impl crate::Resettable for IFCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
