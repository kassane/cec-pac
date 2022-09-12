#[doc = "Register `CAP2` reader"]
pub struct R(crate::R<CAP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP2` writer"]
pub struct W(crate::W<CAP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP2_SPEC>;
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
impl From<crate::W<CAP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_2` reader - This register saves the value copied from the Free Running timer on a programmed edge of ICT2."]
pub type CAP_2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CAP_2` writer - This register saves the value copied from the Free Running timer on a programmed edge of ICT2."]
pub type CAP_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register saves the value copied from the Free Running timer on a programmed edge of ICT2."]
    #[inline(always)]
    pub fn cap_2(&self) -> CAP_2_R {
        CAP_2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register saves the value copied from the Free Running timer on a programmed edge of ICT2."]
    #[inline(always)]
    pub fn cap_2(&mut self) -> CAP_2_W<0> {
        CAP_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap2](index.html) module"]
pub struct CAP2_SPEC;
impl crate::RegisterSpec for CAP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap2::R](R) reader structure"]
impl crate::Readable for CAP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap2::W](W) writer structure"]
impl crate::Writable for CAP2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP2 to value 0"]
impl crate::Resettable for CAP2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
