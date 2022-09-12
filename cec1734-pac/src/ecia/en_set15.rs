#[doc = "Register `EN_SET15` reader"]
pub struct R(crate::R<EN_SET15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_SET15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_SET15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_SET15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN_SET15` writer"]
pub struct W(crate::W<EN_SET15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_SET15_SPEC>;
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
impl From<crate::W<EN_SET15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_SET15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART0` reader - UART0"]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `UART0` writer - UART0"]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET15_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UART0"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W<0> {
        UART0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ15 ENABLE SET\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_set15](index.html) module"]
pub struct EN_SET15_SPEC;
impl crate::RegisterSpec for EN_SET15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en_set15::R](R) reader structure"]
impl crate::Readable for EN_SET15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en_set15::W](W) writer structure"]
impl crate::Writable for EN_SET15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN_SET15 to value 0"]
impl crate::Resettable for EN_SET15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
