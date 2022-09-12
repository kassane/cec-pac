#[doc = "Register `RESULT26` reader"]
pub struct R(crate::R<RESULT26_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT26_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT26_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT26_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO250` reader - GPIO250"]
pub type GPIO250_R = crate::BitReader<bool>;
#[doc = "Field `GPIO260` reader - GPIO260"]
pub type GPIO260_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 8 - GPIO250"]
    #[inline(always)]
    pub fn gpio250(&self) -> GPIO250_R {
        GPIO250_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO260"]
    #[inline(always)]
    pub fn gpio260(&self) -> GPIO260_R {
        GPIO260_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "GIRQ26 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result26](index.html) module"]
pub struct RESULT26_SPEC;
impl crate::RegisterSpec for RESULT26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result26::R](R) reader structure"]
impl crate::Readable for RESULT26_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT26 to value 0"]
impl crate::Resettable for RESULT26_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
