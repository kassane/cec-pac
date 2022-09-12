#[doc = "Register `IVN_STS` reader"]
pub struct R(crate::R<IVN_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVN_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVN_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVN_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FCS` reader - Flash Chip Selects forced high and bus forced low."]
pub type FCS_R = crate::BitReader<bool>;
#[doc = "Field `FPO` reader - Flash Power or RESET# Activated."]
pub type FPO_R = crate::BitReader<bool>;
#[doc = "Field `HRS` reader - Host held in Reset"]
pub type HRS_R = crate::BitReader<bool>;
#[doc = "Field `HIS` reader - Host held Isolated"]
pub type HIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Flash Chip Selects forced high and bus forced low."]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Power or RESET# Activated."]
    #[inline(always)]
    pub fn fpo(&self) -> FPO_R {
        FPO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host held in Reset"]
    #[inline(always)]
    pub fn hrs(&self) -> HRS_R {
        HRS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Host held Isolated"]
    #[inline(always)]
    pub fn his(&self) -> HIS_R {
        HIS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "SPI Intervention Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivn_sts](index.html) module"]
pub struct IVN_STS_SPEC;
impl crate::RegisterSpec for IVN_STS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ivn_sts::R](R) reader structure"]
impl crate::Readable for IVN_STS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IVN_STS to value 0"]
impl crate::Resettable for IVN_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
