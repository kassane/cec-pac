#[doc = "Register `RESULT12` reader"]
pub struct R(crate::R<RESULT12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO200` reader - GPIO 200"]
pub type GPIO200_R = crate::BitReader<bool>;
#[doc = "Field `GPIO201` reader - GPIO 201"]
pub type GPIO201_R = crate::BitReader<bool>;
#[doc = "Field `GPIO202` reader - GPIO 202"]
pub type GPIO202_R = crate::BitReader<bool>;
#[doc = "Field `GPIO203` reader - GPIO 203"]
pub type GPIO203_R = crate::BitReader<bool>;
#[doc = "Field `GPIO204` reader - GPIO 204"]
pub type GPIO204_R = crate::BitReader<bool>;
#[doc = "Field `GPIO223` reader - GPIO 223"]
pub type GPIO223_R = crate::BitReader<bool>;
#[doc = "Field `GPIO224` reader - GPIO 224"]
pub type GPIO224_R = crate::BitReader<bool>;
#[doc = "Field `GPIO227` reader - GPIO 227"]
pub type GPIO227_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - GPIO 200"]
    #[inline(always)]
    pub fn gpio200(&self) -> GPIO200_R {
        GPIO200_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO 201"]
    #[inline(always)]
    pub fn gpio201(&self) -> GPIO201_R {
        GPIO201_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO 202"]
    #[inline(always)]
    pub fn gpio202(&self) -> GPIO202_R {
        GPIO202_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO 203"]
    #[inline(always)]
    pub fn gpio203(&self) -> GPIO203_R {
        GPIO203_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO 204"]
    #[inline(always)]
    pub fn gpio204(&self) -> GPIO204_R {
        GPIO204_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO 223"]
    #[inline(always)]
    pub fn gpio223(&self) -> GPIO223_R {
        GPIO223_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO 224"]
    #[inline(always)]
    pub fn gpio224(&self) -> GPIO224_R {
        GPIO224_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO 227"]
    #[inline(always)]
    pub fn gpio227(&self) -> GPIO227_R {
        GPIO227_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "GIRQ12 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result12](index.html) module"]
pub struct RESULT12_SPEC;
impl crate::RegisterSpec for RESULT12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result12::R](R) reader structure"]
impl crate::Readable for RESULT12_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT12 to value 0"]
impl crate::Resettable for RESULT12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
