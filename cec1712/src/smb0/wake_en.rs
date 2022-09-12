#[doc = "Register `WAKE_EN` reader"]
pub struct R(crate::R<WAKE_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKE_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKE_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKE_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKE_EN` writer"]
pub struct W(crate::W<WAKE_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKE_EN_SPEC>;
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
impl From<crate::W<WAKE_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKE_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_DET_INT_EN` reader - Enable Start Bit Detection Interrupt. The Start Bit Detection Interrupt is wake-capable.\n 1=Start Bit Detection Interrupt enabled; 0=Start Bit Detection Interrupt disabled"]
pub type START_DET_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `START_DET_INT_EN` writer - Enable Start Bit Detection Interrupt. The Start Bit Detection Interrupt is wake-capable.\n 1=Start Bit Detection Interrupt enabled; 0=Start Bit Detection Interrupt disabled"]
pub type START_DET_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKE_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable Start Bit Detection Interrupt. The Start Bit Detection Interrupt is wake-capable.\n 1=Start Bit Detection Interrupt enabled; 0=Start Bit Detection Interrupt disabled"]
    #[inline(always)]
    pub fn start_det_int_en(&self) -> START_DET_INT_EN_R {
        START_DET_INT_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Start Bit Detection Interrupt. The Start Bit Detection Interrupt is wake-capable.\n 1=Start Bit Detection Interrupt enabled; 0=Start Bit Detection Interrupt disabled"]
    #[inline(always)]
    pub fn start_det_int_en(&mut self) -> START_DET_INT_EN_W<0> {
        START_DET_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WAKE ENABLE Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake_en](index.html) module"]
pub struct WAKE_EN_SPEC;
impl crate::RegisterSpec for WAKE_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wake_en::R](R) reader structure"]
impl crate::Readable for WAKE_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wake_en::W](W) writer structure"]
impl crate::Writable for WAKE_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKE_EN to value 0"]
impl crate::Resettable for WAKE_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
