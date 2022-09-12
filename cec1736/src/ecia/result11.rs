#[doc = "Register `RESULT11` reader"]
pub struct R(crate::R<RESULT11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO000` reader - GPIO 000"]
pub type GPIO000_R = crate::BitReader<bool>;
#[doc = "Field `GPIO002` reader - GPIO 002"]
pub type GPIO002_R = crate::BitReader<bool>;
#[doc = "Field `GPIO003` reader - GPIO 003"]
pub type GPIO003_R = crate::BitReader<bool>;
#[doc = "Field `GPIO004` reader - GPIO 004"]
pub type GPIO004_R = crate::BitReader<bool>;
#[doc = "Field `GPIO012` reader - GPIO 012"]
pub type GPIO012_R = crate::BitReader<bool>;
#[doc = "Field `GPIO013` reader - GPIO 013"]
pub type GPIO013_R = crate::BitReader<bool>;
#[doc = "Field `GPIO015` reader - GPIO 015"]
pub type GPIO015_R = crate::BitReader<bool>;
#[doc = "Field `GPIO016` reader - GPIO 016"]
pub type GPIO016_R = crate::BitReader<bool>;
#[doc = "Field `GPIO020` reader - GPIO 020"]
pub type GPIO020_R = crate::BitReader<bool>;
#[doc = "Field `GPIO021` reader - GPIO 021"]
pub type GPIO021_R = crate::BitReader<bool>;
#[doc = "Field `GPIO022` reader - GPIO 022"]
pub type GPIO022_R = crate::BitReader<bool>;
#[doc = "Field `GPIO023` reader - GPIO 023"]
pub type GPIO023_R = crate::BitReader<bool>;
#[doc = "Field `GPIO024` reader - GPIO 024"]
pub type GPIO024_R = crate::BitReader<bool>;
#[doc = "Field `GPIO026` reader - GPIO 026"]
pub type GPIO026_R = crate::BitReader<bool>;
#[doc = "Field `GPIO027` reader - GPIO 027"]
pub type GPIO027_R = crate::BitReader<bool>;
#[doc = "Field `GPIO030` reader - GPIO 030"]
pub type GPIO030_R = crate::BitReader<bool>;
#[doc = "Field `GPIO031` reader - GPIO 031"]
pub type GPIO031_R = crate::BitReader<bool>;
#[doc = "Field `GPIO032` reader - GPIO 032"]
pub type GPIO032_R = crate::BitReader<bool>;
#[doc = "Field `GPIO033` reader - GPIO 033"]
pub type GPIO033_R = crate::BitReader<bool>;
#[doc = "Field `GPIO034` reader - GPIO 034"]
pub type GPIO034_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - GPIO 000"]
    #[inline(always)]
    pub fn gpio000(&self) -> GPIO000_R {
        GPIO000_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO 002"]
    #[inline(always)]
    pub fn gpio002(&self) -> GPIO002_R {
        GPIO002_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO 003"]
    #[inline(always)]
    pub fn gpio003(&self) -> GPIO003_R {
        GPIO003_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO 004"]
    #[inline(always)]
    pub fn gpio004(&self) -> GPIO004_R {
        GPIO004_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO 012"]
    #[inline(always)]
    pub fn gpio012(&self) -> GPIO012_R {
        GPIO012_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO 013"]
    #[inline(always)]
    pub fn gpio013(&self) -> GPIO013_R {
        GPIO013_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO 015"]
    #[inline(always)]
    pub fn gpio015(&self) -> GPIO015_R {
        GPIO015_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO 016"]
    #[inline(always)]
    pub fn gpio016(&self) -> GPIO016_R {
        GPIO016_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO 020"]
    #[inline(always)]
    pub fn gpio020(&self) -> GPIO020_R {
        GPIO020_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO 021"]
    #[inline(always)]
    pub fn gpio021(&self) -> GPIO021_R {
        GPIO021_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO 022"]
    #[inline(always)]
    pub fn gpio022(&self) -> GPIO022_R {
        GPIO022_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO 023"]
    #[inline(always)]
    pub fn gpio023(&self) -> GPIO023_R {
        GPIO023_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO 024"]
    #[inline(always)]
    pub fn gpio024(&self) -> GPIO024_R {
        GPIO024_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO 026"]
    #[inline(always)]
    pub fn gpio026(&self) -> GPIO026_R {
        GPIO026_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO 027"]
    #[inline(always)]
    pub fn gpio027(&self) -> GPIO027_R {
        GPIO027_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO 030"]
    #[inline(always)]
    pub fn gpio030(&self) -> GPIO030_R {
        GPIO030_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO 031"]
    #[inline(always)]
    pub fn gpio031(&self) -> GPIO031_R {
        GPIO031_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO 032"]
    #[inline(always)]
    pub fn gpio032(&self) -> GPIO032_R {
        GPIO032_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO 033"]
    #[inline(always)]
    pub fn gpio033(&self) -> GPIO033_R {
        GPIO033_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO 034"]
    #[inline(always)]
    pub fn gpio034(&self) -> GPIO034_R {
        GPIO034_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "GIRQ11 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result11](index.html) module"]
pub struct RESULT11_SPEC;
impl crate::RegisterSpec for RESULT11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result11::R](R) reader structure"]
impl crate::Readable for RESULT11_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT11 to value 0"]
impl crate::Resettable for RESULT11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
