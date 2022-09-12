#[doc = "Register `RSTS` reader"]
pub struct R(crate::R<RSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NBB` reader - The Bus Busy bit (NBB) is a read-only flag indicating when the bus is in use. A zero indicates that the bus is busy and access is not possible."]
pub type NBB_R = crate::BitReader<bool>;
#[doc = "Field `LAB` reader - The Lost Arbitration Bit (LAB) is set when, in multi-master operation, arbitration is lost to another master on the bus"]
pub type LAB_R = crate::BitReader<bool>;
#[doc = "Field `AAS` reader - The Addressed As Slave bit (AAS) is valid only when PIN is asserted ('0'). When acting as slave, AAS is set when an incoming address over the bus matches the value in the Own Address Register or if the 'general call' address (00h) has been received"]
pub type AAS_R = crate::BitReader<bool>;
#[doc = "Field `LRB_AD0` reader - The Last Received Bit or Address 0 (general call) bit (LRB/AD0) serves a dual function and is valid only while the PIN bit is asserted ('0').\n When the AAS bit is not asserted ('0') (i.e., not addressed as a slave), the LRB/AD0 holds the value of the last received bit over the bus.\n When the AAS bit is asserted ('1') (i.e., addressed as slave), the SMB Controller Core has been addressed as a slave."]
pub type LRB_AD0_R = crate::BitReader<bool>;
#[doc = "Field `BER` reader - When Bus Error (BER) is asserted, a misplaced START or STOP condition or Bus Time-Outs have been detected."]
pub type BER_R = crate::BitReader<bool>;
#[doc = "Field `STS` reader - When in slave receiver mode, STS is asserted ('1') when an externally generated STOP condition is detected. Note that STS is used only in slave receiver mode."]
pub type STS_R = crate::BitReader<bool>;
#[doc = "Field `SAD` reader - SMBus Address Decoded (SAD)"]
pub type SAD_R = crate::BitReader<bool>;
#[doc = "Field `PIN` reader - Pending Interrupt bit"]
pub type PIN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The Bus Busy bit (NBB) is a read-only flag indicating when the bus is in use. A zero indicates that the bus is busy and access is not possible."]
    #[inline(always)]
    pub fn nbb(&self) -> NBB_R {
        NBB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The Lost Arbitration Bit (LAB) is set when, in multi-master operation, arbitration is lost to another master on the bus"]
    #[inline(always)]
    pub fn lab(&self) -> LAB_R {
        LAB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The Addressed As Slave bit (AAS) is valid only when PIN is asserted ('0'). When acting as slave, AAS is set when an incoming address over the bus matches the value in the Own Address Register or if the 'general call' address (00h) has been received"]
    #[inline(always)]
    pub fn aas(&self) -> AAS_R {
        AAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The Last Received Bit or Address 0 (general call) bit (LRB/AD0) serves a dual function and is valid only while the PIN bit is asserted ('0').\n When the AAS bit is not asserted ('0') (i.e., not addressed as a slave), the LRB/AD0 holds the value of the last received bit over the bus.\n When the AAS bit is asserted ('1') (i.e., addressed as slave), the SMB Controller Core has been addressed as a slave."]
    #[inline(always)]
    pub fn lrb_ad0(&self) -> LRB_AD0_R {
        LRB_AD0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When Bus Error (BER) is asserted, a misplaced START or STOP condition or Bus Time-Outs have been detected."]
    #[inline(always)]
    pub fn ber(&self) -> BER_R {
        BER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When in slave receiver mode, STS is asserted ('1') when an externally generated STOP condition is detected. Note that STS is used only in slave receiver mode."]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus Address Decoded (SAD)"]
    #[inline(always)]
    pub fn sad(&self) -> SAD_R {
        SAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending Interrupt bit"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsts](index.html) module"]
pub struct RSTS_SPEC;
impl crate::RegisterSpec for RSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsts::R](R) reader structure"]
impl crate::Readable for RSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTS to value 0"]
impl crate::Resettable for RSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
