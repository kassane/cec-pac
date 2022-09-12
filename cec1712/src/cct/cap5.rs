#[doc = "Register `CAP5` reader"]
pub struct R(crate::R<CAP5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP5` writer"]
pub struct W(crate::W<CAP5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP5_SPEC>;
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
impl From<crate::W<CAP5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_5` reader - This register saves the value copied from the Free Running timer on a programmed edge of ICT5."]
pub type CAP_5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CAP_5` writer - This register saves the value copied from the Free Running timer on a programmed edge of ICT5."]
pub type CAP_5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP5_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register saves the value copied from the Free Running timer on a programmed edge of ICT5."]
    #[inline(always)]
    pub fn cap_5(&self) -> CAP_5_R {
        CAP_5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register saves the value copied from the Free Running timer on a programmed edge of ICT5."]
    #[inline(always)]
    pub fn cap_5(&mut self) -> CAP_5_W<0> {
        CAP_5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap5](index.html) module"]
pub struct CAP5_SPEC;
impl crate::RegisterSpec for CAP5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap5::R](R) reader structure"]
impl crate::Readable for CAP5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap5::W](W) writer structure"]
impl crate::Writable for CAP5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP5 to value 0"]
impl crate::Resettable for CAP5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
