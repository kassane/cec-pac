#[doc = "Register `CFG[%s]` reader"]
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
#[doc = "Register `CFG[%s]` writer"]
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
pub type TEST_R = crate::BitReader<bool>;
#[doc = "Field `TEST` writer - Must be always written with 0."]
pub type TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `PECEN` reader - When the PEC Enable bit (PECEN) is asserted ('1'), Hardware PEC Support is enabled"]
pub type PECEN_R = crate::BitReader<bool>;
#[doc = "Field `PECEN` writer - When the PEC Enable bit (PECEN) is asserted ('1'), Hardware PEC Support is enabled"]
pub type PECEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `FEN` reader - Input filtering enable. Input filtering is required by the I2C specification if external filtering is not available. 1=Input filtering is enabled; 0=Input filtering is disabled."]
pub type FEN_R = crate::BitReader<bool>;
#[doc = "Field `FEN` writer - Input filtering enable. Input filtering is required by the I2C specification if external filtering is not available. 1=Input filtering is enabled; 0=Input filtering is disabled."]
pub type FEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RST` reader - When RESET is asserted ('1'), all logic and registers except for the RESET bit itself are initialized to the power-on default state."]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - When RESET is asserted ('1'), all logic and registers except for the RESET bit itself are initialized to the power-on default state."]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `EN` reader - When ENAB (Enable) is not asserted ('0') (default), the SMB Controller Core is disabled and in the lowest power consumption state (Disabled State). The ENAB bit must be asserted ('1') for normal operation."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - When ENAB (Enable) is not asserted ('0') (default), the SMB Controller Core is disabled and in the lowest power consumption state (Disabled State). The ENAB bit must be asserted ('1') for normal operation."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DSA` reader - 0: Slave Address I2C Compatibility Mode (default). 1: SMBus Address Decode Mode"]
pub type DSA_R = crate::BitReader<bool>;
#[doc = "Field `DSA` writer - 0: Slave Address I2C Compatibility Mode (default). 1: SMBus Address Decode Mode"]
pub type DSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `FAIR` reader - If this bit is 1, the MCTP Fairness protocol is in effect."]
pub type FAIR_R = crate::BitReader<bool>;
#[doc = "Field `FAIR` writer - If this bit is 1, the MCTP Fairness protocol is in effect."]
pub type FAIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `TEST0` reader - Must be always written with 0."]
pub type TEST0_R = crate::BitReader<bool>;
#[doc = "Field `TEST0` writer - Must be always written with 0."]
pub type TEST0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `GC_DIS` reader - This is the General Call Disable bit. 0: the response to the General Call address as a slave is enabled 1: the response to the General Call address as a slave is disabled."]
pub type GC_DIS_R = crate::BitReader<bool>;
#[doc = "Field `GC_DIS` writer - This is the General Call Disable bit. 0: the response to the General Call address as a slave is enabled 1: the response to the General Call address as a slave is disabled."]
pub type GC_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CFG_PROMIS` reader - This bit define the Mode of SM Bus Controler Mode of operation. 0= Normal Operation. 1= Promiscuous Mode Enable."]
pub type CFG_PROMIS_R = crate::BitReader<bool>;
#[doc = "Field `CFG_PROMIS` writer - This bit define the Mode of SM Bus Controler Mode of operation. 0= Normal Operation. 1= Promiscuous Mode Enable."]
pub type CFG_PROMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `FLUSH_SXBUF` reader - A write of a 1 to this bit forces the SMBus Slave Transmit Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
pub type FLUSH_SXBUF_R = crate::BitReader<bool>;
#[doc = "Field `FLUSH_SXBUF` writer - A write of a 1 to this bit forces the SMBus Slave Transmit Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
pub type FLUSH_SXBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `FLUSH_SRBUF` reader - A write of a 1 to this bit forces the SMBus Slave Receive Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
pub type FLUSH_SRBUF_R = crate::BitReader<bool>;
#[doc = "Field `FLUSH_SRBUF` writer - A write of a 1 to this bit forces the SMBus Slave Receive Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
pub type FLUSH_SRBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `FLUSH_MXBUF` reader - A write of a 1 to this bit forces the SMBus Master Transmit Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
pub type FLUSH_MXBUF_R = crate::BitReader<bool>;
#[doc = "Field `FLUSH_MXBUF` writer - A write of a 1 to this bit forces the SMBus Master Transmit Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
pub type FLUSH_MXBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `FLUSH_MRBUF` reader - A write of a 1 to this bit forces the SMBus Master Receive Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
pub type FLUSH_MRBUF_R = crate::BitReader<bool>;
#[doc = "Field `FLUSH_MRBUF` writer - A write of a 1 to this bit forces the SMBus Master Receive Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
pub type FLUSH_MRBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `EN_AAS` reader - 0: Disable the AAS, 1: Enable the AAS Interrupt"]
pub type EN_AAS_R = crate::BitReader<bool>;
#[doc = "Field `EN_AAS` writer - 0: Disable the AAS, 1: Enable the AAS Interrupt"]
pub type EN_AAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `ENIDI` reader - If this bit is 1, the Idle interrupt is enabled. If this bit is 0, the Idle interrupt is disabled."]
pub type ENIDI_R = crate::BitReader<bool>;
#[doc = "Field `ENIDI` writer - If this bit is 1, the Idle interrupt is enabled. If this bit is 0, the Idle interrupt is disabled."]
pub type ENIDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `ENMI` reader - If this bit is 1, the Master Done interrupt is enabled. If this bit is 0, the Master Done interrupt is disabled."]
pub type ENMI_R = crate::BitReader<bool>;
#[doc = "Field `ENMI` writer - If this bit is 1, the Master Done interrupt is enabled. If this bit is 0, the Master Done interrupt is disabled."]
pub type ENMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `ENSI` reader - If this bit is 1, the Slave Done interrupt is enabled. If this bit is 0, the Slave Done interrupt is disabled"]
pub type ENSI_R = crate::BitReader<bool>;
#[doc = "Field `ENSI` writer - If this bit is 1, the Slave Done interrupt is enabled. If this bit is 0, the Slave Done interrupt is disabled"]
pub type ENSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
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
    #[doc = "Bit 6 - Must be always written with 0."]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When the PEC Enable bit (PECEN) is asserted ('1'), Hardware PEC Support is enabled"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Input filtering enable. Input filtering is required by the I2C specification if external filtering is not available. 1=Input filtering is enabled; 0=Input filtering is disabled."]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When RESET is asserted ('1'), all logic and registers except for the RESET bit itself are initialized to the power-on default state."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When ENAB (Enable) is not asserted ('0') (default), the SMB Controller Core is disabled and in the lowest power consumption state (Disabled State). The ENAB bit must be asserted ('1') for normal operation."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 0: Slave Address I2C Compatibility Mode (default). 1: SMBus Address Decode Mode"]
    #[inline(always)]
    pub fn dsa(&self) -> DSA_R {
        DSA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - If this bit is 1, the MCTP Fairness protocol is in effect."]
    #[inline(always)]
    pub fn fair(&self) -> FAIR_R {
        FAIR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Must be always written with 0."]
    #[inline(always)]
    pub fn test0(&self) -> TEST0_R {
        TEST0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the General Call Disable bit. 0: the response to the General Call address as a slave is enabled 1: the response to the General Call address as a slave is disabled."]
    #[inline(always)]
    pub fn gc_dis(&self) -> GC_DIS_R {
        GC_DIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit define the Mode of SM Bus Controler Mode of operation. 0= Normal Operation. 1= Promiscuous Mode Enable."]
    #[inline(always)]
    pub fn cfg_promis(&self) -> CFG_PROMIS_R {
        CFG_PROMIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - A write of a 1 to this bit forces the SMBus Slave Transmit Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
    #[inline(always)]
    pub fn flush_sxbuf(&self) -> FLUSH_SXBUF_R {
        FLUSH_SXBUF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - A write of a 1 to this bit forces the SMBus Slave Receive Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
    #[inline(always)]
    pub fn flush_srbuf(&self) -> FLUSH_SRBUF_R {
        FLUSH_SRBUF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A write of a 1 to this bit forces the SMBus Master Transmit Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
    #[inline(always)]
    pub fn flush_mxbuf(&self) -> FLUSH_MXBUF_R {
        FLUSH_MXBUF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - A write of a 1 to this bit forces the SMBus Master Receive Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
    #[inline(always)]
    pub fn flush_mrbuf(&self) -> FLUSH_MRBUF_R {
        FLUSH_MRBUF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 28 - 0: Disable the AAS, 1: Enable the AAS Interrupt"]
    #[inline(always)]
    pub fn en_aas(&self) -> EN_AAS_R {
        EN_AAS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - If this bit is 1, the Idle interrupt is enabled. If this bit is 0, the Idle interrupt is disabled."]
    #[inline(always)]
    pub fn enidi(&self) -> ENIDI_R {
        ENIDI_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - If this bit is 1, the Master Done interrupt is enabled. If this bit is 0, the Master Done interrupt is disabled."]
    #[inline(always)]
    pub fn enmi(&self) -> ENMI_R {
        ENMI_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If this bit is 1, the Slave Done interrupt is enabled. If this bit is 0, the Slave Done interrupt is disabled"]
    #[inline(always)]
    pub fn ensi(&self) -> ENSI_R {
        ENSI_R::new(((self.bits >> 31) & 1) != 0)
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
    #[doc = "Bit 6 - Must be always written with 0."]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W<6> {
        TEST_W::new(self)
    }
    #[doc = "Bit 7 - When the PEC Enable bit (PECEN) is asserted ('1'), Hardware PEC Support is enabled"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W<7> {
        PECEN_W::new(self)
    }
    #[doc = "Bit 8 - Input filtering enable. Input filtering is required by the I2C specification if external filtering is not available. 1=Input filtering is enabled; 0=Input filtering is disabled."]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W<8> {
        FEN_W::new(self)
    }
    #[doc = "Bit 9 - When RESET is asserted ('1'), all logic and registers except for the RESET bit itself are initialized to the power-on default state."]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<9> {
        RST_W::new(self)
    }
    #[doc = "Bit 10 - When ENAB (Enable) is not asserted ('0') (default), the SMB Controller Core is disabled and in the lowest power consumption state (Disabled State). The ENAB bit must be asserted ('1') for normal operation."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<10> {
        EN_W::new(self)
    }
    #[doc = "Bit 11 - 0: Slave Address I2C Compatibility Mode (default). 1: SMBus Address Decode Mode"]
    #[inline(always)]
    pub fn dsa(&mut self) -> DSA_W<11> {
        DSA_W::new(self)
    }
    #[doc = "Bit 12 - If this bit is 1, the MCTP Fairness protocol is in effect."]
    #[inline(always)]
    pub fn fair(&mut self) -> FAIR_W<12> {
        FAIR_W::new(self)
    }
    #[doc = "Bit 13 - Must be always written with 0."]
    #[inline(always)]
    pub fn test0(&mut self) -> TEST0_W<13> {
        TEST0_W::new(self)
    }
    #[doc = "Bit 14 - This is the General Call Disable bit. 0: the response to the General Call address as a slave is enabled 1: the response to the General Call address as a slave is disabled."]
    #[inline(always)]
    pub fn gc_dis(&mut self) -> GC_DIS_W<14> {
        GC_DIS_W::new(self)
    }
    #[doc = "Bit 15 - This bit define the Mode of SM Bus Controler Mode of operation. 0= Normal Operation. 1= Promiscuous Mode Enable."]
    #[inline(always)]
    pub fn cfg_promis(&mut self) -> CFG_PROMIS_W<15> {
        CFG_PROMIS_W::new(self)
    }
    #[doc = "Bit 16 - A write of a 1 to this bit forces the SMBus Slave Transmit Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
    #[inline(always)]
    pub fn flush_sxbuf(&mut self) -> FLUSH_SXBUF_W<16> {
        FLUSH_SXBUF_W::new(self)
    }
    #[doc = "Bit 17 - A write of a 1 to this bit forces the SMBus Slave Receive Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
    #[inline(always)]
    pub fn flush_srbuf(&mut self) -> FLUSH_SRBUF_W<17> {
        FLUSH_SRBUF_W::new(self)
    }
    #[doc = "Bit 18 - A write of a 1 to this bit forces the SMBus Master Transmit Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
    #[inline(always)]
    pub fn flush_mxbuf(&mut self) -> FLUSH_MXBUF_W<18> {
        FLUSH_MXBUF_W::new(self)
    }
    #[doc = "Bit 19 - A write of a 1 to this bit forces the SMBus Master Receive Buffer Register to be marked empty. A write of 0 has no effect. This is a self-clearing bit."]
    #[inline(always)]
    pub fn flush_mrbuf(&mut self) -> FLUSH_MRBUF_W<19> {
        FLUSH_MRBUF_W::new(self)
    }
    #[doc = "Bit 28 - 0: Disable the AAS, 1: Enable the AAS Interrupt"]
    #[inline(always)]
    pub fn en_aas(&mut self) -> EN_AAS_W<28> {
        EN_AAS_W::new(self)
    }
    #[doc = "Bit 29 - If this bit is 1, the Idle interrupt is enabled. If this bit is 0, the Idle interrupt is disabled."]
    #[inline(always)]
    pub fn enidi(&mut self) -> ENIDI_W<29> {
        ENIDI_W::new(self)
    }
    #[doc = "Bit 30 - If this bit is 1, the Master Done interrupt is enabled. If this bit is 0, the Master Done interrupt is disabled."]
    #[inline(always)]
    pub fn enmi(&mut self) -> ENMI_W<30> {
        ENMI_W::new(self)
    }
    #[doc = "Bit 31 - If this bit is 1, the Slave Done interrupt is enabled. If this bit is 0, the Slave Done interrupt is disabled"]
    #[inline(always)]
    pub fn ensi(&mut self) -> ENSI_W<31> {
        ENSI_W::new(self)
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
#[doc = "`reset()` method sets CFG[%s]
to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
