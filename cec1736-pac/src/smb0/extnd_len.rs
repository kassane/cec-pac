#[doc = "Register `EXTND_LEN` reader"]
pub struct R(crate::R<EXTND_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTND_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTND_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTND_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTND_LEN` reader - Extended Length Register adds 8 MSB bits to the SMBUS Master/Slave Tx/Rx Length fields."]
pub type EXTND_LEN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Extended Length Register adds 8 MSB bits to the SMBUS Master/Slave Tx/Rx Length fields."]
    #[inline(always)]
    pub fn extnd_len(&self) -> EXTND_LEN_R {
        EXTND_LEN_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Extended Length Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extnd_len](index.html) module"]
pub struct EXTND_LEN_SPEC;
impl crate::RegisterSpec for EXTND_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extnd_len::R](R) reader structure"]
impl crate::Readable for EXTND_LEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTND_LEN to value 0"]
impl crate::Resettable for EXTND_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
