#[doc = "Register `EXE` reader"]
pub struct R(crate::R<EXE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXE` writer"]
pub struct W(crate::W<EXE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXE_SPEC>;
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
impl From<crate::W<EXE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Writing a 1 to this bit will start the SPI transfer. Writing a 0 to this bit has no effect. This bit is self-clearing. This bit must not be set to 1 if the field STOP in this register is set to 1."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Writing a 1 to this bit will start the SPI transfer. Writing a 0 to this bit has no effect. This bit is self-clearing. This bit must not be set to 1 if the field STOP in this register is set to 1."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXE_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Writing a 1 to this bit will stop any transfer in progress at the next byte boundary. Writing a 0 to this bit has no effect. This bit is self clearing. This bit must not be set to 1 if the field START in this register is set to 1."]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Writing a 1 to this bit will stop any transfer in progress at the next byte boundary. Writing a 0 to this bit has no effect. This bit is self clearing. This bit must not be set to 1 if the field START in this register is set to 1."]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXE_SPEC, bool, O>;
#[doc = "Field `CLR_DAT_BUFF` reader - Writing a 1 to this bit will clear out the Transmit and Receive FIFOs. Any data stored in the FIFOs is discarded and all count fields are reset. Writing a 0 to this bit has no effect. This bit is self clearing."]
pub type CLR_DAT_BUFF_R = crate::BitReader<bool>;
#[doc = "Field `CLR_DAT_BUFF` writer - Writing a 1 to this bit will clear out the Transmit and Receive FIFOs. Any data stored in the FIFOs is discarded and all count fields are reset. Writing a 0 to this bit has no effect. This bit is self clearing."]
pub type CLR_DAT_BUFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Writing a 1 to this bit will start the SPI transfer. Writing a 0 to this bit has no effect. This bit is self-clearing. This bit must not be set to 1 if the field STOP in this register is set to 1."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Writing a 1 to this bit will stop any transfer in progress at the next byte boundary. Writing a 0 to this bit has no effect. This bit is self clearing. This bit must not be set to 1 if the field START in this register is set to 1."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Writing a 1 to this bit will clear out the Transmit and Receive FIFOs. Any data stored in the FIFOs is discarded and all count fields are reset. Writing a 0 to this bit has no effect. This bit is self clearing."]
    #[inline(always)]
    pub fn clr_dat_buff(&self) -> CLR_DAT_BUFF_R {
        CLR_DAT_BUFF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 to this bit will start the SPI transfer. Writing a 0 to this bit has no effect. This bit is self-clearing. This bit must not be set to 1 if the field STOP in this register is set to 1."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Writing a 1 to this bit will stop any transfer in progress at the next byte boundary. Writing a 0 to this bit has no effect. This bit is self clearing. This bit must not be set to 1 if the field START in this register is set to 1."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<1> {
        STOP_W::new(self)
    }
    #[doc = "Bit 2 - Writing a 1 to this bit will clear out the Transmit and Receive FIFOs. Any data stored in the FIFOs is discarded and all count fields are reset. Writing a 0 to this bit has no effect. This bit is self clearing."]
    #[inline(always)]
    pub fn clr_dat_buff(&mut self) -> CLR_DAT_BUFF_W<2> {
        CLR_DAT_BUFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Execute Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exe](index.html) module"]
pub struct EXE_SPEC;
impl crate::RegisterSpec for EXE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exe::R](R) reader structure"]
impl crate::Readable for EXE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exe::W](W) writer structure"]
impl crate::Writable for EXE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXE to value 0"]
impl crate::Resettable for EXE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
