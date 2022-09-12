#[doc = "Register `SLP_EN_1` reader"]
pub struct R(crate::R<SLP_EN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_EN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_EN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_EN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_EN_1` writer"]
pub struct W(crate::W<SLP_EN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_EN_1_SPEC>;
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
impl From<crate::W<SLP_EN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_EN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_SLP_EN` reader - Interrupt Sleep Enable"]
pub type INT_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `INT_SLP_EN` writer - Interrupt Sleep Enable"]
pub type INT_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `TACH0_SLP_EN` reader - TACH0 Sleep Enable"]
pub type TACH0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TACH0_SLP_EN` writer - TACH0 Sleep Enable"]
pub type TACH0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `PWM0_SLP_EN` reader - PWM0 Sleep Enable"]
pub type PWM0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM0_SLP_EN` writer - PWM0 Sleep Enable"]
pub type PWM0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `PMC_SLP_EN` reader - PMC Sleep Enable"]
pub type PMC_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PMC_SLP_EN` writer - PMC Sleep Enable"]
pub type PMC_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `DMA_SLP_EN` reader - DMA Sleep Enable"]
pub type DMA_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_SLP_EN` writer - DMA Sleep Enable"]
pub type DMA_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `TFDP_SLP_EN` reader - TFDP Sleep Enable"]
pub type TFDP_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TFDP_SLP_EN` writer - TFDP Sleep Enable"]
pub type TFDP_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `PROC_SLP_EN` reader - PROCESSOR Sleep Enable"]
pub type PROC_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PROC_SLP_EN` writer - PROCESSOR Sleep Enable"]
pub type PROC_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `WDT_SLP_EN` reader - WDT Sleep Enable"]
pub type WDT_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_SLP_EN` writer - WDT Sleep Enable"]
pub type WDT_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `SMB0_SLP_EN` reader - SMB0 Sleep Enable"]
pub type SMB0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB0_SLP_EN` writer - SMB0 Sleep Enable"]
pub type SMB0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `TACH1_SLP_EN` reader - TACH1 Sleep Enable"]
pub type TACH1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TACH1_SLP_EN` writer - TACH1 Sleep Enable"]
pub type TACH1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `PWM1_SLP_EN` reader - PWM1 Sleep Enable"]
pub type PWM1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM1_SLP_EN` writer - PWM1 Sleep Enable"]
pub type PWM1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `PWM2_SLP_EN` reader - PWM2 Sleep Enable"]
pub type PWM2_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM2_SLP_EN` writer - PWM2 Sleep Enable"]
pub type PWM2_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `PWM3_SLP_EN` reader - PWM3 Sleep Enable"]
pub type PWM3_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM3_SLP_EN` writer - PWM3 Sleep Enable"]
pub type PWM3_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `PWM4_SLP_EN` reader - PWM4 Sleep Enable"]
pub type PWM4_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM4_SLP_EN` writer - PWM4 Sleep Enable"]
pub type PWM4_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `PWM5_SLP_EN` reader - PWM5 Sleep Enable"]
pub type PWM5_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM5_SLP_EN` writer - PWM5 Sleep Enable"]
pub type PWM5_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `EC_REG_BANK_SLP_EN` reader - EC_REG_BANK Sleep Enable"]
pub type EC_REG_BANK_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `EC_REG_BANK_SLP_EN` writer - EC_REG_BANK Sleep Enable"]
pub type EC_REG_BANK_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `TMR16_0_SLP_EN` reader - TIMER16_0 Sleep Enable"]
pub type TMR16_0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR16_0_SLP_EN` writer - TIMER16_0 Sleep Enable"]
pub type TMR16_0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
#[doc = "Field `TMR16_1_SLP_EN` reader - TIMER16_1 Sleep Enable"]
pub type TMR16_1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR16_1_SLP_EN` writer - TIMER16_1 Sleep Enable"]
pub type TMR16_1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Sleep Enable"]
    #[inline(always)]
    pub fn int_slp_en(&self) -> INT_SLP_EN_R {
        INT_SLP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TACH0 Sleep Enable"]
    #[inline(always)]
    pub fn tach0_slp_en(&self) -> TACH0_SLP_EN_R {
        TACH0_SLP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM0 Sleep Enable"]
    #[inline(always)]
    pub fn pwm0_slp_en(&self) -> PWM0_SLP_EN_R {
        PWM0_SLP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PMC Sleep Enable"]
    #[inline(always)]
    pub fn pmc_slp_en(&self) -> PMC_SLP_EN_R {
        PMC_SLP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Sleep Enable"]
    #[inline(always)]
    pub fn dma_slp_en(&self) -> DMA_SLP_EN_R {
        DMA_SLP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TFDP Sleep Enable"]
    #[inline(always)]
    pub fn tfdp_slp_en(&self) -> TFDP_SLP_EN_R {
        TFDP_SLP_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PROCESSOR Sleep Enable"]
    #[inline(always)]
    pub fn proc_slp_en(&self) -> PROC_SLP_EN_R {
        PROC_SLP_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WDT Sleep Enable"]
    #[inline(always)]
    pub fn wdt_slp_en(&self) -> WDT_SLP_EN_R {
        WDT_SLP_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SMB0 Sleep Enable"]
    #[inline(always)]
    pub fn smb0_slp_en(&self) -> SMB0_SLP_EN_R {
        SMB0_SLP_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TACH1 Sleep Enable"]
    #[inline(always)]
    pub fn tach1_slp_en(&self) -> TACH1_SLP_EN_R {
        TACH1_SLP_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM1 Sleep Enable"]
    #[inline(always)]
    pub fn pwm1_slp_en(&self) -> PWM1_SLP_EN_R {
        PWM1_SLP_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PWM2 Sleep Enable"]
    #[inline(always)]
    pub fn pwm2_slp_en(&self) -> PWM2_SLP_EN_R {
        PWM2_SLP_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PWM3 Sleep Enable"]
    #[inline(always)]
    pub fn pwm3_slp_en(&self) -> PWM3_SLP_EN_R {
        PWM3_SLP_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PWM4 Sleep Enable"]
    #[inline(always)]
    pub fn pwm4_slp_en(&self) -> PWM4_SLP_EN_R {
        PWM4_SLP_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PWM5 Sleep Enable"]
    #[inline(always)]
    pub fn pwm5_slp_en(&self) -> PWM5_SLP_EN_R {
        PWM5_SLP_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - EC_REG_BANK Sleep Enable"]
    #[inline(always)]
    pub fn ec_reg_bank_slp_en(&self) -> EC_REG_BANK_SLP_EN_R {
        EC_REG_BANK_SLP_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TIMER16_0 Sleep Enable"]
    #[inline(always)]
    pub fn tmr16_0_slp_en(&self) -> TMR16_0_SLP_EN_R {
        TMR16_0_SLP_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TIMER16_1 Sleep Enable"]
    #[inline(always)]
    pub fn tmr16_1_slp_en(&self) -> TMR16_1_SLP_EN_R {
        TMR16_1_SLP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Sleep Enable"]
    #[inline(always)]
    pub fn int_slp_en(&mut self) -> INT_SLP_EN_W<0> {
        INT_SLP_EN_W::new(self)
    }
    #[doc = "Bit 2 - TACH0 Sleep Enable"]
    #[inline(always)]
    pub fn tach0_slp_en(&mut self) -> TACH0_SLP_EN_W<2> {
        TACH0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 4 - PWM0 Sleep Enable"]
    #[inline(always)]
    pub fn pwm0_slp_en(&mut self) -> PWM0_SLP_EN_W<4> {
        PWM0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 5 - PMC Sleep Enable"]
    #[inline(always)]
    pub fn pmc_slp_en(&mut self) -> PMC_SLP_EN_W<5> {
        PMC_SLP_EN_W::new(self)
    }
    #[doc = "Bit 6 - DMA Sleep Enable"]
    #[inline(always)]
    pub fn dma_slp_en(&mut self) -> DMA_SLP_EN_W<6> {
        DMA_SLP_EN_W::new(self)
    }
    #[doc = "Bit 7 - TFDP Sleep Enable"]
    #[inline(always)]
    pub fn tfdp_slp_en(&mut self) -> TFDP_SLP_EN_W<7> {
        TFDP_SLP_EN_W::new(self)
    }
    #[doc = "Bit 8 - PROCESSOR Sleep Enable"]
    #[inline(always)]
    pub fn proc_slp_en(&mut self) -> PROC_SLP_EN_W<8> {
        PROC_SLP_EN_W::new(self)
    }
    #[doc = "Bit 9 - WDT Sleep Enable"]
    #[inline(always)]
    pub fn wdt_slp_en(&mut self) -> WDT_SLP_EN_W<9> {
        WDT_SLP_EN_W::new(self)
    }
    #[doc = "Bit 10 - SMB0 Sleep Enable"]
    #[inline(always)]
    pub fn smb0_slp_en(&mut self) -> SMB0_SLP_EN_W<10> {
        SMB0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 11 - TACH1 Sleep Enable"]
    #[inline(always)]
    pub fn tach1_slp_en(&mut self) -> TACH1_SLP_EN_W<11> {
        TACH1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 20 - PWM1 Sleep Enable"]
    #[inline(always)]
    pub fn pwm1_slp_en(&mut self) -> PWM1_SLP_EN_W<20> {
        PWM1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 21 - PWM2 Sleep Enable"]
    #[inline(always)]
    pub fn pwm2_slp_en(&mut self) -> PWM2_SLP_EN_W<21> {
        PWM2_SLP_EN_W::new(self)
    }
    #[doc = "Bit 22 - PWM3 Sleep Enable"]
    #[inline(always)]
    pub fn pwm3_slp_en(&mut self) -> PWM3_SLP_EN_W<22> {
        PWM3_SLP_EN_W::new(self)
    }
    #[doc = "Bit 23 - PWM4 Sleep Enable"]
    #[inline(always)]
    pub fn pwm4_slp_en(&mut self) -> PWM4_SLP_EN_W<23> {
        PWM4_SLP_EN_W::new(self)
    }
    #[doc = "Bit 24 - PWM5 Sleep Enable"]
    #[inline(always)]
    pub fn pwm5_slp_en(&mut self) -> PWM5_SLP_EN_W<24> {
        PWM5_SLP_EN_W::new(self)
    }
    #[doc = "Bit 29 - EC_REG_BANK Sleep Enable"]
    #[inline(always)]
    pub fn ec_reg_bank_slp_en(&mut self) -> EC_REG_BANK_SLP_EN_W<29> {
        EC_REG_BANK_SLP_EN_W::new(self)
    }
    #[doc = "Bit 30 - TIMER16_0 Sleep Enable"]
    #[inline(always)]
    pub fn tmr16_0_slp_en(&mut self) -> TMR16_0_SLP_EN_W<30> {
        TMR16_0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 31 - TIMER16_1 Sleep Enable"]
    #[inline(always)]
    pub fn tmr16_1_slp_en(&mut self) -> TMR16_1_SLP_EN_W<31> {
        TMR16_1_SLP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Enable 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_en_1](index.html) module"]
pub struct SLP_EN_1_SPEC;
impl crate::RegisterSpec for SLP_EN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_en_1::R](R) reader structure"]
impl crate::Readable for SLP_EN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_en_1::W](W) writer structure"]
impl crate::Writable for SLP_EN_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_EN_1 to value 0"]
impl crate::Resettable for SLP_EN_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
