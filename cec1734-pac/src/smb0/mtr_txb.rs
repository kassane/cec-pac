#[doc = "Register `MTR_TXB` reader"]
pub struct R(crate::R<MTR_TXB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTR_TXB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTR_TXB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTR_TXB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTR_TXB` writer"]
pub struct W(crate::W<MTR_TXB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTR_TXB_SPEC>;
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
impl From<crate::W<MTR_TXB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTR_TXB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTXB` reader - MASTER_TRANSMIT_BUFFER"]
pub type MTXB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MTXB` writer - MASTER_TRANSMIT_BUFFER"]
pub type MTXB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTR_TXB_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - MASTER_TRANSMIT_BUFFER"]
    #[inline(always)]
    pub fn mtxb(&self) -> MTXB_R {
        MTXB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MASTER_TRANSMIT_BUFFER"]
    #[inline(always)]
    pub fn mtxb(&mut self) -> MTXB_W<0> {
        MTXB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMBus Master Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtr_txb](index.html) module"]
pub struct MTR_TXB_SPEC;
impl crate::RegisterSpec for MTR_TXB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtr_txb::R](R) reader structure"]
impl crate::Readable for MTR_TXB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtr_txb::W](W) writer structure"]
impl crate::Writable for MTR_TXB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTR_TXB to value 0"]
impl crate::Resettable for MTR_TXB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
