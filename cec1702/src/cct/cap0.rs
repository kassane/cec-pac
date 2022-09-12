#[doc = "Register `CAP0` reader"]
pub struct R(crate::R<CAP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP0` writer"]
pub struct W(crate::W<CAP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP0_SPEC>;
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
impl From<crate::W<CAP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_0` reader - This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
pub type CAP_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CAP_0` writer - This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
pub type CAP_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
    #[inline(always)]
    pub fn cap_0(&self) -> CAP_0_R {
        CAP_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register saves the value copied from the Free Running timer on a programmed edge of ICT0."]
    #[inline(always)]
    pub fn cap_0(&mut self) -> CAP_0_W<0> {
        CAP_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register saves the value copied from the Free Running timer on a programmed edge of ICT0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap0](index.html) module"]
pub struct CAP0_SPEC;
impl crate::RegisterSpec for CAP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap0::R](R) reader structure"]
impl crate::Readable for CAP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap0::W](W) writer structure"]
impl crate::Writable for CAP0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP0 to value 0"]
impl crate::Resettable for CAP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
