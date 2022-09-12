#[doc = "Register `WAKE_STS` reader"]
pub struct R(crate::R<WAKE_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKE_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKE_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKE_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKE_STS` writer"]
pub struct W(crate::W<WAKE_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKE_STS_SPEC>;
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
impl From<crate::W<WAKE_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKE_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_BIT_DET` reader - This bit is set to '1' when a START bit is detected while the controller is enabled. This bit is cleared to '0' when\n written with a '1'. Writes of '0' have no effect. (R/WC)"]
pub type START_BIT_DET_R = crate::BitReader<bool>;
#[doc = "Field `START_BIT_DET` writer - This bit is set to '1' when a START bit is detected while the controller is enabled. This bit is cleared to '0' when\n written with a '1'. Writes of '0' have no effect. (R/WC)"]
pub type START_BIT_DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKE_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is set to '1' when a START bit is detected while the controller is enabled. This bit is cleared to '0' when\n written with a '1'. Writes of '0' have no effect. (R/WC)"]
    #[inline(always)]
    pub fn start_bit_det(&self) -> START_BIT_DET_R {
        START_BIT_DET_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set to '1' when a START bit is detected while the controller is enabled. This bit is cleared to '0' when\n written with a '1'. Writes of '0' have no effect. (R/WC)"]
    #[inline(always)]
    pub fn start_bit_det(&mut self) -> START_BIT_DET_W<0> {
        START_BIT_DET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WAKE STATUS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake_sts](index.html) module"]
pub struct WAKE_STS_SPEC;
impl crate::RegisterSpec for WAKE_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wake_sts::R](R) reader structure"]
impl crate::Readable for WAKE_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wake_sts::W](W) writer structure"]
impl crate::Writable for WAKE_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKE_STS to value 0"]
impl crate::Resettable for WAKE_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
