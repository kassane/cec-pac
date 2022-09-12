#[doc = "Register `FIFO_CR` writer"]
pub struct W(crate::W<FIFO_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CR_SPEC>;
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
impl From<crate::W<FIFO_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXRF` writer - EXRF Enable XMIT and RECV FIFO."]
pub type EXRF_W<'a, const O: u8> = crate::BitWriter<'a, u8, FIFO_CR_SPEC, bool, O>;
#[doc = "Field `CLR_RECV_FIFO` writer - CLEAR_RECV_FIFO Setting this bit to a logic '1' clears all bytes in the RCVR FIFO and resets its counter logic to '0'."]
pub type CLR_RECV_FIFO_W<'a, const O: u8> = crate::BitWriter<'a, u8, FIFO_CR_SPEC, bool, O>;
#[doc = "Field `CLR_XMIT_FIFO` writer - CLEAR_XMIT_FIFO Setting this bit to a logic '1' clears all bytes in the XMIT FIFO and resets its counter logic to '0' . The shift register is not cleared. This bit is self-clearing."]
pub type CLR_XMIT_FIFO_W<'a, const O: u8> = crate::BitWriter<'a, u8, FIFO_CR_SPEC, bool, O>;
#[doc = "Field `DMA_MODE_SEL` writer - DMA_MODE_SELECT Writing to this bit has no effect on the operation of the UART. The RXRDY and TXRDY pins are not available on this chip."]
pub type DMA_MODE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, FIFO_CR_SPEC, bool, O>;
#[doc = "Field `RECV_FIFO_TRIG_LVL` writer - RECV_FIFO_TRIGGER_LEVEL These bits are used to set the trigger level for the RCVR FIFO interrupt."]
pub type RECV_FIFO_TRIG_LVL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, FIFO_CR_SPEC, u8, u8, 2, O>;
impl W {
    #[doc = "Bit 0 - EXRF Enable XMIT and RECV FIFO."]
    #[inline(always)]
    pub fn exrf(&mut self) -> EXRF_W<0> {
        EXRF_W::new(self)
    }
    #[doc = "Bit 1 - CLEAR_RECV_FIFO Setting this bit to a logic '1' clears all bytes in the RCVR FIFO and resets its counter logic to '0'."]
    #[inline(always)]
    pub fn clr_recv_fifo(&mut self) -> CLR_RECV_FIFO_W<1> {
        CLR_RECV_FIFO_W::new(self)
    }
    #[doc = "Bit 2 - CLEAR_XMIT_FIFO Setting this bit to a logic '1' clears all bytes in the XMIT FIFO and resets its counter logic to '0' . The shift register is not cleared. This bit is self-clearing."]
    #[inline(always)]
    pub fn clr_xmit_fifo(&mut self) -> CLR_XMIT_FIFO_W<2> {
        CLR_XMIT_FIFO_W::new(self)
    }
    #[doc = "Bit 3 - DMA_MODE_SELECT Writing to this bit has no effect on the operation of the UART. The RXRDY and TXRDY pins are not available on this chip."]
    #[inline(always)]
    pub fn dma_mode_sel(&mut self) -> DMA_MODE_SEL_W<3> {
        DMA_MODE_SEL_W::new(self)
    }
    #[doc = "Bits 6:7 - RECV_FIFO_TRIGGER_LEVEL These bits are used to set the trigger level for the RCVR FIFO interrupt."]
    #[inline(always)]
    pub fn recv_fifo_trig_lvl(&mut self) -> RECV_FIFO_TRIG_LVL_W<6> {
        RECV_FIFO_TRIG_LVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART FIFO Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_cr](index.html) module"]
pub struct FIFO_CR_SPEC;
impl crate::RegisterSpec for FIFO_CR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [fifo_cr::W](W) writer structure"]
impl crate::Writable for FIFO_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO_CR to value 0"]
impl crate::Resettable for FIFO_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
