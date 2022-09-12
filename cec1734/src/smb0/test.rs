#[doc = "Register `TEST` reader"]
pub struct R(crate::R<TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEST` reader - This register must not be written, or undesirable results may occur."]
pub type TEST_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - This register must not be written, or undesirable results may occur."]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(self.bits)
    }
}
#[doc = "Test\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](index.html) module"]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [test::R](R) reader structure"]
impl crate::Readable for TEST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TEST to value 0x05"]
impl crate::Resettable for TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
