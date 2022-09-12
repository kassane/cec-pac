#[doc = "Register `RESULT16` reader"]
pub struct R(crate::R<RESULT16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKE_ERR` reader - PKE ERR"]
pub type PKE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `PKE_END` reader - PKE END"]
pub type PKE_END_R = crate::BitReader<bool>;
#[doc = "Field `RNG` reader - RNG"]
pub type RNG_R = crate::BitReader<bool>;
#[doc = "Field `AES` reader - AES"]
pub type AES_R = crate::BitReader<bool>;
#[doc = "Field `HASH` reader - HASH"]
pub type HASH_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - PKE ERR"]
    #[inline(always)]
    pub fn pke_err(&self) -> PKE_ERR_R {
        PKE_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PKE END"]
    #[inline(always)]
    pub fn pke_end(&self) -> PKE_END_R {
        PKE_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AES"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HASH"]
    #[inline(always)]
    pub fn hash(&self) -> HASH_R {
        HASH_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "GIRQ16 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result16](index.html) module"]
pub struct RESULT16_SPEC;
impl crate::RegisterSpec for RESULT16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result16::R](R) reader structure"]
impl crate::Readable for RESULT16_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT16 to value 0"]
impl crate::Resettable for RESULT16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
