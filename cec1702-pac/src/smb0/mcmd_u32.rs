#[doc = "Register `MCMD_u32` reader"]
pub struct R(crate::R<MCMD_U32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCMD_U32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCMD_U32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCMD_U32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCMD_u32` writer"]
pub struct W(crate::W<MCMD_U32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCMD_U32_SPEC>;
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
impl From<crate::W<MCMD_U32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCMD_U32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRUN` reader - While this bit is 1, transfer bytes over SMBus. As long as WriteCount is non-zero, a byte from the Master Transmit Buffer is transmitted to the slave device and WriteCount is decremented."]
pub type MRUN_R = crate::BitReader<bool>;
#[doc = "Field `MRUN` writer - While this bit is 1, transfer bytes over SMBus. As long as WriteCount is non-zero, a byte from the Master Transmit Buffer is transmitted to the slave device and WriteCount is decremented."]
pub type MRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMD_U32_SPEC, bool, O>;
#[doc = "Field `MPROCEED` reader - When this bit is 0, the Master State Machine does not transition out of the IDLE or PAUSE states. When this bit is 1, the Master State Machine immediately transitions to the WAIT-BUSBUSY and MRUN-RECEIVE states, respectively."]
pub type MPROCEED_R = crate::BitReader<bool>;
#[doc = "Field `MPROCEED` writer - When this bit is 0, the Master State Machine does not transition out of the IDLE or PAUSE states. When this bit is 1, the Master State Machine immediately transitions to the WAIT-BUSBUSY and MRUN-RECEIVE states, respectively."]
pub type MPROCEED_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMD_U32_SPEC, bool, O>;
#[doc = "Field `START0` reader - If this bit is 1, send a Start bit on the SMBus before the first byte of the WriteCount is sent to the SMBus transmitter."]
pub type START0_R = crate::BitReader<bool>;
#[doc = "Field `START0` writer - If this bit is 1, send a Start bit on the SMBus before the first byte of the WriteCount is sent to the SMBus transmitter."]
pub type START0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMD_U32_SPEC, bool, O>;
#[doc = "Field `STARTN` reader - If this bit is 1, send a Start bit just before the last byte of the WriteCount is sent to the SMBus transmitter."]
pub type STARTN_R = crate::BitReader<bool>;
#[doc = "Field `STARTN` writer - If this bit is 1, send a Start bit just before the last byte of the WriteCount is sent to the SMBus transmitter."]
pub type STARTN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMD_U32_SPEC, bool, O>;
#[doc = "Field `STOP` reader - If this bit is 1, send a Stop bit after the transaction completes."]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - If this bit is 1, send a Stop bit after the transaction completes."]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMD_U32_SPEC, bool, O>;
#[doc = "Field `PEC_TERM` reader - If this bit is 1, a copy of the PEC register is transmitted when WriteCount is 0. After the PEC register is read, both the PEC register and this bit are cleared to 0."]
pub type PEC_TERM_R = crate::BitReader<bool>;
#[doc = "Field `PEC_TERM` writer - If this bit is 1, a copy of the PEC register is transmitted when WriteCount is 0. After the PEC register is read, both the PEC register and this bit are cleared to 0."]
pub type PEC_TERM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMD_U32_SPEC, bool, O>;
#[doc = "Field `READM` reader - If this bit is 1, then the ReadCount field is replaced by the byte that is read from the SMBus when ReadCount\\[7:0\\]
is 1. After ReadCount\\[7:0\\]
is updated, this bit is cleared to 0."]
pub type READM_R = crate::BitReader<bool>;
#[doc = "Field `READM` writer - If this bit is 1, then the ReadCount field is replaced by the byte that is read from the SMBus when ReadCount\\[7:0\\]
is 1. After ReadCount\\[7:0\\]
is updated, this bit is cleared to 0."]
pub type READM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMD_U32_SPEC, bool, O>;
#[doc = "Field `RD_PEC` reader - If this bit is 0, reading from the SMBus stops when ReadCount reaches 0. If this bit is 1, reading continues when ReadCount is 0 for one more byte."]
pub type RD_PEC_R = crate::BitReader<bool>;
#[doc = "Field `RD_PEC` writer - If this bit is 0, reading from the SMBus stops when ReadCount reaches 0. If this bit is 1, reading continues when ReadCount is 0 for one more byte."]
pub type RD_PEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMD_U32_SPEC, bool, O>;
#[doc = "Field `WR_CNT` reader - This field is a count of the number of bytes to transmit to the SMBus from the SMBus Master Transmit Buffer Register It is decremented by 1 for each byte written to the SMBus from the SMBus Master Transmit Buffer Register."]
pub type WR_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_CNT` writer - This field is a count of the number of bytes to transmit to the SMBus from the SMBus Master Transmit Buffer Register It is decremented by 1 for each byte written to the SMBus from the SMBus Master Transmit Buffer Register."]
pub type WR_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCMD_U32_SPEC, u8, u8, 8, O>;
#[doc = "Field `RD_CNT` reader - This field is a count of the number of bytes to read in from the SMBus to the SMBus Master Receive Buffer Register and must be greater than 0 in order for the Master State Machine to initiate a read phase. It is decremented by 1 for each byte read from the SMBus into the SMBus Master Receive Buffer Register. It can be overwritten by the first byte read in from the SMBus."]
pub type RD_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD_CNT` writer - This field is a count of the number of bytes to read in from the SMBus to the SMBus Master Receive Buffer Register and must be greater than 0 in order for the Master State Machine to initiate a read phase. It is decremented by 1 for each byte read from the SMBus into the SMBus Master Receive Buffer Register. It can be overwritten by the first byte read in from the SMBus."]
pub type RD_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCMD_U32_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - While this bit is 1, transfer bytes over SMBus. As long as WriteCount is non-zero, a byte from the Master Transmit Buffer is transmitted to the slave device and WriteCount is decremented."]
    #[inline(always)]
    pub fn mrun(&self) -> MRUN_R {
        MRUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is 0, the Master State Machine does not transition out of the IDLE or PAUSE states. When this bit is 1, the Master State Machine immediately transitions to the WAIT-BUSBUSY and MRUN-RECEIVE states, respectively."]
    #[inline(always)]
    pub fn mproceed(&self) -> MPROCEED_R {
        MPROCEED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - If this bit is 1, send a Start bit on the SMBus before the first byte of the WriteCount is sent to the SMBus transmitter."]
    #[inline(always)]
    pub fn start0(&self) -> START0_R {
        START0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If this bit is 1, send a Start bit just before the last byte of the WriteCount is sent to the SMBus transmitter."]
    #[inline(always)]
    pub fn startn(&self) -> STARTN_R {
        STARTN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If this bit is 1, send a Stop bit after the transaction completes."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If this bit is 1, a copy of the PEC register is transmitted when WriteCount is 0. After the PEC register is read, both the PEC register and this bit are cleared to 0."]
    #[inline(always)]
    pub fn pec_term(&self) -> PEC_TERM_R {
        PEC_TERM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - If this bit is 1, then the ReadCount field is replaced by the byte that is read from the SMBus when ReadCount\\[7:0\\]
is 1. After ReadCount\\[7:0\\]
is updated, this bit is cleared to 0."]
    #[inline(always)]
    pub fn readm(&self) -> READM_R {
        READM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - If this bit is 0, reading from the SMBus stops when ReadCount reaches 0. If this bit is 1, reading continues when ReadCount is 0 for one more byte."]
    #[inline(always)]
    pub fn rd_pec(&self) -> RD_PEC_R {
        RD_PEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - This field is a count of the number of bytes to transmit to the SMBus from the SMBus Master Transmit Buffer Register It is decremented by 1 for each byte written to the SMBus from the SMBus Master Transmit Buffer Register."]
    #[inline(always)]
    pub fn wr_cnt(&self) -> WR_CNT_R {
        WR_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This field is a count of the number of bytes to read in from the SMBus to the SMBus Master Receive Buffer Register and must be greater than 0 in order for the Master State Machine to initiate a read phase. It is decremented by 1 for each byte read from the SMBus into the SMBus Master Receive Buffer Register. It can be overwritten by the first byte read in from the SMBus."]
    #[inline(always)]
    pub fn rd_cnt(&self) -> RD_CNT_R {
        RD_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - While this bit is 1, transfer bytes over SMBus. As long as WriteCount is non-zero, a byte from the Master Transmit Buffer is transmitted to the slave device and WriteCount is decremented."]
    #[inline(always)]
    pub fn mrun(&mut self) -> MRUN_W<0> {
        MRUN_W::new(self)
    }
    #[doc = "Bit 1 - When this bit is 0, the Master State Machine does not transition out of the IDLE or PAUSE states. When this bit is 1, the Master State Machine immediately transitions to the WAIT-BUSBUSY and MRUN-RECEIVE states, respectively."]
    #[inline(always)]
    pub fn mproceed(&mut self) -> MPROCEED_W<1> {
        MPROCEED_W::new(self)
    }
    #[doc = "Bit 8 - If this bit is 1, send a Start bit on the SMBus before the first byte of the WriteCount is sent to the SMBus transmitter."]
    #[inline(always)]
    pub fn start0(&mut self) -> START0_W<8> {
        START0_W::new(self)
    }
    #[doc = "Bit 9 - If this bit is 1, send a Start bit just before the last byte of the WriteCount is sent to the SMBus transmitter."]
    #[inline(always)]
    pub fn startn(&mut self) -> STARTN_W<9> {
        STARTN_W::new(self)
    }
    #[doc = "Bit 10 - If this bit is 1, send a Stop bit after the transaction completes."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<10> {
        STOP_W::new(self)
    }
    #[doc = "Bit 11 - If this bit is 1, a copy of the PEC register is transmitted when WriteCount is 0. After the PEC register is read, both the PEC register and this bit are cleared to 0."]
    #[inline(always)]
    pub fn pec_term(&mut self) -> PEC_TERM_W<11> {
        PEC_TERM_W::new(self)
    }
    #[doc = "Bit 12 - If this bit is 1, then the ReadCount field is replaced by the byte that is read from the SMBus when ReadCount\\[7:0\\]
is 1. After ReadCount\\[7:0\\]
is updated, this bit is cleared to 0."]
    #[inline(always)]
    pub fn readm(&mut self) -> READM_W<12> {
        READM_W::new(self)
    }
    #[doc = "Bit 13 - If this bit is 0, reading from the SMBus stops when ReadCount reaches 0. If this bit is 1, reading continues when ReadCount is 0 for one more byte."]
    #[inline(always)]
    pub fn rd_pec(&mut self) -> RD_PEC_W<13> {
        RD_PEC_W::new(self)
    }
    #[doc = "Bits 16:23 - This field is a count of the number of bytes to transmit to the SMBus from the SMBus Master Transmit Buffer Register It is decremented by 1 for each byte written to the SMBus from the SMBus Master Transmit Buffer Register."]
    #[inline(always)]
    pub fn wr_cnt(&mut self) -> WR_CNT_W<16> {
        WR_CNT_W::new(self)
    }
    #[doc = "Bits 24:31 - This field is a count of the number of bytes to read in from the SMBus to the SMBus Master Receive Buffer Register and must be greater than 0 in order for the Master State Machine to initiate a read phase. It is decremented by 1 for each byte read from the SMBus into the SMBus Master Receive Buffer Register. It can be overwritten by the first byte read in from the SMBus."]
    #[inline(always)]
    pub fn rd_cnt(&mut self) -> RD_CNT_W<24> {
        RD_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMBus Master Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmd_u32](index.html) module"]
pub struct MCMD_U32_SPEC;
impl crate::RegisterSpec for MCMD_U32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcmd_u32::R](R) reader structure"]
impl crate::Readable for MCMD_U32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcmd_u32::W](W) writer structure"]
impl crate::Writable for MCMD_U32_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCMD_u32 to value 0"]
impl crate::Resettable for MCMD_U32_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
