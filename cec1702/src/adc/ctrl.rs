#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT` reader - 0: The ADC is disabled and placed in its lowest power state. 1: ADC block is enabled for operation."]
pub type ACT_R = crate::BitReader<bool>;
#[doc = "Field `ACT` writer - 0: The ADC is disabled and placed in its lowest power state. 1: ADC block is enabled for operation."]
pub type ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `STRT_SIN` reader - (STRT_SIN) 0: The ADC Single Mode is disabled. 1: The ADC Single Mode is enabled. Note: This bit is self-clearing"]
pub type STRT_SIN_R = crate::BitReader<bool>;
#[doc = "Field `STRT_SIN` writer - (STRT_SIN) 0: The ADC Single Mode is disabled. 1: The ADC Single Mode is enabled. Note: This bit is self-clearing"]
pub type STRT_SIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `STRT_RPT` reader - 0: The ADC Repeat Mode is disabled. 1: The ADC Repeat Mode is enabled."]
pub type STRT_RPT_R = crate::BitReader<bool>;
#[doc = "Field `STRT_RPT` writer - 0: The ADC Repeat Mode is disabled. 1: The ADC Repeat Mode is enabled."]
pub type STRT_RPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PWR_SAV_DIS` reader - 0: Power saving feature is enabled. 1: Power saving feature is disabled."]
pub type PWR_SAV_DIS_R = crate::BitReader<bool>;
#[doc = "Field `PWR_SAV_DIS` writer - 0: Power saving feature is enabled. 1: Power saving feature is disabled."]
pub type PWR_SAV_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SFT_RST` reader - (SFT_RST) 1: writing one causes a reset of the ADC block hardware (not the registers) 0: writing zero takes the ADC block out of reset"]
pub type SFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `SFT_RST` writer - (SFT_RST) 1: writing one causes a reset of the ADC block hardware (not the registers) 0: writing zero takes the ADC block out of reset"]
pub type SFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RPT_DONE_STS` reader - 0: ADC repeat-sample conversion is not complete. 1: ADC repeat-sample conversion is completed. (R/WC)"]
pub type RPT_DONE_STS_R = crate::BitReader<bool>;
#[doc = "Field `RPT_DONE_STS` writer - 0: ADC repeat-sample conversion is not complete. 1: ADC repeat-sample conversion is completed. (R/WC)"]
pub type RPT_DONE_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SIN_DONE_STS` reader - 0: ADC single-sample conversion is not complete. 1: ADC single-sample conversion is completed. (R/WC)"]
pub type SIN_DONE_STS_R = crate::BitReader<bool>;
#[doc = "Field `SIN_DONE_STS` writer - 0: ADC single-sample conversion is not complete. 1: ADC single-sample conversion is completed. (R/WC)"]
pub type SIN_DONE_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0: The ADC is disabled and placed in its lowest power state. 1: ADC block is enabled for operation."]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (STRT_SIN) 0: The ADC Single Mode is disabled. 1: The ADC Single Mode is enabled. Note: This bit is self-clearing"]
    #[inline(always)]
    pub fn strt_sin(&self) -> STRT_SIN_R {
        STRT_SIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: The ADC Repeat Mode is disabled. 1: The ADC Repeat Mode is enabled."]
    #[inline(always)]
    pub fn strt_rpt(&self) -> STRT_RPT_R {
        STRT_RPT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Power saving feature is enabled. 1: Power saving feature is disabled."]
    #[inline(always)]
    pub fn pwr_sav_dis(&self) -> PWR_SAV_DIS_R {
        PWR_SAV_DIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - (SFT_RST) 1: writing one causes a reset of the ADC block hardware (not the registers) 0: writing zero takes the ADC block out of reset"]
    #[inline(always)]
    pub fn sft_rst(&self) -> SFT_RST_R {
        SFT_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: ADC repeat-sample conversion is not complete. 1: ADC repeat-sample conversion is completed. (R/WC)"]
    #[inline(always)]
    pub fn rpt_done_sts(&self) -> RPT_DONE_STS_R {
        RPT_DONE_STS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: ADC single-sample conversion is not complete. 1: ADC single-sample conversion is completed. (R/WC)"]
    #[inline(always)]
    pub fn sin_done_sts(&self) -> SIN_DONE_STS_R {
        SIN_DONE_STS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: The ADC is disabled and placed in its lowest power state. 1: ADC block is enabled for operation."]
    #[inline(always)]
    pub fn act(&mut self) -> ACT_W<0> {
        ACT_W::new(self)
    }
    #[doc = "Bit 1 - (STRT_SIN) 0: The ADC Single Mode is disabled. 1: The ADC Single Mode is enabled. Note: This bit is self-clearing"]
    #[inline(always)]
    pub fn strt_sin(&mut self) -> STRT_SIN_W<1> {
        STRT_SIN_W::new(self)
    }
    #[doc = "Bit 2 - 0: The ADC Repeat Mode is disabled. 1: The ADC Repeat Mode is enabled."]
    #[inline(always)]
    pub fn strt_rpt(&mut self) -> STRT_RPT_W<2> {
        STRT_RPT_W::new(self)
    }
    #[doc = "Bit 3 - 0: Power saving feature is enabled. 1: Power saving feature is disabled."]
    #[inline(always)]
    pub fn pwr_sav_dis(&mut self) -> PWR_SAV_DIS_W<3> {
        PWR_SAV_DIS_W::new(self)
    }
    #[doc = "Bit 4 - (SFT_RST) 1: writing one causes a reset of the ADC block hardware (not the registers) 0: writing zero takes the ADC block out of reset"]
    #[inline(always)]
    pub fn sft_rst(&mut self) -> SFT_RST_W<4> {
        SFT_RST_W::new(self)
    }
    #[doc = "Bit 6 - 0: ADC repeat-sample conversion is not complete. 1: ADC repeat-sample conversion is completed. (R/WC)"]
    #[inline(always)]
    pub fn rpt_done_sts(&mut self) -> RPT_DONE_STS_W<6> {
        RPT_DONE_STS_W::new(self)
    }
    #[doc = "Bit 7 - 0: ADC single-sample conversion is not complete. 1: ADC single-sample conversion is completed. (R/WC)"]
    #[inline(always)]
    pub fn sin_done_sts(&mut self) -> SIN_DONE_STS_W<7> {
        SIN_DONE_STS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The ADC Control Register is used to control the behavior of the Analog to Digital Converter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
