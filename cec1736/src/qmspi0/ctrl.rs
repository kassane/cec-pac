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
#[doc = "Field `TX_MODE` reader - This field sets the transmission mode. If this field is set for Dual Mode or Quad Mode then either TX_TRANSFER_ENABLE or RX_TRANSFER_ENABLE must be 0. 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single/Duplex Mode."]
pub type TX_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_MODE` writer - This field sets the transmission mode. If this field is set for Dual Mode or Quad Mode then either TX_TRANSFER_ENABLE or RX_TRANSFER_ENABLE must be 0. 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single/Duplex Mode."]
pub type TX_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_TRANS_EN` reader - This field bit selects the transmit function of the SPI interface. 3=Transmit Enabled in 1 Mode. The MOSI or IO Bus will send out only 1's. The Transmit Buffer will not be used. 2=Transmit Enabled in 0 Mode. The MOSI or IO Bus will send out only 0's. The Transmit Buffer will not be used. 1=Transmit Enabled. Data will be fetched from the Transmit Buffer and sent out on the MOSI or IO Bus. 0=Transmit is Disabled. Not data is sent. This will cause the MOSI be to be undriven, or the IO bus to be undriven if Receive is also disabled."]
pub type TX_TRANS_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_TRANS_EN` writer - This field bit selects the transmit function of the SPI interface. 3=Transmit Enabled in 1 Mode. The MOSI or IO Bus will send out only 1's. The Transmit Buffer will not be used. 2=Transmit Enabled in 0 Mode. The MOSI or IO Bus will send out only 0's. The Transmit Buffer will not be used. 1=Transmit Enabled. Data will be fetched from the Transmit Buffer and sent out on the MOSI or IO Bus. 0=Transmit is Disabled. Not data is sent. This will cause the MOSI be to be undriven, or the IO bus to be undriven if Receive is also disabled."]
pub type TX_TRANS_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_DMA_EN` reader - This bit enables DMA support for Transmit Transfer. If enabled, DMA will be requested to fill the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes. 0=DMA is disabled. All data in the Transmit Buffer must be emptied by firmware"]
pub type TX_DMA_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_DMA_EN` writer - This bit enables DMA support for Transmit Transfer. If enabled, DMA will be requested to fill the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes. 0=DMA is disabled. All data in the Transmit Buffer must be emptied by firmware"]
pub type TX_DMA_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RX_TRANS_EN` reader - This bit enables the receive function of the SPI interface. 1=Receive is enabled. Data received from the SPI Slave is stored in the Receive Buffer 0=Receive is disabled"]
pub type RX_TRANS_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TRANS_EN` writer - This bit enables the receive function of the SPI interface. 1=Receive is enabled. Data received from the SPI Slave is stored in the Receive Buffer 0=Receive is disabled"]
pub type RX_TRANS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RX_DMA_EN` reader - This bit enables DMA support for Receive Transfer. If enabled, DMA will be requested to empty the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Receive Buffer must be emptied by firmware"]
pub type RX_DMA_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_DMA_EN` writer - This bit enables DMA support for Receive Transfer. If enabled, DMA will be requested to empty the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Receive Buffer must be emptied by firmware"]
pub type RX_DMA_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLOSE_TRANS_EN` reader - This selects what action is taken at the end of a transfer. When the transaction closes, the Chip Select de-asserts, the SPI interface returns to IDLE and the DMA interface terminates When Description Buffers are in use this bit must be set only on the Last Buffer. 1=The transaction is terminated 0=The transaction is not terminated"]
pub type CLOSE_TRANS_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLOSE_TRANS_EN` writer - This selects what action is taken at the end of a transfer. When the transaction closes, the Chip Select de-asserts, the SPI interface returns to IDLE and the DMA interface terminates When Description Buffers are in use this bit must be set only on the Last Buffer. 1=The transaction is terminated 0=The transaction is not terminated"]
pub type CLOSE_TRANS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TRANS_UNITS` reader - 3=TRANSFER_LENGTH defined in units of 16-byte segments 2=TRANSFER_LENGTH defined in units of 4-byte segments 1=TRANSFER_LENGTH defined in units of bytes 0=TRANSFER_LENGTH defined in units of bits."]
pub type TRANS_UNITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRANS_UNITS` writer - 3=TRANSFER_LENGTH defined in units of 16-byte segments 2=TRANSFER_LENGTH defined in units of 4-byte segments 1=TRANSFER_LENGTH defined in units of bytes 0=TRANSFER_LENGTH defined in units of bits."]
pub type TRANS_UNITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DESCR_BUFF_PTR` reader - This field selects the first buffer used if Description Buffers are enabled."]
pub type DESCR_BUFF_PTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DESCR_BUFF_PTR` writer - This field selects the first buffer used if Description Buffers are enabled."]
pub type DESCR_BUFF_PTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `DESCR_BUFF_EN` reader - This enables the Description Buffers to be used. 1=Description Buffers in use. The first buffer is defined in DESCRIPTION_BUFFER_POINTER 0=Description Buffers disabled."]
pub type DESCR_BUFF_EN_R = crate::BitReader<bool>;
#[doc = "Field `DESCR_BUFF_EN` writer - This enables the Description Buffers to be used. 1=Description Buffers in use. The first buffer is defined in DESCRIPTION_BUFFER_POINTER 0=Description Buffers disabled."]
pub type DESCR_BUFF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TRANS_LEN` reader - The length of the SPI transfer. The count is in bytes or bits, depending on the value of TRANSFER_LENGTH_BITS. A value of 0 means an infinite length transfer."]
pub type TRANS_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRANS_LEN` writer - The length of the SPI transfer. The count is in bytes or bits, depending on the value of TRANSFER_LENGTH_BITS. A value of 0 means an infinite length transfer."]
pub type TRANS_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:1 - This field sets the transmission mode. If this field is set for Dual Mode or Quad Mode then either TX_TRANSFER_ENABLE or RX_TRANSFER_ENABLE must be 0. 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single/Duplex Mode."]
    #[inline(always)]
    pub fn tx_mode(&self) -> TX_MODE_R {
        TX_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - This field bit selects the transmit function of the SPI interface. 3=Transmit Enabled in 1 Mode. The MOSI or IO Bus will send out only 1's. The Transmit Buffer will not be used. 2=Transmit Enabled in 0 Mode. The MOSI or IO Bus will send out only 0's. The Transmit Buffer will not be used. 1=Transmit Enabled. Data will be fetched from the Transmit Buffer and sent out on the MOSI or IO Bus. 0=Transmit is Disabled. Not data is sent. This will cause the MOSI be to be undriven, or the IO bus to be undriven if Receive is also disabled."]
    #[inline(always)]
    pub fn tx_trans_en(&self) -> TX_TRANS_EN_R {
        TX_TRANS_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - This bit enables DMA support for Transmit Transfer. If enabled, DMA will be requested to fill the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes. 0=DMA is disabled. All data in the Transmit Buffer must be emptied by firmware"]
    #[inline(always)]
    pub fn tx_dma_en(&self) -> TX_DMA_EN_R {
        TX_DMA_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - This bit enables the receive function of the SPI interface. 1=Receive is enabled. Data received from the SPI Slave is stored in the Receive Buffer 0=Receive is disabled"]
    #[inline(always)]
    pub fn rx_trans_en(&self) -> RX_TRANS_EN_R {
        RX_TRANS_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - This bit enables DMA support for Receive Transfer. If enabled, DMA will be requested to empty the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Receive Buffer must be emptied by firmware"]
    #[inline(always)]
    pub fn rx_dma_en(&self) -> RX_DMA_EN_R {
        RX_DMA_EN_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - This selects what action is taken at the end of a transfer. When the transaction closes, the Chip Select de-asserts, the SPI interface returns to IDLE and the DMA interface terminates When Description Buffers are in use this bit must be set only on the Last Buffer. 1=The transaction is terminated 0=The transaction is not terminated"]
    #[inline(always)]
    pub fn close_trans_en(&self) -> CLOSE_TRANS_EN_R {
        CLOSE_TRANS_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 3=TRANSFER_LENGTH defined in units of 16-byte segments 2=TRANSFER_LENGTH defined in units of 4-byte segments 1=TRANSFER_LENGTH defined in units of bytes 0=TRANSFER_LENGTH defined in units of bits."]
    #[inline(always)]
    pub fn trans_units(&self) -> TRANS_UNITS_R {
        TRANS_UNITS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - This field selects the first buffer used if Description Buffers are enabled."]
    #[inline(always)]
    pub fn descr_buff_ptr(&self) -> DESCR_BUFF_PTR_R {
        DESCR_BUFF_PTR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - This enables the Description Buffers to be used. 1=Description Buffers in use. The first buffer is defined in DESCRIPTION_BUFFER_POINTER 0=Description Buffers disabled."]
    #[inline(always)]
    pub fn descr_buff_en(&self) -> DESCR_BUFF_EN_R {
        DESCR_BUFF_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - The length of the SPI transfer. The count is in bytes or bits, depending on the value of TRANSFER_LENGTH_BITS. A value of 0 means an infinite length transfer."]
    #[inline(always)]
    pub fn trans_len(&self) -> TRANS_LEN_R {
        TRANS_LEN_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field sets the transmission mode. If this field is set for Dual Mode or Quad Mode then either TX_TRANSFER_ENABLE or RX_TRANSFER_ENABLE must be 0. 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single/Duplex Mode."]
    #[inline(always)]
    pub fn tx_mode(&mut self) -> TX_MODE_W<0> {
        TX_MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - This field bit selects the transmit function of the SPI interface. 3=Transmit Enabled in 1 Mode. The MOSI or IO Bus will send out only 1's. The Transmit Buffer will not be used. 2=Transmit Enabled in 0 Mode. The MOSI or IO Bus will send out only 0's. The Transmit Buffer will not be used. 1=Transmit Enabled. Data will be fetched from the Transmit Buffer and sent out on the MOSI or IO Bus. 0=Transmit is Disabled. Not data is sent. This will cause the MOSI be to be undriven, or the IO bus to be undriven if Receive is also disabled."]
    #[inline(always)]
    pub fn tx_trans_en(&mut self) -> TX_TRANS_EN_W<2> {
        TX_TRANS_EN_W::new(self)
    }
    #[doc = "Bits 4:5 - This bit enables DMA support for Transmit Transfer. If enabled, DMA will be requested to fill the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes. 0=DMA is disabled. All data in the Transmit Buffer must be emptied by firmware"]
    #[inline(always)]
    pub fn tx_dma_en(&mut self) -> TX_DMA_EN_W<4> {
        TX_DMA_EN_W::new(self)
    }
    #[doc = "Bit 6 - This bit enables the receive function of the SPI interface. 1=Receive is enabled. Data received from the SPI Slave is stored in the Receive Buffer 0=Receive is disabled"]
    #[inline(always)]
    pub fn rx_trans_en(&mut self) -> RX_TRANS_EN_W<6> {
        RX_TRANS_EN_W::new(self)
    }
    #[doc = "Bits 7:8 - This bit enables DMA support for Receive Transfer. If enabled, DMA will be requested to empty the FIFO until either the interface reaches TRANSFER_LENGTH or the DMA sends a termination request. The size defined here must match DMA programmed access size. 1=DMA is enabled.and set to 1 Byte 2=DMA is enabled and set to 2 Bytes 3=DMA is enabled and set to 4 Bytes 0=DMA is disabled. All data in the Receive Buffer must be emptied by firmware"]
    #[inline(always)]
    pub fn rx_dma_en(&mut self) -> RX_DMA_EN_W<7> {
        RX_DMA_EN_W::new(self)
    }
    #[doc = "Bit 9 - This selects what action is taken at the end of a transfer. When the transaction closes, the Chip Select de-asserts, the SPI interface returns to IDLE and the DMA interface terminates When Description Buffers are in use this bit must be set only on the Last Buffer. 1=The transaction is terminated 0=The transaction is not terminated"]
    #[inline(always)]
    pub fn close_trans_en(&mut self) -> CLOSE_TRANS_EN_W<9> {
        CLOSE_TRANS_EN_W::new(self)
    }
    #[doc = "Bits 10:11 - 3=TRANSFER_LENGTH defined in units of 16-byte segments 2=TRANSFER_LENGTH defined in units of 4-byte segments 1=TRANSFER_LENGTH defined in units of bytes 0=TRANSFER_LENGTH defined in units of bits."]
    #[inline(always)]
    pub fn trans_units(&mut self) -> TRANS_UNITS_W<10> {
        TRANS_UNITS_W::new(self)
    }
    #[doc = "Bits 12:15 - This field selects the first buffer used if Description Buffers are enabled."]
    #[inline(always)]
    pub fn descr_buff_ptr(&mut self) -> DESCR_BUFF_PTR_W<12> {
        DESCR_BUFF_PTR_W::new(self)
    }
    #[doc = "Bit 16 - This enables the Description Buffers to be used. 1=Description Buffers in use. The first buffer is defined in DESCRIPTION_BUFFER_POINTER 0=Description Buffers disabled."]
    #[inline(always)]
    pub fn descr_buff_en(&mut self) -> DESCR_BUFF_EN_W<16> {
        DESCR_BUFF_EN_W::new(self)
    }
    #[doc = "Bits 17:31 - The length of the SPI transfer. The count is in bytes or bits, depending on the value of TRANSFER_LENGTH_BITS. A value of 0 means an infinite length transfer."]
    #[inline(always)]
    pub fn trans_len(&mut self) -> TRANS_LEN_W<17> {
        TRANS_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI SPI Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
