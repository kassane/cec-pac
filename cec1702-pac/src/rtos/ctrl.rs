#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLK_EN` reader - 1=RTOS timer counter is enabled\n 0=RTOS timer disabled. All register bits are reset to their default state"]
pub type BLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `BLK_EN` writer - 1=RTOS timer counter is enabled\n 0=RTOS timer disabled. All register bits are reset to their default state"]
pub type BLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `AU_RELOAD` reader - 1=The the RTOS Timer Preload Register is loaded into the timer counter and the counter is restarted when the counter transitions from 1 to 0\n 0=The timer counter halts when it transitions from 1 to 0 and will not restart."]
pub type AU_RELOAD_R = crate::BitReader<bool>;
#[doc = "Field `AU_RELOAD` writer - 1=The the RTOS Timer Preload Register is loaded into the timer counter and the counter is restarted when the counter transitions from 1 to 0\n 0=The timer counter halts when it transitions from 1 to 0 and will not restart."]
pub type AU_RELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TMR_STRT` reader - Writing a 1 to this bit will load the timer counter with the RTOS Timer Preload Register and start counting.\n If the Preload Register is 0, counting will not start and this bit will be cleared to 0.\n Writing a 0 to this bit will halt the counter and clear its contents to 0. The RTOS timer interrupt will not be generated. This bit is automatically cleared if the AUTO_RELOAD bit is 0 and the\n timer counter transitions from 1 to 0."]
pub type TMR_STRT_R = crate::BitReader<bool>;
#[doc = "Field `TMR_STRT` writer - Writing a 1 to this bit will load the timer counter with the RTOS Timer Preload Register and start counting.\n If the Preload Register is 0, counting will not start and this bit will be cleared to 0.\n Writing a 0 to this bit will halt the counter and clear its contents to 0. The RTOS timer interrupt will not be generated. This bit is automatically cleared if the AUTO_RELOAD bit is 0 and the\n timer counter transitions from 1 to 0."]
pub type TMR_STRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EXT_HW_HALT_EN` reader - 1=The timer counter is halted when the external HALT signal is asserted. Counting is always enabled if HALT is de-asserted.\n 0=The HALT signal does not affect the RTOS Timer"]
pub type EXT_HW_HALT_EN_R = crate::BitReader<bool>;
#[doc = "Field `EXT_HW_HALT_EN` writer - 1=The timer counter is halted when the external HALT signal is asserted. Counting is always enabled if HALT is de-asserted.\n 0=The HALT signal does not affect the RTOS Timer"]
pub type EXT_HW_HALT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FW_TMR_HALT` reader - 1=The timer counter is halted. If the counter was running, clearing this bit will restart the counter from the value at which it halted\n 0=The timer counter, if enabled, will continue to run"]
pub type FW_TMR_HALT_R = crate::BitReader<bool>;
#[doc = "Field `FW_TMR_HALT` writer - 1=The timer counter is halted. If the counter was running, clearing this bit will restart the counter from the value at which it halted\n 0=The timer counter, if enabled, will continue to run"]
pub type FW_TMR_HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 1=RTOS timer counter is enabled\n 0=RTOS timer disabled. All register bits are reset to their default state"]
    #[inline(always)]
    pub fn blk_en(&self) -> BLK_EN_R {
        BLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1=The the RTOS Timer Preload Register is loaded into the timer counter and the counter is restarted when the counter transitions from 1 to 0\n 0=The timer counter halts when it transitions from 1 to 0 and will not restart."]
    #[inline(always)]
    pub fn au_reload(&self) -> AU_RELOAD_R {
        AU_RELOAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Writing a 1 to this bit will load the timer counter with the RTOS Timer Preload Register and start counting.\n If the Preload Register is 0, counting will not start and this bit will be cleared to 0.\n Writing a 0 to this bit will halt the counter and clear its contents to 0. The RTOS timer interrupt will not be generated. This bit is automatically cleared if the AUTO_RELOAD bit is 0 and the\n timer counter transitions from 1 to 0."]
    #[inline(always)]
    pub fn tmr_strt(&self) -> TMR_STRT_R {
        TMR_STRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1=The timer counter is halted when the external HALT signal is asserted. Counting is always enabled if HALT is de-asserted.\n 0=The HALT signal does not affect the RTOS Timer"]
    #[inline(always)]
    pub fn ext_hw_halt_en(&self) -> EXT_HW_HALT_EN_R {
        EXT_HW_HALT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1=The timer counter is halted. If the counter was running, clearing this bit will restart the counter from the value at which it halted\n 0=The timer counter, if enabled, will continue to run"]
    #[inline(always)]
    pub fn fw_tmr_halt(&self) -> FW_TMR_HALT_R {
        FW_TMR_HALT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1=RTOS timer counter is enabled\n 0=RTOS timer disabled. All register bits are reset to their default state"]
    #[inline(always)]
    pub fn blk_en(&mut self) -> BLK_EN_W<0> {
        BLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - 1=The the RTOS Timer Preload Register is loaded into the timer counter and the counter is restarted when the counter transitions from 1 to 0\n 0=The timer counter halts when it transitions from 1 to 0 and will not restart."]
    #[inline(always)]
    pub fn au_reload(&mut self) -> AU_RELOAD_W<1> {
        AU_RELOAD_W::new(self)
    }
    #[doc = "Bit 2 - Writing a 1 to this bit will load the timer counter with the RTOS Timer Preload Register and start counting.\n If the Preload Register is 0, counting will not start and this bit will be cleared to 0.\n Writing a 0 to this bit will halt the counter and clear its contents to 0. The RTOS timer interrupt will not be generated. This bit is automatically cleared if the AUTO_RELOAD bit is 0 and the\n timer counter transitions from 1 to 0."]
    #[inline(always)]
    pub fn tmr_strt(&mut self) -> TMR_STRT_W<2> {
        TMR_STRT_W::new(self)
    }
    #[doc = "Bit 3 - 1=The timer counter is halted when the external HALT signal is asserted. Counting is always enabled if HALT is de-asserted.\n 0=The HALT signal does not affect the RTOS Timer"]
    #[inline(always)]
    pub fn ext_hw_halt_en(&mut self) -> EXT_HW_HALT_EN_W<3> {
        EXT_HW_HALT_EN_W::new(self)
    }
    #[doc = "Bit 4 - 1=The timer counter is halted. If the counter was running, clearing this bit will restart the counter from the value at which it halted\n 0=The timer counter, if enabled, will continue to run"]
    #[inline(always)]
    pub fn fw_tmr_halt(&mut self) -> FW_TMR_HALT_W<4> {
        FW_TMR_HALT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTOS Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
