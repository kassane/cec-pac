#[doc = "Register `TAP_ADJ` reader"]
pub struct R(crate::R<TAP_ADJ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAP_ADJ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAP_ADJ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAP_ADJ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAP_ADJ` writer"]
pub struct W(crate::W<TAP_ADJ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAP_ADJ_SPEC>;
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
impl From<crate::W<TAP_ADJ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAP_ADJ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCK_ADJ` reader - This is a signed value that will be added to the Select SCK Tap to come up with the final value for the delay."]
pub type SCK_ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCK_ADJ` writer - This is a signed value that will be added to the Select SCK Tap to come up with the final value for the delay."]
pub type SCK_ADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAP_ADJ_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTRL_ADJ` reader - This is a signed value that will be added to the Select Control Tap to come up with the final value for the delay."]
pub type CTRL_ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTRL_ADJ` writer - This is a signed value that will be added to the Select Control Tap to come up with the final value for the delay."]
pub type CTRL_ADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAP_ADJ_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This is a signed value that will be added to the Select SCK Tap to come up with the final value for the delay."]
    #[inline(always)]
    pub fn sck_adj(&self) -> SCK_ADJ_R {
        SCK_ADJ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This is a signed value that will be added to the Select Control Tap to come up with the final value for the delay."]
    #[inline(always)]
    pub fn ctrl_adj(&self) -> CTRL_ADJ_R {
        CTRL_ADJ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This is a signed value that will be added to the Select SCK Tap to come up with the final value for the delay."]
    #[inline(always)]
    pub fn sck_adj(&mut self) -> SCK_ADJ_W<0> {
        SCK_ADJ_W::new(self)
    }
    #[doc = "Bits 8:15 - This is a signed value that will be added to the Select Control Tap to come up with the final value for the delay."]
    #[inline(always)]
    pub fn ctrl_adj(&mut self) -> CTRL_ADJ_W<8> {
        CTRL_ADJ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI TAP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tap_adj](index.html) module"]
pub struct TAP_ADJ_SPEC;
impl crate::RegisterSpec for TAP_ADJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tap_adj::R](R) reader structure"]
impl crate::Readable for TAP_ADJ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tap_adj::W](W) writer structure"]
impl crate::Writable for TAP_ADJ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAP_ADJ to value 0"]
impl crate::Resettable for TAP_ADJ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
