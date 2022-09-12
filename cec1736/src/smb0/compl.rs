#[doc = "Register `COMPL[%s]` reader"]
pub struct R(crate::R<COMPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPL[%s]` writer"]
pub struct W(crate::W<COMPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPL_SPEC>;
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
impl From<crate::W<COMPL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTEN` reader - When DTEN is asserted ('1'), Device Time-out checking is enabled. When DTEN is not asserted ('0'), Device Time-out checking is disabled."]
pub type DTEN_R = crate::BitReader<bool>;
#[doc = "Field `DTEN` writer - When DTEN is asserted ('1'), Device Time-out checking is enabled. When DTEN is not asserted ('0'), Device Time-out checking is disabled."]
pub type DTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `MCEN` reader - When MCEN is asserted ('1'), Master Cumulative Time-Out checking is enabled. When MCEN is not asserted ('0'), Master Cumulative Time-Out checking is disabled."]
pub type MCEN_R = crate::BitReader<bool>;
#[doc = "Field `MCEN` writer - When MCEN is asserted ('1'), Master Cumulative Time-Out checking is enabled. When MCEN is not asserted ('0'), Master Cumulative Time-Out checking is disabled."]
pub type MCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `SCEN` reader - When SCEN is asserted ('1'), Slave Cumulative Time-Out checking is enabled. When SCEN is not asserted ('0'), Slave Cumulative Time-Out checking is disabled."]
pub type SCEN_R = crate::BitReader<bool>;
#[doc = "Field `SCEN` writer - When SCEN is asserted ('1'), Slave Cumulative Time-Out checking is enabled. When SCEN is not asserted ('0'), Slave Cumulative Time-Out checking is disabled."]
pub type SCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `BIDEN` reader - When BIDEN is asserted ('1'), Bus Idle Detect Time-Out checking is enabled. When BIDEN is not asserted ('0'), Bus Idle Detect Time-Out checking is disabled."]
pub type BIDEN_R = crate::BitReader<bool>;
#[doc = "Field `BIDEN` writer - When BIDEN is asserted ('1'), Bus Idle Detect Time-Out checking is enabled. When BIDEN is not asserted ('0'), Bus Idle Detect Time-Out checking is disabled."]
pub type BIDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `TIMERR` reader - The Time-out Error Detected bit (TIMERR) is asserted ('1') whenever any of the enabled time-out error detect status bits (CHDH, CHDL, SCTO, MCTO and DTO) are asserted."]
pub type TIMERR_R = crate::BitReader<bool>;
#[doc = "Field `TIMERR` writer - The Time-out Error Detected bit (TIMERR) is asserted ('1') whenever any of the enabled time-out error detect status bits (CHDH, CHDL, SCTO, MCTO and DTO) are asserted."]
pub type TIMERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `DTO` reader - DTO is the Device Time-out bit. (R/WC)"]
pub type DTO_R = crate::BitReader<bool>;
#[doc = "Field `DTO` writer - DTO is the Device Time-out bit. (R/WC)"]
pub type DTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `MCTO` reader - MCTO is the Master Cumulative Time-out bit. (R/WC)"]
pub type MCTO_R = crate::BitReader<bool>;
#[doc = "Field `MCTO` writer - MCTO is the Master Cumulative Time-out bit. (R/WC)"]
pub type MCTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `SCTO` reader - SCTO is the Slave Cumulative Time-out bit(R/WC)"]
pub type SCTO_R = crate::BitReader<bool>;
#[doc = "Field `SCTO` writer - SCTO is the Slave Cumulative Time-out bit(R/WC)"]
pub type SCTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `CHDL` reader - CHDL is the clock high time-out detect bit(R/WC)"]
pub type CHDL_R = crate::BitReader<bool>;
#[doc = "Field `CHDL` writer - CHDL is the clock high time-out detect bit(R/WC)"]
pub type CHDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `CHDH` reader - CHDH is the bus idle time-out detect bit(R/WC)"]
pub type CHDH_R = crate::BitReader<bool>;
#[doc = "Field `CHDH` writer - CHDH is the bus idle time-out detect bit(R/WC)"]
pub type CHDH_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `BER` reader - If this bit is 1, the BER bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
pub type BER_R = crate::BitReader<bool>;
#[doc = "Field `BER` writer - If this bit is 1, the BER bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
pub type BER_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `LAB` reader - If this bit is 1, the LAB bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
pub type LAB_R = crate::BitReader<bool>;
#[doc = "Field `LAB` writer - If this bit is 1, the LAB bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
pub type LAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `SNAKR` reader - If this bit is 1, the Slave state machine sent a NACK to the transmitting Master while the Slave was receiving data from the SMBus interface."]
pub type SNAKR_R = crate::BitReader<bool>;
#[doc = "Field `SNAKR` writer - If this bit is 1, the Slave state machine sent a NACK to the transmitting Master while the Slave was receiving data from the SMBus interface."]
pub type SNAKR_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `STR` reader - 0: Slave has just finished the receive phase of a transaction. 1: Slave has just finished the transmit phase of a transaction."]
pub type STR_R = crate::BitReader<bool>;
#[doc = "Field `STR` writer - 0: Slave has just finished the receive phase of a transaction. 1: Slave has just finished the transmit phase of a transaction."]
pub type STR_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `SPROT` reader - If this bit is 1, the WriteCount\\[7:0\\]
counter in the Slave state machine either counted down to 0 before the Master sent a NACK signal, or the Slave received a NACK signal before the counter reached 0."]
pub type SPROT_R = crate::BitReader<bool>;
#[doc = "Field `SPROT` writer - If this bit is 1, the WriteCount\\[7:0\\]
counter in the Slave state machine either counted down to 0 before the Master sent a NACK signal, or the Slave received a NACK signal before the counter reached 0."]
pub type SPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `REP_RD` reader - If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\]
of the byte containing the slave address equal to 1, indicating that the Master requested a Read operation."]
pub type REP_RD_R = crate::BitReader<bool>;
#[doc = "Field `REP_RD` writer - If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\]
of the byte containing the slave address equal to 1, indicating that the Master requested a Read operation."]
pub type REP_RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `REP_WR` reader - If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\]
of the byte containing the slave address equal to 0, indicating that the Master requested a Write operation."]
pub type REP_WR_R = crate::BitReader<bool>;
#[doc = "Field `REP_WR` writer - If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\]
of the byte containing the slave address equal to 0, indicating that the Master requested a Write operation."]
pub type REP_WR_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `MNAKX` reader - If this bit is 1, the Master state machine received a NACK from the receiving Slave while the Master was transmitting data over the SMBus interface. (R/WC)"]
pub type MNAKX_R = crate::BitReader<bool>;
#[doc = "Field `MNAKX` writer - If this bit is 1, the Master state machine received a NACK from the receiving Slave while the Master was transmitting data over the SMBus interface. (R/WC)"]
pub type MNAKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `MTR` reader - 0: Master has just finished the receive phase of a transaction. 1: Master has just finished the transmit phase of a transaction."]
pub type MTR_R = crate::BitReader<bool>;
#[doc = "Field `MTR` writer - 0: Master has just finished the receive phase of a transaction. 1: Master has just finished the transmit phase of a transaction."]
pub type MTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `IDLE` reader - This bit is set when the I2C bus becomes idle (on the rising edge of nBB). (R/WC)"]
pub type IDLE_R = crate::BitReader<bool>;
#[doc = "Field `IDLE` writer - This bit is set when the I2C bus becomes idle (on the rising edge of nBB). (R/WC)"]
pub type IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `MDONE` reader - If this bit is 1, Master State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect. (R/WC)"]
pub type MDONE_R = crate::BitReader<bool>;
#[doc = "Field `MDONE` writer - If this bit is 1, Master State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect. (R/WC)"]
pub type MDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
#[doc = "Field `SDONE` reader - If this bit is 1, Slave State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect.(R/WC)"]
pub type SDONE_R = crate::BitReader<bool>;
#[doc = "Field `SDONE` writer - If this bit is 1, Slave State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect.(R/WC)"]
pub type SDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - When DTEN is asserted ('1'), Device Time-out checking is enabled. When DTEN is not asserted ('0'), Device Time-out checking is disabled."]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When MCEN is asserted ('1'), Master Cumulative Time-Out checking is enabled. When MCEN is not asserted ('0'), Master Cumulative Time-Out checking is disabled."]
    #[inline(always)]
    pub fn mcen(&self) -> MCEN_R {
        MCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When SCEN is asserted ('1'), Slave Cumulative Time-Out checking is enabled. When SCEN is not asserted ('0'), Slave Cumulative Time-Out checking is disabled."]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When BIDEN is asserted ('1'), Bus Idle Detect Time-Out checking is enabled. When BIDEN is not asserted ('0'), Bus Idle Detect Time-Out checking is disabled."]
    #[inline(always)]
    pub fn biden(&self) -> BIDEN_R {
        BIDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The Time-out Error Detected bit (TIMERR) is asserted ('1') whenever any of the enabled time-out error detect status bits (CHDH, CHDL, SCTO, MCTO and DTO) are asserted."]
    #[inline(always)]
    pub fn timerr(&self) -> TIMERR_R {
        TIMERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - DTO is the Device Time-out bit. (R/WC)"]
    #[inline(always)]
    pub fn dto(&self) -> DTO_R {
        DTO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MCTO is the Master Cumulative Time-out bit. (R/WC)"]
    #[inline(always)]
    pub fn mcto(&self) -> MCTO_R {
        MCTO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SCTO is the Slave Cumulative Time-out bit(R/WC)"]
    #[inline(always)]
    pub fn scto(&self) -> SCTO_R {
        SCTO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CHDL is the clock high time-out detect bit(R/WC)"]
    #[inline(always)]
    pub fn chdl(&self) -> CHDL_R {
        CHDL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CHDH is the bus idle time-out detect bit(R/WC)"]
    #[inline(always)]
    pub fn chdh(&self) -> CHDH_R {
        CHDH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - If this bit is 1, the BER bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
    #[inline(always)]
    pub fn ber(&self) -> BER_R {
        BER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If this bit is 1, the LAB bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
    #[inline(always)]
    pub fn lab(&self) -> LAB_R {
        LAB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - If this bit is 1, the Slave state machine sent a NACK to the transmitting Master while the Slave was receiving data from the SMBus interface."]
    #[inline(always)]
    pub fn snakr(&self) -> SNAKR_R {
        SNAKR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: Slave has just finished the receive phase of a transaction. 1: Slave has just finished the transmit phase of a transaction."]
    #[inline(always)]
    pub fn str(&self) -> STR_R {
        STR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - If this bit is 1, the WriteCount\\[7:0\\]
counter in the Slave state machine either counted down to 0 before the Master sent a NACK signal, or the Slave received a NACK signal before the counter reached 0."]
    #[inline(always)]
    pub fn sprot(&self) -> SPROT_R {
        SPROT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\]
of the byte containing the slave address equal to 1, indicating that the Master requested a Read operation."]
    #[inline(always)]
    pub fn rep_rd(&self) -> REP_RD_R {
        REP_RD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\]
of the byte containing the slave address equal to 0, indicating that the Master requested a Write operation."]
    #[inline(always)]
    pub fn rep_wr(&self) -> REP_WR_R {
        REP_WR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - If this bit is 1, the Master state machine received a NACK from the receiving Slave while the Master was transmitting data over the SMBus interface. (R/WC)"]
    #[inline(always)]
    pub fn mnakx(&self) -> MNAKX_R {
        MNAKX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 0: Master has just finished the receive phase of a transaction. 1: Master has just finished the transmit phase of a transaction."]
    #[inline(always)]
    pub fn mtr(&self) -> MTR_R {
        MTR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit is set when the I2C bus becomes idle (on the rising edge of nBB). (R/WC)"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - If this bit is 1, Master State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect. (R/WC)"]
    #[inline(always)]
    pub fn mdone(&self) -> MDONE_R {
        MDONE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If this bit is 1, Slave State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect.(R/WC)"]
    #[inline(always)]
    pub fn sdone(&self) -> SDONE_R {
        SDONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - When DTEN is asserted ('1'), Device Time-out checking is enabled. When DTEN is not asserted ('0'), Device Time-out checking is disabled."]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W<2> {
        DTEN_W::new(self)
    }
    #[doc = "Bit 3 - When MCEN is asserted ('1'), Master Cumulative Time-Out checking is enabled. When MCEN is not asserted ('0'), Master Cumulative Time-Out checking is disabled."]
    #[inline(always)]
    pub fn mcen(&mut self) -> MCEN_W<3> {
        MCEN_W::new(self)
    }
    #[doc = "Bit 4 - When SCEN is asserted ('1'), Slave Cumulative Time-Out checking is enabled. When SCEN is not asserted ('0'), Slave Cumulative Time-Out checking is disabled."]
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W<4> {
        SCEN_W::new(self)
    }
    #[doc = "Bit 5 - When BIDEN is asserted ('1'), Bus Idle Detect Time-Out checking is enabled. When BIDEN is not asserted ('0'), Bus Idle Detect Time-Out checking is disabled."]
    #[inline(always)]
    pub fn biden(&mut self) -> BIDEN_W<5> {
        BIDEN_W::new(self)
    }
    #[doc = "Bit 6 - The Time-out Error Detected bit (TIMERR) is asserted ('1') whenever any of the enabled time-out error detect status bits (CHDH, CHDL, SCTO, MCTO and DTO) are asserted."]
    #[inline(always)]
    pub fn timerr(&mut self) -> TIMERR_W<6> {
        TIMERR_W::new(self)
    }
    #[doc = "Bit 8 - DTO is the Device Time-out bit. (R/WC)"]
    #[inline(always)]
    pub fn dto(&mut self) -> DTO_W<8> {
        DTO_W::new(self)
    }
    #[doc = "Bit 9 - MCTO is the Master Cumulative Time-out bit. (R/WC)"]
    #[inline(always)]
    pub fn mcto(&mut self) -> MCTO_W<9> {
        MCTO_W::new(self)
    }
    #[doc = "Bit 10 - SCTO is the Slave Cumulative Time-out bit(R/WC)"]
    #[inline(always)]
    pub fn scto(&mut self) -> SCTO_W<10> {
        SCTO_W::new(self)
    }
    #[doc = "Bit 11 - CHDL is the clock high time-out detect bit(R/WC)"]
    #[inline(always)]
    pub fn chdl(&mut self) -> CHDL_W<11> {
        CHDL_W::new(self)
    }
    #[doc = "Bit 12 - CHDH is the bus idle time-out detect bit(R/WC)"]
    #[inline(always)]
    pub fn chdh(&mut self) -> CHDH_W<12> {
        CHDH_W::new(self)
    }
    #[doc = "Bit 13 - If this bit is 1, the BER bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
    #[inline(always)]
    pub fn ber(&mut self) -> BER_W<13> {
        BER_W::new(self)
    }
    #[doc = "Bit 14 - If this bit is 1, the LAB bit in the Status register was set while either the Slave state machine or the Master state machine was not in the Idle state.(R/WC)"]
    #[inline(always)]
    pub fn lab(&mut self) -> LAB_W<14> {
        LAB_W::new(self)
    }
    #[doc = "Bit 16 - If this bit is 1, the Slave state machine sent a NACK to the transmitting Master while the Slave was receiving data from the SMBus interface."]
    #[inline(always)]
    pub fn snakr(&mut self) -> SNAKR_W<16> {
        SNAKR_W::new(self)
    }
    #[doc = "Bit 17 - 0: Slave has just finished the receive phase of a transaction. 1: Slave has just finished the transmit phase of a transaction."]
    #[inline(always)]
    pub fn str(&mut self) -> STR_W<17> {
        STR_W::new(self)
    }
    #[doc = "Bit 19 - If this bit is 1, the WriteCount\\[7:0\\]
counter in the Slave state machine either counted down to 0 before the Master sent a NACK signal, or the Slave received a NACK signal before the counter reached 0."]
    #[inline(always)]
    pub fn sprot(&mut self) -> SPROT_W<19> {
        SPROT_W::new(self)
    }
    #[doc = "Bit 20 - If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\]
of the byte containing the slave address equal to 1, indicating that the Master requested a Read operation."]
    #[inline(always)]
    pub fn rep_rd(&mut self) -> REP_RD_W<20> {
        REP_RD_W::new(self)
    }
    #[doc = "Bit 21 - If this bit is 1, the Slave State Machine stopped because it detected a Repeat Start bit, with bit\\[0\\]
of the byte containing the slave address equal to 0, indicating that the Master requested a Write operation."]
    #[inline(always)]
    pub fn rep_wr(&mut self) -> REP_WR_W<21> {
        REP_WR_W::new(self)
    }
    #[doc = "Bit 24 - If this bit is 1, the Master state machine received a NACK from the receiving Slave while the Master was transmitting data over the SMBus interface. (R/WC)"]
    #[inline(always)]
    pub fn mnakx(&mut self) -> MNAKX_W<24> {
        MNAKX_W::new(self)
    }
    #[doc = "Bit 25 - 0: Master has just finished the receive phase of a transaction. 1: Master has just finished the transmit phase of a transaction."]
    #[inline(always)]
    pub fn mtr(&mut self) -> MTR_W<25> {
        MTR_W::new(self)
    }
    #[doc = "Bit 29 - This bit is set when the I2C bus becomes idle (on the rising edge of nBB). (R/WC)"]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W<29> {
        IDLE_W::new(self)
    }
    #[doc = "Bit 30 - If this bit is 1, Master State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect. (R/WC)"]
    #[inline(always)]
    pub fn mdone(&mut self) -> MDONE_W<30> {
        MDONE_W::new(self)
    }
    #[doc = "Bit 31 - If this bit is 1, Slave State machine completed operation and returned to the Idle state. It is cleared when written with a 1. Writes of a 0 have no effect.(R/WC)"]
    #[inline(always)]
    pub fn sdone(&mut self) -> SDONE_W<31> {
        SDONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Completion Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compl](index.html) module"]
pub struct COMPL_SPEC;
impl crate::RegisterSpec for COMPL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [compl::R](R) reader structure"]
impl crate::Readable for COMPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compl::W](W) writer structure"]
impl crate::Writable for COMPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMPL[%s]
to value 0"]
impl crate::Resettable for COMPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
