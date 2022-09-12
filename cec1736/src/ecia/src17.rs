#[doc = "Register `SRC17` reader"]
pub struct R(crate::R<SRC17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC17` writer"]
pub struct W(crate::W<SRC17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC17_SPEC>;
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
impl From<crate::W<SRC17_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LED0` reader - Breating LED0"]
pub type LED0_R = crate::BitReader<bool>;
#[doc = "Field `LED0` writer - Breating LED0"]
pub type LED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC17_SPEC, bool, O>;
#[doc = "Field `LED1` reader - Breating LED1"]
pub type LED1_R = crate::BitReader<bool>;
#[doc = "Field `LED1` writer - Breating LED1"]
pub type LED1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC17_SPEC, bool, O>;
impl R {
    #[doc = "Bit 13 - Breating LED0"]
    #[inline(always)]
    pub fn led0(&self) -> LED0_R {
        LED0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Breating LED1"]
    #[inline(always)]
    pub fn led1(&self) -> LED1_R {
        LED1_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Breating LED0"]
    #[inline(always)]
    pub fn led0(&mut self) -> LED0_W<13> {
        LED0_W::new(self)
    }
    #[doc = "Bit 14 - Breating LED1"]
    #[inline(always)]
    pub fn led1(&mut self) -> LED1_W<14> {
        LED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ17 SOURCE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src17](index.html) module"]
pub struct SRC17_SPEC;
impl crate::RegisterSpec for SRC17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src17::R](R) reader structure"]
impl crate::Readable for SRC17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src17::W](W) writer structure"]
impl crate::Writable for SRC17_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRC17 to value 0"]
impl crate::Resettable for SRC17_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
