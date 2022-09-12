#[doc = "Register `SLV_TXB` reader"]
pub struct R(crate::R<SLV_TXB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLV_TXB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLV_TXB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLV_TXB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLV_TXB` writer"]
pub struct W(crate::W<SLV_TXB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLV_TXB_SPEC>;
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
impl From<crate::W<SLV_TXB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLV_TXB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STXB` reader - SLAVE_TRANSMIT_BUFFER"]
pub type STXB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STXB` writer - SLAVE_TRANSMIT_BUFFER"]
pub type STXB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLV_TXB_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SLAVE_TRANSMIT_BUFFER"]
    #[inline(always)]
    pub fn stxb(&self) -> STXB_R {
        STXB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SLAVE_TRANSMIT_BUFFER"]
    #[inline(always)]
    pub fn stxb(&mut self) -> STXB_W<0> {
        STXB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMBus Slave Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slv_txb](index.html) module"]
pub struct SLV_TXB_SPEC;
impl crate::RegisterSpec for SLV_TXB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slv_txb::R](R) reader structure"]
impl crate::Readable for SLV_TXB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slv_txb::W](W) writer structure"]
impl crate::Writable for SLV_TXB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLV_TXB to value 0"]
impl crate::Resettable for SLV_TXB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
