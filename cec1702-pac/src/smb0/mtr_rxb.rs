#[doc = "Register `MTR_RXB` reader"]
pub struct R(crate::R<MTR_RXB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTR_RXB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTR_RXB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTR_RXB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTR_RXB` writer"]
pub struct W(crate::W<MTR_RXB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTR_RXB_SPEC>;
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
impl From<crate::W<MTR_RXB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTR_RXB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRXB` reader - MRXB"]
pub type MRXB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MRXB` writer - MRXB"]
pub type MRXB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTR_RXB_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - MRXB"]
    #[inline(always)]
    pub fn mrxb(&self) -> MRXB_R {
        MRXB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MRXB"]
    #[inline(always)]
    pub fn mrxb(&mut self) -> MRXB_W<0> {
        MRXB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMBus Master Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtr_rxb](index.html) module"]
pub struct MTR_RXB_SPEC;
impl crate::RegisterSpec for MTR_RXB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtr_rxb::R](R) reader structure"]
impl crate::Readable for MTR_RXB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtr_rxb::W](W) writer structure"]
impl crate::Writable for MTR_RXB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTR_RXB to value 0"]
impl crate::Resettable for MTR_RXB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
