#[doc = "Register `BUF_CNT_STS` reader"]
pub struct R(crate::R<BUF_CNT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_CNT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_CNT_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_CNT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_CNT_STS` writer"]
pub struct W(crate::W<BUF_CNT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_CNT_STS_SPEC>;
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
impl From<crate::W<BUF_CNT_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_CNT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BUFF_CNT` reader - This is a count of the number of bytes currently valid in the Transmit Buffer."]
pub type TX_BUFF_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_BUFF_CNT` writer - This is a count of the number of bytes currently valid in the Transmit Buffer."]
pub type TX_BUFF_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BUF_CNT_STS_SPEC, u16, u16, 16, O>;
#[doc = "Field `RX_BUFF_CNT` reader - This is a count of the number of bytes currently valid in the Receive Buffer."]
pub type RX_BUFF_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX_BUFF_CNT` writer - This is a count of the number of bytes currently valid in the Receive Buffer."]
pub type RX_BUFF_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BUF_CNT_STS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This is a count of the number of bytes currently valid in the Transmit Buffer."]
    #[inline(always)]
    pub fn tx_buff_cnt(&self) -> TX_BUFF_CNT_R {
        TX_BUFF_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This is a count of the number of bytes currently valid in the Receive Buffer."]
    #[inline(always)]
    pub fn rx_buff_cnt(&self) -> RX_BUFF_CNT_R {
        RX_BUFF_CNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This is a count of the number of bytes currently valid in the Transmit Buffer."]
    #[inline(always)]
    pub fn tx_buff_cnt(&mut self) -> TX_BUFF_CNT_W<0> {
        TX_BUFF_CNT_W::new(self)
    }
    #[doc = "Bits 16:31 - This is a count of the number of bytes currently valid in the Receive Buffer."]
    #[inline(always)]
    pub fn rx_buff_cnt(&mut self) -> RX_BUFF_CNT_W<16> {
        RX_BUFF_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Buffer Count Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_cnt_sts](index.html) module"]
pub struct BUF_CNT_STS_SPEC;
impl crate::RegisterSpec for BUF_CNT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_cnt_sts::R](R) reader structure"]
impl crate::Readable for BUF_CNT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_cnt_sts::W](W) writer structure"]
impl crate::Writable for BUF_CNT_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF_CNT_STS to value 0"]
impl crate::Resettable for BUF_CNT_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
