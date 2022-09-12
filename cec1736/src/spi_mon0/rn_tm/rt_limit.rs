#[doc = "Register `RT_LIMIT` reader"]
pub struct R(crate::R<RT_LIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RT_LIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RT_LIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RT_LIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RT_LIMIT` writer"]
pub struct W(crate::W<RT_LIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RT_LIMIT_SPEC>;
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
impl From<crate::W<RT_LIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RT_LIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LMT` reader - Limit Register"]
pub type LMT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LMT` writer - Limit Register"]
pub type LMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RT_LIMIT_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - Limit Register"]
    #[inline(always)]
    pub fn lmt(&self) -> LMT_R {
        LMT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Limit Register"]
    #[inline(always)]
    pub fn lmt(&mut self) -> LMT_W<0> {
        LMT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Runtime Monitoring Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rt_limit](index.html) module"]
pub struct RT_LIMIT_SPEC;
impl crate::RegisterSpec for RT_LIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rt_limit::R](R) reader structure"]
impl crate::Readable for RT_LIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rt_limit::W](W) writer structure"]
impl crate::Writable for RT_LIMIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RT_LIMIT to value 0"]
impl crate::Resettable for RT_LIMIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
