#[doc = "Register `RESULT17` reader"]
pub struct R(crate::R<RESULT17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LED0` reader - Breating LED0"]
pub type LED0_R = crate::BitReader<bool>;
#[doc = "Field `LED1` reader - Breating LED1"]
pub type LED1_R = crate::BitReader<bool>;
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
#[doc = "GIRQ17 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result17](index.html) module"]
pub struct RESULT17_SPEC;
impl crate::RegisterSpec for RESULT17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result17::R](R) reader structure"]
impl crate::Readable for RESULT17_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT17 to value 0"]
impl crate::Resettable for RESULT17_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
