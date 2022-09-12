#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRL` reader - 1= The Hibernation Timer has a resolution of 0.125s per LSB, which yields a maximum time in excess of 2 hours.\n 0= The Hibernation Timer has a resolution of 30.5us per LSB, which yields a maximum time of ~2seconds."]
pub type CTRL_R = crate::BitReader<bool>;
#[doc = "Field `CTRL` writer - 1= The Hibernation Timer has a resolution of 0.125s per LSB, which yields a maximum time in excess of 2 hours.\n 0= The Hibernation Timer has a resolution of 30.5us per LSB, which yields a maximum time of ~2seconds."]
pub type CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 1= The Hibernation Timer has a resolution of 0.125s per LSB, which yields a maximum time in excess of 2 hours.\n 0= The Hibernation Timer has a resolution of 30.5us per LSB, which yields a maximum time of ~2seconds."]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1= The Hibernation Timer has a resolution of 0.125s per LSB, which yields a maximum time in excess of 2 hours.\n 0= The Hibernation Timer has a resolution of 30.5us per LSB, which yields a maximum time of ~2seconds."]
    #[inline(always)]
    pub fn ctrl(&mut self) -> CTRL_W<0> {
        CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HTimer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
