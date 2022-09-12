#[doc = "Register `INT_TEMP` reader"]
pub struct R(crate::R<INT_TEMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_TEMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_TEMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_TEMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INT_TEMP` reader - Internal Temp Byte Register"]
pub type INT_TEMP_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Internal Temp Byte Register"]
    #[inline(always)]
    pub fn int_temp(&self) -> INT_TEMP_R {
        INT_TEMP_R::new(self.bits)
    }
}
#[doc = "Internal Temp Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_temp](index.html) module"]
pub struct INT_TEMP_SPEC;
impl crate::RegisterSpec for INT_TEMP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [int_temp::R](R) reader structure"]
impl crate::Readable for INT_TEMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_TEMP to value 0"]
impl crate::Resettable for INT_TEMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
