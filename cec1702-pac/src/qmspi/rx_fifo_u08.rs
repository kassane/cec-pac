#[doc = "Register `RX_FIFO_u08[%s]` reader"]
pub struct R(crate::R<RX_FIFO_U08_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_U08_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FIFO_U08_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FIFO_U08_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_FIFO_u08[%s]` writer"]
pub struct W(crate::W<RX_FIFO_U08_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_FIFO_U08_SPEC>;
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
impl From<crate::W<RX_FIFO_U08_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_FIFO_U08_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_BUF` reader - Buffer that stores data from the external SPI Slave device to the SPI Master (this block), which is received over MISO or IO.\n Reads from this register will empty the Rx FIFO. A 1 Byte read will have valid data on bits \\[7:0\\]
and a Word read will have data on bits \\[15:0\\].\n It is possible to request more data than the FIFO has (underflow condition), but this will cause an error (Rx Buffer Error).\n Read accesses to this register decrement the RECEIVE_BUFFER_COUNT field."]
pub type RX_BUF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RX_BUF` writer - Buffer that stores data from the external SPI Slave device to the SPI Master (this block), which is received over MISO or IO.\n Reads from this register will empty the Rx FIFO. A 1 Byte read will have valid data on bits \\[7:0\\]
and a Word read will have data on bits \\[15:0\\].\n It is possible to request more data than the FIFO has (underflow condition), but this will cause an error (Rx Buffer Error).\n Read accesses to this register decrement the RECEIVE_BUFFER_COUNT field."]
pub type RX_BUF_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RX_FIFO_U08_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Buffer that stores data from the external SPI Slave device to the SPI Master (this block), which is received over MISO or IO.\n Reads from this register will empty the Rx FIFO. A 1 Byte read will have valid data on bits \\[7:0\\]
and a Word read will have data on bits \\[15:0\\].\n It is possible to request more data than the FIFO has (underflow condition), but this will cause an error (Rx Buffer Error).\n Read accesses to this register decrement the RECEIVE_BUFFER_COUNT field."]
    #[inline(always)]
    pub fn rx_buf(&self) -> RX_BUF_R {
        RX_BUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer that stores data from the external SPI Slave device to the SPI Master (this block), which is received over MISO or IO.\n Reads from this register will empty the Rx FIFO. A 1 Byte read will have valid data on bits \\[7:0\\]
and a Word read will have data on bits \\[15:0\\].\n It is possible to request more data than the FIFO has (underflow condition), but this will cause an error (Rx Buffer Error).\n Read accesses to this register decrement the RECEIVE_BUFFER_COUNT field."]
    #[inline(always)]
    pub fn rx_buf(&mut self) -> RX_BUF_W<0> {
        RX_BUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_u08](index.html) module"]
pub struct RX_FIFO_U08_SPEC;
impl crate::RegisterSpec for RX_FIFO_U08_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rx_fifo_u08::R](R) reader structure"]
impl crate::Readable for RX_FIFO_U08_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_fifo_u08::W](W) writer structure"]
impl crate::Writable for RX_FIFO_U08_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_FIFO_u08[%s]
to value 0"]
impl crate::Resettable for RX_FIFO_U08_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
