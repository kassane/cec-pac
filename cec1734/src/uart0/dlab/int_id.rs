#[doc = "Register `INT_ID` reader"]
pub struct R(crate::R<INT_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IPEND` reader - IPEND This bit can be used in either a hardwired prioritized or polled environment to indicate whether an interrupt is pending."]
pub type IPEND_R = crate::BitReader<bool>;
#[doc = "Field `INTID` reader - INTID These bits identify the highest priority interrupt pending"]
pub type INTID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFO_EN` reader - These two bits are set when the FIFO CONTROL Register bit 0 equals 1."]
pub type FIFO_EN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - IPEND This bit can be used in either a hardwired prioritized or polled environment to indicate whether an interrupt is pending."]
    #[inline(always)]
    pub fn ipend(&self) -> IPEND_R {
        IPEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - INTID These bits identify the highest priority interrupt pending"]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 6:7 - These two bits are set when the FIFO CONTROL Register bit 0 equals 1."]
    #[inline(always)]
    pub fn fifo_en(&self) -> FIFO_EN_R {
        FIFO_EN_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "UART Interrupt Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_id](index.html) module"]
pub struct INT_ID_SPEC;
impl crate::RegisterSpec for INT_ID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [int_id::R](R) reader structure"]
impl crate::Readable for INT_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ID to value 0"]
impl crate::Resettable for INT_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
