#[doc = "Register `STS` reader"]
pub struct R(crate::R<STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STS` writer"]
pub struct W(crate::W<STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STS_SPEC>;
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
impl From<crate::W<STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRANS_COMPL` reader - In Manual Mode (neither DMA nor Description Buffers are enabled), this bit will be set to 1 when the transfer matches TRANSFER_LENGTH.\n If DMA Mode is enabled, this bit will be set to 1 when DMA_COMPLETE is set to 1. In Description Buffer Mode, this bit will be set to 1 only when the Last Buffer completes its transfer.\n In all cases, this bit will be set to 1 if the STOP bit is set to 1 and the controller has completed the current 8 bits being copied.\n 1=Transfer completed; 0=Transfer not complete."]
pub type TRANS_COMPL_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_COMPL` writer - In Manual Mode (neither DMA nor Description Buffers are enabled), this bit will be set to 1 when the transfer matches TRANSFER_LENGTH.\n If DMA Mode is enabled, this bit will be set to 1 when DMA_COMPLETE is set to 1. In Description Buffer Mode, this bit will be set to 1 only when the Last Buffer completes its transfer.\n In all cases, this bit will be set to 1 if the STOP bit is set to 1 and the controller has completed the current 8 bits being copied.\n 1=Transfer completed; 0=Transfer not complete."]
pub type TRANS_COMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `DMA_COMPL` reader - This field has no meaning if DMA is not enabled. This bit will be set to 1 when the DMA controller asserts the DONE signal to the SPI controller.\n This occurs either when the SPI controller has closed the DMA transfer, or the DMA channel has completed its count. If both Transmit and Receive DMA transfers are\n active, then this bit will only assert after both have completed. If CLOSE_TRANSFER_ENABLE is enabled, DMA_COMPLETE and TRANSFER_COMPLETE will be asserted simultaneously.\n This status is not inhibited by the description buffers, so it can fire on all valid description buffers while operating in that mode.\n 1=DMA completed; 0=DMA not completed."]
pub type DMA_COMPL_R = crate::BitReader<bool>;
#[doc = "Field `DMA_COMPL` writer - This field has no meaning if DMA is not enabled. This bit will be set to 1 when the DMA controller asserts the DONE signal to the SPI controller.\n This occurs either when the SPI controller has closed the DMA transfer, or the DMA channel has completed its count. If both Transmit and Receive DMA transfers are\n active, then this bit will only assert after both have completed. If CLOSE_TRANSFER_ENABLE is enabled, DMA_COMPLETE and TRANSFER_COMPLETE will be asserted simultaneously.\n This status is not inhibited by the description buffers, so it can fire on all valid description buffers while operating in that mode.\n 1=DMA completed; 0=DMA not completed."]
pub type DMA_COMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `TX_BUFF_ERR` reader - 1=Overflow error occurred (attempt to write to a full Transmit Buffer)\n 0=No overflow occurred."]
pub type TX_BUFF_ERR_R = crate::BitReader<bool>;
#[doc = "Field `TX_BUFF_ERR` writer - 1=Overflow error occurred (attempt to write to a full Transmit Buffer)\n 0=No overflow occurred."]
pub type TX_BUFF_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `RX_BUFF_ERR` reader - 1=Underflow error occurred (attempt to read from an empty Receive Buffer)\n 0=No underflow occurred."]
pub type RX_BUFF_ERR_R = crate::BitReader<bool>;
#[doc = "Field `RX_BUFF_ERR` writer - 1=Underflow error occurred (attempt to read from an empty Receive Buffer)\n 0=No underflow occurred."]
pub type RX_BUFF_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `PRGM_ERR` reader - This bit if a programming error is detected.\n 1=Programming Error detected; 0=No programming error detected."]
pub type PRGM_ERR_R = crate::BitReader<bool>;
#[doc = "Field `PRGM_ERR` writer - This bit if a programming error is detected.\n 1=Programming Error detected; 0=No programming error detected."]
pub type PRGM_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `TX_BUFF_FULL` reader - 1=The Transmit Buffer is full\n 0=The Transmit Buffer is not full."]
pub type TX_BUFF_FULL_R = crate::BitReader<bool>;
#[doc = "Field `TX_BUFF_FULL` writer - 1=The Transmit Buffer is full\n 0=The Transmit Buffer is not full."]
pub type TX_BUFF_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `TX_BUFF_EMP` reader - 1=The Transmit Buffer is empty\n 0=The Transmit Buffer is not empty."]
pub type TX_BUFF_EMP_R = crate::BitReader<bool>;
#[doc = "Field `TX_BUFF_EMP` writer - 1=The Transmit Buffer is empty\n 0=The Transmit Buffer is not empty."]
pub type TX_BUFF_EMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `TX_BUFF_REQ` reader - This status is asserted if the Transmit Buffer reaches a high water mark established by the TRANSMIT_BUFFER_TRIGGER field.\n 1=TRANSMIT_BUFFER_COUNT is less than or equal to TRANSMIT_BUFFER_TRIGGER; 0=TRANSMIT_BUFFER_COUNT is greater than TRANSMIT_BUFFER_TRIGGER."]
pub type TX_BUFF_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TX_BUFF_REQ` writer - This status is asserted if the Transmit Buffer reaches a high water mark established by the TRANSMIT_BUFFER_TRIGGER field.\n 1=TRANSMIT_BUFFER_COUNT is less than or equal to TRANSMIT_BUFFER_TRIGGER; 0=TRANSMIT_BUFFER_COUNT is greater than TRANSMIT_BUFFER_TRIGGER."]
pub type TX_BUFF_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `TX_BUFF_STALL` reader - 1=The SPI interface had been stalled due to a flow issue (an attempt by the interface to read from an empty Transmit Buffer)\n 0=No stalls occurred."]
pub type TX_BUFF_STALL_R = crate::BitReader<bool>;
#[doc = "Field `TX_BUFF_STALL` writer - 1=The SPI interface had been stalled due to a flow issue (an attempt by the interface to read from an empty Transmit Buffer)\n 0=No stalls occurred."]
pub type TX_BUFF_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `RX_BUFF_FULL` reader - 1=The Receive Buffer is full\n 0=The Receive Buffer is not full."]
pub type RX_BUFF_FULL_R = crate::BitReader<bool>;
#[doc = "Field `RX_BUFF_FULL` writer - 1=The Receive Buffer is full\n 0=The Receive Buffer is not full."]
pub type RX_BUFF_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `RX_BUFF_EMP` reader - 1=The Receive Buffer is empty\n 0=The Receive Buffer is not empty."]
pub type RX_BUFF_EMP_R = crate::BitReader<bool>;
#[doc = "Field `RX_BUFF_EMP` writer - 1=The Receive Buffer is empty\n 0=The Receive Buffer is not empty."]
pub type RX_BUFF_EMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `RX_BUFF_REQ` reader - This status is asserted if the Receive Buffer reaches a high water mark established by the RECEIVE_BUFFER_TRIGGER field.\n 1=RECEIVE_BUFFER_COUNT is greater than or equal to RECEIVE_BUFFER_TRIGGER\n 0=RECEIVE_BUFFER_COUNT is less than RECEIVE_BUFFER_TRIGGER."]
pub type RX_BUFF_REQ_R = crate::BitReader<bool>;
#[doc = "Field `RX_BUFF_REQ` writer - This status is asserted if the Receive Buffer reaches a high water mark established by the RECEIVE_BUFFER_TRIGGER field.\n 1=RECEIVE_BUFFER_COUNT is greater than or equal to RECEIVE_BUFFER_TRIGGER\n 0=RECEIVE_BUFFER_COUNT is less than RECEIVE_BUFFER_TRIGGER."]
pub type RX_BUFF_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `RX_BUFF_STALL` reader - 1=The SPI interface had been stalled due to a flow issue (an attempt by the interface to write to a full Receive Buffer)\n 0=No stalls occurred."]
pub type RX_BUFF_STALL_R = crate::BitReader<bool>;
#[doc = "Field `RX_BUFF_STALL` writer - 1=The SPI interface had been stalled due to a flow issue (an attempt by the interface to write to a full Receive Buffer)\n 0=No stalls occurred."]
pub type RX_BUFF_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `TRANS_ACTIV` reader - 1=A transfer is currently executing\n 0=No transfer currently in progress."]
pub type TRANS_ACTIV_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_ACTIV` writer - 1=A transfer is currently executing\n 0=No transfer currently in progress."]
pub type TRANS_ACTIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `CUR_DESCR_BUF` reader - This field shows the Description Buffer currently active. This field has no meaning if Description Buffers are not enabled."]
pub type CUR_DESCR_BUF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CUR_DESCR_BUF` writer - This field shows the Description Buffer currently active. This field has no meaning if Description Buffers are not enabled."]
pub type CUR_DESCR_BUF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STS_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - In Manual Mode (neither DMA nor Description Buffers are enabled), this bit will be set to 1 when the transfer matches TRANSFER_LENGTH.\n If DMA Mode is enabled, this bit will be set to 1 when DMA_COMPLETE is set to 1. In Description Buffer Mode, this bit will be set to 1 only when the Last Buffer completes its transfer.\n In all cases, this bit will be set to 1 if the STOP bit is set to 1 and the controller has completed the current 8 bits being copied.\n 1=Transfer completed; 0=Transfer not complete."]
    #[inline(always)]
    pub fn trans_compl(&self) -> TRANS_COMPL_R {
        TRANS_COMPL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This field has no meaning if DMA is not enabled. This bit will be set to 1 when the DMA controller asserts the DONE signal to the SPI controller.\n This occurs either when the SPI controller has closed the DMA transfer, or the DMA channel has completed its count. If both Transmit and Receive DMA transfers are\n active, then this bit will only assert after both have completed. If CLOSE_TRANSFER_ENABLE is enabled, DMA_COMPLETE and TRANSFER_COMPLETE will be asserted simultaneously.\n This status is not inhibited by the description buffers, so it can fire on all valid description buffers while operating in that mode.\n 1=DMA completed; 0=DMA not completed."]
    #[inline(always)]
    pub fn dma_compl(&self) -> DMA_COMPL_R {
        DMA_COMPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1=Overflow error occurred (attempt to write to a full Transmit Buffer)\n 0=No overflow occurred."]
    #[inline(always)]
    pub fn tx_buff_err(&self) -> TX_BUFF_ERR_R {
        TX_BUFF_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1=Underflow error occurred (attempt to read from an empty Receive Buffer)\n 0=No underflow occurred."]
    #[inline(always)]
    pub fn rx_buff_err(&self) -> RX_BUFF_ERR_R {
        RX_BUFF_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit if a programming error is detected.\n 1=Programming Error detected; 0=No programming error detected."]
    #[inline(always)]
    pub fn prgm_err(&self) -> PRGM_ERR_R {
        PRGM_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 1=The Transmit Buffer is full\n 0=The Transmit Buffer is not full."]
    #[inline(always)]
    pub fn tx_buff_full(&self) -> TX_BUFF_FULL_R {
        TX_BUFF_FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1=The Transmit Buffer is empty\n 0=The Transmit Buffer is not empty."]
    #[inline(always)]
    pub fn tx_buff_emp(&self) -> TX_BUFF_EMP_R {
        TX_BUFF_EMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This status is asserted if the Transmit Buffer reaches a high water mark established by the TRANSMIT_BUFFER_TRIGGER field.\n 1=TRANSMIT_BUFFER_COUNT is less than or equal to TRANSMIT_BUFFER_TRIGGER; 0=TRANSMIT_BUFFER_COUNT is greater than TRANSMIT_BUFFER_TRIGGER."]
    #[inline(always)]
    pub fn tx_buff_req(&self) -> TX_BUFF_REQ_R {
        TX_BUFF_REQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1=The SPI interface had been stalled due to a flow issue (an attempt by the interface to read from an empty Transmit Buffer)\n 0=No stalls occurred."]
    #[inline(always)]
    pub fn tx_buff_stall(&self) -> TX_BUFF_STALL_R {
        TX_BUFF_STALL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1=The Receive Buffer is full\n 0=The Receive Buffer is not full."]
    #[inline(always)]
    pub fn rx_buff_full(&self) -> RX_BUFF_FULL_R {
        RX_BUFF_FULL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1=The Receive Buffer is empty\n 0=The Receive Buffer is not empty."]
    #[inline(always)]
    pub fn rx_buff_emp(&self) -> RX_BUFF_EMP_R {
        RX_BUFF_EMP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This status is asserted if the Receive Buffer reaches a high water mark established by the RECEIVE_BUFFER_TRIGGER field.\n 1=RECEIVE_BUFFER_COUNT is greater than or equal to RECEIVE_BUFFER_TRIGGER\n 0=RECEIVE_BUFFER_COUNT is less than RECEIVE_BUFFER_TRIGGER."]
    #[inline(always)]
    pub fn rx_buff_req(&self) -> RX_BUFF_REQ_R {
        RX_BUFF_REQ_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1=The SPI interface had been stalled due to a flow issue (an attempt by the interface to write to a full Receive Buffer)\n 0=No stalls occurred."]
    #[inline(always)]
    pub fn rx_buff_stall(&self) -> RX_BUFF_STALL_R {
        RX_BUFF_STALL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1=A transfer is currently executing\n 0=No transfer currently in progress."]
    #[inline(always)]
    pub fn trans_activ(&self) -> TRANS_ACTIV_R {
        TRANS_ACTIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:27 - This field shows the Description Buffer currently active. This field has no meaning if Description Buffers are not enabled."]
    #[inline(always)]
    pub fn cur_descr_buf(&self) -> CUR_DESCR_BUF_R {
        CUR_DESCR_BUF_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - In Manual Mode (neither DMA nor Description Buffers are enabled), this bit will be set to 1 when the transfer matches TRANSFER_LENGTH.\n If DMA Mode is enabled, this bit will be set to 1 when DMA_COMPLETE is set to 1. In Description Buffer Mode, this bit will be set to 1 only when the Last Buffer completes its transfer.\n In all cases, this bit will be set to 1 if the STOP bit is set to 1 and the controller has completed the current 8 bits being copied.\n 1=Transfer completed; 0=Transfer not complete."]
    #[inline(always)]
    pub fn trans_compl(&mut self) -> TRANS_COMPL_W<0> {
        TRANS_COMPL_W::new(self)
    }
    #[doc = "Bit 1 - This field has no meaning if DMA is not enabled. This bit will be set to 1 when the DMA controller asserts the DONE signal to the SPI controller.\n This occurs either when the SPI controller has closed the DMA transfer, or the DMA channel has completed its count. If both Transmit and Receive DMA transfers are\n active, then this bit will only assert after both have completed. If CLOSE_TRANSFER_ENABLE is enabled, DMA_COMPLETE and TRANSFER_COMPLETE will be asserted simultaneously.\n This status is not inhibited by the description buffers, so it can fire on all valid description buffers while operating in that mode.\n 1=DMA completed; 0=DMA not completed."]
    #[inline(always)]
    pub fn dma_compl(&mut self) -> DMA_COMPL_W<1> {
        DMA_COMPL_W::new(self)
    }
    #[doc = "Bit 2 - 1=Overflow error occurred (attempt to write to a full Transmit Buffer)\n 0=No overflow occurred."]
    #[inline(always)]
    pub fn tx_buff_err(&mut self) -> TX_BUFF_ERR_W<2> {
        TX_BUFF_ERR_W::new(self)
    }
    #[doc = "Bit 3 - 1=Underflow error occurred (attempt to read from an empty Receive Buffer)\n 0=No underflow occurred."]
    #[inline(always)]
    pub fn rx_buff_err(&mut self) -> RX_BUFF_ERR_W<3> {
        RX_BUFF_ERR_W::new(self)
    }
    #[doc = "Bit 4 - This bit if a programming error is detected.\n 1=Programming Error detected; 0=No programming error detected."]
    #[inline(always)]
    pub fn prgm_err(&mut self) -> PRGM_ERR_W<4> {
        PRGM_ERR_W::new(self)
    }
    #[doc = "Bit 8 - 1=The Transmit Buffer is full\n 0=The Transmit Buffer is not full."]
    #[inline(always)]
    pub fn tx_buff_full(&mut self) -> TX_BUFF_FULL_W<8> {
        TX_BUFF_FULL_W::new(self)
    }
    #[doc = "Bit 9 - 1=The Transmit Buffer is empty\n 0=The Transmit Buffer is not empty."]
    #[inline(always)]
    pub fn tx_buff_emp(&mut self) -> TX_BUFF_EMP_W<9> {
        TX_BUFF_EMP_W::new(self)
    }
    #[doc = "Bit 10 - This status is asserted if the Transmit Buffer reaches a high water mark established by the TRANSMIT_BUFFER_TRIGGER field.\n 1=TRANSMIT_BUFFER_COUNT is less than or equal to TRANSMIT_BUFFER_TRIGGER; 0=TRANSMIT_BUFFER_COUNT is greater than TRANSMIT_BUFFER_TRIGGER."]
    #[inline(always)]
    pub fn tx_buff_req(&mut self) -> TX_BUFF_REQ_W<10> {
        TX_BUFF_REQ_W::new(self)
    }
    #[doc = "Bit 11 - 1=The SPI interface had been stalled due to a flow issue (an attempt by the interface to read from an empty Transmit Buffer)\n 0=No stalls occurred."]
    #[inline(always)]
    pub fn tx_buff_stall(&mut self) -> TX_BUFF_STALL_W<11> {
        TX_BUFF_STALL_W::new(self)
    }
    #[doc = "Bit 12 - 1=The Receive Buffer is full\n 0=The Receive Buffer is not full."]
    #[inline(always)]
    pub fn rx_buff_full(&mut self) -> RX_BUFF_FULL_W<12> {
        RX_BUFF_FULL_W::new(self)
    }
    #[doc = "Bit 13 - 1=The Receive Buffer is empty\n 0=The Receive Buffer is not empty."]
    #[inline(always)]
    pub fn rx_buff_emp(&mut self) -> RX_BUFF_EMP_W<13> {
        RX_BUFF_EMP_W::new(self)
    }
    #[doc = "Bit 14 - This status is asserted if the Receive Buffer reaches a high water mark established by the RECEIVE_BUFFER_TRIGGER field.\n 1=RECEIVE_BUFFER_COUNT is greater than or equal to RECEIVE_BUFFER_TRIGGER\n 0=RECEIVE_BUFFER_COUNT is less than RECEIVE_BUFFER_TRIGGER."]
    #[inline(always)]
    pub fn rx_buff_req(&mut self) -> RX_BUFF_REQ_W<14> {
        RX_BUFF_REQ_W::new(self)
    }
    #[doc = "Bit 15 - 1=The SPI interface had been stalled due to a flow issue (an attempt by the interface to write to a full Receive Buffer)\n 0=No stalls occurred."]
    #[inline(always)]
    pub fn rx_buff_stall(&mut self) -> RX_BUFF_STALL_W<15> {
        RX_BUFF_STALL_W::new(self)
    }
    #[doc = "Bit 16 - 1=A transfer is currently executing\n 0=No transfer currently in progress."]
    #[inline(always)]
    pub fn trans_activ(&mut self) -> TRANS_ACTIV_W<16> {
        TRANS_ACTIV_W::new(self)
    }
    #[doc = "Bits 24:27 - This field shows the Description Buffer currently active. This field has no meaning if Description Buffers are not enabled."]
    #[inline(always)]
    pub fn cur_descr_buf(&mut self) -> CUR_DESCR_BUF_W<24> {
        CUR_DESCR_BUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts::R](R) reader structure"]
impl crate::Readable for STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sts::W](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STS to value 0x2200"]
impl crate::Resettable for STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2200
    }
}
