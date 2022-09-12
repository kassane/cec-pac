#[doc = "Register `ISTS` reader"]
pub struct R(crate::R<ISTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISTS` writer"]
pub struct W(crate::W<ISTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISTS_SPEC>;
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
impl From<crate::W<ISTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUS_ERROR` reader - This is an interrupt source register. This flags when there is an Error detected over the internal 32-bit Bus.\n 1: Error detected. (R/WC)"]
pub type BUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `BUS_ERROR` writer - This is an interrupt source register. This flags when there is an Error detected over the internal 32-bit Bus.\n 1: Error detected. (R/WC)"]
pub type BUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u8, ISTS_SPEC, bool, O>;
#[doc = "Field `FLOW_CTRL` reader - This is an interrupt source register. This flags when the DMA Channel has encountered a Hardware Flow Control Request\n after the DMA Channel has completed the transfer. This means the Master Device is attempting to overflow the DMA.\n 1=Hardware Flow Control is requesting after the transfer has completed\n 0=No Hardware Flow Control event"]
pub type FLOW_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `FLOW_CTRL` writer - This is an interrupt source register. This flags when the DMA Channel has encountered a Hardware Flow Control Request\n after the DMA Channel has completed the transfer. This means the Master Device is attempting to overflow the DMA.\n 1=Hardware Flow Control is requesting after the transfer has completed\n 0=No Hardware Flow Control event"]
pub type FLOW_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u8, ISTS_SPEC, bool, O>;
#[doc = "Field `DONE` reader - This is an interrupt source register. This flags when the DMA Channel has completed a transfer successfully on its side.\n A completed transfer is defined as when the DMA Channel reaches its limit; Memory Start Address equals Memory End Address.\n A completion due to a Hardware Flow Control Terminate will not flag this interrupt.\n 1=Memory Start Address equals Memory End Address\n 0=Memory Start Address does not equal Memory End Address"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - This is an interrupt source register. This flags when the DMA Channel has completed a transfer successfully on its side.\n A completed transfer is defined as when the DMA Channel reaches its limit; Memory Start Address equals Memory End Address.\n A completion due to a Hardware Flow Control Terminate will not flag this interrupt.\n 1=Memory Start Address equals Memory End Address\n 0=Memory Start Address does not equal Memory End Address"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ISTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This is an interrupt source register. This flags when there is an Error detected over the internal 32-bit Bus.\n 1: Error detected. (R/WC)"]
    #[inline(always)]
    pub fn bus_error(&self) -> BUS_ERROR_R {
        BUS_ERROR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is an interrupt source register. This flags when the DMA Channel has encountered a Hardware Flow Control Request\n after the DMA Channel has completed the transfer. This means the Master Device is attempting to overflow the DMA.\n 1=Hardware Flow Control is requesting after the transfer has completed\n 0=No Hardware Flow Control event"]
    #[inline(always)]
    pub fn flow_ctrl(&self) -> FLOW_CTRL_R {
        FLOW_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is an interrupt source register. This flags when the DMA Channel has completed a transfer successfully on its side.\n A completed transfer is defined as when the DMA Channel reaches its limit; Memory Start Address equals Memory End Address.\n A completion due to a Hardware Flow Control Terminate will not flag this interrupt.\n 1=Memory Start Address equals Memory End Address\n 0=Memory Start Address does not equal Memory End Address"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is an interrupt source register. This flags when there is an Error detected over the internal 32-bit Bus.\n 1: Error detected. (R/WC)"]
    #[inline(always)]
    pub fn bus_error(&mut self) -> BUS_ERROR_W<0> {
        BUS_ERROR_W::new(self)
    }
    #[doc = "Bit 1 - This is an interrupt source register. This flags when the DMA Channel has encountered a Hardware Flow Control Request\n after the DMA Channel has completed the transfer. This means the Master Device is attempting to overflow the DMA.\n 1=Hardware Flow Control is requesting after the transfer has completed\n 0=No Hardware Flow Control event"]
    #[inline(always)]
    pub fn flow_ctrl(&mut self) -> FLOW_CTRL_W<1> {
        FLOW_CTRL_W::new(self)
    }
    #[doc = "Bit 2 - This is an interrupt source register. This flags when the DMA Channel has completed a transfer successfully on its side.\n A completed transfer is defined as when the DMA Channel reaches its limit; Memory Start Address equals Memory End Address.\n A completion due to a Hardware Flow Control Terminate will not flag this interrupt.\n 1=Memory Start Address equals Memory End Address\n 0=Memory Start Address does not equal Memory End Address"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<2> {
        DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel N Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ists](index.html) module"]
pub struct ISTS_SPEC;
impl crate::RegisterSpec for ISTS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ists::R](R) reader structure"]
impl crate::Readable for ISTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ists::W](W) writer structure"]
impl crate::Writable for ISTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISTS to value 0"]
impl crate::Resettable for ISTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
