#[doc = "Register `EC_PRIV_EN3` reader"]
pub struct R(crate::R<EC_PRIV_EN3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EC_PRIV_EN3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EC_PRIV_EN3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EC_PRIV_EN3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EC_PRIV_EN3` writer"]
pub struct W(crate::W<EC_PRIV_EN3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EC_PRIV_EN3_SPEC>;
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
impl From<crate::W<EC_PRIV_EN3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EC_PRIV_EN3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_TIM0` reader - Hibernation TIMER 0 Privilege Enable."]
pub type HIB_TIM0_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TIM0` writer - Hibernation TIMER 0 Privilege Enable."]
pub type HIB_TIM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN3_SPEC, bool, O>;
#[doc = "Field `SMB_I2C1` reader - SMB I2C 1 Privilege Enable."]
pub type SMB_I2C1_R = crate::BitReader<bool>;
#[doc = "Field `SMB_I2C1` writer - SMB I2C 1 Privilege Enable."]
pub type SMB_I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN3_SPEC, bool, O>;
#[doc = "Field `SMB_I2C2` reader - SMB I2C 2 Privilege Enable."]
pub type SMB_I2C2_R = crate::BitReader<bool>;
#[doc = "Field `SMB_I2C2` writer - SMB I2C 2 Privilege Enable."]
pub type SMB_I2C2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN3_SPEC, bool, O>;
#[doc = "Field `SMB_I2C3` reader - SMB I2C 3 Privilege Enable."]
pub type SMB_I2C3_R = crate::BitReader<bool>;
#[doc = "Field `SMB_I2C3` writer - SMB I2C 3 Privilege Enable."]
pub type SMB_I2C3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN3_SPEC, bool, O>;
#[doc = "Field `LED0` reader - LED 0 Privilege Enable."]
pub type LED0_R = crate::BitReader<bool>;
#[doc = "Field `LED0` writer - LED 0 Privilege Enable."]
pub type LED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN3_SPEC, bool, O>;
#[doc = "Field `LED1` reader - LED 1 Privilege Enable."]
pub type LED1_R = crate::BitReader<bool>;
#[doc = "Field `LED1` writer - LED 1 Privilege Enable."]
pub type LED1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN3_SPEC, bool, O>;
#[doc = "Field `SMB_I2C4` reader - SMB I2C 4 Privilege Enable."]
pub type SMB_I2C4_R = crate::BitReader<bool>;
#[doc = "Field `SMB_I2C4` writer - SMB I2C 4 Privilege Enable."]
pub type SMB_I2C4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN3_SPEC, bool, O>;
#[doc = "Field `CRYPTO` reader - Crypto Privilege Enable."]
pub type CRYPTO_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO` writer - Crypto Privilege Enable."]
pub type CRYPTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN3_SPEC, bool, O>;
#[doc = "Field `HIB_TIM1` reader - Hibernation Timer 1 Privilege Enable."]
pub type HIB_TIM1_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TIM1` writer - Hibernation Timer 1 Privilege Enable."]
pub type HIB_TIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN3_SPEC, bool, O>;
#[doc = "Field `CCT0` reader - Capture Compare Timer Privilege Enable."]
pub type CCT0_R = crate::BitReader<bool>;
#[doc = "Field `CCT0` writer - Capture Compare Timer Privilege Enable."]
pub type CCT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 10 - Hibernation TIMER 0 Privilege Enable."]
    #[inline(always)]
    pub fn hib_tim0(&self) -> HIB_TIM0_R {
        HIB_TIM0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - SMB I2C 1 Privilege Enable."]
    #[inline(always)]
    pub fn smb_i2c1(&self) -> SMB_I2C1_R {
        SMB_I2C1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SMB I2C 2 Privilege Enable."]
    #[inline(always)]
    pub fn smb_i2c2(&self) -> SMB_I2C2_R {
        SMB_I2C2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMB I2C 3 Privilege Enable."]
    #[inline(always)]
    pub fn smb_i2c3(&self) -> SMB_I2C3_R {
        SMB_I2C3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LED 0 Privilege Enable."]
    #[inline(always)]
    pub fn led0(&self) -> LED0_R {
        LED0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LED 1 Privilege Enable."]
    #[inline(always)]
    pub fn led1(&self) -> LED1_R {
        LED1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - SMB I2C 4 Privilege Enable."]
    #[inline(always)]
    pub fn smb_i2c4(&self) -> SMB_I2C4_R {
        SMB_I2C4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 26 - Crypto Privilege Enable."]
    #[inline(always)]
    pub fn crypto(&self) -> CRYPTO_R {
        CRYPTO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - Hibernation Timer 1 Privilege Enable."]
    #[inline(always)]
    pub fn hib_tim1(&self) -> HIB_TIM1_R {
        HIB_TIM1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Capture Compare Timer Privilege Enable."]
    #[inline(always)]
    pub fn cct0(&self) -> CCT0_R {
        CCT0_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Hibernation TIMER 0 Privilege Enable."]
    #[inline(always)]
    pub fn hib_tim0(&mut self) -> HIB_TIM0_W<10> {
        HIB_TIM0_W::new(self)
    }
    #[doc = "Bit 13 - SMB I2C 1 Privilege Enable."]
    #[inline(always)]
    pub fn smb_i2c1(&mut self) -> SMB_I2C1_W<13> {
        SMB_I2C1_W::new(self)
    }
    #[doc = "Bit 14 - SMB I2C 2 Privilege Enable."]
    #[inline(always)]
    pub fn smb_i2c2(&mut self) -> SMB_I2C2_W<14> {
        SMB_I2C2_W::new(self)
    }
    #[doc = "Bit 15 - SMB I2C 3 Privilege Enable."]
    #[inline(always)]
    pub fn smb_i2c3(&mut self) -> SMB_I2C3_W<15> {
        SMB_I2C3_W::new(self)
    }
    #[doc = "Bit 16 - LED 0 Privilege Enable."]
    #[inline(always)]
    pub fn led0(&mut self) -> LED0_W<16> {
        LED0_W::new(self)
    }
    #[doc = "Bit 17 - LED 1 Privilege Enable."]
    #[inline(always)]
    pub fn led1(&mut self) -> LED1_W<17> {
        LED1_W::new(self)
    }
    #[doc = "Bit 20 - SMB I2C 4 Privilege Enable."]
    #[inline(always)]
    pub fn smb_i2c4(&mut self) -> SMB_I2C4_W<20> {
        SMB_I2C4_W::new(self)
    }
    #[doc = "Bit 26 - Crypto Privilege Enable."]
    #[inline(always)]
    pub fn crypto(&mut self) -> CRYPTO_W<26> {
        CRYPTO_W::new(self)
    }
    #[doc = "Bit 29 - Hibernation Timer 1 Privilege Enable."]
    #[inline(always)]
    pub fn hib_tim1(&mut self) -> HIB_TIM1_W<29> {
        HIB_TIM1_W::new(self)
    }
    #[doc = "Bit 30 - Capture Compare Timer Privilege Enable."]
    #[inline(always)]
    pub fn cct0(&mut self) -> CCT0_W<30> {
        CCT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EC Priviliges 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec_priv_en3](index.html) module"]
pub struct EC_PRIV_EN3_SPEC;
impl crate::RegisterSpec for EC_PRIV_EN3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ec_priv_en3::R](R) reader structure"]
impl crate::Readable for EC_PRIV_EN3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ec_priv_en3::W](W) writer structure"]
impl crate::Writable for EC_PRIV_EN3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EC_PRIV_EN3 to value 0"]
impl crate::Resettable for EC_PRIV_EN3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
