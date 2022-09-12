#[doc = "Register `TX_FIFO_u16[%s]` reader"]
pub struct R(crate::R<TX_FIFO_U16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_FIFO_U16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_FIFO_U16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_FIFO_U16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_FIFO_u16[%s]` writer"]
pub struct W(crate::W<TX_FIFO_U16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_FIFO_U16_SPEC>;
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
impl From<crate::W<TX_FIFO_U16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_FIFO_U16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BUF` reader - Writes to this register store data to be transmitted from the SPI Master to the external SPI Slave.\n Writes to this block will be written to the Transmit FIFO. A 1 Byte write fills 1 byte of the FIFO. A Word write fills 2 Bytes and a Doubleword write fills 4 bytes.\n The data must always be aligned to the bottom most byte (so 1 byte write is on bits \\[7:0\\]
and Word write is on \\[15:0\\]).\n An overflow condition, TRANSMIT_BUFFER_ERROR, if a write to a full FIFO occurs.\n Write accesses to this register increment the TRANSMIT_BUFFER_COUNT field."]
pub type TX_BUF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TX_BUF` writer - Writes to this register store data to be transmitted from the SPI Master to the external SPI Slave.\n Writes to this block will be written to the Transmit FIFO. A 1 Byte write fills 1 byte of the FIFO. A Word write fills 2 Bytes and a Doubleword write fills 4 bytes.\n The data must always be aligned to the bottom most byte (so 1 byte write is on bits \\[7:0\\]
and Word write is on \\[15:0\\]).\n An overflow condition, TRANSMIT_BUFFER_ERROR, if a write to a full FIFO occurs.\n Write accesses to this register increment the TRANSMIT_BUFFER_COUNT field."]
pub type TX_BUF_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TX_FIFO_U16_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Writes to this register store data to be transmitted from the SPI Master to the external SPI Slave.\n Writes to this block will be written to the Transmit FIFO. A 1 Byte write fills 1 byte of the FIFO. A Word write fills 2 Bytes and a Doubleword write fills 4 bytes.\n The data must always be aligned to the bottom most byte (so 1 byte write is on bits \\[7:0\\]
and Word write is on \\[15:0\\]).\n An overflow condition, TRANSMIT_BUFFER_ERROR, if a write to a full FIFO occurs.\n Write accesses to this register increment the TRANSMIT_BUFFER_COUNT field."]
    #[inline(always)]
    pub fn tx_buf(&self) -> TX_BUF_R {
        TX_BUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Writes to this register store data to be transmitted from the SPI Master to the external SPI Slave.\n Writes to this block will be written to the Transmit FIFO. A 1 Byte write fills 1 byte of the FIFO. A Word write fills 2 Bytes and a Doubleword write fills 4 bytes.\n The data must always be aligned to the bottom most byte (so 1 byte write is on bits \\[7:0\\]
and Word write is on \\[15:0\\]).\n An overflow condition, TRANSMIT_BUFFER_ERROR, if a write to a full FIFO occurs.\n Write accesses to this register increment the TRANSMIT_BUFFER_COUNT field."]
    #[inline(always)]
    pub fn tx_buf(&mut self) -> TX_BUF_W<0> {
        TX_BUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_u16](index.html) module"]
pub struct TX_FIFO_U16_SPEC;
impl crate::RegisterSpec for TX_FIFO_U16_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tx_fifo_u16::R](R) reader structure"]
impl crate::Readable for TX_FIFO_U16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_fifo_u16::W](W) writer structure"]
impl crate::Writable for TX_FIFO_U16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_FIFO_u16[%s]
to value 0"]
impl crate::Resettable for TX_FIFO_U16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
