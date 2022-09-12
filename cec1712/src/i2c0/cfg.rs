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
#[doc = "Field `PORT_SEL` reader - The PORT SEL \\[3:0\\]
bits determine which one of 16 possible bus ports apply to the active 2-wire SDAT and SCLK bus pair."]
pub type PORT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORT_SEL` writer - The PORT SEL \\[3:0\\]
bits determine which one of 16 possible bus ports apply to the active 2-wire SDAT and SCLK bus pair."]
pub type PORT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TCEN` reader - When the Timing Check Enable bit (TCEN) is asserted ('1'), Bus Time-Outs are enabled"]
pub type TCEN_R = crate::BitReader<bool>;
#[doc = "Field `TCEN` writer - When the Timing Check Enable bit (TCEN) is asserted ('1'), Bus Time-Outs are enabled"]
pub type TCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `SLOW_CLK` reader - When this bit is 1, the base period for the Bus Clock Register is multiplied by 4, and thus the frequency is divided by 4."]
pub type SLOW_CLK_R = crate::BitReader<bool>;
#[doc = "Field `SLOW_CLK` writer - When this bit is 1, the base period for the Bus Clock Register is multiplied by 4, and thus the frequency is divided by 4."]
pub type SLOW_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `TEST` reader - Must be always written with 0."]
pub type TEST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEST` writer - Must be always written with 0."]
pub type TEST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `FEN` reader - Input filtering enable. Input filtering is required by the I2C specification if external filtering is not available.\n 1=Input filtering is enabled; 0=Input filtering is disabled."]
pub type FEN_R = crate::BitReader<bool>;
#[doc = "Field `FEN` writer - Input filtering enable. Input filtering is required by the I2C specification if external filtering is not available.\n 1=Input filtering is enabled; 0=Input filtering is disabled."]
pub type FEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RST` reader - When RESET is asserted ('1'), all logic and registers except for the RESET bit itself are initialized to the power-on default state."]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - When RESET is asserted ('1'), all logic and registers except for the RESET bit itself are initialized to the power-on default state."]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `EN` reader - When ENAB (Enable) is not asserted ('0') (default), the SMB Controller Core is disabled and in the lowest power consumption state (Disabled State).\n The ENAB bit must be asserted ('1') for normal operation."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - When ENAB (Enable) is not asserted ('0') (default), the SMB Controller Core is disabled and in the lowest power consumption state (Disabled State).\n The ENAB bit must be asserted ('1') for normal operation."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `TEST0` reader - Must be always written with 0."]
pub type TEST0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEST0` writer - Must be always written with 0."]
pub type TEST0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `GC_DIS` reader - This is the General Call Disable bit.\n 0: the response to the General Call address as a slave is enabled\n 1: the response to the General Call address as a slave is disabled."]
pub type GC_DIS_R = crate::BitReader<bool>;
#[doc = "Field `GC_DIS` writer - This is the General Call Disable bit.\n 0: the response to the General Call address as a slave is enabled\n 1: the response to the General Call address as a slave is disabled."]
pub type GC_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CNFG_PROMIS` reader - This is the configur Promiscuous bit.\n 0: Normal operation is enabled. \n 1: Promiscuous Mode enabled , General Call Address disabled, Promiscuous Address Interrupt function enabled. Stall 9th clock of address byte enabled. Address byte ACK/NAK done by Promiscuous ACK setting.\n"]
pub type CNFG_PROMIS_R = crate::BitReader<bool>;
#[doc = "Field `CNFG_PROMIS` writer - This is the configur Promiscuous bit.\n 0: Normal operation is enabled. \n 1: Promiscuous Mode enabled , General Call Address disabled, Promiscuous Address Interrupt function enabled. Stall 9th clock of address byte enabled. Address byte ACK/NAK done by Promiscuous ACK setting.\n"]
pub type CNFG_PROMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - The PORT SEL \\[3:0\\]
bits determine which one of 16 possible bus ports apply to the active 2-wire SDAT and SCLK bus pair."]
    #[inline(always)]
    pub fn port_sel(&self) -> PORT_SEL_R {
        PORT_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - When the Timing Check Enable bit (TCEN) is asserted ('1'), Bus Time-Outs are enabled"]
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When this bit is 1, the base period for the Bus Clock Register is multiplied by 4, and thus the frequency is divided by 4."]
    #[inline(always)]
    pub fn slow_clk(&self) -> SLOW_CLK_R {
        SLOW_CLK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Must be always written with 0."]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Input filtering enable. Input filtering is required by the I2C specification if external filtering is not available.\n 1=Input filtering is enabled; 0=Input filtering is disabled."]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When RESET is asserted ('1'), all logic and registers except for the RESET bit itself are initialized to the power-on default state."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When ENAB (Enable) is not asserted ('0') (default), the SMB Controller Core is disabled and in the lowest power consumption state (Disabled State).\n The ENAB bit must be asserted ('1') for normal operation."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Must be always written with 0."]
    #[inline(always)]
    pub fn test0(&self) -> TEST0_R {
        TEST0_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - This is the General Call Disable bit.\n 0: the response to the General Call address as a slave is enabled\n 1: the response to the General Call address as a slave is disabled."]
    #[inline(always)]
    pub fn gc_dis(&self) -> GC_DIS_R {
        GC_DIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the configur Promiscuous bit.\n 0: Normal operation is enabled. \n 1: Promiscuous Mode enabled , General Call Address disabled, Promiscuous Address Interrupt function enabled. Stall 9th clock of address byte enabled. Address byte ACK/NAK done by Promiscuous ACK setting.\n"]
    #[inline(always)]
    pub fn cnfg_promis(&self) -> CNFG_PROMIS_R {
        CNFG_PROMIS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The PORT SEL \\[3:0\\]
bits determine which one of 16 possible bus ports apply to the active 2-wire SDAT and SCLK bus pair."]
    #[inline(always)]
    pub fn port_sel(&mut self) -> PORT_SEL_W<0> {
        PORT_SEL_W::new(self)
    }
    #[doc = "Bit 4 - When the Timing Check Enable bit (TCEN) is asserted ('1'), Bus Time-Outs are enabled"]
    #[inline(always)]
    pub fn tcen(&mut self) -> TCEN_W<4> {
        TCEN_W::new(self)
    }
    #[doc = "Bit 5 - When this bit is 1, the base period for the Bus Clock Register is multiplied by 4, and thus the frequency is divided by 4."]
    #[inline(always)]
    pub fn slow_clk(&mut self) -> SLOW_CLK_W<5> {
        SLOW_CLK_W::new(self)
    }
    #[doc = "Bits 6:7 - Must be always written with 0."]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W<6> {
        TEST_W::new(self)
    }
    #[doc = "Bit 8 - Input filtering enable. Input filtering is required by the I2C specification if external filtering is not available.\n 1=Input filtering is enabled; 0=Input filtering is disabled."]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W<8> {
        FEN_W::new(self)
    }
    #[doc = "Bit 9 - When RESET is asserted ('1'), all logic and registers except for the RESET bit itself are initialized to the power-on default state."]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<9> {
        RST_W::new(self)
    }
    #[doc = "Bit 10 - When ENAB (Enable) is not asserted ('0') (default), the SMB Controller Core is disabled and in the lowest power consumption state (Disabled State).\n The ENAB bit must be asserted ('1') for normal operation."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<10> {
        EN_W::new(self)
    }
    #[doc = "Bits 11:13 - Must be always written with 0."]
    #[inline(always)]
    pub fn test0(&mut self) -> TEST0_W<11> {
        TEST0_W::new(self)
    }
    #[doc = "Bit 14 - This is the General Call Disable bit.\n 0: the response to the General Call address as a slave is enabled\n 1: the response to the General Call address as a slave is disabled."]
    #[inline(always)]
    pub fn gc_dis(&mut self) -> GC_DIS_W<14> {
        GC_DIS_W::new(self)
    }
    #[doc = "Bit 15 - This is the configur Promiscuous bit.\n 0: Normal operation is enabled. \n 1: Promiscuous Mode enabled , General Call Address disabled, Promiscuous Address Interrupt function enabled. Stall 9th clock of address byte enabled. Address byte ACK/NAK done by Promiscuous ACK setting.\n"]
    #[inline(always)]
    pub fn cnfg_promis(&mut self) -> CNFG_PROMIS_W<15> {
        CNFG_PROMIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
