#[doc = "Register `MAN_MOD_DATA` reader"]
pub struct R(crate::R<MAN_MOD_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAN_MOD_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAN_MOD_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAN_MOD_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAN_MOD_DATA` writer"]
pub struct W(crate::W<MAN_MOD_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAN_MOD_DATA_SPEC>;
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
impl From<crate::W<MAN_MOD_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAN_MOD_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP_DATA` reader - Manual mode data: This field connects to the eFUSE data output pins."]
pub type IP_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IP_DATA` writer - Manual mode data: This field connects to the eFUSE data output pins."]
pub type IP_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAN_MOD_DATA_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Manual mode data: This field connects to the eFUSE data output pins."]
    #[inline(always)]
    pub fn ip_data(&self) -> IP_DATA_R {
        IP_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Manual mode data: This field connects to the eFUSE data output pins."]
    #[inline(always)]
    pub fn ip_data(&mut self) -> IP_DATA_W<0> {
        IP_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MANUAL MODE DATA REGISTER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man_mod_data](index.html) module"]
pub struct MAN_MOD_DATA_SPEC;
impl crate::RegisterSpec for MAN_MOD_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [man_mod_data::R](R) reader structure"]
impl crate::Readable for MAN_MOD_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [man_mod_data::W](W) writer structure"]
impl crate::Writable for MAN_MOD_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAN_MOD_DATA to value 0"]
impl crate::Resettable for MAN_MOD_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
