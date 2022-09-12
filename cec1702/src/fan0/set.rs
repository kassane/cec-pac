#[doc = "Register `SET` reader"]
pub struct R(crate::R<SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SET` writer"]
pub struct W(crate::W<SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_SPEC>;
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
impl From<crate::W<SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAN_SETTING` reader - The Fan Driver Setting used to control the output of the Fan Driver."]
pub type FAN_SETTING_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FAN_SETTING` writer - The Fan Driver Setting used to control the output of the Fan Driver."]
pub type FAN_SETTING_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SET_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 6:15 - The Fan Driver Setting used to control the output of the Fan Driver."]
    #[inline(always)]
    pub fn fan_setting(&self) -> FAN_SETTING_R {
        FAN_SETTING_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:15 - The Fan Driver Setting used to control the output of the Fan Driver."]
    #[inline(always)]
    pub fn fan_setting(&mut self) -> FAN_SETTING_W<6> {
        FAN_SETTING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Fan Driver Setting used to control the output of the Fan Driver.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set](index.html) module"]
pub struct SET_SPEC;
impl crate::RegisterSpec for SET_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [set::R](R) reader structure"]
impl crate::Readable for SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [set::W](W) writer structure"]
impl crate::Writable for SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SET to value 0"]
impl crate::Resettable for SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
