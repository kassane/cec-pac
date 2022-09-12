#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERDAI` reader - ERDAI This bit enables the Received Data Available Interrupt (and timeout interrupts in the FIFO mode) when set to logic '1'."]
pub type ERDAI_R = crate::BitReader<bool>;
#[doc = "Field `ERDAI` writer - ERDAI This bit enables the Received Data Available Interrupt (and timeout interrupts in the FIFO mode) when set to logic '1'."]
pub type ERDAI_W<'a, const O: u8> = crate::BitWriter<'a, u8, IEN_SPEC, bool, O>;
#[doc = "Field `ETHREI` reader - ETHREI This bit enables the Transmitter Holding Register Empty Interrupt when set to logic '1'."]
pub type ETHREI_R = crate::BitReader<bool>;
#[doc = "Field `ETHREI` writer - ETHREI This bit enables the Transmitter Holding Register Empty Interrupt when set to logic '1'."]
pub type ETHREI_W<'a, const O: u8> = crate::BitWriter<'a, u8, IEN_SPEC, bool, O>;
#[doc = "Field `ELSI` reader - ELSI This bit enables the Received Line Status Interrupt when set to logic '1'."]
pub type ELSI_R = crate::BitReader<bool>;
#[doc = "Field `ELSI` writer - ELSI This bit enables the Received Line Status Interrupt when set to logic '1'."]
pub type ELSI_W<'a, const O: u8> = crate::BitWriter<'a, u8, IEN_SPEC, bool, O>;
#[doc = "Field `EMSI` reader - EMSI This bit enables the MODEM Status Interrupt when set to logic '1'."]
pub type EMSI_R = crate::BitReader<bool>;
#[doc = "Field `EMSI` writer - EMSI This bit enables the MODEM Status Interrupt when set to logic '1'."]
pub type EMSI_W<'a, const O: u8> = crate::BitWriter<'a, u8, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ERDAI This bit enables the Received Data Available Interrupt (and timeout interrupts in the FIFO mode) when set to logic '1'."]
    #[inline(always)]
    pub fn erdai(&self) -> ERDAI_R {
        ERDAI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ETHREI This bit enables the Transmitter Holding Register Empty Interrupt when set to logic '1'."]
    #[inline(always)]
    pub fn ethrei(&self) -> ETHREI_R {
        ETHREI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ELSI This bit enables the Received Line Status Interrupt when set to logic '1'."]
    #[inline(always)]
    pub fn elsi(&self) -> ELSI_R {
        ELSI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EMSI This bit enables the MODEM Status Interrupt when set to logic '1'."]
    #[inline(always)]
    pub fn emsi(&self) -> EMSI_R {
        EMSI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ERDAI This bit enables the Received Data Available Interrupt (and timeout interrupts in the FIFO mode) when set to logic '1'."]
    #[inline(always)]
    pub fn erdai(&mut self) -> ERDAI_W<0> {
        ERDAI_W::new(self)
    }
    #[doc = "Bit 1 - ETHREI This bit enables the Transmitter Holding Register Empty Interrupt when set to logic '1'."]
    #[inline(always)]
    pub fn ethrei(&mut self) -> ETHREI_W<1> {
        ETHREI_W::new(self)
    }
    #[doc = "Bit 2 - ELSI This bit enables the Received Line Status Interrupt when set to logic '1'."]
    #[inline(always)]
    pub fn elsi(&mut self) -> ELSI_W<2> {
        ELSI_W::new(self)
    }
    #[doc = "Bit 3 - EMSI This bit enables the MODEM Status Interrupt when set to logic '1'."]
    #[inline(always)]
    pub fn emsi(&mut self) -> EMSI_W<3> {
        EMSI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Interrupt Enable Register (DLAB=0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
