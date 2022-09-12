#[doc = "Register `EC_PRIV_EN4` reader"]
pub struct R(crate::R<EC_PRIV_EN4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EC_PRIV_EN4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EC_PRIV_EN4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EC_PRIV_EN4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EC_PRIV_EN4` writer"]
pub struct W(crate::W<EC_PRIV_EN4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EC_PRIV_EN4_SPEC>;
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
impl From<crate::W<EC_PRIV_EN4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EC_PRIV_EN4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIMON0` reader - SPI Monitor 0 Privilege Enable."]
pub type SPIMON0_R = crate::BitReader<bool>;
#[doc = "Field `SPIMON0` writer - SPI Monitor 0 Privilege Enable."]
pub type SPIMON0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN4_SPEC, bool, O>;
#[doc = "Field `SPIMON1` reader - SPI Monitor 1 Privilege Enable."]
pub type SPIMON1_R = crate::BitReader<bool>;
#[doc = "Field `SPIMON1` writer - SPI Monitor 1 Privilege Enable."]
pub type SPIMON1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN4_SPEC, bool, O>;
#[doc = "Field `RTOS_TIM` reader - RTOS Timer Privilege Enable."]
pub type RTOS_TIM_R = crate::BitReader<bool>;
#[doc = "Field `RTOS_TIM` writer - RTOS Timer Privilege Enable."]
pub type RTOS_TIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN4_SPEC, bool, O>;
#[doc = "Field `QMSPI0` reader - QMSPI 0 Privilege Enable."]
pub type QMSPI0_R = crate::BitReader<bool>;
#[doc = "Field `QMSPI0` writer - QMSPI 0 Privilege Enable."]
pub type QMSPI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN4_SPEC, bool, O>;
#[doc = "Field `UART0` reader - UART 0 Privilege Enable."]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `UART0` writer - UART 0 Privilege Enable."]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN4_SPEC, bool, O>;
#[doc = "Field `SPISLV0` reader - SPISLV 0 Privilege Enable."]
pub type SPISLV0_R = crate::BitReader<bool>;
#[doc = "Field `SPISLV0` writer - SPISLV 0 Privilege Enable."]
pub type SPISLV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN4_SPEC, bool, O>;
#[doc = "Field `SPISLV1` reader - SPISLV 1 Privilege Enable."]
pub type SPISLV1_R = crate::BitReader<bool>;
#[doc = "Field `SPISLV1` writer - SPISLV 1 Privilege Enable."]
pub type SPISLV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN4_SPEC, bool, O>;
#[doc = "Field `QMSPI1` reader - QMSPI 1 Privilege Enable."]
pub type QMSPI1_R = crate::BitReader<bool>;
#[doc = "Field `QMSPI1` writer - QMSPI 1 Privilege Enable."]
pub type QMSPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN4_SPEC, bool, O>;
#[doc = "Field `VBAT_REG` reader - VBAT Register Privilege Enable."]
pub type VBAT_REG_R = crate::BitReader<bool>;
#[doc = "Field `VBAT_REG` writer - VBAT Register Privilege Enable."]
pub type VBAT_REG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SPI Monitor 0 Privilege Enable."]
    #[inline(always)]
    pub fn spimon0(&self) -> SPIMON0_R {
        SPIMON0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Monitor 1 Privilege Enable."]
    #[inline(always)]
    pub fn spimon1(&self) -> SPIMON1_R {
        SPIMON1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - RTOS Timer Privilege Enable."]
    #[inline(always)]
    pub fn rtos_tim(&self) -> RTOS_TIM_R {
        RTOS_TIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - QMSPI 0 Privilege Enable."]
    #[inline(always)]
    pub fn qmspi0(&self) -> QMSPI0_R {
        QMSPI0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART 0 Privilege Enable."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - SPISLV 0 Privilege Enable."]
    #[inline(always)]
    pub fn spislv0(&self) -> SPISLV0_R {
        SPISLV0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - SPISLV 1 Privilege Enable."]
    #[inline(always)]
    pub fn spislv1(&self) -> SPISLV1_R {
        SPISLV1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - QMSPI 1 Privilege Enable."]
    #[inline(always)]
    pub fn qmspi1(&self) -> QMSPI1_R {
        QMSPI1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - VBAT Register Privilege Enable."]
    #[inline(always)]
    pub fn vbat_reg(&self) -> VBAT_REG_R {
        VBAT_REG_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Monitor 0 Privilege Enable."]
    #[inline(always)]
    pub fn spimon0(&mut self) -> SPIMON0_W<0> {
        SPIMON0_W::new(self)
    }
    #[doc = "Bit 1 - SPI Monitor 1 Privilege Enable."]
    #[inline(always)]
    pub fn spimon1(&mut self) -> SPIMON1_W<1> {
        SPIMON1_W::new(self)
    }
    #[doc = "Bit 6 - RTOS Timer Privilege Enable."]
    #[inline(always)]
    pub fn rtos_tim(&mut self) -> RTOS_TIM_W<6> {
        RTOS_TIM_W::new(self)
    }
    #[doc = "Bit 8 - QMSPI 0 Privilege Enable."]
    #[inline(always)]
    pub fn qmspi0(&mut self) -> QMSPI0_W<8> {
        QMSPI0_W::new(self)
    }
    #[doc = "Bit 9 - UART 0 Privilege Enable."]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W<9> {
        UART0_W::new(self)
    }
    #[doc = "Bit 16 - SPISLV 0 Privilege Enable."]
    #[inline(always)]
    pub fn spislv0(&mut self) -> SPISLV0_W<16> {
        SPISLV0_W::new(self)
    }
    #[doc = "Bit 21 - SPISLV 1 Privilege Enable."]
    #[inline(always)]
    pub fn spislv1(&mut self) -> SPISLV1_W<21> {
        SPISLV1_W::new(self)
    }
    #[doc = "Bit 22 - QMSPI 1 Privilege Enable."]
    #[inline(always)]
    pub fn qmspi1(&mut self) -> QMSPI1_W<22> {
        QMSPI1_W::new(self)
    }
    #[doc = "Bit 23 - VBAT Register Privilege Enable."]
    #[inline(always)]
    pub fn vbat_reg(&mut self) -> VBAT_REG_W<23> {
        VBAT_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EC Priviliges 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec_priv_en4](index.html) module"]
pub struct EC_PRIV_EN4_SPEC;
impl crate::RegisterSpec for EC_PRIV_EN4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ec_priv_en4::R](R) reader structure"]
impl crate::Readable for EC_PRIV_EN4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ec_priv_en4::W](W) writer structure"]
impl crate::Writable for EC_PRIV_EN4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EC_PRIV_EN4 to value 0"]
impl crate::Resettable for EC_PRIV_EN4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
