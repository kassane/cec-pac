#[doc = "Register `BUF_CNT_TRIG` reader"]
pub struct R(crate::R<BUF_CNT_TRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_CNT_TRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_CNT_TRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_CNT_TRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_CNT_TRIG` writer"]
pub struct W(crate::W<BUF_CNT_TRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_CNT_TRIG_SPEC>;
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
impl From<crate::W<BUF_CNT_TRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_CNT_TRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BUF_TRIG` reader - An interrupt is triggered if the TRANSMIT_BUFFER_COUNT field is less than or equal to this value. A value of 0 disables the interrupt."]
pub type TX_BUF_TRIG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_BUF_TRIG` writer - An interrupt is triggered if the TRANSMIT_BUFFER_COUNT field is less than or equal to this value. A value of 0 disables the interrupt."]
pub type TX_BUF_TRIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BUF_CNT_TRIG_SPEC, u16, u16, 16, O>;
#[doc = "Field `RX_BUF_TRIG` reader - An interrupt is triggered if the RECEIVE_BUFFER_COUNT field is greater than or equal to this value. A value of 0 disables the interrupt."]
pub type RX_BUF_TRIG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX_BUF_TRIG` writer - An interrupt is triggered if the RECEIVE_BUFFER_COUNT field is greater than or equal to this value. A value of 0 disables the interrupt."]
pub type RX_BUF_TRIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BUF_CNT_TRIG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - An interrupt is triggered if the TRANSMIT_BUFFER_COUNT field is less than or equal to this value. A value of 0 disables the interrupt."]
    #[inline(always)]
    pub fn tx_buf_trig(&self) -> TX_BUF_TRIG_R {
        TX_BUF_TRIG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - An interrupt is triggered if the RECEIVE_BUFFER_COUNT field is greater than or equal to this value. A value of 0 disables the interrupt."]
    #[inline(always)]
    pub fn rx_buf_trig(&self) -> RX_BUF_TRIG_R {
        RX_BUF_TRIG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - An interrupt is triggered if the TRANSMIT_BUFFER_COUNT field is less than or equal to this value. A value of 0 disables the interrupt."]
    #[inline(always)]
    pub fn tx_buf_trig(&mut self) -> TX_BUF_TRIG_W<0> {
        TX_BUF_TRIG_W::new(self)
    }
    #[doc = "Bits 16:31 - An interrupt is triggered if the RECEIVE_BUFFER_COUNT field is greater than or equal to this value. A value of 0 disables the interrupt."]
    #[inline(always)]
    pub fn rx_buf_trig(&mut self) -> RX_BUF_TRIG_W<16> {
        RX_BUF_TRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Buffer Count Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_cnt_trig](index.html) module"]
pub struct BUF_CNT_TRIG_SPEC;
impl crate::RegisterSpec for BUF_CNT_TRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_cnt_trig::R](R) reader structure"]
impl crate::Readable for BUF_CNT_TRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_cnt_trig::W](W) writer structure"]
impl crate::Writable for BUF_CNT_TRIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF_CNT_TRIG to value 0"]
impl crate::Resettable for BUF_CNT_TRIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
