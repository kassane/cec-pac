#[doc = "Register `EC_PRIV_EN1` reader"]
pub struct R(crate::R<EC_PRIV_EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EC_PRIV_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EC_PRIV_EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EC_PRIV_EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EC_PRIV_EN1` writer"]
pub struct W(crate::W<EC_PRIV_EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EC_PRIV_EN1_SPEC>;
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
impl From<crate::W<EC_PRIV_EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EC_PRIV_EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR` reader - Interrupt Privilege Enable."]
pub type INTR_R = crate::BitReader<bool>;
#[doc = "Field `INTR` writer - Interrupt Privilege Enable."]
pub type INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN1_SPEC, bool, O>;
#[doc = "Field `PWM0` reader - PWM 0 Privilege Enable."]
pub type PWM0_R = crate::BitReader<bool>;
#[doc = "Field `PWM0` writer - PWM 0 Privilege Enable."]
pub type PWM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN1_SPEC, bool, O>;
#[doc = "Field `PMC` reader - PMC Privilege Enable."]
pub type PMC_R = crate::BitReader<bool>;
#[doc = "Field `PMC` writer - PMC Privilege Enable."]
pub type PMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN1_SPEC, bool, O>;
#[doc = "Field `DMA` reader - DMA Privilege Enable."]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - DMA Privilege Enable."]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN1_SPEC, bool, O>;
#[doc = "Field `TFDP` reader - TFDP Privilege Enable."]
pub type TFDP_R = crate::BitReader<bool>;
#[doc = "Field `TFDP` writer - TFDP Privilege Enable."]
pub type TFDP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN1_SPEC, bool, O>;
#[doc = "Field `WDT` reader - WDT Privilege Enable."]
pub type WDT_R = crate::BitReader<bool>;
#[doc = "Field `WDT` writer - WDT Privilege Enable."]
pub type WDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN1_SPEC, bool, O>;
#[doc = "Field `SMB_I2C0` reader - SMB I2C 0 Privilege Enable."]
pub type SMB_I2C0_R = crate::BitReader<bool>;
#[doc = "Field `SMB_I2C0` writer - SMB I2C 0 Privilege Enable."]
pub type SMB_I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN1_SPEC, bool, O>;
#[doc = "Field `EC_REGS` reader - EC Registers Privilege Enable."]
pub type EC_REGS_R = crate::BitReader<bool>;
#[doc = "Field `EC_REGS` writer - EC Registers Privilege Enable."]
pub type EC_REGS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN1_SPEC, bool, O>;
#[doc = "Field `BASIC_TMR0` reader - Basic Timer 0 Privilege Enable."]
pub type BASIC_TMR0_R = crate::BitReader<bool>;
#[doc = "Field `BASIC_TMR0` writer - Basic Timer 0 Privilege Enable."]
pub type BASIC_TMR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN1_SPEC, bool, O>;
#[doc = "Field `BASIC_TMR1` reader - Basic Timer 1 Privilege Enable."]
pub type BASIC_TMR1_R = crate::BitReader<bool>;
#[doc = "Field `BASIC_TMR1` writer - Basic Timer 1 Privilege Enable."]
pub type BASIC_TMR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_PRIV_EN1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Privilege Enable."]
    #[inline(always)]
    pub fn intr(&self) -> INTR_R {
        INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - PWM 0 Privilege Enable."]
    #[inline(always)]
    pub fn pwm0(&self) -> PWM0_R {
        PWM0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PMC Privilege Enable."]
    #[inline(always)]
    pub fn pmc(&self) -> PMC_R {
        PMC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Privilege Enable."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TFDP Privilege Enable."]
    #[inline(always)]
    pub fn tfdp(&self) -> TFDP_R {
        TFDP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - WDT Privilege Enable."]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SMB I2C 0 Privilege Enable."]
    #[inline(always)]
    pub fn smb_i2c0(&self) -> SMB_I2C0_R {
        SMB_I2C0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 29 - EC Registers Privilege Enable."]
    #[inline(always)]
    pub fn ec_regs(&self) -> EC_REGS_R {
        EC_REGS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Basic Timer 0 Privilege Enable."]
    #[inline(always)]
    pub fn basic_tmr0(&self) -> BASIC_TMR0_R {
        BASIC_TMR0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Basic Timer 1 Privilege Enable."]
    #[inline(always)]
    pub fn basic_tmr1(&self) -> BASIC_TMR1_R {
        BASIC_TMR1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Privilege Enable."]
    #[inline(always)]
    pub fn intr(&mut self) -> INTR_W<0> {
        INTR_W::new(self)
    }
    #[doc = "Bit 4 - PWM 0 Privilege Enable."]
    #[inline(always)]
    pub fn pwm0(&mut self) -> PWM0_W<4> {
        PWM0_W::new(self)
    }
    #[doc = "Bit 5 - PMC Privilege Enable."]
    #[inline(always)]
    pub fn pmc(&mut self) -> PMC_W<5> {
        PMC_W::new(self)
    }
    #[doc = "Bit 6 - DMA Privilege Enable."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<6> {
        DMA_W::new(self)
    }
    #[doc = "Bit 7 - TFDP Privilege Enable."]
    #[inline(always)]
    pub fn tfdp(&mut self) -> TFDP_W<7> {
        TFDP_W::new(self)
    }
    #[doc = "Bit 9 - WDT Privilege Enable."]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<9> {
        WDT_W::new(self)
    }
    #[doc = "Bit 10 - SMB I2C 0 Privilege Enable."]
    #[inline(always)]
    pub fn smb_i2c0(&mut self) -> SMB_I2C0_W<10> {
        SMB_I2C0_W::new(self)
    }
    #[doc = "Bit 29 - EC Registers Privilege Enable."]
    #[inline(always)]
    pub fn ec_regs(&mut self) -> EC_REGS_W<29> {
        EC_REGS_W::new(self)
    }
    #[doc = "Bit 30 - Basic Timer 0 Privilege Enable."]
    #[inline(always)]
    pub fn basic_tmr0(&mut self) -> BASIC_TMR0_W<30> {
        BASIC_TMR0_W::new(self)
    }
    #[doc = "Bit 31 - Basic Timer 1 Privilege Enable."]
    #[inline(always)]
    pub fn basic_tmr1(&mut self) -> BASIC_TMR1_W<31> {
        BASIC_TMR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EC Priviliges 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec_priv_en1](index.html) module"]
pub struct EC_PRIV_EN1_SPEC;
impl crate::RegisterSpec for EC_PRIV_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ec_priv_en1::R](R) reader structure"]
impl crate::Readable for EC_PRIV_EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ec_priv_en1::W](W) writer structure"]
impl crate::Writable for EC_PRIV_EN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EC_PRIV_EN1 to value 0"]
impl crate::Resettable for EC_PRIV_EN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
