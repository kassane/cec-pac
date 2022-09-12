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
#[doc = "Field `OUTOF_LIM_EN` reader - TACH_OUT_OF_LIMIT_ENABLE This bit is used to enable the TACH_OUT_OF_LIMIT_STATUS bit in the TACHx Status Register to generate an interrupt event.\n 1=Enable interrupt output from Tach block\n 0=Disable interrupt output from Tach block (default)"]
pub type OUTOF_LIM_EN_R = crate::BitReader<bool>;
#[doc = "Field `OUTOF_LIM_EN` writer - TACH_OUT_OF_LIMIT_ENABLE This bit is used to enable the TACH_OUT_OF_LIMIT_STATUS bit in the TACHx Status Register to generate an interrupt event.\n 1=Enable interrupt output from Tach block\n 0=Disable interrupt output from Tach block (default)"]
pub type OUTOF_LIM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EN` reader - TACH_ENABLE 1= TACH Monitoring enabled, clocks enabled. 0= TACH Idle, clocks gated"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - TACH_ENABLE 1= TACH Monitoring enabled, clocks enabled. 0= TACH Idle, clocks gated"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FILT_EN` reader - FILTER_ENABLE This filter is used to remove high frequency glitches from Tach Input. When this filter is enabled, Tach input pulses less than two 100kHz_- Clk periods wide get filtered.\n 1= Filter enabled\n 0= Filter disabled (default)\n It is recommended that the Tach input filter always be enabled."]
pub type FILT_EN_R = crate::BitReader<bool>;
#[doc = "Field `FILT_EN` writer - FILTER_ENABLE This filter is used to remove high frequency glitches from Tach Input. When this filter is enabled, Tach input pulses less than two 100kHz_- Clk periods wide get filtered.\n 1= Filter enabled\n 0= Filter disabled (default)\n It is recommended that the Tach input filter always be enabled."]
pub type FILT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RD_MOD_SEL` reader - TACH_READING_MODE_SELECT\n 1=Counter is incremented on the rising edge of the 100kHz_Clk input. The counter is latched into the TACHX_COUNTER field and reset when the programmed number of edges is detected.\n 0=Counter is incremented when Tach Input transitions from low-tohigh state (default)"]
pub type RD_MOD_SEL_R = crate::BitReader<bool>;
#[doc = "Field `RD_MOD_SEL` writer - TACH_READING_MODE_SELECT\n 1=Counter is incremented on the rising edge of the 100kHz_Clk input. The counter is latched into the TACHX_COUNTER field and reset when the programmed number of edges is detected.\n 0=Counter is incremented when Tach Input transitions from low-tohigh state (default)"]
pub type RD_MOD_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EDGES` reader - TACH_EDGES A Tach signal is a square wave with a 50 percent duty cycle. Typically, two Tach periods represents one revolution of the fan. A Tach period consists of three Tach edges. This programmed value represents the number of Tach edges that will be used to determine the interval for which the number of 100kHz_Clk pulses will be counted\n 11b=9 Tach edges (4 Tach periods)\n 10b=5 Tach edges (2 Tach periods)\n 01b=3 Tach edges (1 Tach period)\n 00b=2 Tach edges (1/2 Tach period)"]
pub type EDGES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDGES` writer - TACH_EDGES A Tach signal is a square wave with a 50 percent duty cycle. Typically, two Tach periods represents one revolution of the fan. A Tach period consists of three Tach edges. This programmed value represents the number of Tach edges that will be used to determine the interval for which the number of 100kHz_Clk pulses will be counted\n 11b=9 Tach edges (4 Tach periods)\n 10b=5 Tach edges (2 Tach periods)\n 01b=3 Tach edges (1 Tach period)\n 00b=2 Tach edges (1/2 Tach period)"]
pub type EDGES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CNT_RDY_INT_EN` reader - COUNT_READY_INT_EN 1=Enable Count Ready interrupt from Tach block, 0=Disable Count Ready interrupt from Tach block"]
pub type CNT_RDY_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CNT_RDY_INT_EN` writer - COUNT_READY_INT_EN 1=Enable Count Ready interrupt from Tach block, 0=Disable Count Ready interrupt from Tach block"]
pub type CNT_RDY_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `IN_INT_EN` reader - TACH_INPUT_INT_EN 1=Enable Tach Input toggle interrupt from Tach block, 0=Disable Tach Input toggle interrupt from Tach block"]
pub type IN_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `IN_INT_EN` writer - TACH_INPUT_INT_EN 1=Enable Tach Input toggle interrupt from Tach block, 0=Disable Tach Input toggle interrupt from Tach block"]
pub type IN_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CNTR` reader - This 16-bit field contains the latched value of the internal Tach pulse counter, which may be configured by the Tach Reading Mode Select field to operate as a free-running counter or to be gated by the Tach input signal."]
pub type CNTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNTR` writer - This 16-bit field contains the latched value of the internal Tach pulse counter, which may be configured by the Tach Reading Mode Select field to operate as a free-running counter or to be gated by the Tach input signal."]
pub type CNTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - TACH_OUT_OF_LIMIT_ENABLE This bit is used to enable the TACH_OUT_OF_LIMIT_STATUS bit in the TACHx Status Register to generate an interrupt event.\n 1=Enable interrupt output from Tach block\n 0=Disable interrupt output from Tach block (default)"]
    #[inline(always)]
    pub fn outof_lim_en(&self) -> OUTOF_LIM_EN_R {
        OUTOF_LIM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TACH_ENABLE 1= TACH Monitoring enabled, clocks enabled. 0= TACH Idle, clocks gated"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - FILTER_ENABLE This filter is used to remove high frequency glitches from Tach Input. When this filter is enabled, Tach input pulses less than two 100kHz_- Clk periods wide get filtered.\n 1= Filter enabled\n 0= Filter disabled (default)\n It is recommended that the Tach input filter always be enabled."]
    #[inline(always)]
    pub fn filt_en(&self) -> FILT_EN_R {
        FILT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - TACH_READING_MODE_SELECT\n 1=Counter is incremented on the rising edge of the 100kHz_Clk input. The counter is latched into the TACHX_COUNTER field and reset when the programmed number of edges is detected.\n 0=Counter is incremented when Tach Input transitions from low-tohigh state (default)"]
    #[inline(always)]
    pub fn rd_mod_sel(&self) -> RD_MOD_SEL_R {
        RD_MOD_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - TACH_EDGES A Tach signal is a square wave with a 50 percent duty cycle. Typically, two Tach periods represents one revolution of the fan. A Tach period consists of three Tach edges. This programmed value represents the number of Tach edges that will be used to determine the interval for which the number of 100kHz_Clk pulses will be counted\n 11b=9 Tach edges (4 Tach periods)\n 10b=5 Tach edges (2 Tach periods)\n 01b=3 Tach edges (1 Tach period)\n 00b=2 Tach edges (1/2 Tach period)"]
    #[inline(always)]
    pub fn edges(&self) -> EDGES_R {
        EDGES_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 14 - COUNT_READY_INT_EN 1=Enable Count Ready interrupt from Tach block, 0=Disable Count Ready interrupt from Tach block"]
    #[inline(always)]
    pub fn cnt_rdy_int_en(&self) -> CNT_RDY_INT_EN_R {
        CNT_RDY_INT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TACH_INPUT_INT_EN 1=Enable Tach Input toggle interrupt from Tach block, 0=Disable Tach Input toggle interrupt from Tach block"]
    #[inline(always)]
    pub fn in_int_en(&self) -> IN_INT_EN_R {
        IN_INT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - This 16-bit field contains the latched value of the internal Tach pulse counter, which may be configured by the Tach Reading Mode Select field to operate as a free-running counter or to be gated by the Tach input signal."]
    #[inline(always)]
    pub fn cntr(&self) -> CNTR_R {
        CNTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - TACH_OUT_OF_LIMIT_ENABLE This bit is used to enable the TACH_OUT_OF_LIMIT_STATUS bit in the TACHx Status Register to generate an interrupt event.\n 1=Enable interrupt output from Tach block\n 0=Disable interrupt output from Tach block (default)"]
    #[inline(always)]
    pub fn outof_lim_en(&mut self) -> OUTOF_LIM_EN_W<0> {
        OUTOF_LIM_EN_W::new(self)
    }
    #[doc = "Bit 1 - TACH_ENABLE 1= TACH Monitoring enabled, clocks enabled. 0= TACH Idle, clocks gated"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<1> {
        EN_W::new(self)
    }
    #[doc = "Bit 8 - FILTER_ENABLE This filter is used to remove high frequency glitches from Tach Input. When this filter is enabled, Tach input pulses less than two 100kHz_- Clk periods wide get filtered.\n 1= Filter enabled\n 0= Filter disabled (default)\n It is recommended that the Tach input filter always be enabled."]
    #[inline(always)]
    pub fn filt_en(&mut self) -> FILT_EN_W<8> {
        FILT_EN_W::new(self)
    }
    #[doc = "Bit 10 - TACH_READING_MODE_SELECT\n 1=Counter is incremented on the rising edge of the 100kHz_Clk input. The counter is latched into the TACHX_COUNTER field and reset when the programmed number of edges is detected.\n 0=Counter is incremented when Tach Input transitions from low-tohigh state (default)"]
    #[inline(always)]
    pub fn rd_mod_sel(&mut self) -> RD_MOD_SEL_W<10> {
        RD_MOD_SEL_W::new(self)
    }
    #[doc = "Bits 11:12 - TACH_EDGES A Tach signal is a square wave with a 50 percent duty cycle. Typically, two Tach periods represents one revolution of the fan. A Tach period consists of three Tach edges. This programmed value represents the number of Tach edges that will be used to determine the interval for which the number of 100kHz_Clk pulses will be counted\n 11b=9 Tach edges (4 Tach periods)\n 10b=5 Tach edges (2 Tach periods)\n 01b=3 Tach edges (1 Tach period)\n 00b=2 Tach edges (1/2 Tach period)"]
    #[inline(always)]
    pub fn edges(&mut self) -> EDGES_W<11> {
        EDGES_W::new(self)
    }
    #[doc = "Bit 14 - COUNT_READY_INT_EN 1=Enable Count Ready interrupt from Tach block, 0=Disable Count Ready interrupt from Tach block"]
    #[inline(always)]
    pub fn cnt_rdy_int_en(&mut self) -> CNT_RDY_INT_EN_W<14> {
        CNT_RDY_INT_EN_W::new(self)
    }
    #[doc = "Bit 15 - TACH_INPUT_INT_EN 1=Enable Tach Input toggle interrupt from Tach block, 0=Disable Tach Input toggle interrupt from Tach block"]
    #[inline(always)]
    pub fn in_int_en(&mut self) -> IN_INT_EN_W<15> {
        IN_INT_EN_W::new(self)
    }
    #[doc = "Bits 16:31 - This 16-bit field contains the latched value of the internal Tach pulse counter, which may be configured by the Tach Reading Mode Select field to operate as a free-running counter or to be gated by the Tach input signal."]
    #[inline(always)]
    pub fn cntr(&mut self) -> CNTR_W<16> {
        CNTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TACHx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
