#[doc = "Register `SRC10` reader"]
pub struct R(crate::R<SRC10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC10` writer"]
pub struct W(crate::W<SRC10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC10_SPEC>;
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
impl From<crate::W<SRC10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO045` reader - GPIO 045"]
pub type GPIO045_R = crate::BitReader<bool>;
#[doc = "Field `GPIO045` writer - GPIO 045"]
pub type GPIO045_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC10_SPEC, bool, O>;
#[doc = "Field `GPIO046` reader - GPIO 046"]
pub type GPIO046_R = crate::BitReader<bool>;
#[doc = "Field `GPIO046` writer - GPIO 046"]
pub type GPIO046_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC10_SPEC, bool, O>;
#[doc = "Field `GPIO047` reader - GPIO 047"]
pub type GPIO047_R = crate::BitReader<bool>;
#[doc = "Field `GPIO047` writer - GPIO 047"]
pub type GPIO047_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC10_SPEC, bool, O>;
#[doc = "Field `GPIO050` reader - GPIO 050"]
pub type GPIO050_R = crate::BitReader<bool>;
#[doc = "Field `GPIO050` writer - GPIO 050"]
pub type GPIO050_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC10_SPEC, bool, O>;
#[doc = "Field `GPIO053` reader - GPIO 053"]
pub type GPIO053_R = crate::BitReader<bool>;
#[doc = "Field `GPIO053` writer - GPIO 053"]
pub type GPIO053_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC10_SPEC, bool, O>;
#[doc = "Field `GPIO055` reader - GPIO 055"]
pub type GPIO055_R = crate::BitReader<bool>;
#[doc = "Field `GPIO055` writer - GPIO 055"]
pub type GPIO055_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC10_SPEC, bool, O>;
#[doc = "Field `GPIO056` reader - GPIO 056"]
pub type GPIO056_R = crate::BitReader<bool>;
#[doc = "Field `GPIO056` writer - GPIO 056"]
pub type GPIO056_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC10_SPEC, bool, O>;
#[doc = "Field `GPIO057` reader - GPIO 057"]
pub type GPIO057_R = crate::BitReader<bool>;
#[doc = "Field `GPIO057` writer - GPIO 057"]
pub type GPIO057_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC10_SPEC, bool, O>;
#[doc = "Field `GPIO063` reader - GPIO 060"]
pub type GPIO063_R = crate::BitReader<bool>;
#[doc = "Field `GPIO063` writer - GPIO 060"]
pub type GPIO063_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC10_SPEC, bool, O>;
#[doc = "Field `GPIO070` reader - GPIO 070"]
pub type GPIO070_R = crate::BitReader<bool>;
#[doc = "Field `GPIO070` writer - GPIO 070"]
pub type GPIO070_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC10_SPEC, bool, O>;
#[doc = "Field `GPIO071` reader - GPIO 071"]
pub type GPIO071_R = crate::BitReader<bool>;
#[doc = "Field `GPIO071` writer - GPIO 071"]
pub type GPIO071_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC10_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - GPIO 045"]
    #[inline(always)]
    pub fn gpio045(&self) -> GPIO045_R {
        GPIO045_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO 046"]
    #[inline(always)]
    pub fn gpio046(&self) -> GPIO046_R {
        GPIO046_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO 047"]
    #[inline(always)]
    pub fn gpio047(&self) -> GPIO047_R {
        GPIO047_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO 050"]
    #[inline(always)]
    pub fn gpio050(&self) -> GPIO050_R {
        GPIO050_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO 053"]
    #[inline(always)]
    pub fn gpio053(&self) -> GPIO053_R {
        GPIO053_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO 055"]
    #[inline(always)]
    pub fn gpio055(&self) -> GPIO055_R {
        GPIO055_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO 056"]
    #[inline(always)]
    pub fn gpio056(&self) -> GPIO056_R {
        GPIO056_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO 057"]
    #[inline(always)]
    pub fn gpio057(&self) -> GPIO057_R {
        GPIO057_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO 060"]
    #[inline(always)]
    pub fn gpio063(&self) -> GPIO063_R {
        GPIO063_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO 070"]
    #[inline(always)]
    pub fn gpio070(&self) -> GPIO070_R {
        GPIO070_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO 071"]
    #[inline(always)]
    pub fn gpio071(&self) -> GPIO071_R {
        GPIO071_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - GPIO 045"]
    #[inline(always)]
    pub fn gpio045(&mut self) -> GPIO045_W<5> {
        GPIO045_W::new(self)
    }
    #[doc = "Bit 6 - GPIO 046"]
    #[inline(always)]
    pub fn gpio046(&mut self) -> GPIO046_W<6> {
        GPIO046_W::new(self)
    }
    #[doc = "Bit 7 - GPIO 047"]
    #[inline(always)]
    pub fn gpio047(&mut self) -> GPIO047_W<7> {
        GPIO047_W::new(self)
    }
    #[doc = "Bit 8 - GPIO 050"]
    #[inline(always)]
    pub fn gpio050(&mut self) -> GPIO050_W<8> {
        GPIO050_W::new(self)
    }
    #[doc = "Bit 11 - GPIO 053"]
    #[inline(always)]
    pub fn gpio053(&mut self) -> GPIO053_W<11> {
        GPIO053_W::new(self)
    }
    #[doc = "Bit 13 - GPIO 055"]
    #[inline(always)]
    pub fn gpio055(&mut self) -> GPIO055_W<13> {
        GPIO055_W::new(self)
    }
    #[doc = "Bit 14 - GPIO 056"]
    #[inline(always)]
    pub fn gpio056(&mut self) -> GPIO056_W<14> {
        GPIO056_W::new(self)
    }
    #[doc = "Bit 15 - GPIO 057"]
    #[inline(always)]
    pub fn gpio057(&mut self) -> GPIO057_W<15> {
        GPIO057_W::new(self)
    }
    #[doc = "Bit 19 - GPIO 060"]
    #[inline(always)]
    pub fn gpio063(&mut self) -> GPIO063_W<19> {
        GPIO063_W::new(self)
    }
    #[doc = "Bit 24 - GPIO 070"]
    #[inline(always)]
    pub fn gpio070(&mut self) -> GPIO070_W<24> {
        GPIO070_W::new(self)
    }
    #[doc = "Bit 25 - GPIO 071"]
    #[inline(always)]
    pub fn gpio071(&mut self) -> GPIO071_W<25> {
        GPIO071_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ10 SOURCE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src10](index.html) module"]
pub struct SRC10_SPEC;
impl crate::RegisterSpec for SRC10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src10::R](R) reader structure"]
impl crate::Readable for SRC10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src10::W](W) writer structure"]
impl crate::Writable for SRC10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRC10 to value 0"]
impl crate::Resettable for SRC10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
