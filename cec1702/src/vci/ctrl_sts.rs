#[doc = "Register `CTRL_STS` reader"]
pub struct R(crate::R<CTRL_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_STS` writer"]
pub struct W(crate::W<CTRL_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_STS_SPEC>;
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
impl From<crate::W<CTRL_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCI_IN` reader - These bits provide the latched state of the associated VCI_IN# pin, if latching is enabled or the current state of the pin\n if latching is not enabled. In both cases, the value is determined after the action of the VCI Polarity Register.\n Note: The VCI_IN\\[6:0\\]# bits default to the state of their respective input pins."]
pub type VCI_IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCI_IN` writer - These bits provide the latched state of the associated VCI_IN# pin, if latching is enabled or the current state of the pin\n if latching is not enabled. In both cases, the value is determined after the action of the VCI Polarity Register.\n Note: The VCI_IN\\[6:0\\]# bits default to the state of their respective input pins."]
pub type VCI_IN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_STS_SPEC, u8, u8, 7, O>;
#[doc = "Field `VCI_OVRD_IN` reader - This bit provides the current status of the VCI_OVRD_IN pin.\n Note: The VCI_OVRD_IN bit defaults to the state of the respective input pin."]
pub type VCI_OVRD_IN_R = crate::BitReader<bool>;
#[doc = "Field `VCI_OVRD_IN` writer - This bit provides the current status of the VCI_OVRD_IN pin.\n Note: The VCI_OVRD_IN bit defaults to the state of the respective input pin."]
pub type VCI_OVRD_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_STS_SPEC, bool, O>;
#[doc = "Field `VCI_OUT` reader - This bit provides the current status of the VCI_OUT pin."]
pub type VCI_OUT_R = crate::BitReader<bool>;
#[doc = "Field `VCI_OUT` writer - This bit provides the current status of the VCI_OUT pin."]
pub type VCI_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_STS_SPEC, bool, O>;
#[doc = "Field `VCI_FW_CTRL` reader - This bit can allow EC firmware to control the state of the VCI_OUT pin. For example, when VTR_PWRGD is asserted and the\n FW_EXT bit is 1, clearing the VCI_FW_CNTRL bit de-asserts the active high VCI_OUT pin. BIOS must set this bit to 1 prior to setting\n the FW_EXT bit to 1 on power up, in order to avoid glitches on the VCI_OUT pin."]
pub type VCI_FW_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `VCI_FW_CTRL` writer - This bit can allow EC firmware to control the state of the VCI_OUT pin. For example, when VTR_PWRGD is asserted and the\n FW_EXT bit is 1, clearing the VCI_FW_CNTRL bit de-asserts the active high VCI_OUT pin. BIOS must set this bit to 1 prior to setting\n the FW_EXT bit to 1 on power up, in order to avoid glitches on the VCI_OUT pin."]
pub type VCI_FW_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_STS_SPEC, bool, O>;
#[doc = "Field `FW_EXT` reader - This bit controls selecting between the external VBAT-Powered Control Interface inputs, or the VCI_FW_CNTRL bit output to control the VCI_OUT pin.\n 1=VCI_OUT is determined by the VCI_FW_CNTRL field, when VTR is active\n 0=VCI_OUT is determined by the external inputs.\n Note: This bit used to be called GPO/nEXT. The name was changed to distinguish it from the BGPOs, which are elsewhere, and to remove a / in a bit name."]
pub type FW_EXT_R = crate::BitReader<bool>;
#[doc = "Field `FW_EXT` writer - This bit controls selecting between the external VBAT-Powered Control Interface inputs, or the VCI_FW_CNTRL bit output to control the VCI_OUT pin.\n 1=VCI_OUT is determined by the VCI_FW_CNTRL field, when VTR is active\n 0=VCI_OUT is determined by the external inputs.\n Note: This bit used to be called GPO/nEXT. The name was changed to distinguish it from the BGPOs, which are elsewhere, and to remove a / in a bit name."]
pub type FW_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_STS_SPEC, bool, O>;
#[doc = "Field `FLTRS_BYPASS` reader - The Filters Bypass bit is used to enable and disable the input filters on the VCI_IN# pins.\n 1=Filters disabled; 0=Filters enabled (default)."]
pub type FLTRS_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `FLTRS_BYPASS` writer - The Filters Bypass bit is used to enable and disable the input filters on the VCI_IN# pins.\n 1=Filters disabled; 0=Filters enabled (default)."]
pub type FLTRS_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_STS_SPEC, bool, O>;
#[doc = "Field `WK_ALRM` reader - If enabled by WK_ALRM_LE, this bit is set to 1 if the Week Alarm signal is asserted. It is reset by writes to WK_ALRM_LS."]
pub type WK_ALRM_R = crate::BitReader<bool>;
#[doc = "Field `WK_ALRM` writer - If enabled by WK_ALRM_LE, this bit is set to 1 if the Week Alarm signal is asserted. It is reset by writes to WK_ALRM_LS."]
pub type WK_ALRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_STS_SPEC, bool, O>;
#[doc = "Field `RTC_ALRM` reader - If enabled by RTC_ALRM_LE, this bit is set to 1 if the RTC Alarm signal is asserted. It is reset by writes to RTC_ALRM_LS."]
pub type RTC_ALRM_R = crate::BitReader<bool>;
#[doc = "Field `RTC_ALRM` writer - If enabled by RTC_ALRM_LE, this bit is set to 1 if the RTC Alarm signal is asserted. It is reset by writes to RTC_ALRM_LS."]
pub type RTC_ALRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - These bits provide the latched state of the associated VCI_IN# pin, if latching is enabled or the current state of the pin\n if latching is not enabled. In both cases, the value is determined after the action of the VCI Polarity Register.\n Note: The VCI_IN\\[6:0\\]# bits default to the state of their respective input pins."]
    #[inline(always)]
    pub fn vci_in(&self) -> VCI_IN_R {
        VCI_IN_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - This bit provides the current status of the VCI_OVRD_IN pin.\n Note: The VCI_OVRD_IN bit defaults to the state of the respective input pin."]
    #[inline(always)]
    pub fn vci_ovrd_in(&self) -> VCI_OVRD_IN_R {
        VCI_OVRD_IN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit provides the current status of the VCI_OUT pin."]
    #[inline(always)]
    pub fn vci_out(&self) -> VCI_OUT_R {
        VCI_OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit can allow EC firmware to control the state of the VCI_OUT pin. For example, when VTR_PWRGD is asserted and the\n FW_EXT bit is 1, clearing the VCI_FW_CNTRL bit de-asserts the active high VCI_OUT pin. BIOS must set this bit to 1 prior to setting\n the FW_EXT bit to 1 on power up, in order to avoid glitches on the VCI_OUT pin."]
    #[inline(always)]
    pub fn vci_fw_ctrl(&self) -> VCI_FW_CTRL_R {
        VCI_FW_CTRL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit controls selecting between the external VBAT-Powered Control Interface inputs, or the VCI_FW_CNTRL bit output to control the VCI_OUT pin.\n 1=VCI_OUT is determined by the VCI_FW_CNTRL field, when VTR is active\n 0=VCI_OUT is determined by the external inputs.\n Note: This bit used to be called GPO/nEXT. The name was changed to distinguish it from the BGPOs, which are elsewhere, and to remove a / in a bit name."]
    #[inline(always)]
    pub fn fw_ext(&self) -> FW_EXT_R {
        FW_EXT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The Filters Bypass bit is used to enable and disable the input filters on the VCI_IN# pins.\n 1=Filters disabled; 0=Filters enabled (default)."]
    #[inline(always)]
    pub fn fltrs_bypass(&self) -> FLTRS_BYPASS_R {
        FLTRS_BYPASS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - If enabled by WK_ALRM_LE, this bit is set to 1 if the Week Alarm signal is asserted. It is reset by writes to WK_ALRM_LS."]
    #[inline(always)]
    pub fn wk_alrm(&self) -> WK_ALRM_R {
        WK_ALRM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If enabled by RTC_ALRM_LE, this bit is set to 1 if the RTC Alarm signal is asserted. It is reset by writes to RTC_ALRM_LS."]
    #[inline(always)]
    pub fn rtc_alrm(&self) -> RTC_ALRM_R {
        RTC_ALRM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - These bits provide the latched state of the associated VCI_IN# pin, if latching is enabled or the current state of the pin\n if latching is not enabled. In both cases, the value is determined after the action of the VCI Polarity Register.\n Note: The VCI_IN\\[6:0\\]# bits default to the state of their respective input pins."]
    #[inline(always)]
    pub fn vci_in(&mut self) -> VCI_IN_W<0> {
        VCI_IN_W::new(self)
    }
    #[doc = "Bit 8 - This bit provides the current status of the VCI_OVRD_IN pin.\n Note: The VCI_OVRD_IN bit defaults to the state of the respective input pin."]
    #[inline(always)]
    pub fn vci_ovrd_in(&mut self) -> VCI_OVRD_IN_W<8> {
        VCI_OVRD_IN_W::new(self)
    }
    #[doc = "Bit 9 - This bit provides the current status of the VCI_OUT pin."]
    #[inline(always)]
    pub fn vci_out(&mut self) -> VCI_OUT_W<9> {
        VCI_OUT_W::new(self)
    }
    #[doc = "Bit 10 - This bit can allow EC firmware to control the state of the VCI_OUT pin. For example, when VTR_PWRGD is asserted and the\n FW_EXT bit is 1, clearing the VCI_FW_CNTRL bit de-asserts the active high VCI_OUT pin. BIOS must set this bit to 1 prior to setting\n the FW_EXT bit to 1 on power up, in order to avoid glitches on the VCI_OUT pin."]
    #[inline(always)]
    pub fn vci_fw_ctrl(&mut self) -> VCI_FW_CTRL_W<10> {
        VCI_FW_CTRL_W::new(self)
    }
    #[doc = "Bit 11 - This bit controls selecting between the external VBAT-Powered Control Interface inputs, or the VCI_FW_CNTRL bit output to control the VCI_OUT pin.\n 1=VCI_OUT is determined by the VCI_FW_CNTRL field, when VTR is active\n 0=VCI_OUT is determined by the external inputs.\n Note: This bit used to be called GPO/nEXT. The name was changed to distinguish it from the BGPOs, which are elsewhere, and to remove a / in a bit name."]
    #[inline(always)]
    pub fn fw_ext(&mut self) -> FW_EXT_W<11> {
        FW_EXT_W::new(self)
    }
    #[doc = "Bit 12 - The Filters Bypass bit is used to enable and disable the input filters on the VCI_IN# pins.\n 1=Filters disabled; 0=Filters enabled (default)."]
    #[inline(always)]
    pub fn fltrs_bypass(&mut self) -> FLTRS_BYPASS_W<12> {
        FLTRS_BYPASS_W::new(self)
    }
    #[doc = "Bit 16 - If enabled by WK_ALRM_LE, this bit is set to 1 if the Week Alarm signal is asserted. It is reset by writes to WK_ALRM_LS."]
    #[inline(always)]
    pub fn wk_alrm(&mut self) -> WK_ALRM_W<16> {
        WK_ALRM_W::new(self)
    }
    #[doc = "Bit 17 - If enabled by RTC_ALRM_LE, this bit is set to 1 if the RTC Alarm signal is asserted. It is reset by writes to RTC_ALRM_LS."]
    #[inline(always)]
    pub fn rtc_alrm(&mut self) -> RTC_ALRM_W<17> {
        RTC_ALRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VCI Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_sts](index.html) module"]
pub struct CTRL_STS_SPEC;
impl crate::RegisterSpec for CTRL_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_sts::R](R) reader structure"]
impl crate::Readable for CTRL_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_sts::W](W) writer structure"]
impl crate::Writable for CTRL_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_STS to value 0"]
impl crate::Resettable for CTRL_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
