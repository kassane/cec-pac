#[doc = "Register `TAPS` reader"]
pub struct R(crate::R<TAPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAPS` writer"]
pub struct W(crate::W<TAPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAPS_SPEC>;
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
impl From<crate::W<TAPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCK_TAP` reader - This will select the tap point for the feed-back SCK."]
pub type SCK_TAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCK_TAP` writer - This will select the tap point for the feed-back SCK."]
pub type SCK_TAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAPS_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTRL_TAP` reader - This will select the tap point for signals that go from the System Domain."]
pub type CTRL_TAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTRL_TAP` writer - This will select the tap point for signals that go from the System Domain."]
pub type CTRL_TAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAPS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This will select the tap point for the feed-back SCK."]
    #[inline(always)]
    pub fn sck_tap(&self) -> SCK_TAP_R {
        SCK_TAP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This will select the tap point for signals that go from the System Domain."]
    #[inline(always)]
    pub fn ctrl_tap(&self) -> CTRL_TAP_R {
        CTRL_TAP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This will select the tap point for the feed-back SCK."]
    #[inline(always)]
    pub fn sck_tap(&mut self) -> SCK_TAP_W<0> {
        SCK_TAP_W::new(self)
    }
    #[doc = "Bits 8:15 - This will select the tap point for signals that go from the System Domain."]
    #[inline(always)]
    pub fn ctrl_tap(&mut self) -> CTRL_TAP_W<8> {
        CTRL_TAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI TAPs Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taps](index.html) module"]
pub struct TAPS_SPEC;
impl crate::RegisterSpec for TAPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taps::R](R) reader structure"]
impl crate::Readable for TAPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taps::W](W) writer structure"]
impl crate::Writable for TAPS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAPS to value 0"]
impl crate::Resettable for TAPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
