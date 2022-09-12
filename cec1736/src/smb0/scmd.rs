#[doc = "Register `SCMD[%s]` reader"]
pub struct R(crate::R<SCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCMD[%s]` writer"]
pub struct W(crate::W<SCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRUN` reader - Setting this bit to 1 enables the Slave State Machine to operate."]
pub type SRUN_R = crate::BitReader<bool>;
#[doc = "Field `SRUN` writer - Setting this bit to 1 enables the Slave State Machine to operate."]
pub type SRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCMD_SPEC, bool, O>;
#[doc = "Field `SPROCEED` reader - When this bit is 0, the Slave State Machine does not transition out of the IDLE, REPEAT_START_WRITE or REPEAT_START_READ states. When this bit is 1, the Slave State Machine immediately transitions to the START_WAIT, RECEIVE and TRANSMIT states, respectively."]
pub type SPROCEED_R = crate::BitReader<bool>;
#[doc = "Field `SPROCEED` writer - When this bit is 0, the Slave State Machine does not transition out of the IDLE, REPEAT_START_WRITE or REPEAT_START_READ states. When this bit is 1, the Slave State Machine immediately transitions to the START_WAIT, RECEIVE and TRANSMIT states, respectively."]
pub type SPROCEED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCMD_SPEC, bool, O>;
#[doc = "Field `PEC` reader - If Slave_WriteCount is 0 and Slave_PEC is 1 when the Master requests data, the PEC Register is copied to the DATA register. After the PEC Register is copied to the SMBus, the PEC Register is cleared and Slave_PEC is set to 0."]
pub type PEC_R = crate::BitReader<bool>;
#[doc = "Field `PEC` writer - If Slave_WriteCount is 0 and Slave_PEC is 1 when the Master requests data, the PEC Register is copied to the DATA register. After the PEC Register is copied to the SMBus, the PEC Register is cleared and Slave_PEC is set to 0."]
pub type PEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCMD_SPEC, bool, O>;
#[doc = "Field `WR_CNT` reader - This field is set to the number of bytes software expects to send to the Master."]
pub type WR_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_CNT` writer - This field is set to the number of bytes software expects to send to the Master."]
pub type WR_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCMD_SPEC, u8, u8, 8, O>;
#[doc = "Field `RD_CNT` reader - This field is decremented each time a byte is copied from DATA to the SMBus Slave Receive Buffer Register."]
pub type RD_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD_CNT` writer - This field is decremented each time a byte is copied from DATA to the SMBus Slave Receive Buffer Register."]
pub type RD_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCMD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Setting this bit to 1 enables the Slave State Machine to operate."]
    #[inline(always)]
    pub fn srun(&self) -> SRUN_R {
        SRUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is 0, the Slave State Machine does not transition out of the IDLE, REPEAT_START_WRITE or REPEAT_START_READ states. When this bit is 1, the Slave State Machine immediately transitions to the START_WAIT, RECEIVE and TRANSMIT states, respectively."]
    #[inline(always)]
    pub fn sproceed(&self) -> SPROCEED_R {
        SPROCEED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If Slave_WriteCount is 0 and Slave_PEC is 1 when the Master requests data, the PEC Register is copied to the DATA register. After the PEC Register is copied to the SMBus, the PEC Register is cleared and Slave_PEC is set to 0."]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - This field is set to the number of bytes software expects to send to the Master."]
    #[inline(always)]
    pub fn wr_cnt(&self) -> WR_CNT_R {
        WR_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This field is decremented each time a byte is copied from DATA to the SMBus Slave Receive Buffer Register."]
    #[inline(always)]
    pub fn rd_cnt(&self) -> RD_CNT_R {
        RD_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit to 1 enables the Slave State Machine to operate."]
    #[inline(always)]
    pub fn srun(&mut self) -> SRUN_W<0> {
        SRUN_W::new(self)
    }
    #[doc = "Bit 1 - When this bit is 0, the Slave State Machine does not transition out of the IDLE, REPEAT_START_WRITE or REPEAT_START_READ states. When this bit is 1, the Slave State Machine immediately transitions to the START_WAIT, RECEIVE and TRANSMIT states, respectively."]
    #[inline(always)]
    pub fn sproceed(&mut self) -> SPROCEED_W<1> {
        SPROCEED_W::new(self)
    }
    #[doc = "Bit 2 - If Slave_WriteCount is 0 and Slave_PEC is 1 when the Master requests data, the PEC Register is copied to the DATA register. After the PEC Register is copied to the SMBus, the PEC Register is cleared and Slave_PEC is set to 0."]
    #[inline(always)]
    pub fn pec(&mut self) -> PEC_W<2> {
        PEC_W::new(self)
    }
    #[doc = "Bits 8:15 - This field is set to the number of bytes software expects to send to the Master."]
    #[inline(always)]
    pub fn wr_cnt(&mut self) -> WR_CNT_W<8> {
        WR_CNT_W::new(self)
    }
    #[doc = "Bits 16:23 - This field is decremented each time a byte is copied from DATA to the SMBus Slave Receive Buffer Register."]
    #[inline(always)]
    pub fn rd_cnt(&mut self) -> RD_CNT_W<16> {
        RD_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMBus Slave Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmd](index.html) module"]
pub struct SCMD_SPEC;
impl crate::RegisterSpec for SCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scmd::R](R) reader structure"]
impl crate::Readable for SCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scmd::W](W) writer structure"]
impl crate::Writable for SCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCMD[%s]
to value 0"]
impl crate::Resettable for SCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
