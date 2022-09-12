#[doc = "Register `RESULT8` reader"]
pub struct R(crate::R<RESULT8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO140` reader - GPIO 140"]
pub type GPIO140_R = crate::BitReader<bool>;
#[doc = "Field `GPIO143` reader - GPIO 143"]
pub type GPIO143_R = crate::BitReader<bool>;
#[doc = "Field `GPIO144` reader - GPIO 144"]
pub type GPIO144_R = crate::BitReader<bool>;
#[doc = "Field `GPIO145` reader - GPIO 145"]
pub type GPIO145_R = crate::BitReader<bool>;
#[doc = "Field `GPIO146` reader - GPIO 146"]
pub type GPIO146_R = crate::BitReader<bool>;
#[doc = "Field `GPIO147` reader - GPIO 147"]
pub type GPIO147_R = crate::BitReader<bool>;
#[doc = "Field `GPIO150` reader - GPIO 150"]
pub type GPIO150_R = crate::BitReader<bool>;
#[doc = "Field `GPIO156` reader - GPIO 156"]
pub type GPIO156_R = crate::BitReader<bool>;
#[doc = "Field `GPIO157` reader - GPIO 157"]
pub type GPIO157_R = crate::BitReader<bool>;
#[doc = "Field `GPIO163` reader - GPIO 163"]
pub type GPIO163_R = crate::BitReader<bool>;
#[doc = "Field `GPIO165` reader - GPIO 165"]
pub type GPIO165_R = crate::BitReader<bool>;
#[doc = "Field `GPIO166` reader - GPIO 166"]
pub type GPIO166_R = crate::BitReader<bool>;
#[doc = "Field `GPIO170` reader - GPIO 170"]
pub type GPIO170_R = crate::BitReader<bool>;
#[doc = "Field `GPIO171` reader - GPIO 171"]
pub type GPIO171_R = crate::BitReader<bool>;
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
#[doc = "GIRQ8 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result8](index.html) module"]
pub struct RESULT8_SPEC;
impl crate::RegisterSpec for RESULT8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result8::R](R) reader structure"]
impl crate::Readable for RESULT8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT8 to value 0"]
impl crate::Resettable for RESULT8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
