#[doc = "Register `FILL_STS` reader"]
pub struct R(crate::R<FILL_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILL_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILL_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILL_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILL_STS` writer"]
pub struct W(crate::W<FILL_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILL_STS_SPEC>;
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
impl From<crate::W<FILL_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILL_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - This bit is set to '1b' when the CRC calculation has completed from either normal or forced termination.\n It is cleared to '0b' when the DMA controller starts a new transfer on the channel."]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - This bit is set to '1b' when the CRC calculation has completed from either normal or forced termination.\n It is cleared to '0b' when the DMA controller starts a new transfer on the channel."]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FILL_STS_SPEC, bool, O>;
#[doc = "Field `RUNNING` reader - This bit is set to '1b' when the DMA controller starts the post-transfer transmission of the CRC.\n It is only set when the post-transfer is enabled by the CRC_POST_TRANSFER_ENABLE field. This bit is cleared\n to '0b' when the post-transfer completes."]
pub type RUNNING_R = crate::BitReader<bool>;
#[doc = "Field `RUNNING` writer - This bit is set to '1b' when the DMA controller starts the post-transfer transmission of the CRC.\n It is only set when the post-transfer is enabled by the CRC_POST_TRANSFER_ENABLE field. This bit is cleared\n to '0b' when the post-transfer completes."]
pub type RUNNING_W<'a, const O: u8> = crate::BitWriter<'a, u32, FILL_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is set to '1b' when the CRC calculation has completed from either normal or forced termination.\n It is cleared to '0b' when the DMA controller starts a new transfer on the channel."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set to '1b' when the DMA controller starts the post-transfer transmission of the CRC.\n It is only set when the post-transfer is enabled by the CRC_POST_TRANSFER_ENABLE field. This bit is cleared\n to '0b' when the post-transfer completes."]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set to '1b' when the CRC calculation has completed from either normal or forced termination.\n It is cleared to '0b' when the DMA controller starts a new transfer on the channel."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<0> {
        DONE_W::new(self)
    }
    #[doc = "Bit 1 - This bit is set to '1b' when the DMA controller starts the post-transfer transmission of the CRC.\n It is only set when the post-transfer is enabled by the CRC_POST_TRANSFER_ENABLE field. This bit is cleared\n to '0b' when the post-transfer completes."]
    #[inline(always)]
    pub fn running(&mut self) -> RUNNING_W<1> {
        RUNNING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA CHANNEL N FILL STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fill_sts](index.html) module"]
pub struct FILL_STS_SPEC;
impl crate::RegisterSpec for FILL_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fill_sts::R](R) reader structure"]
impl crate::Readable for FILL_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fill_sts::W](W) writer structure"]
impl crate::Writable for FILL_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILL_STS to value 0"]
impl crate::Resettable for FILL_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
