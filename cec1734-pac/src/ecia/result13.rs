#[doc = "Register `RESULT13` reader"]
pub struct R(crate::R<RESULT13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2CSMB0` reader - I2CSMB0"]
pub type I2CSMB0_R = crate::BitReader<bool>;
#[doc = "Field `I2CSMB1` reader - I2CSMB1"]
pub type I2CSMB1_R = crate::BitReader<bool>;
#[doc = "Field `I2CSMB2` reader - I2CSMB2"]
pub type I2CSMB2_R = crate::BitReader<bool>;
#[doc = "Field `I2CSMB3` reader - I2CSMB3"]
pub type I2CSMB3_R = crate::BitReader<bool>;
#[doc = "Field `I2CSMB4` reader - I2CSMB4"]
pub type I2CSMB4_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - I2CSMB0"]
    #[inline(always)]
    pub fn i2csmb0(&self) -> I2CSMB0_R {
        I2CSMB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2CSMB1"]
    #[inline(always)]
    pub fn i2csmb1(&self) -> I2CSMB1_R {
        I2CSMB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2CSMB2"]
    #[inline(always)]
    pub fn i2csmb2(&self) -> I2CSMB2_R {
        I2CSMB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2CSMB3"]
    #[inline(always)]
    pub fn i2csmb3(&self) -> I2CSMB3_R {
        I2CSMB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2CSMB4"]
    #[inline(always)]
    pub fn i2csmb4(&self) -> I2CSMB4_R {
        I2CSMB4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "GIRQ13 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result13](index.html) module"]
pub struct RESULT13_SPEC;
impl crate::RegisterSpec for RESULT13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result13::R](R) reader structure"]
impl crate::Readable for RESULT13_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT13 to value 0"]
impl crate::Resettable for RESULT13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
