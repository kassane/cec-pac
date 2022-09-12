#[doc = "Register `RESULT10` reader"]
pub struct R(crate::R<RESULT10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO045` reader - GPIO 045"]
pub type GPIO045_R = crate::BitReader<bool>;
#[doc = "Field `GPIO046` reader - GPIO 046"]
pub type GPIO046_R = crate::BitReader<bool>;
#[doc = "Field `GPIO047` reader - GPIO 047"]
pub type GPIO047_R = crate::BitReader<bool>;
#[doc = "Field `GPIO050` reader - GPIO 050"]
pub type GPIO050_R = crate::BitReader<bool>;
#[doc = "Field `GPIO053` reader - GPIO 053"]
pub type GPIO053_R = crate::BitReader<bool>;
#[doc = "Field `GPIO055` reader - GPIO 055"]
pub type GPIO055_R = crate::BitReader<bool>;
#[doc = "Field `GPIO056` reader - GPIO 056"]
pub type GPIO056_R = crate::BitReader<bool>;
#[doc = "Field `GPIO057` reader - GPIO 057"]
pub type GPIO057_R = crate::BitReader<bool>;
#[doc = "Field `GPIO063` reader - GPIO 060"]
pub type GPIO063_R = crate::BitReader<bool>;
#[doc = "Field `GPIO070` reader - GPIO 070"]
pub type GPIO070_R = crate::BitReader<bool>;
#[doc = "Field `GPIO071` reader - GPIO 071"]
pub type GPIO071_R = crate::BitReader<bool>;
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
#[doc = "GIRQ10 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result10](index.html) module"]
pub struct RESULT10_SPEC;
impl crate::RegisterSpec for RESULT10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result10::R](R) reader structure"]
impl crate::Readable for RESULT10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT10 to value 0"]
impl crate::Resettable for RESULT10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
