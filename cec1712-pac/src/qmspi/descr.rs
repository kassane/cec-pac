#[doc = "Register `DESCR[%s]` reader"]
pub struct R(crate::R<DESCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESCR[%s]` writer"]
pub struct W(crate::W<DESCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESCR_SPEC>;
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
impl From<crate::W<DESCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INFACE_MOD` reader - This field sets the transmission mode. If this field is set for Dual Mode or Quad Mode then either TX_TRANSFER_ENABLE or RX_TRANSFER_ENABLE must be 0.\n 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single/Duplex Mode."]
pub type INFACE_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INFACE_MOD` writer - This field sets the transmission mode. If this field is set for Dual Mode or Quad Mode then either TX_TRANSFER_ENABLE or RX_TRANSFER_ENABLE must be 0.\n 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single/Duplex Mode."]
pub type INFACE_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_TRANS_EN` reader - This field bit selects the transmit function of the SPI interface.\n 3=Transmit Enabled in 1 Mode. The MOSI or IO Bus will send out only 1's. The Transmit Buffer will not be used\n 2=Transmit Enabled in 0 Mode. The MOSI or IO Bus will send out only 0's. The Transmit Buffer will not be used.\n 1=Transmit Enabled. Data will be fetched from the Transmit Buffer and sent out on the MOSI or IO Bus.\n 0=Transmit is Disabled. No data is sent. This will cause the MOSI be to be undriven, or the IO bus to be undriven if Receive is also disabled."]
pub type TX_TRANS_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_TRANS_EN` writer - This field bit selects the transmit function of the SPI interface.\n 3=Transmit Enabled in 1 Mode. The MOSI or IO Bus will send out only 1's. The Transmit Buffer will not be used\n 2=Transmit Enabled in 0 Mode. The MOSI or IO Bus will send out only 0's. The Transmit Buffer will not be used.\n 1=Transmit Enabled. Data will be fetched from the Transmit Buffer and sent out on the MOSI or IO Bus.\n 0=Transmit is Disabled. No data is sent. This will cause the MOSI be to be undriven, or the IO bus to be undriven if Receive is also disabled."]
pub type TX_TRANS_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_DMA_EN` reader - This bit enables DMA support for Transmit Transfer. If enabled, DMA will be requested to fill the FIFO until either the interface\n reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size.\n 1=DMA is enabled.and set to 1 Byte\n 2=DMA is enabled and set to 2 Bytes\n 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Transmit Buffer must be emptied by firmware."]
pub type TX_DMA_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_DMA_EN` writer - This bit enables DMA support for Transmit Transfer. If enabled, DMA will be requested to fill the FIFO until either the interface\n reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size.\n 1=DMA is enabled.and set to 1 Byte\n 2=DMA is enabled and set to 2 Bytes\n 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Transmit Buffer must be emptied by firmware."]
pub type TX_DMA_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RX_TRANS_EN` reader - This bit enables the receive function of the SPI interface.\n 1=Receive is enabled. Data received from the SPI Slave is stored in the Receive Buffer; 0=Receive is disabled."]
pub type RX_TRANS_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TRANS_EN` writer - This bit enables the receive function of the SPI interface.\n 1=Receive is enabled. Data received from the SPI Slave is stored in the Receive Buffer; 0=Receive is disabled."]
pub type RX_TRANS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DESCR_SPEC, bool, O>;
#[doc = "Field `RX_DMA_EN` reader - This bit enables DMA support for Receive Transfer. If enabled, DMA will be requested to empty the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request.\n The size defined here must match DMA programmed access size.\n 1=DMA is enabled.and set to 1 Byte\n 2=DMA is enabled and set to 2 Bytes\n 3=DMA is enabled and set to 4 Bytes\n 0=DMA is disabled. All data in the Receive Buffer must be emptied by firmware."]
pub type RX_DMA_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_DMA_EN` writer - This bit enables DMA support for Receive Transfer. If enabled, DMA will be requested to empty the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request.\n The size defined here must match DMA programmed access size.\n 1=DMA is enabled.and set to 1 Byte\n 2=DMA is enabled and set to 2 Bytes\n 3=DMA is enabled and set to 4 Bytes\n 0=DMA is disabled. All data in the Receive Buffer must be emptied by firmware."]
pub type RX_DMA_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLOSE_TRANS_EN` reader - This selects what action is taken at the end of a transfer. This bit must be set only on the Last Buffer.\n 1=The transfer is terminated. The Chip Select de-asserts, the SPI interface returns to IDLE and the DMA interface completes the transfer.\n 0=The transfer is not closed. Chip Select remains asserted and the DMA interface and the SPI interface remain active"]
pub type CLOSE_TRANS_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLOSE_TRANS_EN` writer - This selects what action is taken at the end of a transfer. This bit must be set only on the Last Buffer.\n 1=The transfer is terminated. The Chip Select de-asserts, the SPI interface returns to IDLE and the DMA interface completes the transfer.\n 0=The transfer is not closed. Chip Select remains asserted and the DMA interface and the SPI interface remain active"]
pub type CLOSE_TRANS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DESCR_SPEC, bool, O>;
#[doc = "Field `TRANS_LEN_BITS` reader - 1=TRANSFER_LENGTH defined in bits\n 0=TRANSFER_LENGTH defined in bytes"]
pub type TRANS_LEN_BITS_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_LEN_BITS` writer - 1=TRANSFER_LENGTH defined in bits\n 0=TRANSFER_LENGTH defined in bytes"]
pub type TRANS_LEN_BITS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DESCR_SPEC, bool, O>;
#[doc = "Field `DESCR_BUF_LAST` reader - If this bit is 1 then this is the last Description Buffer in the chain. When the transfer described by this buffer completes the TRANSFER_COMPLETE status will be set to 1.\n If this bit is 0, then this is not the last buffer in use. When the transfer completes the next buffer will be activated, and no additional status will be asserted."]
pub type DESCR_BUF_LAST_R = crate::BitReader<bool>;
#[doc = "Field `DESCR_BUF_LAST` writer - If this bit is 1 then this is the last Description Buffer in the chain. When the transfer described by this buffer completes the TRANSFER_COMPLETE status will be set to 1.\n If this bit is 0, then this is not the last buffer in use. When the transfer completes the next buffer will be activated, and no additional status will be asserted."]
pub type DESCR_BUF_LAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DESCR_SPEC, bool, O>;
#[doc = "Field `DESCR_BUF_NXT_PTR` reader - This defines the next buffer to be used if Description Buffers are enabled and this is not the last buffer. This can point to the current buffer, creating an infinite loop."]
pub type DESCR_BUF_NXT_PTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DESCR_BUF_NXT_PTR` writer - This defines the next buffer to be used if Description Buffers are enabled and this is not the last buffer. This can point to the current buffer, creating an infinite loop."]
pub type DESCR_BUF_NXT_PTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TX_LEN` reader - The length of the SPI transfer. The count is in bytes or bits, depending on the value of TRANSFER_LENGTH_BITS. A value of 0 means an infinite length transfer."]
pub type TX_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_LEN` writer - The length of the SPI transfer. The count is in bytes or bits, depending on the value of TRANSFER_LENGTH_BITS. A value of 0 means an infinite length transfer."]
pub type TX_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:1 - This field sets the transmission mode. If this field is set for Dual Mode or Quad Mode then either TX_TRANSFER_ENABLE or RX_TRANSFER_ENABLE must be 0.\n 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single/Duplex Mode."]
    #[inline(always)]
    pub fn inface_mod(&self) -> INFACE_MOD_R {
        INFACE_MOD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - This field bit selects the transmit function of the SPI interface.\n 3=Transmit Enabled in 1 Mode. The MOSI or IO Bus will send out only 1's. The Transmit Buffer will not be used\n 2=Transmit Enabled in 0 Mode. The MOSI or IO Bus will send out only 0's. The Transmit Buffer will not be used.\n 1=Transmit Enabled. Data will be fetched from the Transmit Buffer and sent out on the MOSI or IO Bus.\n 0=Transmit is Disabled. No data is sent. This will cause the MOSI be to be undriven, or the IO bus to be undriven if Receive is also disabled."]
    #[inline(always)]
    pub fn tx_trans_en(&self) -> TX_TRANS_EN_R {
        TX_TRANS_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - This bit enables DMA support for Transmit Transfer. If enabled, DMA will be requested to fill the FIFO until either the interface\n reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size.\n 1=DMA is enabled.and set to 1 Byte\n 2=DMA is enabled and set to 2 Bytes\n 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Transmit Buffer must be emptied by firmware."]
    #[inline(always)]
    pub fn tx_dma_en(&self) -> TX_DMA_EN_R {
        TX_DMA_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - This bit enables the receive function of the SPI interface.\n 1=Receive is enabled. Data received from the SPI Slave is stored in the Receive Buffer; 0=Receive is disabled."]
    #[inline(always)]
    pub fn rx_trans_en(&self) -> RX_TRANS_EN_R {
        RX_TRANS_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - This bit enables DMA support for Receive Transfer. If enabled, DMA will be requested to empty the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request.\n The size defined here must match DMA programmed access size.\n 1=DMA is enabled.and set to 1 Byte\n 2=DMA is enabled and set to 2 Bytes\n 3=DMA is enabled and set to 4 Bytes\n 0=DMA is disabled. All data in the Receive Buffer must be emptied by firmware."]
    #[inline(always)]
    pub fn rx_dma_en(&self) -> RX_DMA_EN_R {
        RX_DMA_EN_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - This selects what action is taken at the end of a transfer. This bit must be set only on the Last Buffer.\n 1=The transfer is terminated. The Chip Select de-asserts, the SPI interface returns to IDLE and the DMA interface completes the transfer.\n 0=The transfer is not closed. Chip Select remains asserted and the DMA interface and the SPI interface remain active"]
    #[inline(always)]
    pub fn close_trans_en(&self) -> CLOSE_TRANS_EN_R {
        CLOSE_TRANS_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1=TRANSFER_LENGTH defined in bits\n 0=TRANSFER_LENGTH defined in bytes"]
    #[inline(always)]
    pub fn trans_len_bits(&self) -> TRANS_LEN_BITS_R {
        TRANS_LEN_BITS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If this bit is 1 then this is the last Description Buffer in the chain. When the transfer described by this buffer completes the TRANSFER_COMPLETE status will be set to 1.\n If this bit is 0, then this is not the last buffer in use. When the transfer completes the next buffer will be activated, and no additional status will be asserted."]
    #[inline(always)]
    pub fn descr_buf_last(&self) -> DESCR_BUF_LAST_R {
        DESCR_BUF_LAST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - This defines the next buffer to be used if Description Buffers are enabled and this is not the last buffer. This can point to the current buffer, creating an infinite loop."]
    #[inline(always)]
    pub fn descr_buf_nxt_ptr(&self) -> DESCR_BUF_NXT_PTR_R {
        DESCR_BUF_NXT_PTR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - The length of the SPI transfer. The count is in bytes or bits, depending on the value of TRANSFER_LENGTH_BITS. A value of 0 means an infinite length transfer."]
    #[inline(always)]
    pub fn tx_len(&self) -> TX_LEN_R {
        TX_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field sets the transmission mode. If this field is set for Dual Mode or Quad Mode then either TX_TRANSFER_ENABLE or RX_TRANSFER_ENABLE must be 0.\n 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single/Duplex Mode."]
    #[inline(always)]
    pub fn inface_mod(&mut self) -> INFACE_MOD_W<0> {
        INFACE_MOD_W::new(self)
    }
    #[doc = "Bits 2:3 - This field bit selects the transmit function of the SPI interface.\n 3=Transmit Enabled in 1 Mode. The MOSI or IO Bus will send out only 1's. The Transmit Buffer will not be used\n 2=Transmit Enabled in 0 Mode. The MOSI or IO Bus will send out only 0's. The Transmit Buffer will not be used.\n 1=Transmit Enabled. Data will be fetched from the Transmit Buffer and sent out on the MOSI or IO Bus.\n 0=Transmit is Disabled. No data is sent. This will cause the MOSI be to be undriven, or the IO bus to be undriven if Receive is also disabled."]
    #[inline(always)]
    pub fn tx_trans_en(&mut self) -> TX_TRANS_EN_W<2> {
        TX_TRANS_EN_W::new(self)
    }
    #[doc = "Bits 4:5 - This bit enables DMA support for Transmit Transfer. If enabled, DMA will be requested to fill the FIFO until either the interface\n reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size.\n 1=DMA is enabled.and set to 1 Byte\n 2=DMA is enabled and set to 2 Bytes\n 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Transmit Buffer must be emptied by firmware."]
    #[inline(always)]
    pub fn tx_dma_en(&mut self) -> TX_DMA_EN_W<4> {
        TX_DMA_EN_W::new(self)
    }
    #[doc = "Bit 6 - This bit enables the receive function of the SPI interface.\n 1=Receive is enabled. Data received from the SPI Slave is stored in the Receive Buffer; 0=Receive is disabled."]
    #[inline(always)]
    pub fn rx_trans_en(&mut self) -> RX_TRANS_EN_W<6> {
        RX_TRANS_EN_W::new(self)
    }
    #[doc = "Bits 7:8 - This bit enables DMA support for Receive Transfer. If enabled, DMA will be requested to empty the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request.\n The size defined here must match DMA programmed access size.\n 1=DMA is enabled.and set to 1 Byte\n 2=DMA is enabled and set to 2 Bytes\n 3=DMA is enabled and set to 4 Bytes\n 0=DMA is disabled. All data in the Receive Buffer must be emptied by firmware."]
    #[inline(always)]
    pub fn rx_dma_en(&mut self) -> RX_DMA_EN_W<7> {
        RX_DMA_EN_W::new(self)
    }
    #[doc = "Bit 9 - This selects what action is taken at the end of a transfer. This bit must be set only on the Last Buffer.\n 1=The transfer is terminated. The Chip Select de-asserts, the SPI interface returns to IDLE and the DMA interface completes the transfer.\n 0=The transfer is not closed. Chip Select remains asserted and the DMA interface and the SPI interface remain active"]
    #[inline(always)]
    pub fn close_trans_en(&mut self) -> CLOSE_TRANS_EN_W<9> {
        CLOSE_TRANS_EN_W::new(self)
    }
    #[doc = "Bit 10 - 1=TRANSFER_LENGTH defined in bits\n 0=TRANSFER_LENGTH defined in bytes"]
    #[inline(always)]
    pub fn trans_len_bits(&mut self) -> TRANS_LEN_BITS_W<10> {
        TRANS_LEN_BITS_W::new(self)
    }
    #[doc = "Bit 11 - If this bit is 1 then this is the last Description Buffer in the chain. When the transfer described by this buffer completes the TRANSFER_COMPLETE status will be set to 1.\n If this bit is 0, then this is not the last buffer in use. When the transfer completes the next buffer will be activated, and no additional status will be asserted."]
    #[inline(always)]
    pub fn descr_buf_last(&mut self) -> DESCR_BUF_LAST_W<11> {
        DESCR_BUF_LAST_W::new(self)
    }
    #[doc = "Bits 12:15 - This defines the next buffer to be used if Description Buffers are enabled and this is not the last buffer. This can point to the current buffer, creating an infinite loop."]
    #[inline(always)]
    pub fn descr_buf_nxt_ptr(&mut self) -> DESCR_BUF_NXT_PTR_W<12> {
        DESCR_BUF_NXT_PTR_W::new(self)
    }
    #[doc = "Bits 16:31 - The length of the SPI transfer. The count is in bytes or bits, depending on the value of TRANSFER_LENGTH_BITS. A value of 0 means an infinite length transfer."]
    #[inline(always)]
    pub fn tx_len(&mut self) -> TX_LEN_W<16> {
        TX_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Description Buffer 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr](index.html) module"]
pub struct DESCR_SPEC;
impl crate::RegisterSpec for DESCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [descr::R](R) reader structure"]
impl crate::Readable for DESCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [descr::W](W) writer structure"]
impl crate::Writable for DESCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DESCR[%s]
to value 0"]
impl crate::Resettable for DESCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
