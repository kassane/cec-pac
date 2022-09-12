#[doc = "Register `WCTRL` writer"]
pub struct W(crate::W<WCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCTRL_SPEC>;
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
impl From<crate::W<WCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACK` writer - The Acknowledge bit (ACK) must normally be asserted ('1'). This causes the controller to send an acknowledge automatically after each byte (this occurs during the 9th clock pulse). The ACK bit must not be asserted ('0') when the controller is operating in master/receiver mode and requires no further data to be sent from the slave transmitter. This causes a negative acknowledge on the I2C bus, which halts further transmission from the slave device."]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCTRL_SPEC, bool, O>;
#[doc = "Field `STO` writer - See STA description"]
pub type STO_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCTRL_SPEC, bool, O>;
#[doc = "Field `STA` writer - The STA and STO bits control the generation of the I2C Start condition and the transmission of the Slave Address and R/nW bit (from the Data Register), generation of repeated Start condition, and generation of the Stop condition"]
pub type STA_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCTRL_SPEC, bool, O>;
#[doc = "Field `ENI` writer - Enable Interrupt bit (ENI) controls the Interrupt Interface"]
pub type ENI_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCTRL_SPEC, bool, O>;
#[doc = "Field `ESO` writer - The Enable Serial Output bit (ESO) enables and disables the SMB Controller Core serial data output (SDAT)"]
pub type ESO_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCTRL_SPEC, bool, O>;
#[doc = "Field `PIN` writer - The Pending Interrupt Not (PIN) bit serves as a software reset function. Writing the PIN bit to a logic '1' de-asserts all status bits except for the nBB bit which is not affected by the PIN bit. The PIN bit is a self-clearing bit. Writing this bit to a logic '0' has no effect."]
pub type PIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCTRL_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - The Acknowledge bit (ACK) must normally be asserted ('1'). This causes the controller to send an acknowledge automatically after each byte (this occurs during the 9th clock pulse). The ACK bit must not be asserted ('0') when the controller is operating in master/receiver mode and requires no further data to be sent from the slave transmitter. This causes a negative acknowledge on the I2C bus, which halts further transmission from the slave device."]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W<0> {
        ACK_W::new(self)
    }
    #[doc = "Bit 1 - See STA description"]
    #[inline(always)]
    pub fn sto(&mut self) -> STO_W<1> {
        STO_W::new(self)
    }
    #[doc = "Bit 2 - The STA and STO bits control the generation of the I2C Start condition and the transmission of the Slave Address and R/nW bit (from the Data Register), generation of repeated Start condition, and generation of the Stop condition"]
    #[inline(always)]
    pub fn sta(&mut self) -> STA_W<2> {
        STA_W::new(self)
    }
    #[doc = "Bit 3 - Enable Interrupt bit (ENI) controls the Interrupt Interface"]
    #[inline(always)]
    pub fn eni(&mut self) -> ENI_W<3> {
        ENI_W::new(self)
    }
    #[doc = "Bit 6 - The Enable Serial Output bit (ESO) enables and disables the SMB Controller Core serial data output (SDAT)"]
    #[inline(always)]
    pub fn eso(&mut self) -> ESO_W<6> {
        ESO_W::new(self)
    }
    #[doc = "Bit 7 - The Pending Interrupt Not (PIN) bit serves as a software reset function. Writing the PIN bit to a logic '1' de-asserts all status bits except for the nBB bit which is not affected by the PIN bit. The PIN bit is a self-clearing bit. Writing this bit to a logic '0' has no effect."]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W<7> {
        PIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wctrl](index.html) module"]
pub struct WCTRL_SPEC;
impl crate::RegisterSpec for WCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wctrl::W](W) writer structure"]
impl crate::Writable for WCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WCTRL to value 0"]
impl crate::Resettable for WCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
