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
#[doc = "Field `STS_EN_BUS_ERR` reader - This is an interrupt enable for DMA Channel Interrupt:Status Bus Error.\n 1=Enable Interrupt\n 0=Disable Interrupt"]
pub type STS_EN_BUS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `STS_EN_BUS_ERR` writer - This is an interrupt enable for DMA Channel Interrupt:Status Bus Error.\n 1=Enable Interrupt\n 0=Disable Interrupt"]
pub type STS_EN_BUS_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, IEN_SPEC, bool, O>;
#[doc = "Field `STS_EN_FLOW_CTRL` reader - This is an interrupt enable for DMA Channel Interrupt:Status Flow Control Error.\n 1=Enable Interrupt\n 0=Disable Interrupt"]
pub type STS_EN_FLOW_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `STS_EN_FLOW_CTRL` writer - This is an interrupt enable for DMA Channel Interrupt:Status Flow Control Error.\n 1=Enable Interrupt\n 0=Disable Interrupt"]
pub type STS_EN_FLOW_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u8, IEN_SPEC, bool, O>;
#[doc = "Field `STS_EN_DONE` reader - This is an interrupt enable for DMA Channel Interrupt:Status Done.\n 1=Enable Interrupt\n 0=Disable Interrupt"]
pub type STS_EN_DONE_R = crate::BitReader<bool>;
#[doc = "Field `STS_EN_DONE` writer - This is an interrupt enable for DMA Channel Interrupt:Status Done.\n 1=Enable Interrupt\n 0=Disable Interrupt"]
pub type STS_EN_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This is an interrupt enable for DMA Channel Interrupt:Status Bus Error.\n 1=Enable Interrupt\n 0=Disable Interrupt"]
    #[inline(always)]
    pub fn sts_en_bus_err(&self) -> STS_EN_BUS_ERR_R {
        STS_EN_BUS_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is an interrupt enable for DMA Channel Interrupt:Status Flow Control Error.\n 1=Enable Interrupt\n 0=Disable Interrupt"]
    #[inline(always)]
    pub fn sts_en_flow_ctrl(&self) -> STS_EN_FLOW_CTRL_R {
        STS_EN_FLOW_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is an interrupt enable for DMA Channel Interrupt:Status Done.\n 1=Enable Interrupt\n 0=Disable Interrupt"]
    #[inline(always)]
    pub fn sts_en_done(&self) -> STS_EN_DONE_R {
        STS_EN_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is an interrupt enable for DMA Channel Interrupt:Status Bus Error.\n 1=Enable Interrupt\n 0=Disable Interrupt"]
    #[inline(always)]
    pub fn sts_en_bus_err(&mut self) -> STS_EN_BUS_ERR_W<0> {
        STS_EN_BUS_ERR_W::new(self)
    }
    #[doc = "Bit 1 - This is an interrupt enable for DMA Channel Interrupt:Status Flow Control Error.\n 1=Enable Interrupt\n 0=Disable Interrupt"]
    #[inline(always)]
    pub fn sts_en_flow_ctrl(&mut self) -> STS_EN_FLOW_CTRL_W<1> {
        STS_EN_FLOW_CTRL_W::new(self)
    }
    #[doc = "Bit 2 - This is an interrupt enable for DMA Channel Interrupt:Status Done.\n 1=Enable Interrupt\n 0=Disable Interrupt"]
    #[inline(always)]
    pub fn sts_en_done(&mut self) -> STS_EN_DONE_W<2> {
        STS_EN_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA CHANNEL N INTERRUPT ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
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
