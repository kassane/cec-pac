#[doc = "Register `CRC_POST_STS` reader"]
pub struct R(crate::R<CRC_POST_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_POST_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_POST_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_POST_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_POST_STS` writer"]
pub struct W(crate::W<CRC_POST_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_POST_STS_SPEC>;
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
impl From<crate::W<CRC_POST_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_POST_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_DONE` reader - This bit is set to '1b' when the CRC calculation has completed from either normal or forced termination. It is cleared to '0b' when the DMA controller starts a new transfer on the channel."]
pub type CRC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `CRC_DONE` writer - This bit is set to '1b' when the CRC calculation has completed from either normal or forced termination. It is cleared to '0b' when the DMA controller starts a new transfer on the channel."]
pub type CRC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRC_POST_STS_SPEC, bool, O>;
#[doc = "Field `CRC_RUNNING` reader - This bit is set to '1b' when the DMA controller starts the post-transfer transmission of the CRC. It is only set when the post-transfer is enabled by the CRC_POST_TRANSFER_ENABLE field. This bit is cleared to '0b' when the post-transfer completes."]
pub type CRC_RUNNING_R = crate::BitReader<bool>;
#[doc = "Field `CRC_RUNNING` writer - This bit is set to '1b' when the DMA controller starts the post-transfer transmission of the CRC. It is only set when the post-transfer is enabled by the CRC_POST_TRANSFER_ENABLE field. This bit is cleared to '0b' when the post-transfer completes."]
pub type CRC_RUNNING_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRC_POST_STS_SPEC, bool, O>;
#[doc = "Field `CRC_DATA_DONE` reader - This bit is set to '1b' when the DMA controller has completed the post-transfer of the CRC data. This bit is cleared to '0b' when the a new DMA transfer starts."]
pub type CRC_DATA_DONE_R = crate::BitReader<bool>;
#[doc = "Field `CRC_DATA_DONE` writer - This bit is set to '1b' when the DMA controller has completed the post-transfer of the CRC data. This bit is cleared to '0b' when the a new DMA transfer starts."]
pub type CRC_DATA_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRC_POST_STS_SPEC, bool, O>;
#[doc = "Field `CRC_DATA_READY` reader - This bit is set to '1b' when the DMA controller is processing the post-transfer of the CRC data. This bit is cleared to '0b' when the post-transfer completes."]
pub type CRC_DATA_READY_R = crate::BitReader<bool>;
#[doc = "Field `CRC_DATA_READY` writer - This bit is set to '1b' when the DMA controller is processing the post-transfer of the CRC data. This bit is cleared to '0b' when the post-transfer completes."]
pub type CRC_DATA_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRC_POST_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is set to '1b' when the CRC calculation has completed from either normal or forced termination. It is cleared to '0b' when the DMA controller starts a new transfer on the channel."]
    #[inline(always)]
    pub fn crc_done(&self) -> CRC_DONE_R {
        CRC_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set to '1b' when the DMA controller starts the post-transfer transmission of the CRC. It is only set when the post-transfer is enabled by the CRC_POST_TRANSFER_ENABLE field. This bit is cleared to '0b' when the post-transfer completes."]
    #[inline(always)]
    pub fn crc_running(&self) -> CRC_RUNNING_R {
        CRC_RUNNING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set to '1b' when the DMA controller has completed the post-transfer of the CRC data. This bit is cleared to '0b' when the a new DMA transfer starts."]
    #[inline(always)]
    pub fn crc_data_done(&self) -> CRC_DATA_DONE_R {
        CRC_DATA_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set to '1b' when the DMA controller is processing the post-transfer of the CRC data. This bit is cleared to '0b' when the post-transfer completes."]
    #[inline(always)]
    pub fn crc_data_ready(&self) -> CRC_DATA_READY_R {
        CRC_DATA_READY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set to '1b' when the CRC calculation has completed from either normal or forced termination. It is cleared to '0b' when the DMA controller starts a new transfer on the channel."]
    #[inline(always)]
    pub fn crc_done(&mut self) -> CRC_DONE_W<0> {
        CRC_DONE_W::new(self)
    }
    #[doc = "Bit 1 - This bit is set to '1b' when the DMA controller starts the post-transfer transmission of the CRC. It is only set when the post-transfer is enabled by the CRC_POST_TRANSFER_ENABLE field. This bit is cleared to '0b' when the post-transfer completes."]
    #[inline(always)]
    pub fn crc_running(&mut self) -> CRC_RUNNING_W<1> {
        CRC_RUNNING_W::new(self)
    }
    #[doc = "Bit 2 - This bit is set to '1b' when the DMA controller has completed the post-transfer of the CRC data. This bit is cleared to '0b' when the a new DMA transfer starts."]
    #[inline(always)]
    pub fn crc_data_done(&mut self) -> CRC_DATA_DONE_W<2> {
        CRC_DATA_DONE_W::new(self)
    }
    #[doc = "Bit 3 - This bit is set to '1b' when the DMA controller is processing the post-transfer of the CRC data. This bit is cleared to '0b' when the post-transfer completes."]
    #[inline(always)]
    pub fn crc_data_ready(&mut self) -> CRC_DATA_READY_W<3> {
        CRC_DATA_READY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA CHANNEL N CRC POST STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_post_sts](index.html) module"]
pub struct CRC_POST_STS_SPEC;
impl crate::RegisterSpec for CRC_POST_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_post_sts::R](R) reader structure"]
impl crate::Readable for CRC_POST_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_post_sts::W](W) writer structure"]
impl crate::Writable for CRC_POST_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC_POST_STS to value 0"]
impl crate::Resettable for CRC_POST_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
