#[doc = "Register `MAN_CTRL` reader"]
pub struct R(crate::R<MAN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAN_CTRL` writer"]
pub struct W(crate::W<MAN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAN_CTRL_SPEC>;
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
impl From<crate::W<MAN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAN_ENABLE` reader - Manual mode enable bit: 1=Manual mode is enabled and this register interfaces to the eFUSE;\n 0=Normal mode, internal controller interfaces to eFUSE IP. This bit only takes affect when REG_CTRL.EXT_PRGM bit is 0."]
pub type MAN_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `MAN_ENABLE` writer - Manual mode enable bit: 1=Manual mode is enabled and this register interfaces to the eFUSE;\n 0=Normal mode, internal controller interfaces to eFUSE IP. This bit only takes affect when REG_CTRL.EXT_PRGM bit is 0."]
pub type MAN_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, MAN_CTRL_SPEC, bool, O>;
#[doc = "Field `IP_CS` reader - eFUSE chip select (CS) pin: 1=eFUSE is enabled for PROGRAM/READ modes; 0=eFUSE is disabled and in low power state."]
pub type IP_CS_R = crate::BitReader<bool>;
#[doc = "Field `IP_CS` writer - eFUSE chip select (CS) pin: 1=eFUSE is enabled for PROGRAM/READ modes; 0=eFUSE is disabled and in low power state."]
pub type IP_CS_W<'a, const O: u8> = crate::BitWriter<'a, u16, MAN_CTRL_SPEC, bool, O>;
#[doc = "Field `IP_PRGM_EN` reader - eFUSE program enable. Can also be considered the write signal: 1=eFUSE is programming; 0=eFUSE is in read mode."]
pub type IP_PRGM_EN_R = crate::BitReader<bool>;
#[doc = "Field `IP_PRGM_EN` writer - eFUSE program enable. Can also be considered the write signal: 1=eFUSE is programming; 0=eFUSE is in read mode."]
pub type IP_PRGM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, MAN_CTRL_SPEC, bool, O>;
#[doc = "Field `IP_PRCHG` reader - eFUSE precharge: 1=outputs are being precharged; 0=outputs are not precharged."]
pub type IP_PRCHG_R = crate::BitReader<bool>;
#[doc = "Field `IP_PRCHG` writer - eFUSE precharge: 1=outputs are being precharged; 0=outputs are not precharged."]
pub type IP_PRCHG_W<'a, const O: u8> = crate::BitWriter<'a, u16, MAN_CTRL_SPEC, bool, O>;
#[doc = "Field `IP_SENSE_PULSE` reader - eFUSE sense, outputs are valid on falling edge of this bit."]
pub type IP_SENSE_PULSE_R = crate::BitReader<bool>;
#[doc = "Field `IP_SENSE_PULSE` writer - eFUSE sense, outputs are valid on falling edge of this bit."]
pub type IP_SENSE_PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u16, MAN_CTRL_SPEC, bool, O>;
#[doc = "Field `IP_OE` reader - eFUSE output enable. The IP might tri-state at various times, so this bit isolates the outputs to avoid potential crowbar.\n 1=eFUSE outputs enabled for read; 0=eFUSE outputs isolated"]
pub type IP_OE_R = crate::BitReader<bool>;
#[doc = "Field `IP_OE` writer - eFUSE output enable. The IP might tri-state at various times, so this bit isolates the outputs to avoid potential crowbar.\n 1=eFUSE outputs enabled for read; 0=eFUSE outputs isolated"]
pub type IP_OE_W<'a, const O: u8> = crate::BitWriter<'a, u16, MAN_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Manual mode enable bit: 1=Manual mode is enabled and this register interfaces to the eFUSE;\n 0=Normal mode, internal controller interfaces to eFUSE IP. This bit only takes affect when REG_CTRL.EXT_PRGM bit is 0."]
    #[inline(always)]
    pub fn man_enable(&self) -> MAN_ENABLE_R {
        MAN_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - eFUSE chip select (CS) pin: 1=eFUSE is enabled for PROGRAM/READ modes; 0=eFUSE is disabled and in low power state."]
    #[inline(always)]
    pub fn ip_cs(&self) -> IP_CS_R {
        IP_CS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - eFUSE program enable. Can also be considered the write signal: 1=eFUSE is programming; 0=eFUSE is in read mode."]
    #[inline(always)]
    pub fn ip_prgm_en(&self) -> IP_PRGM_EN_R {
        IP_PRGM_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - eFUSE precharge: 1=outputs are being precharged; 0=outputs are not precharged."]
    #[inline(always)]
    pub fn ip_prchg(&self) -> IP_PRCHG_R {
        IP_PRCHG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - eFUSE sense, outputs are valid on falling edge of this bit."]
    #[inline(always)]
    pub fn ip_sense_pulse(&self) -> IP_SENSE_PULSE_R {
        IP_SENSE_PULSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - eFUSE output enable. The IP might tri-state at various times, so this bit isolates the outputs to avoid potential crowbar.\n 1=eFUSE outputs enabled for read; 0=eFUSE outputs isolated"]
    #[inline(always)]
    pub fn ip_oe(&self) -> IP_OE_R {
        IP_OE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Manual mode enable bit: 1=Manual mode is enabled and this register interfaces to the eFUSE;\n 0=Normal mode, internal controller interfaces to eFUSE IP. This bit only takes affect when REG_CTRL.EXT_PRGM bit is 0."]
    #[inline(always)]
    pub fn man_enable(&mut self) -> MAN_ENABLE_W<0> {
        MAN_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - eFUSE chip select (CS) pin: 1=eFUSE is enabled for PROGRAM/READ modes; 0=eFUSE is disabled and in low power state."]
    #[inline(always)]
    pub fn ip_cs(&mut self) -> IP_CS_W<1> {
        IP_CS_W::new(self)
    }
    #[doc = "Bit 2 - eFUSE program enable. Can also be considered the write signal: 1=eFUSE is programming; 0=eFUSE is in read mode."]
    #[inline(always)]
    pub fn ip_prgm_en(&mut self) -> IP_PRGM_EN_W<2> {
        IP_PRGM_EN_W::new(self)
    }
    #[doc = "Bit 3 - eFUSE precharge: 1=outputs are being precharged; 0=outputs are not precharged."]
    #[inline(always)]
    pub fn ip_prchg(&mut self) -> IP_PRCHG_W<3> {
        IP_PRCHG_W::new(self)
    }
    #[doc = "Bit 4 - eFUSE sense, outputs are valid on falling edge of this bit."]
    #[inline(always)]
    pub fn ip_sense_pulse(&mut self) -> IP_SENSE_PULSE_W<4> {
        IP_SENSE_PULSE_W::new(self)
    }
    #[doc = "Bit 5 - eFUSE output enable. The IP might tri-state at various times, so this bit isolates the outputs to avoid potential crowbar.\n 1=eFUSE outputs enabled for read; 0=eFUSE outputs isolated"]
    #[inline(always)]
    pub fn ip_oe(&mut self) -> IP_OE_W<5> {
        IP_OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manual Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man_ctrl](index.html) module"]
pub struct MAN_CTRL_SPEC;
impl crate::RegisterSpec for MAN_CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [man_ctrl::R](R) reader structure"]
impl crate::Readable for MAN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [man_ctrl::W](W) writer structure"]
impl crate::Writable for MAN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAN_CTRL to value 0"]
impl crate::Resettable for MAN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
