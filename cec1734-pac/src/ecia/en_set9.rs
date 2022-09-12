#[doc = "Register `EN_SET9` reader"]
pub struct R(crate::R<EN_SET9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_SET9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_SET9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_SET9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN_SET9` writer"]
pub struct W(crate::W<EN_SET9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_SET9_SPEC>;
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
impl From<crate::W<EN_SET9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_SET9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO104` reader - GPIO 104"]
pub type GPIO104_R = crate::BitReader<bool>;
#[doc = "Field `GPIO104` writer - GPIO 104"]
pub type GPIO104_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO105` reader - GPIO 105"]
pub type GPIO105_R = crate::BitReader<bool>;
#[doc = "Field `GPIO105` writer - GPIO 105"]
pub type GPIO105_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO106` reader - GPIO 106"]
pub type GPIO106_R = crate::BitReader<bool>;
#[doc = "Field `GPIO106` writer - GPIO 106"]
pub type GPIO106_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO107` reader - GPIO 107"]
pub type GPIO107_R = crate::BitReader<bool>;
#[doc = "Field `GPIO107` writer - GPIO 107"]
pub type GPIO107_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO112` reader - GPIO 112"]
pub type GPIO112_R = crate::BitReader<bool>;
#[doc = "Field `GPIO112` writer - GPIO 112"]
pub type GPIO112_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO113` reader - GPIO 113"]
pub type GPIO113_R = crate::BitReader<bool>;
#[doc = "Field `GPIO113` writer - GPIO 113"]
pub type GPIO113_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO120` reader - GPIO 120"]
pub type GPIO120_R = crate::BitReader<bool>;
#[doc = "Field `GPIO120` writer - GPIO 120"]
pub type GPIO120_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO121` reader - GPIO 121"]
pub type GPIO121_R = crate::BitReader<bool>;
#[doc = "Field `GPIO121` writer - GPIO 121"]
pub type GPIO121_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO122` reader - GPIO 122"]
pub type GPIO122_R = crate::BitReader<bool>;
#[doc = "Field `GPIO122` writer - GPIO 122"]
pub type GPIO122_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO123` reader - GPIO 123"]
pub type GPIO123_R = crate::BitReader<bool>;
#[doc = "Field `GPIO123` writer - GPIO 123"]
pub type GPIO123_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO124` reader - GPIO 124"]
pub type GPIO124_R = crate::BitReader<bool>;
#[doc = "Field `GPIO124` writer - GPIO 124"]
pub type GPIO124_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO125` reader - GPIO 125"]
pub type GPIO125_R = crate::BitReader<bool>;
#[doc = "Field `GPIO125` writer - GPIO 125"]
pub type GPIO125_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO126` reader - GPIO 126"]
pub type GPIO126_R = crate::BitReader<bool>;
#[doc = "Field `GPIO126` writer - GPIO 126"]
pub type GPIO126_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO127` reader - GPIO 127"]
pub type GPIO127_R = crate::BitReader<bool>;
#[doc = "Field `GPIO127` writer - GPIO 127"]
pub type GPIO127_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO130` reader - GPIO 130"]
pub type GPIO130_R = crate::BitReader<bool>;
#[doc = "Field `GPIO130` writer - GPIO 130"]
pub type GPIO130_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO131` reader - GPIO 131"]
pub type GPIO131_R = crate::BitReader<bool>;
#[doc = "Field `GPIO131` writer - GPIO 131"]
pub type GPIO131_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
#[doc = "Field `GPIO132` reader - GPIO 132"]
pub type GPIO132_R = crate::BitReader<bool>;
#[doc = "Field `GPIO132` writer - GPIO 132"]
pub type GPIO132_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET9_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - GPIO 104"]
    #[inline(always)]
    pub fn gpio104(&self) -> GPIO104_R {
        GPIO104_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO 105"]
    #[inline(always)]
    pub fn gpio105(&self) -> GPIO105_R {
        GPIO105_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO 106"]
    #[inline(always)]
    pub fn gpio106(&self) -> GPIO106_R {
        GPIO106_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO 107"]
    #[inline(always)]
    pub fn gpio107(&self) -> GPIO107_R {
        GPIO107_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO 112"]
    #[inline(always)]
    pub fn gpio112(&self) -> GPIO112_R {
        GPIO112_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO 113"]
    #[inline(always)]
    pub fn gpio113(&self) -> GPIO113_R {
        GPIO113_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO 120"]
    #[inline(always)]
    pub fn gpio120(&self) -> GPIO120_R {
        GPIO120_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO 121"]
    #[inline(always)]
    pub fn gpio121(&self) -> GPIO121_R {
        GPIO121_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO 122"]
    #[inline(always)]
    pub fn gpio122(&self) -> GPIO122_R {
        GPIO122_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO 123"]
    #[inline(always)]
    pub fn gpio123(&self) -> GPIO123_R {
        GPIO123_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO 124"]
    #[inline(always)]
    pub fn gpio124(&self) -> GPIO124_R {
        GPIO124_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO 125"]
    #[inline(always)]
    pub fn gpio125(&self) -> GPIO125_R {
        GPIO125_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO 126"]
    #[inline(always)]
    pub fn gpio126(&self) -> GPIO126_R {
        GPIO126_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO 127"]
    #[inline(always)]
    pub fn gpio127(&self) -> GPIO127_R {
        GPIO127_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO 130"]
    #[inline(always)]
    pub fn gpio130(&self) -> GPIO130_R {
        GPIO130_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO 131"]
    #[inline(always)]
    pub fn gpio131(&self) -> GPIO131_R {
        GPIO131_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO 132"]
    #[inline(always)]
    pub fn gpio132(&self) -> GPIO132_R {
        GPIO132_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - GPIO 104"]
    #[inline(always)]
    pub fn gpio104(&mut self) -> GPIO104_W<4> {
        GPIO104_W::new(self)
    }
    #[doc = "Bit 5 - GPIO 105"]
    #[inline(always)]
    pub fn gpio105(&mut self) -> GPIO105_W<5> {
        GPIO105_W::new(self)
    }
    #[doc = "Bit 6 - GPIO 106"]
    #[inline(always)]
    pub fn gpio106(&mut self) -> GPIO106_W<6> {
        GPIO106_W::new(self)
    }
    #[doc = "Bit 7 - GPIO 107"]
    #[inline(always)]
    pub fn gpio107(&mut self) -> GPIO107_W<7> {
        GPIO107_W::new(self)
    }
    #[doc = "Bit 10 - GPIO 112"]
    #[inline(always)]
    pub fn gpio112(&mut self) -> GPIO112_W<10> {
        GPIO112_W::new(self)
    }
    #[doc = "Bit 11 - GPIO 113"]
    #[inline(always)]
    pub fn gpio113(&mut self) -> GPIO113_W<11> {
        GPIO113_W::new(self)
    }
    #[doc = "Bit 16 - GPIO 120"]
    #[inline(always)]
    pub fn gpio120(&mut self) -> GPIO120_W<16> {
        GPIO120_W::new(self)
    }
    #[doc = "Bit 17 - GPIO 121"]
    #[inline(always)]
    pub fn gpio121(&mut self) -> GPIO121_W<17> {
        GPIO121_W::new(self)
    }
    #[doc = "Bit 18 - GPIO 122"]
    #[inline(always)]
    pub fn gpio122(&mut self) -> GPIO122_W<18> {
        GPIO122_W::new(self)
    }
    #[doc = "Bit 19 - GPIO 123"]
    #[inline(always)]
    pub fn gpio123(&mut self) -> GPIO123_W<19> {
        GPIO123_W::new(self)
    }
    #[doc = "Bit 20 - GPIO 124"]
    #[inline(always)]
    pub fn gpio124(&mut self) -> GPIO124_W<20> {
        GPIO124_W::new(self)
    }
    #[doc = "Bit 21 - GPIO 125"]
    #[inline(always)]
    pub fn gpio125(&mut self) -> GPIO125_W<21> {
        GPIO125_W::new(self)
    }
    #[doc = "Bit 22 - GPIO 126"]
    #[inline(always)]
    pub fn gpio126(&mut self) -> GPIO126_W<22> {
        GPIO126_W::new(self)
    }
    #[doc = "Bit 23 - GPIO 127"]
    #[inline(always)]
    pub fn gpio127(&mut self) -> GPIO127_W<23> {
        GPIO127_W::new(self)
    }
    #[doc = "Bit 24 - GPIO 130"]
    #[inline(always)]
    pub fn gpio130(&mut self) -> GPIO130_W<24> {
        GPIO130_W::new(self)
    }
    #[doc = "Bit 25 - GPIO 131"]
    #[inline(always)]
    pub fn gpio131(&mut self) -> GPIO131_W<25> {
        GPIO131_W::new(self)
    }
    #[doc = "Bit 26 - GPIO 132"]
    #[inline(always)]
    pub fn gpio132(&mut self) -> GPIO132_W<26> {
        GPIO132_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ9 ENABLE SET\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_set9](index.html) module"]
pub struct EN_SET9_SPEC;
impl crate::RegisterSpec for EN_SET9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en_set9::R](R) reader structure"]
impl crate::Readable for EN_SET9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en_set9::W](W) writer structure"]
impl crate::Writable for EN_SET9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN_SET9 to value 0"]
impl crate::Resettable for EN_SET9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
