#[doc = "Register `SRC8` reader"]
pub struct R(crate::R<SRC8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC8` writer"]
pub struct W(crate::W<SRC8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC8_SPEC>;
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
impl From<crate::W<SRC8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO140` reader - GPIO 140"]
pub type GPIO140_R = crate::BitReader<bool>;
#[doc = "Field `GPIO140` writer - GPIO 140"]
pub type GPIO140_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
#[doc = "Field `GPIO143` reader - GPIO 143"]
pub type GPIO143_R = crate::BitReader<bool>;
#[doc = "Field `GPIO143` writer - GPIO 143"]
pub type GPIO143_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
#[doc = "Field `GPIO144` reader - GPIO 144"]
pub type GPIO144_R = crate::BitReader<bool>;
#[doc = "Field `GPIO144` writer - GPIO 144"]
pub type GPIO144_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
#[doc = "Field `GPIO145` reader - GPIO 145"]
pub type GPIO145_R = crate::BitReader<bool>;
#[doc = "Field `GPIO145` writer - GPIO 145"]
pub type GPIO145_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
#[doc = "Field `GPIO146` reader - GPIO 146"]
pub type GPIO146_R = crate::BitReader<bool>;
#[doc = "Field `GPIO146` writer - GPIO 146"]
pub type GPIO146_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
#[doc = "Field `GPIO147` reader - GPIO 147"]
pub type GPIO147_R = crate::BitReader<bool>;
#[doc = "Field `GPIO147` writer - GPIO 147"]
pub type GPIO147_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
#[doc = "Field `GPIO150` reader - GPIO 150"]
pub type GPIO150_R = crate::BitReader<bool>;
#[doc = "Field `GPIO150` writer - GPIO 150"]
pub type GPIO150_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
#[doc = "Field `GPIO156` reader - GPIO 156"]
pub type GPIO156_R = crate::BitReader<bool>;
#[doc = "Field `GPIO156` writer - GPIO 156"]
pub type GPIO156_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
#[doc = "Field `GPIO157` reader - GPIO 157"]
pub type GPIO157_R = crate::BitReader<bool>;
#[doc = "Field `GPIO157` writer - GPIO 157"]
pub type GPIO157_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
#[doc = "Field `GPIO163` reader - GPIO 163"]
pub type GPIO163_R = crate::BitReader<bool>;
#[doc = "Field `GPIO163` writer - GPIO 163"]
pub type GPIO163_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
#[doc = "Field `GPIO165` reader - GPIO 165"]
pub type GPIO165_R = crate::BitReader<bool>;
#[doc = "Field `GPIO165` writer - GPIO 165"]
pub type GPIO165_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
#[doc = "Field `GPIO166` reader - GPIO 166"]
pub type GPIO166_R = crate::BitReader<bool>;
#[doc = "Field `GPIO166` writer - GPIO 166"]
pub type GPIO166_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
#[doc = "Field `GPIO170` reader - GPIO 170"]
pub type GPIO170_R = crate::BitReader<bool>;
#[doc = "Field `GPIO170` writer - GPIO 170"]
pub type GPIO170_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
#[doc = "Field `GPIO171` reader - GPIO 171"]
pub type GPIO171_R = crate::BitReader<bool>;
#[doc = "Field `GPIO171` writer - GPIO 171"]
pub type GPIO171_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC8_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPIO 140"]
    #[inline(always)]
    pub fn gpio140(&self) -> GPIO140_R {
        GPIO140_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO 143"]
    #[inline(always)]
    pub fn gpio143(&self) -> GPIO143_R {
        GPIO143_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO 144"]
    #[inline(always)]
    pub fn gpio144(&self) -> GPIO144_R {
        GPIO144_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO 145"]
    #[inline(always)]
    pub fn gpio145(&self) -> GPIO145_R {
        GPIO145_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO 146"]
    #[inline(always)]
    pub fn gpio146(&self) -> GPIO146_R {
        GPIO146_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO 147"]
    #[inline(always)]
    pub fn gpio147(&self) -> GPIO147_R {
        GPIO147_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO 150"]
    #[inline(always)]
    pub fn gpio150(&self) -> GPIO150_R {
        GPIO150_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO 156"]
    #[inline(always)]
    pub fn gpio156(&self) -> GPIO156_R {
        GPIO156_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO 157"]
    #[inline(always)]
    pub fn gpio157(&self) -> GPIO157_R {
        GPIO157_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO 163"]
    #[inline(always)]
    pub fn gpio163(&self) -> GPIO163_R {
        GPIO163_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO 165"]
    #[inline(always)]
    pub fn gpio165(&self) -> GPIO165_R {
        GPIO165_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO 166"]
    #[inline(always)]
    pub fn gpio166(&self) -> GPIO166_R {
        GPIO166_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO 170"]
    #[inline(always)]
    pub fn gpio170(&self) -> GPIO170_R {
        GPIO170_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO 171"]
    #[inline(always)]
    pub fn gpio171(&self) -> GPIO171_R {
        GPIO171_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO 140"]
    #[inline(always)]
    pub fn gpio140(&mut self) -> GPIO140_W<0> {
        GPIO140_W::new(self)
    }
    #[doc = "Bit 3 - GPIO 143"]
    #[inline(always)]
    pub fn gpio143(&mut self) -> GPIO143_W<3> {
        GPIO143_W::new(self)
    }
    #[doc = "Bit 4 - GPIO 144"]
    #[inline(always)]
    pub fn gpio144(&mut self) -> GPIO144_W<4> {
        GPIO144_W::new(self)
    }
    #[doc = "Bit 5 - GPIO 145"]
    #[inline(always)]
    pub fn gpio145(&mut self) -> GPIO145_W<5> {
        GPIO145_W::new(self)
    }
    #[doc = "Bit 6 - GPIO 146"]
    #[inline(always)]
    pub fn gpio146(&mut self) -> GPIO146_W<6> {
        GPIO146_W::new(self)
    }
    #[doc = "Bit 7 - GPIO 147"]
    #[inline(always)]
    pub fn gpio147(&mut self) -> GPIO147_W<7> {
        GPIO147_W::new(self)
    }
    #[doc = "Bit 8 - GPIO 150"]
    #[inline(always)]
    pub fn gpio150(&mut self) -> GPIO150_W<8> {
        GPIO150_W::new(self)
    }
    #[doc = "Bit 14 - GPIO 156"]
    #[inline(always)]
    pub fn gpio156(&mut self) -> GPIO156_W<14> {
        GPIO156_W::new(self)
    }
    #[doc = "Bit 15 - GPIO 157"]
    #[inline(always)]
    pub fn gpio157(&mut self) -> GPIO157_W<15> {
        GPIO157_W::new(self)
    }
    #[doc = "Bit 19 - GPIO 163"]
    #[inline(always)]
    pub fn gpio163(&mut self) -> GPIO163_W<19> {
        GPIO163_W::new(self)
    }
    #[doc = "Bit 21 - GPIO 165"]
    #[inline(always)]
    pub fn gpio165(&mut self) -> GPIO165_W<21> {
        GPIO165_W::new(self)
    }
    #[doc = "Bit 22 - GPIO 166"]
    #[inline(always)]
    pub fn gpio166(&mut self) -> GPIO166_W<22> {
        GPIO166_W::new(self)
    }
    #[doc = "Bit 24 - GPIO 170"]
    #[inline(always)]
    pub fn gpio170(&mut self) -> GPIO170_W<24> {
        GPIO170_W::new(self)
    }
    #[doc = "Bit 25 - GPIO 171"]
    #[inline(always)]
    pub fn gpio171(&mut self) -> GPIO171_W<25> {
        GPIO171_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ8 SOURCE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src8](index.html) module"]
pub struct SRC8_SPEC;
impl crate::RegisterSpec for SRC8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src8::R](R) reader structure"]
impl crate::Readable for SRC8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src8::W](W) writer structure"]
impl crate::Writable for SRC8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRC8 to value 0"]
impl crate::Resettable for SRC8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
