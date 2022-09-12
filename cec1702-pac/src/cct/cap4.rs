#[doc = "Register `CAP4` reader"]
pub struct R(crate::R<CAP4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP4` writer"]
pub struct W(crate::W<CAP4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP4_SPEC>;
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
impl From<crate::W<CAP4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_4` reader - This register saves the value copied from the Free Running timer on a programmed edge of ICT4."]
pub type CAP_4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CAP_4` writer - This register saves the value copied from the Free Running timer on a programmed edge of ICT4."]
pub type CAP_4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register saves the value copied from the Free Running timer on a programmed edge of ICT4."]
    #[inline(always)]
    pub fn cap_4(&self) -> CAP_4_R {
        CAP_4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register saves the value copied from the Free Running timer on a programmed edge of ICT4."]
    #[inline(always)]
    pub fn cap_4(&mut self) -> CAP_4_W<0> {
        CAP_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap4](index.html) module"]
pub struct CAP4_SPEC;
impl crate::RegisterSpec for CAP4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap4::R](R) reader structure"]
impl crate::Readable for CAP4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap4::W](W) writer structure"]
impl crate::Writable for CAP4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP4 to value 0"]
impl crate::Resettable for CAP4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
