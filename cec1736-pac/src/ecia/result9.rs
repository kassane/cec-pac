#[doc = "Register `RESULT9` reader"]
pub struct R(crate::R<RESULT9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO104` reader - GPIO 104"]
pub type GPIO104_R = crate::BitReader<bool>;
#[doc = "Field `GPIO105` reader - GPIO 105"]
pub type GPIO105_R = crate::BitReader<bool>;
#[doc = "Field `GPIO106` reader - GPIO 106"]
pub type GPIO106_R = crate::BitReader<bool>;
#[doc = "Field `GPIO107` reader - GPIO 107"]
pub type GPIO107_R = crate::BitReader<bool>;
#[doc = "Field `GPIO112` reader - GPIO 112"]
pub type GPIO112_R = crate::BitReader<bool>;
#[doc = "Field `GPIO113` reader - GPIO 113"]
pub type GPIO113_R = crate::BitReader<bool>;
#[doc = "Field `GPIO120` reader - GPIO 120"]
pub type GPIO120_R = crate::BitReader<bool>;
#[doc = "Field `GPIO121` reader - GPIO 121"]
pub type GPIO121_R = crate::BitReader<bool>;
#[doc = "Field `GPIO122` reader - GPIO 122"]
pub type GPIO122_R = crate::BitReader<bool>;
#[doc = "Field `GPIO123` reader - GPIO 123"]
pub type GPIO123_R = crate::BitReader<bool>;
#[doc = "Field `GPIO124` reader - GPIO 124"]
pub type GPIO124_R = crate::BitReader<bool>;
#[doc = "Field `GPIO125` reader - GPIO 125"]
pub type GPIO125_R = crate::BitReader<bool>;
#[doc = "Field `GPIO126` reader - GPIO 126"]
pub type GPIO126_R = crate::BitReader<bool>;
#[doc = "Field `GPIO127` reader - GPIO 127"]
pub type GPIO127_R = crate::BitReader<bool>;
#[doc = "Field `GPIO130` reader - GPIO 130"]
pub type GPIO130_R = crate::BitReader<bool>;
#[doc = "Field `GPIO131` reader - GPIO 131"]
pub type GPIO131_R = crate::BitReader<bool>;
#[doc = "Field `GPIO132` reader - GPIO 132"]
pub type GPIO132_R = crate::BitReader<bool>;
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
#[doc = "GIRQ9 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result9](index.html) module"]
pub struct RESULT9_SPEC;
impl crate::RegisterSpec for RESULT9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result9::R](R) reader structure"]
impl crate::Readable for RESULT9_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT9 to value 0"]
impl crate::Resettable for RESULT9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
