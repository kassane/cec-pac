#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRL` reader - CONTROL 3=PWM is always on 2=LED blinking (standard PWM) 1=LED breathing configuration 0=PWM is always off. All internal registers and counters are reset to 0. Clocks are gated"]
pub type CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTRL` writer - CONTROL 3=PWM is always on 2=LED blinking (standard PWM) 1=LED breathing configuration 0=PWM is always off. All internal registers and counters are reset to 0. Clocks are gated"]
pub type CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLK_SRC` reader - 1=Clock source is the 48 MHz clock, 0=Clock source is the 32.768 KHz clock"]
pub type CLK_SRC_R = crate::BitReader<bool>;
#[doc = "Field `CLK_SRC` writer - 1=Clock source is the 48 MHz clock, 0=Clock source is the 32.768 KHz clock"]
pub type CLK_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `SYNCH` reader - SYNCHRONIZE When this bit is '1', all counters for all LEDs are reset to their initial values. When this bit is '0' in the LED Configuration Register for all LEDs, then all counters for LEDs that are configured to blink or breathe will increment or decrement, as required."]
pub type SYNCH_R = crate::BitReader<bool>;
#[doc = "Field `SYNCH` writer - SYNCHRONIZE When this bit is '1', all counters for all LEDs are reset to their initial values. When this bit is '0' in the LED Configuration Register for all LEDs, then all counters for LEDs that are configured to blink or breathe will increment or decrement, as required."]
pub type SYNCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `PWM_SIZE` reader - PWM_SIZE This bit controls the behavior of PWM: 3=Reserved 2=PWM is configured as a 6-bit PWM 1=PWM is configured as a 7-bit PWM 0=PWM is configured as an 8-bit PWM"]
pub type PWM_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWM_SIZE` writer - PWM_SIZE This bit controls the behavior of PWM: 3=Reserved 2=PWM is configured as a 6-bit PWM 1=PWM is configured as a 7-bit PWM 0=PWM is configured as an 8-bit PWM"]
pub type PWM_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `EN_UPDATE` reader - ENABLE_UPDATE This bit is set to 1 when written with a '1'. Writes of '0' have no effect. Hardware clears this bit to 0 when the breathing configuration registers are updated at the end of a PWM period. The current state of the bit is readable any time."]
pub type EN_UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `EN_UPDATE` writer - ENABLE_UPDATE This bit is set to 1 when written with a '1'. Writes of '0' have no effect. Hardware clears this bit to 0 when the breathing configuration registers are updated at the end of a PWM period. The current state of the bit is readable any time."]
pub type EN_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RST` reader - RESET Writes of '1' to this bit resets the PWM registers to their default values. This bit is self clearing. Writes of '0' to this bit have no effect."]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - RESET Writes of '1' to this bit resets the PWM registers to their default values. This bit is self clearing. Writes of '0' to this bit have no effect."]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `WDT_RELOAD` reader - WDT_RELOAD The PWM Watchdog Timer counter reload value. On system reset, it defaults to 14h, which corresponds to a 4 second Watchdog timeout value."]
pub type WDT_RELOAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_RELOAD` writer - WDT_RELOAD The PWM Watchdog Timer counter reload value. On system reset, it defaults to 14h, which corresponds to a 4 second Watchdog timeout value."]
pub type WDT_RELOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `SYMMETRY` reader - SYMMETRY 1=The rising and falling ramp times are in Asymmetric mode. 0=The rising and falling ramp times are in Symmetric mode."]
pub type SYMMETRY_R = crate::BitReader<bool>;
#[doc = "Field `SYMMETRY` writer - SYMMETRY 1=The rising and falling ramp times are in Asymmetric mode. 0=The rising and falling ramp times are in Symmetric mode."]
pub type SYMMETRY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - CONTROL 3=PWM is always on 2=LED blinking (standard PWM) 1=LED breathing configuration 0=PWM is always off. All internal registers and counters are reset to 0. Clocks are gated"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 1=Clock source is the 48 MHz clock, 0=Clock source is the 32.768 KHz clock"]
    #[inline(always)]
    pub fn clk_src(&self) -> CLK_SRC_R {
        CLK_SRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SYNCHRONIZE When this bit is '1', all counters for all LEDs are reset to their initial values. When this bit is '0' in the LED Configuration Register for all LEDs, then all counters for LEDs that are configured to blink or breathe will increment or decrement, as required."]
    #[inline(always)]
    pub fn synch(&self) -> SYNCH_R {
        SYNCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - PWM_SIZE This bit controls the behavior of PWM: 3=Reserved 2=PWM is configured as a 6-bit PWM 1=PWM is configured as a 7-bit PWM 0=PWM is configured as an 8-bit PWM"]
    #[inline(always)]
    pub fn pwm_size(&self) -> PWM_SIZE_R {
        PWM_SIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ENABLE_UPDATE This bit is set to 1 when written with a '1'. Writes of '0' have no effect. Hardware clears this bit to 0 when the breathing configuration registers are updated at the end of a PWM period. The current state of the bit is readable any time."]
    #[inline(always)]
    pub fn en_update(&self) -> EN_UPDATE_R {
        EN_UPDATE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RESET Writes of '1' to this bit resets the PWM registers to their default values. This bit is self clearing. Writes of '0' to this bit have no effect."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - WDT_RELOAD The PWM Watchdog Timer counter reload value. On system reset, it defaults to 14h, which corresponds to a 4 second Watchdog timeout value."]
    #[inline(always)]
    pub fn wdt_reload(&self) -> WDT_RELOAD_R {
        WDT_RELOAD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - SYMMETRY 1=The rising and falling ramp times are in Asymmetric mode. 0=The rising and falling ramp times are in Symmetric mode."]
    #[inline(always)]
    pub fn symmetry(&self) -> SYMMETRY_R {
        SYMMETRY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CONTROL 3=PWM is always on 2=LED blinking (standard PWM) 1=LED breathing configuration 0=PWM is always off. All internal registers and counters are reset to 0. Clocks are gated"]
    #[inline(always)]
    pub fn ctrl(&mut self) -> CTRL_W<0> {
        CTRL_W::new(self)
    }
    #[doc = "Bit 2 - 1=Clock source is the 48 MHz clock, 0=Clock source is the 32.768 KHz clock"]
    #[inline(always)]
    pub fn clk_src(&mut self) -> CLK_SRC_W<2> {
        CLK_SRC_W::new(self)
    }
    #[doc = "Bit 3 - SYNCHRONIZE When this bit is '1', all counters for all LEDs are reset to their initial values. When this bit is '0' in the LED Configuration Register for all LEDs, then all counters for LEDs that are configured to blink or breathe will increment or decrement, as required."]
    #[inline(always)]
    pub fn synch(&mut self) -> SYNCH_W<3> {
        SYNCH_W::new(self)
    }
    #[doc = "Bits 4:5 - PWM_SIZE This bit controls the behavior of PWM: 3=Reserved 2=PWM is configured as a 6-bit PWM 1=PWM is configured as a 7-bit PWM 0=PWM is configured as an 8-bit PWM"]
    #[inline(always)]
    pub fn pwm_size(&mut self) -> PWM_SIZE_W<4> {
        PWM_SIZE_W::new(self)
    }
    #[doc = "Bit 6 - ENABLE_UPDATE This bit is set to 1 when written with a '1'. Writes of '0' have no effect. Hardware clears this bit to 0 when the breathing configuration registers are updated at the end of a PWM period. The current state of the bit is readable any time."]
    #[inline(always)]
    pub fn en_update(&mut self) -> EN_UPDATE_W<6> {
        EN_UPDATE_W::new(self)
    }
    #[doc = "Bit 7 - RESET Writes of '1' to this bit resets the PWM registers to their default values. This bit is self clearing. Writes of '0' to this bit have no effect."]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<7> {
        RST_W::new(self)
    }
    #[doc = "Bits 8:15 - WDT_RELOAD The PWM Watchdog Timer counter reload value. On system reset, it defaults to 14h, which corresponds to a 4 second Watchdog timeout value."]
    #[inline(always)]
    pub fn wdt_reload(&mut self) -> WDT_RELOAD_W<8> {
        WDT_RELOAD_W::new(self)
    }
    #[doc = "Bit 16 - SYMMETRY 1=The rising and falling ramp times are in Asymmetric mode. 0=The rising and falling ramp times are in Symmetric mode."]
    #[inline(always)]
    pub fn symmetry(&mut self) -> SYMMETRY_W<16> {
        SYMMETRY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LED Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0x1400"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1400
    }
}
