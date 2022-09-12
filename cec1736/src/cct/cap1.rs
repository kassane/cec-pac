#[doc = "Register `CAP1` reader"]
pub struct R(crate::R<CAP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP1` writer"]
pub struct W(crate::W<CAP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP1_SPEC>;
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
impl From<crate::W<CAP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_1` reader - This register saves the value copied from the Free Running timer on a programmed edge of ICT1."]
pub type CAP_1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CAP_1` writer - This register saves the value copied from the Free Running timer on a programmed edge of ICT1."]
pub type CAP_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register saves the value copied from the Free Running timer on a programmed edge of ICT1."]
    #[inline(always)]
    pub fn cap_1(&self) -> CAP_1_R {
        CAP_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register saves the value copied from the Free Running timer on a programmed edge of ICT1."]
    #[inline(always)]
    pub fn cap_1(&mut self) -> CAP_1_W<0> {
        CAP_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap1](index.html) module"]
pub struct CAP1_SPEC;
impl crate::RegisterSpec for CAP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap1::R](R) reader structure"]
impl crate::Readable for CAP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap1::W](W) writer structure"]
impl crate::Writable for CAP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP1 to value 0"]
impl crate::Resettable for CAP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
