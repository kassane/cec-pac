#[doc = "Register `MTMON_VIOADDR` reader"]
pub struct R(crate::R<MTMON_VIOADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTMON_VIOADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTMON_VIOADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTMON_VIOADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - Byte address at which the error occurred, within the designated Flash"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Byte address at which the error occurred, within the designated Flash"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "Match Monitor Violation Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtmon_vioaddr](index.html) module"]
pub struct MTMON_VIOADDR_SPEC;
impl crate::RegisterSpec for MTMON_VIOADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtmon_vioaddr::R](R) reader structure"]
impl crate::Readable for MTMON_VIOADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MTMON_VIOADDR to value 0"]
impl crate::Resettable for MTMON_VIOADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
