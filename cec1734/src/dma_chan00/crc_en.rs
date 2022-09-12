#[doc = "Register `CRC_EN` reader"]
pub struct R(crate::R<CRC_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_EN` writer"]
pub struct W(crate::W<CRC_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_EN_SPEC>;
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
impl From<crate::W<CRC_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - 1=Enable the calculation of CRC-32 for DMA Channel N 0=Disable the calculation of CRC-32 for DMA Channel N"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - 1=Enable the calculation of CRC-32 for DMA Channel N 0=Disable the calculation of CRC-32 for DMA Channel N"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRC_EN_SPEC, bool, O>;
#[doc = "Field `POST_TRANS` reader - The bit enables the transfer of the calculated CRC-32 after the completion of the DMA transaction. If the DMA transaction is aborted by either firmware or an internal bus error, the transfer will not occur. If the target of the DMA transfer is a device and the device signaled the termination of the DMA transaction, the CRC post transfer will not occur. 1=Enable the transfer of CRC-32 for DMA Channel N after the DMA transaction completes 0=Disable the automatic transfer of the CRC"]
pub type POST_TRANS_R = crate::BitReader<bool>;
#[doc = "Field `POST_TRANS` writer - The bit enables the transfer of the calculated CRC-32 after the completion of the DMA transaction. If the DMA transaction is aborted by either firmware or an internal bus error, the transfer will not occur. If the target of the DMA transfer is a device and the device signaled the termination of the DMA transaction, the CRC post transfer will not occur. 1=Enable the transfer of CRC-32 for DMA Channel N after the DMA transaction completes 0=Disable the automatic transfer of the CRC"]
pub type POST_TRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRC_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 1=Enable the calculation of CRC-32 for DMA Channel N 0=Disable the calculation of CRC-32 for DMA Channel N"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit enables the transfer of the calculated CRC-32 after the completion of the DMA transaction. If the DMA transaction is aborted by either firmware or an internal bus error, the transfer will not occur. If the target of the DMA transfer is a device and the device signaled the termination of the DMA transaction, the CRC post transfer will not occur. 1=Enable the transfer of CRC-32 for DMA Channel N after the DMA transaction completes 0=Disable the automatic transfer of the CRC"]
    #[inline(always)]
    pub fn post_trans(&self) -> POST_TRANS_R {
        POST_TRANS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1=Enable the calculation of CRC-32 for DMA Channel N 0=Disable the calculation of CRC-32 for DMA Channel N"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 1 - The bit enables the transfer of the calculated CRC-32 after the completion of the DMA transaction. If the DMA transaction is aborted by either firmware or an internal bus error, the transfer will not occur. If the target of the DMA transfer is a device and the device signaled the termination of the DMA transaction, the CRC post transfer will not occur. 1=Enable the transfer of CRC-32 for DMA Channel N after the DMA transaction completes 0=Disable the automatic transfer of the CRC"]
    #[inline(always)]
    pub fn post_trans(&mut self) -> POST_TRANS_W<1> {
        POST_TRANS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA CHANNEL N CRC ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_en](index.html) module"]
pub struct CRC_EN_SPEC;
impl crate::RegisterSpec for CRC_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_en::R](R) reader structure"]
impl crate::Readable for CRC_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_en::W](W) writer structure"]
impl crate::Writable for CRC_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC_EN to value 0"]
impl crate::Resettable for CRC_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
