#[doc = "Register `TIMERX_CTRL` reader"]
pub struct R(crate::R<TIMERX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMERX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMERX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMERX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMERX_CTRL` writer"]
pub struct W(crate::W<TIMERX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMERX_CTRL_SPEC>;
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
impl From<crate::W<TIMERX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMERX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - This bit is used to start and stop the timer. This bit does not reset the timer count but does reset the timer\n pulse output. This bit will be cleared when the timer stops counting in One-Shot mode. The ENABLE bit is cleared after a\n RESET cycle has completed. Firmware must poll the RESET bit in order to determine when the timer is active after reset.\n 1=Timer is enabled; 0=Timer is disabled."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - This bit is used to start and stop the timer. This bit does not reset the timer count but does reset the timer\n pulse output. This bit will be cleared when the timer stops counting in One-Shot mode. The ENABLE bit is cleared after a\n RESET cycle has completed. Firmware must poll the RESET bit in order to determine when the timer is active after reset.\n 1=Timer is enabled; 0=Timer is disabled."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERX_CTRL_SPEC, bool, O>;
#[doc = "Field `RST` reader - This bit stops the timer and resets the internal counter to the value in the Timer Reload Register. This bit\n also clears the ENABLE bit if it is set. This bit is self-clearing after the timer is reset. Firmware must poll the\n RESET bit in order to determine when the timer is active after reset. Interrupts are blocked only when RESET takes\n effect and the ENABLE bit is cleared. If interrupts are not desired, firmware must mask the interrupt in the interrupt\n block. 1=Timer reset; 0=Normal timer operation."]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - This bit stops the timer and resets the internal counter to the value in the Timer Reload Register. This bit\n also clears the ENABLE bit if it is set. This bit is self-clearing after the timer is reset. Firmware must poll the\n RESET bit in order to determine when the timer is active after reset. Interrupts are blocked only when RESET takes\n effect and the ENABLE bit is cleared. If interrupts are not desired, firmware must mask the interrupt in the interrupt\n block. 1=Timer reset; 0=Normal timer operation."]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERX_CTRL_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Timer Mode. 3=Measurement Mode; 2=One Shot Mode; 1=Event Mode; 0=Timer Mode."]
pub type MODE_R = crate::FieldReader<u8, MODESELECT_A>;
#[doc = "Timer Mode. 3=Measurement Mode; 2=One Shot Mode; 1=Event Mode; 0=Timer Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODESELECT_A {
    #[doc = "0: 0=Timer Mode"]
    TIMER_MODE = 0,
    #[doc = "1: 1=Event Mode"]
    EVENT_MODE = 1,
    #[doc = "2: 2=One Shot Mode"]
    ONE_SHOT_MODE = 2,
    #[doc = "3: 3=Measurement Mode"]
    MEASUREMENT_MODE = 3,
}
impl From<MODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODESELECT_A {
        match self.bits {
            0 => MODESELECT_A::TIMER_MODE,
            1 => MODESELECT_A::EVENT_MODE,
            2 => MODESELECT_A::ONE_SHOT_MODE,
            3 => MODESELECT_A::MEASUREMENT_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE`"]
    #[inline(always)]
    pub fn is_timer_mode(&self) -> bool {
        *self == MODESELECT_A::TIMER_MODE
    }
    #[doc = "Checks if the value of the field is `EVENT_MODE`"]
    #[inline(always)]
    pub fn is_event_mode(&self) -> bool {
        *self == MODESELECT_A::EVENT_MODE
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT_MODE`"]
    #[inline(always)]
    pub fn is_one_shot_mode(&self) -> bool {
        *self == MODESELECT_A::ONE_SHOT_MODE
    }
    #[doc = "Checks if the value of the field is `MEASUREMENT_MODE`"]
    #[inline(always)]
    pub fn is_measurement_mode(&self) -> bool {
        *self == MODESELECT_A::MEASUREMENT_MODE
    }
}
#[doc = "Field `MODE` writer - Timer Mode. 3=Measurement Mode; 2=One Shot Mode; 1=Event Mode; 0=Timer Mode."]
pub type MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMERX_CTRL_SPEC, u8, MODESELECT_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "0=Timer Mode"]
    #[inline(always)]
    pub fn timer_mode(self) -> &'a mut W {
        self.variant(MODESELECT_A::TIMER_MODE)
    }
    #[doc = "1=Event Mode"]
    #[inline(always)]
    pub fn event_mode(self) -> &'a mut W {
        self.variant(MODESELECT_A::EVENT_MODE)
    }
    #[doc = "2=One Shot Mode"]
    #[inline(always)]
    pub fn one_shot_mode(self) -> &'a mut W {
        self.variant(MODESELECT_A::ONE_SHOT_MODE)
    }
    #[doc = "3=Measurement Mode"]
    #[inline(always)]
    pub fn measurement_mode(self) -> &'a mut W {
        self.variant(MODESELECT_A::MEASUREMENT_MODE)
    }
}
#[doc = "Field `INPOL` reader - This bit selects the polarity of the TINx input. 1=TINx is active low; 0=TINx is active high."]
pub type INPOL_R = crate::BitReader<bool>;
#[doc = "Field `INPOL` writer - This bit selects the polarity of the TINx input. 1=TINx is active low; 0=TINx is active high."]
pub type INPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERX_CTRL_SPEC, bool, O>;
#[doc = "Field `UPDN` reader - In Event Mode, this bit selects the timer count direction. In Timer Mode enables timer control by the TINx input pin.\n Event Mode: 1=The timer counts up; 0=The timer counts down.\n Timer Mode:; 1=TINx pin pauses the timer when de-asserted; 0=TINx pin has no effect on the timer."]
pub type UPDN_R = crate::BitReader<bool>;
#[doc = "Field `UPDN` writer - In Event Mode, this bit selects the timer count direction. In Timer Mode enables timer control by the TINx input pin.\n Event Mode: 1=The timer counts up; 0=The timer counts down.\n Timer Mode:; 1=TINx pin pauses the timer when de-asserted; 0=TINx pin has no effect on the timer."]
pub type UPDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERX_CTRL_SPEC, bool, O>;
#[doc = "Field `TOUT_EN` reader - This bit enables the TOUTx pin. 1=TOUTx pin function is enabled; 0=TOUTx pin is inactive."]
pub type TOUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `TOUT_EN` writer - This bit enables the TOUTx pin. 1=TOUTx pin function is enabled; 0=TOUTx pin is inactive."]
pub type TOUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERX_CTRL_SPEC, bool, O>;
#[doc = "Field `RLOAD` reader - Reload Control. This bit controls how the timer is reloaded on overflow or underflow in Event and Timer modes.\n It has no effect in One shot mode. 1=Reload timer from Timer Reload Register and continue counting;\n 0=Roll timer over to FFFFh and continue counting when counting down and rolls over to 0000h and continues counting when counting up."]
pub type RLOAD_R = crate::BitReader<bool>;
#[doc = "Field `RLOAD` writer - Reload Control. This bit controls how the timer is reloaded on overflow or underflow in Event and Timer modes.\n It has no effect in One shot mode. 1=Reload timer from Timer Reload Register and continue counting;\n 0=Roll timer over to FFFFh and continue counting when counting down and rolls over to 0000h and continues counting when counting up."]
pub type RLOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERX_CTRL_SPEC, bool, O>;
#[doc = "Field `FIL_BYPASS` reader - This bit is used to enable or disable the noise filter on the TINx input signal. 1=Bypass Mode: input filter disabled.\n The TINx input directly affects the timer; 0=Filter Mode: input filter enabled. The TINx input is filtered by the input filter."]
pub type FIL_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `FIL_BYPASS` writer - This bit is used to enable or disable the noise filter on the TINx input signal. 1=Bypass Mode: input filter disabled.\n The TINx input directly affects the timer; 0=Filter Mode: input filter enabled. The TINx input is filtered by the input filter."]
pub type FIL_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERX_CTRL_SPEC, bool, O>;
#[doc = "Field `PD` reader - Power Down. 1=The timer is powered down and all clocks are gated; 0=The timer is in a running state."]
pub type PD_R = crate::BitReader<bool>;
#[doc = "Field `PD` writer - Power Down. 1=The timer is powered down and all clocks are gated; 0=The timer is in a running state."]
pub type PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERX_CTRL_SPEC, bool, O>;
#[doc = "Field `TOUT_POL` reader - This bit determines the polarity of the TOUTx output signal. In timer modes that toggle the TOUTx signal,\n this polarity bit will not have a perceivable difference, except to determine the inactive state. In One-Shot mode\n this determines if the pulsed output is active high or active low. 1=Active low; 0=Active high."]
pub type TOUT_POL_R = crate::BitReader<bool>;
#[doc = "Field `TOUT_POL` writer - This bit determines the polarity of the TOUTx output signal. In timer modes that toggle the TOUTx signal,\n this polarity bit will not have a perceivable difference, except to determine the inactive state. In One-Shot mode\n this determines if the pulsed output is active high or active low. 1=Active low; 0=Active high."]
pub type TOUT_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERX_CTRL_SPEC, bool, O>;
#[doc = "Field `SLP_EN` reader - This bit reflects the current state of the timer's Sleep_Enable input signal. 1=Normal operation; 0=Sleep Mode is requested."]
pub type SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SLP_EN` writer - This bit reflects the current state of the timer's Sleep_Enable input signal. 1=Normal operation; 0=Sleep Mode is requested."]
pub type SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERX_CTRL_SPEC, bool, O>;
#[doc = "Field `TMRX_CLK_REQ` reader - This bit reflects the current state of the timer's Clock_Required output signal. 1=The main clock is required by this block;\n 0=The main clock is not required by this block."]
pub type TMRX_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TMRX_CLK_REQ` writer - This bit reflects the current state of the timer's Clock_Required output signal. 1=The main clock is required by this block;\n 0=The main clock is not required by this block."]
pub type TMRX_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERX_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to start and stop the timer. This bit does not reset the timer count but does reset the timer\n pulse output. This bit will be cleared when the timer stops counting in One-Shot mode. The ENABLE bit is cleared after a\n RESET cycle has completed. Firmware must poll the RESET bit in order to determine when the timer is active after reset.\n 1=Timer is enabled; 0=Timer is disabled."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit stops the timer and resets the internal counter to the value in the Timer Reload Register. This bit\n also clears the ENABLE bit if it is set. This bit is self-clearing after the timer is reset. Firmware must poll the\n RESET bit in order to determine when the timer is active after reset. Interrupts are blocked only when RESET takes\n effect and the ENABLE bit is cleared. If interrupts are not desired, firmware must mask the interrupt in the interrupt\n block. 1=Timer reset; 0=Normal timer operation."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Timer Mode. 3=Measurement Mode; 2=One Shot Mode; 1=Event Mode; 0=Timer Mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - This bit selects the polarity of the TINx input. 1=TINx is active low; 0=TINx is active high."]
    #[inline(always)]
    pub fn inpol(&self) -> INPOL_R {
        INPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - In Event Mode, this bit selects the timer count direction. In Timer Mode enables timer control by the TINx input pin.\n Event Mode: 1=The timer counts up; 0=The timer counts down.\n Timer Mode:; 1=TINx pin pauses the timer when de-asserted; 0=TINx pin has no effect on the timer."]
    #[inline(always)]
    pub fn updn(&self) -> UPDN_R {
        UPDN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit enables the TOUTx pin. 1=TOUTx pin function is enabled; 0=TOUTx pin is inactive."]
    #[inline(always)]
    pub fn tout_en(&self) -> TOUT_EN_R {
        TOUT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reload Control. This bit controls how the timer is reloaded on overflow or underflow in Event and Timer modes.\n It has no effect in One shot mode. 1=Reload timer from Timer Reload Register and continue counting;\n 0=Roll timer over to FFFFh and continue counting when counting down and rolls over to 0000h and continues counting when counting up."]
    #[inline(always)]
    pub fn rload(&self) -> RLOAD_R {
        RLOAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit is used to enable or disable the noise filter on the TINx input signal. 1=Bypass Mode: input filter disabled.\n The TINx input directly affects the timer; 0=Filter Mode: input filter enabled. The TINx input is filtered by the input filter."]
    #[inline(always)]
    pub fn fil_bypass(&self) -> FIL_BYPASS_R {
        FIL_BYPASS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Power Down. 1=The timer is powered down and all clocks are gated; 0=The timer is in a running state."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit determines the polarity of the TOUTx output signal. In timer modes that toggle the TOUTx signal,\n this polarity bit will not have a perceivable difference, except to determine the inactive state. In One-Shot mode\n this determines if the pulsed output is active high or active low. 1=Active low; 0=Active high."]
    #[inline(always)]
    pub fn tout_pol(&self) -> TOUT_POL_R {
        TOUT_POL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit reflects the current state of the timer's Sleep_Enable input signal. 1=Normal operation; 0=Sleep Mode is requested."]
    #[inline(always)]
    pub fn slp_en(&self) -> SLP_EN_R {
        SLP_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit reflects the current state of the timer's Clock_Required output signal. 1=The main clock is required by this block;\n 0=The main clock is not required by this block."]
    #[inline(always)]
    pub fn tmrx_clk_req(&self) -> TMRX_CLK_REQ_R {
        TMRX_CLK_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to start and stop the timer. This bit does not reset the timer count but does reset the timer\n pulse output. This bit will be cleared when the timer stops counting in One-Shot mode. The ENABLE bit is cleared after a\n RESET cycle has completed. Firmware must poll the RESET bit in order to determine when the timer is active after reset.\n 1=Timer is enabled; 0=Timer is disabled."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - This bit stops the timer and resets the internal counter to the value in the Timer Reload Register. This bit\n also clears the ENABLE bit if it is set. This bit is self-clearing after the timer is reset. Firmware must poll the\n RESET bit in order to determine when the timer is active after reset. Interrupts are blocked only when RESET takes\n effect and the ENABLE bit is cleared. If interrupts are not desired, firmware must mask the interrupt in the interrupt\n block. 1=Timer reset; 0=Normal timer operation."]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<1> {
        RST_W::new(self)
    }
    #[doc = "Bits 2:3 - Timer Mode. 3=Measurement Mode; 2=One Shot Mode; 1=Event Mode; 0=Timer Mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - This bit selects the polarity of the TINx input. 1=TINx is active low; 0=TINx is active high."]
    #[inline(always)]
    pub fn inpol(&mut self) -> INPOL_W<4> {
        INPOL_W::new(self)
    }
    #[doc = "Bit 5 - In Event Mode, this bit selects the timer count direction. In Timer Mode enables timer control by the TINx input pin.\n Event Mode: 1=The timer counts up; 0=The timer counts down.\n Timer Mode:; 1=TINx pin pauses the timer when de-asserted; 0=TINx pin has no effect on the timer."]
    #[inline(always)]
    pub fn updn(&mut self) -> UPDN_W<5> {
        UPDN_W::new(self)
    }
    #[doc = "Bit 6 - This bit enables the TOUTx pin. 1=TOUTx pin function is enabled; 0=TOUTx pin is inactive."]
    #[inline(always)]
    pub fn tout_en(&mut self) -> TOUT_EN_W<6> {
        TOUT_EN_W::new(self)
    }
    #[doc = "Bit 7 - Reload Control. This bit controls how the timer is reloaded on overflow or underflow in Event and Timer modes.\n It has no effect in One shot mode. 1=Reload timer from Timer Reload Register and continue counting;\n 0=Roll timer over to FFFFh and continue counting when counting down and rolls over to 0000h and continues counting when counting up."]
    #[inline(always)]
    pub fn rload(&mut self) -> RLOAD_W<7> {
        RLOAD_W::new(self)
    }
    #[doc = "Bit 8 - This bit is used to enable or disable the noise filter on the TINx input signal. 1=Bypass Mode: input filter disabled.\n The TINx input directly affects the timer; 0=Filter Mode: input filter enabled. The TINx input is filtered by the input filter."]
    #[inline(always)]
    pub fn fil_bypass(&mut self) -> FIL_BYPASS_W<8> {
        FIL_BYPASS_W::new(self)
    }
    #[doc = "Bit 9 - Power Down. 1=The timer is powered down and all clocks are gated; 0=The timer is in a running state."]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W<9> {
        PD_W::new(self)
    }
    #[doc = "Bit 10 - This bit determines the polarity of the TOUTx output signal. In timer modes that toggle the TOUTx signal,\n this polarity bit will not have a perceivable difference, except to determine the inactive state. In One-Shot mode\n this determines if the pulsed output is active high or active low. 1=Active low; 0=Active high."]
    #[inline(always)]
    pub fn tout_pol(&mut self) -> TOUT_POL_W<10> {
        TOUT_POL_W::new(self)
    }
    #[doc = "Bit 11 - This bit reflects the current state of the timer's Sleep_Enable input signal. 1=Normal operation; 0=Sleep Mode is requested."]
    #[inline(always)]
    pub fn slp_en(&mut self) -> SLP_EN_W<11> {
        SLP_EN_W::new(self)
    }
    #[doc = "Bit 12 - This bit reflects the current state of the timer's Clock_Required output signal. 1=The main clock is required by this block;\n 0=The main clock is not required by this block."]
    #[inline(always)]
    pub fn tmrx_clk_req(&mut self) -> TMRX_CLK_REQ_W<12> {
        TMRX_CLK_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This bit reflects the current state of the timer's Clock_Required output signal.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timerx_ctrl](index.html) module"]
pub struct TIMERX_CTRL_SPEC;
impl crate::RegisterSpec for TIMERX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timerx_ctrl::R](R) reader structure"]
impl crate::Readable for TIMERX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timerx_ctrl::W](W) writer structure"]
impl crate::Writable for TIMERX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMERX_CTRL to value 0"]
impl crate::Resettable for TIMERX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
