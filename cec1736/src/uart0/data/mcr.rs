#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTR` reader - DTR This bit controls the Data Terminal Ready (nDTR) output."]
pub type DTR_R = crate::BitReader<bool>;
#[doc = "Field `DTR` writer - DTR This bit controls the Data Terminal Ready (nDTR) output."]
pub type DTR_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCR_SPEC, bool, O>;
#[doc = "Field `RTS` reader - RTS This bit controls the Request To Send (nRTS) output."]
pub type RTS_R = crate::BitReader<bool>;
#[doc = "Field `RTS` writer - RTS This bit controls the Request To Send (nRTS) output."]
pub type RTS_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCR_SPEC, bool, O>;
#[doc = "Field `OUT1` reader - OUT1 This bit controls the Output 1 (OUT1) bit."]
pub type OUT1_R = crate::BitReader<bool>;
#[doc = "Field `OUT1` writer - OUT1 This bit controls the Output 1 (OUT1) bit."]
pub type OUT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCR_SPEC, bool, O>;
#[doc = "Field `OUT2` reader - OUT2 This bit is used to enable an UART interrupt."]
pub type OUT2_R = crate::BitReader<bool>;
#[doc = "Field `OUT2` writer - OUT2 This bit is used to enable an UART interrupt."]
pub type OUT2_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCR_SPEC, bool, O>;
#[doc = "Field `LOOPBACK` reader - LOOPBACK This bit provides the loopback feature for diagnostic testing of the Serial Port."]
pub type LOOPBACK_R = crate::BitReader<bool>;
#[doc = "Field `LOOPBACK` writer - LOOPBACK This bit provides the loopback feature for diagnostic testing of the Serial Port."]
pub type LOOPBACK_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DTR This bit controls the Data Terminal Ready (nDTR) output."]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTS This bit controls the Request To Send (nRTS) output."]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OUT1 This bit controls the Output 1 (OUT1) bit."]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OUT2 This bit is used to enable an UART interrupt."]
    #[inline(always)]
    pub fn out2(&self) -> OUT2_R {
        OUT2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LOOPBACK This bit provides the loopback feature for diagnostic testing of the Serial Port."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTR This bit controls the Data Terminal Ready (nDTR) output."]
    #[inline(always)]
    pub fn dtr(&mut self) -> DTR_W<0> {
        DTR_W::new(self)
    }
    #[doc = "Bit 1 - RTS This bit controls the Request To Send (nRTS) output."]
    #[inline(always)]
    pub fn rts(&mut self) -> RTS_W<1> {
        RTS_W::new(self)
    }
    #[doc = "Bit 2 - OUT1 This bit controls the Output 1 (OUT1) bit."]
    #[inline(always)]
    pub fn out1(&mut self) -> OUT1_W<2> {
        OUT1_W::new(self)
    }
    #[doc = "Bit 3 - OUT2 This bit is used to enable an UART interrupt."]
    #[inline(always)]
    pub fn out2(&mut self) -> OUT2_W<3> {
        OUT2_W::new(self)
    }
    #[doc = "Bit 4 - LOOPBACK This bit provides the loopback feature for diagnostic testing of the Serial Port."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W<4> {
        LOOPBACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Modem Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
