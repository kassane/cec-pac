#[doc = "Register `LSR` reader"]
pub struct R(crate::R<LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_READY` reader - DATA_READY Data Ready. It is set to a logic '1' whenever a complete incoming character has been received and transferred into the Receiver Buffer Register or the FIFO"]
pub type DATA_READY_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN` reader - OVERRUN Overrun Error."]
pub type OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `PE` reader - PARITY ERROR Parity Error."]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_ERR` reader - FRAME_ERROR Framing Error."]
pub type FRAME_ERR_R = crate::BitReader<bool>;
#[doc = "Field `BRK_INTR` reader - BREAK_INTERRUPT Break Interrupt."]
pub type BRK_INTR_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_EMPTY` reader - TRANSMIT_EMPTY Transmitter Holding Register Empty Bit 5 indicates that the Serial Port is ready to accept a new character for transmission."]
pub type TRANS_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_ERR` reader - Transmitter Empty. Bit 6 is set to a logic '1' whenever the Transmitter Holding Register (THR) and Transmitter Shift Register (TSR) are both empty."]
pub type TRANS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_ERR` reader - FIFO_ERROR"]
pub type FIFO_ERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DATA_READY Data Ready. It is set to a logic '1' whenever a complete incoming character has been received and transferred into the Receiver Buffer Register or the FIFO"]
    #[inline(always)]
    pub fn data_ready(&self) -> DATA_READY_R {
        DATA_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OVERRUN Overrun Error."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PARITY ERROR Parity Error."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FRAME_ERROR Framing Error."]
    #[inline(always)]
    pub fn frame_err(&self) -> FRAME_ERR_R {
        FRAME_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BREAK_INTERRUPT Break Interrupt."]
    #[inline(always)]
    pub fn brk_intr(&self) -> BRK_INTR_R {
        BRK_INTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TRANSMIT_EMPTY Transmitter Holding Register Empty Bit 5 indicates that the Serial Port is ready to accept a new character for transmission."]
    #[inline(always)]
    pub fn trans_empty(&self) -> TRANS_EMPTY_R {
        TRANS_EMPTY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty. Bit 6 is set to a logic '1' whenever the Transmitter Holding Register (THR) and Transmitter Shift Register (TSR) are both empty."]
    #[inline(always)]
    pub fn trans_err(&self) -> TRANS_ERR_R {
        TRANS_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FIFO_ERROR"]
    #[inline(always)]
    pub fn fifo_err(&self) -> FIFO_ERR_R {
        FIFO_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART Line Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](index.html) module"]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lsr::R](R) reader structure"]
impl crate::Readable for LSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSR to value 0"]
impl crate::Resettable for LSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
