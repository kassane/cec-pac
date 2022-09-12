#[doc = "Register `LATCH_RST` reader"]
pub struct R(crate::R<LATCH_RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LATCH_RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LATCH_RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LATCH_RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LATCH_RST` writer"]
pub struct W(crate::W<LATCH_RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LATCH_RST_SPEC>;
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
impl From<crate::W<LATCH_RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LATCH_RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LS` reader - Latch Resets. When a Latch Resets bit is written with a '1', the corresponding VCI_INi# latch is de-asserted ('1'). The VCI_INi#\n input to the latch has priority over the Latch Reset input, so firmware cannot reset the latch while the VCI_INi# pin is asserted. Firmware\n should sample the state of the pin in the VCI Register before attempting to reset the latch. As noted in the Latch Enable Register,\n the assertion level is determined by the VCI_IN_POL bit. Reads of this register are undefined."]
pub type LS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LS` writer - Latch Resets. When a Latch Resets bit is written with a '1', the corresponding VCI_INi# latch is de-asserted ('1'). The VCI_INi#\n input to the latch has priority over the Latch Reset input, so firmware cannot reset the latch while the VCI_INi# pin is asserted. Firmware\n should sample the state of the pin in the VCI Register before attempting to reset the latch. As noted in the Latch Enable Register,\n the assertion level is determined by the VCI_IN_POL bit. Reads of this register are undefined."]
pub type LS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LATCH_RST_SPEC, u8, u8, 4, O>;
#[doc = "Field `WK_ALRM_LS` reader - Week Alarm Latch Reset. When this bit is written with a '1', the Week Alarm Event latch is reset.\n The Week Alarm input to the latch has priority over the Reset input Reads of this register are undefined."]
pub type WK_ALRM_LS_R = crate::BitReader<bool>;
#[doc = "Field `WK_ALRM_LS` writer - Week Alarm Latch Reset. When this bit is written with a '1', the Week Alarm Event latch is reset.\n The Week Alarm input to the latch has priority over the Reset input Reads of this register are undefined."]
pub type WK_ALRM_LS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LATCH_RST_SPEC, bool, O>;
#[doc = "Field `RTC_ALRM_LS` reader - RTC Alarm Latch Reset. When this bit is written with a '1', the RTC Alarm Event latch is reset.\n The RTC Alarm input to the latch has priority over the Reset input Reads of this register are undefined."]
pub type RTC_ALRM_LS_R = crate::BitReader<bool>;
#[doc = "Field `RTC_ALRM_LS` writer - RTC Alarm Latch Reset. When this bit is written with a '1', the RTC Alarm Event latch is reset.\n The RTC Alarm input to the latch has priority over the Reset input Reads of this register are undefined."]
pub type RTC_ALRM_LS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LATCH_RST_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Latch Resets. When a Latch Resets bit is written with a '1', the corresponding VCI_INi# latch is de-asserted ('1'). The VCI_INi#\n input to the latch has priority over the Latch Reset input, so firmware cannot reset the latch while the VCI_INi# pin is asserted. Firmware\n should sample the state of the pin in the VCI Register before attempting to reset the latch. As noted in the Latch Enable Register,\n the assertion level is determined by the VCI_IN_POL bit. Reads of this register are undefined."]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Week Alarm Latch Reset. When this bit is written with a '1', the Week Alarm Event latch is reset.\n The Week Alarm input to the latch has priority over the Reset input Reads of this register are undefined."]
    #[inline(always)]
    pub fn wk_alrm_ls(&self) -> WK_ALRM_LS_R {
        WK_ALRM_LS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RTC Alarm Latch Reset. When this bit is written with a '1', the RTC Alarm Event latch is reset.\n The RTC Alarm input to the latch has priority over the Reset input Reads of this register are undefined."]
    #[inline(always)]
    pub fn rtc_alrm_ls(&self) -> RTC_ALRM_LS_R {
        RTC_ALRM_LS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Latch Resets. When a Latch Resets bit is written with a '1', the corresponding VCI_INi# latch is de-asserted ('1'). The VCI_INi#\n input to the latch has priority over the Latch Reset input, so firmware cannot reset the latch while the VCI_INi# pin is asserted. Firmware\n should sample the state of the pin in the VCI Register before attempting to reset the latch. As noted in the Latch Enable Register,\n the assertion level is determined by the VCI_IN_POL bit. Reads of this register are undefined."]
    #[inline(always)]
    pub fn ls(&mut self) -> LS_W<0> {
        LS_W::new(self)
    }
    #[doc = "Bit 16 - Week Alarm Latch Reset. When this bit is written with a '1', the Week Alarm Event latch is reset.\n The Week Alarm input to the latch has priority over the Reset input Reads of this register are undefined."]
    #[inline(always)]
    pub fn wk_alrm_ls(&mut self) -> WK_ALRM_LS_W<16> {
        WK_ALRM_LS_W::new(self)
    }
    #[doc = "Bit 17 - RTC Alarm Latch Reset. When this bit is written with a '1', the RTC Alarm Event latch is reset.\n The RTC Alarm input to the latch has priority over the Reset input Reads of this register are undefined."]
    #[inline(always)]
    pub fn rtc_alrm_ls(&mut self) -> RTC_ALRM_LS_W<17> {
        RTC_ALRM_LS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Latch Resets Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [latch_rst](index.html) module"]
pub struct LATCH_RST_SPEC;
impl crate::RegisterSpec for LATCH_RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [latch_rst::R](R) reader structure"]
impl crate::Readable for LATCH_RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [latch_rst::W](W) writer structure"]
impl crate::Writable for LATCH_RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LATCH_RST to value 0"]
impl crate::Resettable for LATCH_RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
