#[doc = "Register `FREE_RUN` reader"]
pub struct R(crate::R<FREE_RUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREE_RUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREE_RUN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREE_RUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREE_RUN` writer"]
pub struct W(crate::W<FREE_RUN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREE_RUN_SPEC>;
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
impl From<crate::W<FREE_RUN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREE_RUN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMR` reader - This register contains the current value of the Free Running Timer."]
pub type TMR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TMR` writer - This register contains the current value of the Free Running Timer."]
pub type TMR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FREE_RUN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register contains the current value of the Free Running Timer."]
    #[inline(always)]
    pub fn tmr(&self) -> TMR_R {
        TMR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register contains the current value of the Free Running Timer."]
    #[inline(always)]
    pub fn tmr(&mut self) -> TMR_W<0> {
        TMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the current value of the Free Running Timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [free_run](index.html) module"]
pub struct FREE_RUN_SPEC;
impl crate::RegisterSpec for FREE_RUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [free_run::R](R) reader structure"]
impl crate::Readable for FREE_RUN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [free_run::W](W) writer structure"]
impl crate::Writable for FREE_RUN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREE_RUN to value 0"]
impl crate::Resettable for FREE_RUN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
