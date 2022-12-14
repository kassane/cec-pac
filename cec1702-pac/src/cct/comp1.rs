#[doc = "Register `COMP1` reader"]
pub struct R(crate::R<COMP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP1` writer"]
pub struct W(crate::W<COMP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP1_SPEC>;
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
impl From<crate::W<COMP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP_1` reader - A COMPARE 1 interrupt is generated when this register matches the value in the Free Running Timer."]
pub type COMP_1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COMP_1` writer - A COMPARE 1 interrupt is generated when this register matches the value in the Free Running Timer."]
pub type COMP_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - A COMPARE 1 interrupt is generated when this register matches the value in the Free Running Timer."]
    #[inline(always)]
    pub fn comp_1(&self) -> COMP_1_R {
        COMP_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A COMPARE 1 interrupt is generated when this register matches the value in the Free Running Timer."]
    #[inline(always)]
    pub fn comp_1(&mut self) -> COMP_1_W<0> {
        COMP_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A COMPARE 1 interrupt is generated when this register matches the value in the Free Running Timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1](index.html) module"]
pub struct COMP1_SPEC;
impl crate::RegisterSpec for COMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp1::R](R) reader structure"]
impl crate::Readable for COMP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp1::W](W) writer structure"]
impl crate::Writable for COMP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP1 to value 0"]
impl crate::Resettable for COMP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
