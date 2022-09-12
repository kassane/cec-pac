#[doc = "Register `LATCH_EN` reader"]
pub struct R(crate::R<LATCH_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LATCH_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LATCH_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LATCH_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LATCH_EN` writer"]
pub struct W(crate::W<LATCH_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LATCH_EN_SPEC>;
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
impl From<crate::W<LATCH_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LATCH_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LE` reader - Latching Enables. Latching occurs after the Polarity configuration, so a VCI_INi# pin is asserted when it is '0' if VCI_IN_POL is '0',\n and asserted when it is '1' if VCI_IN_POL is '1'. For each bit in the field: \n 1=Enabled. Assertions of the VCI_INi# pin are held until the latch is reset by writing the corresponding LS bit\n 0=Not Enabled. The VCI_INi# signal is not latched but passed directly to the VCI_OUT logic"]
pub type LE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LE` writer - Latching Enables. Latching occurs after the Polarity configuration, so a VCI_INi# pin is asserted when it is '0' if VCI_IN_POL is '0',\n and asserted when it is '1' if VCI_IN_POL is '1'. For each bit in the field: \n 1=Enabled. Assertions of the VCI_INi# pin are held until the latch is reset by writing the corresponding LS bit\n 0=Not Enabled. The VCI_INi# signal is not latched but passed directly to the VCI_OUT logic"]
pub type LE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LATCH_EN_SPEC, u8, u8, 7, O>;
#[doc = "Field `WK_ALRM_LE` reader - Latch enable for the Week Alarm Power-Up signal.\n 1=Enabled. Assertions of the Week Alarm are held until the latch is reset by writing the corresponding LS bit\n 0=Not Enabled. The Week Alarm signal is not latched but passed directly to the VCI_OUT logic"]
pub type WK_ALRM_LE_R = crate::BitReader<bool>;
#[doc = "Field `WK_ALRM_LE` writer - Latch enable for the Week Alarm Power-Up signal.\n 1=Enabled. Assertions of the Week Alarm are held until the latch is reset by writing the corresponding LS bit\n 0=Not Enabled. The Week Alarm signal is not latched but passed directly to the VCI_OUT logic"]
pub type WK_ALRM_LE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LATCH_EN_SPEC, bool, O>;
#[doc = "Field `RTC_ALRM_LE` reader - Latch enable for the RTC Power-Up signal.\n 1=Enabled. Assertions of the RTC Alarm are held until the latch is reset by writing the corresponding LS bit\n 0=Not Enabled. The RTC Alarm signal is not latched but passed directly to the VCI_OUT logic"]
pub type RTC_ALRM_LE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_ALRM_LE` writer - Latch enable for the RTC Power-Up signal.\n 1=Enabled. Assertions of the RTC Alarm are held until the latch is reset by writing the corresponding LS bit\n 0=Not Enabled. The RTC Alarm signal is not latched but passed directly to the VCI_OUT logic"]
pub type RTC_ALRM_LE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LATCH_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Latching Enables. Latching occurs after the Polarity configuration, so a VCI_INi# pin is asserted when it is '0' if VCI_IN_POL is '0',\n and asserted when it is '1' if VCI_IN_POL is '1'. For each bit in the field: \n 1=Enabled. Assertions of the VCI_INi# pin are held until the latch is reset by writing the corresponding LS bit\n 0=Not Enabled. The VCI_INi# signal is not latched but passed directly to the VCI_OUT logic"]
    #[inline(always)]
    pub fn le(&self) -> LE_R {
        LE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Latch enable for the Week Alarm Power-Up signal.\n 1=Enabled. Assertions of the Week Alarm are held until the latch is reset by writing the corresponding LS bit\n 0=Not Enabled. The Week Alarm signal is not latched but passed directly to the VCI_OUT logic"]
    #[inline(always)]
    pub fn wk_alrm_le(&self) -> WK_ALRM_LE_R {
        WK_ALRM_LE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Latch enable for the RTC Power-Up signal.\n 1=Enabled. Assertions of the RTC Alarm are held until the latch is reset by writing the corresponding LS bit\n 0=Not Enabled. The RTC Alarm signal is not latched but passed directly to the VCI_OUT logic"]
    #[inline(always)]
    pub fn rtc_alrm_le(&self) -> RTC_ALRM_LE_R {
        RTC_ALRM_LE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Latching Enables. Latching occurs after the Polarity configuration, so a VCI_INi# pin is asserted when it is '0' if VCI_IN_POL is '0',\n and asserted when it is '1' if VCI_IN_POL is '1'. For each bit in the field: \n 1=Enabled. Assertions of the VCI_INi# pin are held until the latch is reset by writing the corresponding LS bit\n 0=Not Enabled. The VCI_INi# signal is not latched but passed directly to the VCI_OUT logic"]
    #[inline(always)]
    pub fn le(&mut self) -> LE_W<0> {
        LE_W::new(self)
    }
    #[doc = "Bit 16 - Latch enable for the Week Alarm Power-Up signal.\n 1=Enabled. Assertions of the Week Alarm are held until the latch is reset by writing the corresponding LS bit\n 0=Not Enabled. The Week Alarm signal is not latched but passed directly to the VCI_OUT logic"]
    #[inline(always)]
    pub fn wk_alrm_le(&mut self) -> WK_ALRM_LE_W<16> {
        WK_ALRM_LE_W::new(self)
    }
    #[doc = "Bit 17 - Latch enable for the RTC Power-Up signal.\n 1=Enabled. Assertions of the RTC Alarm are held until the latch is reset by writing the corresponding LS bit\n 0=Not Enabled. The RTC Alarm signal is not latched but passed directly to the VCI_OUT logic"]
    #[inline(always)]
    pub fn rtc_alrm_le(&mut self) -> RTC_ALRM_LE_W<17> {
        RTC_ALRM_LE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Latch Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [latch_en](index.html) module"]
pub struct LATCH_EN_SPEC;
impl crate::RegisterSpec for LATCH_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [latch_en::R](R) reader structure"]
impl crate::Readable for LATCH_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [latch_en::W](W) writer structure"]
impl crate::Writable for LATCH_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LATCH_EN to value 0x30"]
impl crate::Resettable for LATCH_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
