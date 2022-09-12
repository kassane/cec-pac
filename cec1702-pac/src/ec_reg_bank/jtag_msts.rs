#[doc = "Register `JTAG_MSTS` reader"]
pub struct R(crate::R<JTAG_MSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JTAG_MSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JTAG_MSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JTAG_MSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JTM_DONE` reader - This bit is set to '1b' when the JTAG Master Command Register is written. It becomes '0b' when shifting has completed.\n Software can poll this bit to determine when a command has completed and it is therefore safe to remove the data in the JTAG Master TDO\n Register and load new data into the JTAG Master TMS Register and the JTAG Master TDI Register."]
pub type JTM_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This bit is set to '1b' when the JTAG Master Command Register is written. It becomes '0b' when shifting has completed.\n Software can poll this bit to determine when a command has completed and it is therefore safe to remove the data in the JTAG Master TDO\n Register and load new data into the JTAG Master TMS Register and the JTAG Master TDI Register."]
    #[inline(always)]
    pub fn jtm_done(&self) -> JTM_DONE_R {
        JTM_DONE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "JTAG Master Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtag_msts](index.html) module"]
pub struct JTAG_MSTS_SPEC;
impl crate::RegisterSpec for JTAG_MSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jtag_msts::R](R) reader structure"]
impl crate::Readable for JTAG_MSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JTAG_MSTS to value 0"]
impl crate::Resettable for JTAG_MSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
