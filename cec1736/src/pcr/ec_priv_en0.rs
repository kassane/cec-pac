#[doc = "Register `EC_PRIV_EN0` reader"]
pub struct R(crate::R<EC_PRIV_EN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EC_PRIV_EN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EC_PRIV_EN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EC_PRIV_EN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EC_PRIV_EN0` writer"]
pub struct W(crate::W<EC_PRIV_EN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EC_PRIV_EN0_SPEC>;
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
impl From<crate::W<EC_PRIV_EN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EC_PRIV_EN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTP` reader - OTP Privilege Enable."]
pub type OTP_R = crate::BitReader<bool>;
#[doc = "Field `OTP` writer - OTP Privilege Enable."]
pub type OTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN0_SPEC, bool, O>;
#[doc = "Field `HOST_REG` reader - Host Register Bank Privilege Enable."]
pub type HOST_REG_R = crate::BitReader<bool>;
#[doc = "Field `HOST_REG` writer - Host Register Bank Privilege Enable."]
pub type HOST_REG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN0_SPEC, bool, O>;
#[doc = "Field `TST_SPI` reader - Test SPI Privilege Enable."]
pub type TST_SPI_R = crate::BitReader<bool>;
#[doc = "Field `TST_SPI` writer - Test SPI Privilege Enable."]
pub type TST_SPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN0_SPEC, bool, O>;
#[doc = "Field `GPIO` reader - GPIO Privilege Enable."]
pub type GPIO_R = crate::BitReader<bool>;
#[doc = "Field `GPIO` writer - GPIO Privilege Enable."]
pub type GPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN0_SPEC, bool, O>;
#[doc = "Field `PCR` reader - PCR Privilege Enable."]
pub type PCR_R = crate::BitReader<bool>;
#[doc = "Field `PCR` writer - PCR Privilege Enable."]
pub type PCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - OTP Privilege Enable."]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Host Register Bank Privilege Enable."]
    #[inline(always)]
    pub fn host_reg(&self) -> HOST_REG_R {
        HOST_REG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Test SPI Privilege Enable."]
    #[inline(always)]
    pub fn tst_spi(&self) -> TST_SPI_R {
        TST_SPI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO Privilege Enable."]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCR Privilege Enable."]
    #[inline(always)]
    pub fn pcr(&self) -> PCR_R {
        PCR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - OTP Privilege Enable."]
    #[inline(always)]
    pub fn otp(&mut self) -> OTP_W<1> {
        OTP_W::new(self)
    }
    #[doc = "Bit 4 - Host Register Bank Privilege Enable."]
    #[inline(always)]
    pub fn host_reg(&mut self) -> HOST_REG_W<4> {
        HOST_REG_W::new(self)
    }
    #[doc = "Bit 5 - Test SPI Privilege Enable."]
    #[inline(always)]
    pub fn tst_spi(&mut self) -> TST_SPI_W<5> {
        TST_SPI_W::new(self)
    }
    #[doc = "Bit 6 - GPIO Privilege Enable."]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W<6> {
        GPIO_W::new(self)
    }
    #[doc = "Bit 7 - PCR Privilege Enable."]
    #[inline(always)]
    pub fn pcr(&mut self) -> PCR_W<7> {
        PCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EC Priviliges 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec_priv_en0](index.html) module"]
pub struct EC_PRIV_EN0_SPEC;
impl crate::RegisterSpec for EC_PRIV_EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ec_priv_en0::R](R) reader structure"]
impl crate::Readable for EC_PRIV_EN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ec_priv_en0::W](W) writer structure"]
impl crate::Writable for EC_PRIV_EN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EC_PRIV_EN0 to value 0"]
impl crate::Resettable for EC_PRIV_EN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
