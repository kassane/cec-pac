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
#[doc = "Field `EN` reader - This enables the block for operation. 1=This block will function normally; 0=This block will gate its clock and go into its lowest power state"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - This enables the block for operation. 1=This block will function normally; 0=This block will gate its clock and go into its lowest power state"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CNT_UP` reader - This selects the counter direction. When the counter in incrementing the counter will saturate and trigger the event when it reaches all F's. When the counter is decrementing the counter will saturate when it reaches 0h. 1=The counter will increment; 0=The counter will decrement"]
pub type CNT_UP_R = crate::BitReader<bool>;
#[doc = "Field `CNT_UP` writer - This selects the counter direction. When the counter in incrementing the counter will saturate and trigger the event when it reaches all F's. When the counter is decrementing the counter will saturate when it reaches 0h. 1=The counter will increment; 0=The counter will decrement"]
pub type CNT_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `AU_RESTRT` reader - This will select the action taken upon completing a count. 1=The counter will automatically restart the count, using the contents of the Timer Preload Register to load the Timer Count Register. The interrupt will be set in edge mode 0=The counter will simply enter a done state and wait for further control inputs. The interrupt will be set in level mode."]
pub type AU_RESTRT_R = crate::BitReader<bool>;
#[doc = "Field `AU_RESTRT` writer - This will select the action taken upon completing a count. 1=The counter will automatically restart the count, using the contents of the Timer Preload Register to load the Timer Count Register. The interrupt will be set in edge mode 0=The counter will simply enter a done state and wait for further control inputs. The interrupt will be set in level mode."]
pub type AU_RESTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SFT_RST` reader - This is a soft reset. This is self clearing 1 cycle after it is written. Firmware does not need to wait before reconfiguring the Basic Timer following soft reset."]
pub type SFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `SFT_RST` writer - This is a soft reset. This is self clearing 1 cycle after it is written. Firmware does not need to wait before reconfiguring the Basic Timer following soft reset."]
pub type SFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `STRT` reader - This bit triggers the timer counter. The counter will operate until it hits its terminating condition. This will clear this bit. It should be noted that when operating in restart mode, there is no terminating condition for the counter, so this bit will never clear. Clearing this bit will halt the timer counter."]
pub type STRT_R = crate::BitReader<bool>;
#[doc = "Field `STRT` writer - This bit triggers the timer counter. The counter will operate until it hits its terminating condition. This will clear this bit. It should be noted that when operating in restart mode, there is no terminating condition for the counter, so this bit will never clear. Clearing this bit will halt the timer counter."]
pub type STRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RLD` reader - This bit reloads the counter without interrupting it operation. This will not function if the timer has already completed (when the START bit in this register is '0'). This is used to periodically prevent the timer from firing when an event occurs. Usage while the timer is off may result in erroneous behaviour."]
pub type RLD_R = crate::BitReader<bool>;
#[doc = "Field `RLD` writer - This bit reloads the counter without interrupting it operation. This will not function if the timer has already completed (when the START bit in this register is '0'). This is used to periodically prevent the timer from firing when an event occurs. Usage while the timer is off may result in erroneous behaviour."]
pub type RLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `HLT` reader - This is a halt bit. This will halt the timer as long as it is active. Once the halt is inactive, the timer will start from where it left off. 1=Timer is halted. It stops counting. The clock divider will also be reset. 0=Timer runs normally."]
pub type HLT_R = crate::BitReader<bool>;
#[doc = "Field `HLT` writer - This is a halt bit. This will halt the timer as long as it is active. Once the halt is inactive, the timer will start from where it left off. 1=Timer is halted. It stops counting. The clock divider will also be reset. 0=Timer runs normally."]
pub type HLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PRESCALE` reader - This is used to divide down the system clock through clock enables to lower the power consumption of the block and allow slow timers. Updating this value during operation may result in erroneous clock enable pulses until the clock divider restarts. The number of clocks per clock enable pulse is (Value + 1); a setting of 0 runs at the full clock speed, while a setting of 1 runs at half speed."]
pub type PRESCALE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRESCALE` writer - This is used to divide down the system clock through clock enables to lower the power consumption of the block and allow slow timers. Updating this value during operation may result in erroneous clock enable pulses until the clock divider restarts. The number of clocks per clock enable pulse is (Value + 1); a setting of 0 runs at the full clock speed, while a setting of 1 runs at half speed."]
pub type PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - This enables the block for operation. 1=This block will function normally; 0=This block will gate its clock and go into its lowest power state"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - This selects the counter direction. When the counter in incrementing the counter will saturate and trigger the event when it reaches all F's. When the counter is decrementing the counter will saturate when it reaches 0h. 1=The counter will increment; 0=The counter will decrement"]
    #[inline(always)]
    pub fn cnt_up(&self) -> CNT_UP_R {
        CNT_UP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This will select the action taken upon completing a count. 1=The counter will automatically restart the count, using the contents of the Timer Preload Register to load the Timer Count Register. The interrupt will be set in edge mode 0=The counter will simply enter a done state and wait for further control inputs. The interrupt will be set in level mode."]
    #[inline(always)]
    pub fn au_restrt(&self) -> AU_RESTRT_R {
        AU_RESTRT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is a soft reset. This is self clearing 1 cycle after it is written. Firmware does not need to wait before reconfiguring the Basic Timer following soft reset."]
    #[inline(always)]
    pub fn sft_rst(&self) -> SFT_RST_R {
        SFT_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit triggers the timer counter. The counter will operate until it hits its terminating condition. This will clear this bit. It should be noted that when operating in restart mode, there is no terminating condition for the counter, so this bit will never clear. Clearing this bit will halt the timer counter."]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit reloads the counter without interrupting it operation. This will not function if the timer has already completed (when the START bit in this register is '0'). This is used to periodically prevent the timer from firing when an event occurs. Usage while the timer is off may result in erroneous behaviour."]
    #[inline(always)]
    pub fn rld(&self) -> RLD_R {
        RLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is a halt bit. This will halt the timer as long as it is active. Once the halt is inactive, the timer will start from where it left off. 1=Timer is halted. It stops counting. The clock divider will also be reset. 0=Timer runs normally."]
    #[inline(always)]
    pub fn hlt(&self) -> HLT_R {
        HLT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - This is used to divide down the system clock through clock enables to lower the power consumption of the block and allow slow timers. Updating this value during operation may result in erroneous clock enable pulses until the clock divider restarts. The number of clocks per clock enable pulse is (Value + 1); a setting of 0 runs at the full clock speed, while a setting of 1 runs at half speed."]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - This enables the block for operation. 1=This block will function normally; 0=This block will gate its clock and go into its lowest power state"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 2 - This selects the counter direction. When the counter in incrementing the counter will saturate and trigger the event when it reaches all F's. When the counter is decrementing the counter will saturate when it reaches 0h. 1=The counter will increment; 0=The counter will decrement"]
    #[inline(always)]
    pub fn cnt_up(&mut self) -> CNT_UP_W<2> {
        CNT_UP_W::new(self)
    }
    #[doc = "Bit 3 - This will select the action taken upon completing a count. 1=The counter will automatically restart the count, using the contents of the Timer Preload Register to load the Timer Count Register. The interrupt will be set in edge mode 0=The counter will simply enter a done state and wait for further control inputs. The interrupt will be set in level mode."]
    #[inline(always)]
    pub fn au_restrt(&mut self) -> AU_RESTRT_W<3> {
        AU_RESTRT_W::new(self)
    }
    #[doc = "Bit 4 - This is a soft reset. This is self clearing 1 cycle after it is written. Firmware does not need to wait before reconfiguring the Basic Timer following soft reset."]
    #[inline(always)]
    pub fn sft_rst(&mut self) -> SFT_RST_W<4> {
        SFT_RST_W::new(self)
    }
    #[doc = "Bit 5 - This bit triggers the timer counter. The counter will operate until it hits its terminating condition. This will clear this bit. It should be noted that when operating in restart mode, there is no terminating condition for the counter, so this bit will never clear. Clearing this bit will halt the timer counter."]
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<5> {
        STRT_W::new(self)
    }
    #[doc = "Bit 6 - This bit reloads the counter without interrupting it operation. This will not function if the timer has already completed (when the START bit in this register is '0'). This is used to periodically prevent the timer from firing when an event occurs. Usage while the timer is off may result in erroneous behaviour."]
    #[inline(always)]
    pub fn rld(&mut self) -> RLD_W<6> {
        RLD_W::new(self)
    }
    #[doc = "Bit 7 - This is a halt bit. This will halt the timer as long as it is active. Once the halt is inactive, the timer will start from where it left off. 1=Timer is halted. It stops counting. The clock divider will also be reset. 0=Timer runs normally."]
    #[inline(always)]
    pub fn hlt(&mut self) -> HLT_W<7> {
        HLT_W::new(self)
    }
    #[doc = "Bits 16:31 - This is used to divide down the system clock through clock enables to lower the power consumption of the block and allow slow timers. Updating this value during operation may result in erroneous clock enable pulses until the clock divider restarts. The number of clocks per clock enable pulse is (Value + 1); a setting of 0 runs at the full clock speed, while a setting of 1 runs at half speed."]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W<16> {
        PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
