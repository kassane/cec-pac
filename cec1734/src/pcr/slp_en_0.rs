#[doc = "Register `SLP_EN_0` reader"]
pub struct R(crate::R<SLP_EN_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_EN_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_EN_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_EN_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_EN_0` writer"]
pub struct W(crate::W<SLP_EN_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_EN_0_SPEC>;
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
impl From<crate::W<SLP_EN_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_EN_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STAP_SLP_EN` reader - STAP Sleep Enable"]
pub type STAP_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `STAP_SLP_EN` writer - STAP Sleep Enable"]
pub type STAP_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_0_SPEC, bool, O>;
#[doc = "Field `OTP_SLP_EN` reader - OTP Sleep Enable"]
pub type OTP_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `OTP_SLP_EN` writer - OTP Sleep Enable"]
pub type OTP_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_0_SPEC, bool, O>;
#[doc = "Field `IMSPI_SLP_EN` reader - IMSPI Sleep Enable"]
pub type IMSPI_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `IMSPI_SLP_EN` writer - IMSPI Sleep Enable"]
pub type IMSPI_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_0_SPEC, bool, O>;
#[doc = "Field `CHPTST_SLP_EN` reader - Chip Test Sleep Enable"]
pub type CHPTST_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `CHPTST_SLP_EN` writer - Chip Test Sleep Enable"]
pub type CHPTST_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_0_SPEC, bool, O>;
#[doc = "Field `HRBNK_SLP_EN` reader - Host Register Bank Sleep Enable"]
pub type HRBNK_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `HRBNK_SLP_EN` writer - Host Register Bank Sleep Enable"]
pub type HRBNK_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_0_SPEC, bool, O>;
#[doc = "Field `TSTSPI_SLP_EN` reader - Test SPI Sleep Enable"]
pub type TSTSPI_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TSTSPI_SLP_EN` writer - Test SPI Sleep Enable"]
pub type TSTSPI_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_0_SPEC, bool, O>;
#[doc = "Field `GPIO_SLP_EN` reader - GPIO Sleep Enable"]
pub type GPIO_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_SLP_EN` writer - GPIO Sleep Enable"]
pub type GPIO_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_0_SPEC, bool, O>;
#[doc = "Field `PCR_SLP_EN` reader - PCR Sleep Enable"]
pub type PCR_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PCR_SLP_EN` writer - PCR Sleep Enable"]
pub type PCR_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - STAP Sleep Enable"]
    #[inline(always)]
    pub fn stap_slp_en(&self) -> STAP_SLP_EN_R {
        STAP_SLP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OTP Sleep Enable"]
    #[inline(always)]
    pub fn otp_slp_en(&self) -> OTP_SLP_EN_R {
        OTP_SLP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IMSPI Sleep Enable"]
    #[inline(always)]
    pub fn imspi_slp_en(&self) -> IMSPI_SLP_EN_R {
        IMSPI_SLP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Chip Test Sleep Enable"]
    #[inline(always)]
    pub fn chptst_slp_en(&self) -> CHPTST_SLP_EN_R {
        CHPTST_SLP_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host Register Bank Sleep Enable"]
    #[inline(always)]
    pub fn hrbnk_slp_en(&self) -> HRBNK_SLP_EN_R {
        HRBNK_SLP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Test SPI Sleep Enable"]
    #[inline(always)]
    pub fn tstspi_slp_en(&self) -> TSTSPI_SLP_EN_R {
        TSTSPI_SLP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO Sleep Enable"]
    #[inline(always)]
    pub fn gpio_slp_en(&self) -> GPIO_SLP_EN_R {
        GPIO_SLP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCR Sleep Enable"]
    #[inline(always)]
    pub fn pcr_slp_en(&self) -> PCR_SLP_EN_R {
        PCR_SLP_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STAP Sleep Enable"]
    #[inline(always)]
    pub fn stap_slp_en(&mut self) -> STAP_SLP_EN_W<0> {
        STAP_SLP_EN_W::new(self)
    }
    #[doc = "Bit 1 - OTP Sleep Enable"]
    #[inline(always)]
    pub fn otp_slp_en(&mut self) -> OTP_SLP_EN_W<1> {
        OTP_SLP_EN_W::new(self)
    }
    #[doc = "Bit 2 - IMSPI Sleep Enable"]
    #[inline(always)]
    pub fn imspi_slp_en(&mut self) -> IMSPI_SLP_EN_W<2> {
        IMSPI_SLP_EN_W::new(self)
    }
    #[doc = "Bit 3 - Chip Test Sleep Enable"]
    #[inline(always)]
    pub fn chptst_slp_en(&mut self) -> CHPTST_SLP_EN_W<3> {
        CHPTST_SLP_EN_W::new(self)
    }
    #[doc = "Bit 4 - Host Register Bank Sleep Enable"]
    #[inline(always)]
    pub fn hrbnk_slp_en(&mut self) -> HRBNK_SLP_EN_W<4> {
        HRBNK_SLP_EN_W::new(self)
    }
    #[doc = "Bit 5 - Test SPI Sleep Enable"]
    #[inline(always)]
    pub fn tstspi_slp_en(&mut self) -> TSTSPI_SLP_EN_W<5> {
        TSTSPI_SLP_EN_W::new(self)
    }
    #[doc = "Bit 6 - GPIO Sleep Enable"]
    #[inline(always)]
    pub fn gpio_slp_en(&mut self) -> GPIO_SLP_EN_W<6> {
        GPIO_SLP_EN_W::new(self)
    }
    #[doc = "Bit 7 - PCR Sleep Enable"]
    #[inline(always)]
    pub fn pcr_slp_en(&mut self) -> PCR_SLP_EN_W<7> {
        PCR_SLP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Enable 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_en_0](index.html) module"]
pub struct SLP_EN_0_SPEC;
impl crate::RegisterSpec for SLP_EN_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_en_0::R](R) reader structure"]
impl crate::Readable for SLP_EN_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_en_0::W](W) writer structure"]
impl crate::Writable for SLP_EN_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_EN_0 to value 0"]
impl crate::Resettable for SLP_EN_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
