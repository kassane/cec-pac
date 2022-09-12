#[doc = "Register `DEBUG_CTRL` reader"]
pub struct R(crate::R<DEBUG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_CTRL` writer"]
pub struct W(crate::W<DEBUG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_CTRL_SPEC>;
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
impl From<crate::W<DEBUG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - DEBUG_EN (JTAG_EN) This bit enables the JTAG/SWD debug port.\n 0= JTAG/SWD port disabled. JTAG/SWD cannot be enabled (i.e., the TRST# pin is ignored and the JTAG signals remain in their non-JTAG state)\n 1= JTAG/SWD port enabled. A high on TRST# enables JTAG or SWD, as determined by SWD_EN."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - DEBUG_EN (JTAG_EN) This bit enables the JTAG/SWD debug port.\n 0= JTAG/SWD port disabled. JTAG/SWD cannot be enabled (i.e., the TRST# pin is ignored and the JTAG signals remain in their non-JTAG state)\n 1= JTAG/SWD port enabled. A high on TRST# enables JTAG or SWD, as determined by SWD_EN."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_CTRL_SPEC, bool, O>;
#[doc = "Field `PIN_CFG` reader - This field determines which pins are affected by the TRST# debug enable pin. 3=Reserved 2=The pins associated with the JTAG TCK and TMS switch to the debug interface when TRST# is de-asserted high. The pins associated with TDI and TDO remain controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) is required for debugging and the Serial Wire Viewer is not required 1=The pins associated with the JTAG TCK, TMS and TDO switch to the debug interface when TRST# is de-asserted high. The pin associated with TDI remains controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) and Serial Wire Viewer (SWV) are both required for debugging 0=All four pins associated with JTAG (TCK, TMS, TDI and TDO) switch to the debug interface when TRST# is de-asserted high. This setting should be used when the JTAG TAP controller is required for debugging."]
pub type PIN_CFG_R = crate::FieldReader<u8, PIN_CFGSELECT_A>;
#[doc = "This field determines which pins are affected by the TRST# debug enable pin. 3=Reserved 2=The pins associated with the JTAG TCK and TMS switch to the debug interface when TRST# is de-asserted high. The pins associated with TDI and TDO remain controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) is required for debugging and the Serial Wire Viewer is not required 1=The pins associated with the JTAG TCK, TMS and TDO switch to the debug interface when TRST# is de-asserted high. The pin associated with TDI remains controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) and Serial Wire Viewer (SWV) are both required for debugging 0=All four pins associated with JTAG (TCK, TMS, TDI and TDO) switch to the debug interface when TRST# is de-asserted high. This setting should be used when the JTAG TAP controller is required for debugging.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PIN_CFGSELECT_A {
    #[doc = "2: 2=The pins associated with the JTAG TCK and TMS switch to the debug interface when TRST# is de-asserted high. The pins\n associated with TDI and TDO remain controlled by the associated GPIO. This setting should be used when the ARM Serial\n Wire Debug (SWD) is required for debugging and the Serial Wire Viewer is not required"]
    JTAG_TCK_TMS = 2,
    #[doc = "1: 1=The pins associated with the JTAG TCK, TMS and TDO switch to the debug interface when TRST# is de-asserted high. The pin\n associated with TDI remains controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug\n (SWD) and Serial Wire Viewer (SWV) are both required for debugging"]
    JTAG_TCK_TMS_TDO = 1,
    #[doc = "0: 0=All four pins associated with JTAG (TCK, TMS, TDI and TDO) switch to the debug interface when TRST# is de-asserted high.\n This setting should be used when the JTAG TAP controller is required for debugging."]
    JTAG_TCK_TMS_TDO_TDI = 0,
}
impl From<PIN_CFGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PIN_CFGSELECT_A) -> Self {
        variant as _
    }
}
impl PIN_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PIN_CFGSELECT_A> {
        match self.bits {
            2 => Some(PIN_CFGSELECT_A::JTAG_TCK_TMS),
            1 => Some(PIN_CFGSELECT_A::JTAG_TCK_TMS_TDO),
            0 => Some(PIN_CFGSELECT_A::JTAG_TCK_TMS_TDO_TDI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `JTAG_TCK_TMS`"]
    #[inline(always)]
    pub fn is_jtag_tck_tms(&self) -> bool {
        *self == PIN_CFGSELECT_A::JTAG_TCK_TMS
    }
    #[doc = "Checks if the value of the field is `JTAG_TCK_TMS_TDO`"]
    #[inline(always)]
    pub fn is_jtag_tck_tms_tdo(&self) -> bool {
        *self == PIN_CFGSELECT_A::JTAG_TCK_TMS_TDO
    }
    #[doc = "Checks if the value of the field is `JTAG_TCK_TMS_TDO_TDI`"]
    #[inline(always)]
    pub fn is_jtag_tck_tms_tdo_tdi(&self) -> bool {
        *self == PIN_CFGSELECT_A::JTAG_TCK_TMS_TDO_TDI
    }
}
#[doc = "Field `PIN_CFG` writer - This field determines which pins are affected by the TRST# debug enable pin. 3=Reserved 2=The pins associated with the JTAG TCK and TMS switch to the debug interface when TRST# is de-asserted high. The pins associated with TDI and TDO remain controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) is required for debugging and the Serial Wire Viewer is not required 1=The pins associated with the JTAG TCK, TMS and TDO switch to the debug interface when TRST# is de-asserted high. The pin associated with TDI remains controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) and Serial Wire Viewer (SWV) are both required for debugging 0=All four pins associated with JTAG (TCK, TMS, TDI and TDO) switch to the debug interface when TRST# is de-asserted high. This setting should be used when the JTAG TAP controller is required for debugging."]
pub type PIN_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG_CTRL_SPEC, u8, PIN_CFGSELECT_A, 2, O>;
impl<'a, const O: u8> PIN_CFG_W<'a, O> {
    #[doc = "2=The pins associated with the JTAG TCK and TMS switch to the debug interface when TRST# is de-asserted high. The pins\n associated with TDI and TDO remain controlled by the associated GPIO. This setting should be used when the ARM Serial\n Wire Debug (SWD) is required for debugging and the Serial Wire Viewer is not required"]
    #[inline(always)]
    pub fn jtag_tck_tms(self) -> &'a mut W {
        self.variant(PIN_CFGSELECT_A::JTAG_TCK_TMS)
    }
    #[doc = "1=The pins associated with the JTAG TCK, TMS and TDO switch to the debug interface when TRST# is de-asserted high. The pin\n associated with TDI remains controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug\n (SWD) and Serial Wire Viewer (SWV) are both required for debugging"]
    #[inline(always)]
    pub fn jtag_tck_tms_tdo(self) -> &'a mut W {
        self.variant(PIN_CFGSELECT_A::JTAG_TCK_TMS_TDO)
    }
    #[doc = "0=All four pins associated with JTAG (TCK, TMS, TDI and TDO) switch to the debug interface when TRST# is de-asserted high.\n This setting should be used when the JTAG TAP controller is required for debugging."]
    #[inline(always)]
    pub fn jtag_tck_tms_tdo_tdi(self) -> &'a mut W {
        self.variant(PIN_CFGSELECT_A::JTAG_TCK_TMS_TDO_TDI)
    }
}
#[doc = "Field `PU_EN` reader - If this bit is set to '1b' internal pull-up resistors are automatically enabled on the appropriate debugging port\n wires whenever the debug port is enabled (the DEBUG_EN bit in this register is '1b' and the JTAG_RST# pin is high). The setting\n of DEBUG_PIN_CFG determines which pins have pull-ups enabled when the debug port is enabled."]
pub type PU_EN_R = crate::BitReader<bool>;
#[doc = "Field `PU_EN` writer - If this bit is set to '1b' internal pull-up resistors are automatically enabled on the appropriate debugging port\n wires whenever the debug port is enabled (the DEBUG_EN bit in this register is '1b' and the JTAG_RST# pin is high). The setting\n of DEBUG_PIN_CFG determines which pins have pull-ups enabled when the debug port is enabled."]
pub type PU_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_CTRL_SPEC, bool, O>;
#[doc = "Field `BSP_EN` reader - This bit sets the boundary scan tap controller accessibility from JTAG port.\n 1= Boundary scan tap controller accessibile through JTAG Port.\n 0= Boundary scan tap controller not accessibile through JTAG Port.\n"]
pub type BSP_EN_R = crate::BitReader<bool>;
#[doc = "Field `BSP_EN` writer - This bit sets the boundary scan tap controller accessibility from JTAG port.\n 1= Boundary scan tap controller accessibile through JTAG Port.\n 0= Boundary scan tap controller not accessibile through JTAG Port.\n"]
pub type BSP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DEBUG_EN (JTAG_EN) This bit enables the JTAG/SWD debug port.\n 0= JTAG/SWD port disabled. JTAG/SWD cannot be enabled (i.e., the TRST# pin is ignored and the JTAG signals remain in their non-JTAG state)\n 1= JTAG/SWD port enabled. A high on TRST# enables JTAG or SWD, as determined by SWD_EN."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - This field determines which pins are affected by the TRST# debug enable pin. 3=Reserved 2=The pins associated with the JTAG TCK and TMS switch to the debug interface when TRST# is de-asserted high. The pins associated with TDI and TDO remain controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) is required for debugging and the Serial Wire Viewer is not required 1=The pins associated with the JTAG TCK, TMS and TDO switch to the debug interface when TRST# is de-asserted high. The pin associated with TDI remains controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) and Serial Wire Viewer (SWV) are both required for debugging 0=All four pins associated with JTAG (TCK, TMS, TDI and TDO) switch to the debug interface when TRST# is de-asserted high. This setting should be used when the JTAG TAP controller is required for debugging."]
    #[inline(always)]
    pub fn pin_cfg(&self) -> PIN_CFG_R {
        PIN_CFG_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - If this bit is set to '1b' internal pull-up resistors are automatically enabled on the appropriate debugging port\n wires whenever the debug port is enabled (the DEBUG_EN bit in this register is '1b' and the JTAG_RST# pin is high). The setting\n of DEBUG_PIN_CFG determines which pins have pull-ups enabled when the debug port is enabled."]
    #[inline(always)]
    pub fn pu_en(&self) -> PU_EN_R {
        PU_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit sets the boundary scan tap controller accessibility from JTAG port.\n 1= Boundary scan tap controller accessibile through JTAG Port.\n 0= Boundary scan tap controller not accessibile through JTAG Port.\n"]
    #[inline(always)]
    pub fn bsp_en(&self) -> BSP_EN_R {
        BSP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DEBUG_EN (JTAG_EN) This bit enables the JTAG/SWD debug port.\n 0= JTAG/SWD port disabled. JTAG/SWD cannot be enabled (i.e., the TRST# pin is ignored and the JTAG signals remain in their non-JTAG state)\n 1= JTAG/SWD port enabled. A high on TRST# enables JTAG or SWD, as determined by SWD_EN."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - This field determines which pins are affected by the TRST# debug enable pin. 3=Reserved 2=The pins associated with the JTAG TCK and TMS switch to the debug interface when TRST# is de-asserted high. The pins associated with TDI and TDO remain controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) is required for debugging and the Serial Wire Viewer is not required 1=The pins associated with the JTAG TCK, TMS and TDO switch to the debug interface when TRST# is de-asserted high. The pin associated with TDI remains controlled by the associated GPIO. This setting should be used when the ARM Serial Wire Debug (SWD) and Serial Wire Viewer (SWV) are both required for debugging 0=All four pins associated with JTAG (TCK, TMS, TDI and TDO) switch to the debug interface when TRST# is de-asserted high. This setting should be used when the JTAG TAP controller is required for debugging."]
    #[inline(always)]
    pub fn pin_cfg(&mut self) -> PIN_CFG_W<1> {
        PIN_CFG_W::new(self)
    }
    #[doc = "Bit 3 - If this bit is set to '1b' internal pull-up resistors are automatically enabled on the appropriate debugging port\n wires whenever the debug port is enabled (the DEBUG_EN bit in this register is '1b' and the JTAG_RST# pin is high). The setting\n of DEBUG_PIN_CFG determines which pins have pull-ups enabled when the debug port is enabled."]
    #[inline(always)]
    pub fn pu_en(&mut self) -> PU_EN_W<3> {
        PU_EN_W::new(self)
    }
    #[doc = "Bit 4 - This bit sets the boundary scan tap controller accessibility from JTAG port.\n 1= Boundary scan tap controller accessibile through JTAG Port.\n 0= Boundary scan tap controller not accessibile through JTAG Port.\n"]
    #[inline(always)]
    pub fn bsp_en(&mut self) -> BSP_EN_W<4> {
        BSP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_ctrl](index.html) module"]
pub struct DEBUG_CTRL_SPEC;
impl crate::RegisterSpec for DEBUG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_ctrl::R](R) reader structure"]
impl crate::Readable for DEBUG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_ctrl::W](W) writer structure"]
impl crate::Writable for DEBUG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG_CTRL to value 0"]
impl crate::Resettable for DEBUG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
