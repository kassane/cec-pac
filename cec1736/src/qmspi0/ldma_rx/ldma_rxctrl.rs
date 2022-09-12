#[doc = "Register `LDMA_RXCTRL` reader"]
pub struct R(crate::R<LDMA_RXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDMA_RXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDMA_RXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDMA_RXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDMA_RXCTRL` writer"]
pub struct W(crate::W<LDMA_RXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDMA_RXCTRL_SPEC>;
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
impl From<crate::W<LDMA_RXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDMA_RXCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_EN` reader - This enables the Local RX DMA Channel."]
pub type CH_EN_R = crate::BitReader<bool>;
#[doc = "Field `CH_EN` writer - This enables the Local RX DMA Channel."]
pub type CH_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDMA_RXCTRL_SPEC, bool, O>;
#[doc = "Field `RSTRT_EN` reader - This bit automatically re-enables the Local DMA Channel after completion of previous transfer."]
pub type RSTRT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RSTRT_EN` writer - This bit automatically re-enables the Local DMA Channel after completion of previous transfer."]
pub type RSTRT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDMA_RXCTRL_SPEC, bool, O>;
#[doc = "Field `BUF_ADDR_EN` reader - Address re-enable will automatically re-enables the same address upon completion previous transfer."]
pub type BUF_ADDR_EN_R = crate::BitReader<bool>;
#[doc = "Field `BUF_ADDR_EN` writer - Address re-enable will automatically re-enables the same address upon completion previous transfer."]
pub type BUF_ADDR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDMA_RXCTRL_SPEC, bool, O>;
#[doc = "Field `OVRD_LEN` reader - Override Lenght will override the length field to the QMSPI protocol FSM with the length programmed into the Local DMA."]
pub type OVRD_LEN_R = crate::BitReader<bool>;
#[doc = "Field `OVRD_LEN` writer - Override Lenght will override the length field to the QMSPI protocol FSM with the length programmed into the Local DMA."]
pub type OVRD_LEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDMA_RXCTRL_SPEC, bool, O>;
#[doc = "Field `ACS_SZ` reader - Access Size selects the AHB Access Size."]
pub type ACS_SZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACS_SZ` writer - Access Size selects the AHB Access Size."]
pub type ACS_SZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDMA_RXCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `INC_ADDR_EN` reader - When set, the DMA Channels Start Address will increment on every access. If not set the address will not increment."]
pub type INC_ADDR_EN_R = crate::BitReader<bool>;
#[doc = "Field `INC_ADDR_EN` writer - When set, the DMA Channels Start Address will increment on every access. If not set the address will not increment."]
pub type INC_ADDR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDMA_RXCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This enables the Local RX DMA Channel."]
    #[inline(always)]
    pub fn ch_en(&self) -> CH_EN_R {
        CH_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit automatically re-enables the Local DMA Channel after completion of previous transfer."]
    #[inline(always)]
    pub fn rstrt_en(&self) -> RSTRT_EN_R {
        RSTRT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Address re-enable will automatically re-enables the same address upon completion previous transfer."]
    #[inline(always)]
    pub fn buf_addr_en(&self) -> BUF_ADDR_EN_R {
        BUF_ADDR_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Override Lenght will override the length field to the QMSPI protocol FSM with the length programmed into the Local DMA."]
    #[inline(always)]
    pub fn ovrd_len(&self) -> OVRD_LEN_R {
        OVRD_LEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Access Size selects the AHB Access Size."]
    #[inline(always)]
    pub fn acs_sz(&self) -> ACS_SZ_R {
        ACS_SZ_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - When set, the DMA Channels Start Address will increment on every access. If not set the address will not increment."]
    #[inline(always)]
    pub fn inc_addr_en(&self) -> INC_ADDR_EN_R {
        INC_ADDR_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This enables the Local RX DMA Channel."]
    #[inline(always)]
    pub fn ch_en(&mut self) -> CH_EN_W<0> {
        CH_EN_W::new(self)
    }
    #[doc = "Bit 1 - This bit automatically re-enables the Local DMA Channel after completion of previous transfer."]
    #[inline(always)]
    pub fn rstrt_en(&mut self) -> RSTRT_EN_W<1> {
        RSTRT_EN_W::new(self)
    }
    #[doc = "Bit 2 - Address re-enable will automatically re-enables the same address upon completion previous transfer."]
    #[inline(always)]
    pub fn buf_addr_en(&mut self) -> BUF_ADDR_EN_W<2> {
        BUF_ADDR_EN_W::new(self)
    }
    #[doc = "Bit 3 - Override Lenght will override the length field to the QMSPI protocol FSM with the length programmed into the Local DMA."]
    #[inline(always)]
    pub fn ovrd_len(&mut self) -> OVRD_LEN_W<3> {
        OVRD_LEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Access Size selects the AHB Access Size."]
    #[inline(always)]
    pub fn acs_sz(&mut self) -> ACS_SZ_W<4> {
        ACS_SZ_W::new(self)
    }
    #[doc = "Bit 6 - When set, the DMA Channels Start Address will increment on every access. If not set the address will not increment."]
    #[inline(always)]
    pub fn inc_addr_en(&mut self) -> INC_ADDR_EN_W<6> {
        INC_ADDR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI RX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldma_rxctrl](index.html) module"]
pub struct LDMA_RXCTRL_SPEC;
impl crate::RegisterSpec for LDMA_RXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldma_rxctrl::R](R) reader structure"]
impl crate::Readable for LDMA_RXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldma_rxctrl::W](W) writer structure"]
impl crate::Writable for LDMA_RXCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDMA_RXCTRL to value 0"]
impl crate::Resettable for LDMA_RXCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
