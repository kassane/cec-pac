#[doc = "Register `CLK_REQ_1` reader"]
pub struct R(crate::R<CLK_REQ_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_REQ_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_REQ_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_REQ_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_REQ_1` writer"]
pub struct W(crate::W<CLK_REQ_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_REQ_1_SPEC>;
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
impl From<crate::W<CLK_REQ_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_REQ_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_CLK_REQ` reader - Interrupt Clock Reuqired"]
pub type INT_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `INT_CLK_REQ` writer - Interrupt Clock Reuqired"]
pub type INT_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `TACH0_CLK_REQ` reader - TACH0 Clock Reuqired"]
pub type TACH0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TACH0_CLK_REQ` writer - TACH0 Clock Reuqired"]
pub type TACH0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PWM0_CLK_REQ` reader - PWM0 Clock Reuqired"]
pub type PWM0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PWM0_CLK_REQ` writer - PWM0 Clock Reuqired"]
pub type PWM0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PMC_CLK_REQ` reader - PMC Clock Reuqired"]
pub type PMC_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PMC_CLK_REQ` writer - PMC Clock Reuqired"]
pub type PMC_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `DMA_CLK_REQ` reader - DMA Clock Reuqired"]
pub type DMA_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CLK_REQ` writer - DMA Clock Reuqired"]
pub type DMA_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `TFDP_CLK_REQ` reader - TFDP Clock Reuqired"]
pub type TFDP_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TFDP_CLK_REQ` writer - TFDP Clock Reuqired"]
pub type TFDP_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PROCESSOR_CLK_REQ` reader - PROCESSOR Clock Reuqired"]
pub type PROCESSOR_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PROCESSOR_CLK_REQ` writer - PROCESSOR Clock Reuqired"]
pub type PROCESSOR_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `WDT_CLK_REQ` reader - WDT Clock Reuqired"]
pub type WDT_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `WDT_CLK_REQ` writer - WDT Clock Reuqired"]
pub type WDT_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `SMB0_CLK_REQ` reader - SMB0 Clock Reuqired"]
pub type SMB0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SMB0_CLK_REQ` writer - SMB0 Clock Reuqired"]
pub type SMB0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `TACH1_CLK_REQ` reader - TACH1 Clock Reuqired"]
pub type TACH1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TACH1_CLK_REQ` writer - TACH1 Clock Reuqired"]
pub type TACH1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PWM1_CLK_REQ` reader - PWM1 Clock Reuqired"]
pub type PWM1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PWM1_CLK_REQ` writer - PWM1 Clock Reuqired"]
pub type PWM1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PWM2_CLK_REQ` reader - PWM2 Clock Reuqired"]
pub type PWM2_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PWM2_CLK_REQ` writer - PWM2 Clock Reuqired"]
pub type PWM2_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PWM3_CLK_REQ` reader - PWM3 Clock Reuqired"]
pub type PWM3_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PWM3_CLK_REQ` writer - PWM3 Clock Reuqired"]
pub type PWM3_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PWM4_CLK_REQ` reader - PWM4 Clock Reuqired"]
pub type PWM4_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PWM4_CLK_REQ` writer - PWM4 Clock Reuqired"]
pub type PWM4_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PWM5_CLK_REQ` reader - PWM5 Clock Reuqired"]
pub type PWM5_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PWM5_CLK_REQ` writer - PWM5 Clock Reuqired"]
pub type PWM5_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `EC_REG_BANK_CLK_REQ` reader - EC_REG_BANK Clock Reuqired"]
pub type EC_REG_BANK_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `EC_REG_BANK_CLK_REQ` writer - EC_REG_BANK Clock Reuqired"]
pub type EC_REG_BANK_CLK_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `TMR16_0_CLK_REQ` reader - TIMER16_0 Clock Reuqired"]
pub type TMR16_0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TMR16_0_CLK_REQ` writer - TIMER16_0 Clock Reuqired"]
pub type TMR16_0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `TMR16_1_CLK_REQ` reader - TIMER16_1 Clock Reuqired"]
pub type TMR16_1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TMR16_1_CLK_REQ` writer - TIMER16_1 Clock Reuqired"]
pub type TMR16_1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Clock Reuqired"]
    #[inline(always)]
    pub fn int_clk_req(&self) -> INT_CLK_REQ_R {
        INT_CLK_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TACH0 Clock Reuqired"]
    #[inline(always)]
    pub fn tach0_clk_req(&self) -> TACH0_CLK_REQ_R {
        TACH0_CLK_REQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM0 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm0_clk_req(&self) -> PWM0_CLK_REQ_R {
        PWM0_CLK_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PMC Clock Reuqired"]
    #[inline(always)]
    pub fn pmc_clk_req(&self) -> PMC_CLK_REQ_R {
        PMC_CLK_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Clock Reuqired"]
    #[inline(always)]
    pub fn dma_clk_req(&self) -> DMA_CLK_REQ_R {
        DMA_CLK_REQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TFDP Clock Reuqired"]
    #[inline(always)]
    pub fn tfdp_clk_req(&self) -> TFDP_CLK_REQ_R {
        TFDP_CLK_REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PROCESSOR Clock Reuqired"]
    #[inline(always)]
    pub fn processor_clk_req(&self) -> PROCESSOR_CLK_REQ_R {
        PROCESSOR_CLK_REQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WDT Clock Reuqired"]
    #[inline(always)]
    pub fn wdt_clk_req(&self) -> WDT_CLK_REQ_R {
        WDT_CLK_REQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SMB0 Clock Reuqired"]
    #[inline(always)]
    pub fn smb0_clk_req(&self) -> SMB0_CLK_REQ_R {
        SMB0_CLK_REQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TACH1 Clock Reuqired"]
    #[inline(always)]
    pub fn tach1_clk_req(&self) -> TACH1_CLK_REQ_R {
        TACH1_CLK_REQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM1 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm1_clk_req(&self) -> PWM1_CLK_REQ_R {
        PWM1_CLK_REQ_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PWM2 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm2_clk_req(&self) -> PWM2_CLK_REQ_R {
        PWM2_CLK_REQ_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PWM3 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm3_clk_req(&self) -> PWM3_CLK_REQ_R {
        PWM3_CLK_REQ_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PWM4 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm4_clk_req(&self) -> PWM4_CLK_REQ_R {
        PWM4_CLK_REQ_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PWM5 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm5_clk_req(&self) -> PWM5_CLK_REQ_R {
        PWM5_CLK_REQ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - EC_REG_BANK Clock Reuqired"]
    #[inline(always)]
    pub fn ec_reg_bank_clk_req(&self) -> EC_REG_BANK_CLK_REQ_R {
        EC_REG_BANK_CLK_REQ_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TIMER16_0 Clock Reuqired"]
    #[inline(always)]
    pub fn tmr16_0_clk_req(&self) -> TMR16_0_CLK_REQ_R {
        TMR16_0_CLK_REQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TIMER16_1 Clock Reuqired"]
    #[inline(always)]
    pub fn tmr16_1_clk_req(&self) -> TMR16_1_CLK_REQ_R {
        TMR16_1_CLK_REQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Clock Reuqired"]
    #[inline(always)]
    pub fn int_clk_req(&mut self) -> INT_CLK_REQ_W<0> {
        INT_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 2 - TACH0 Clock Reuqired"]
    #[inline(always)]
    pub fn tach0_clk_req(&mut self) -> TACH0_CLK_REQ_W<2> {
        TACH0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 4 - PWM0 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm0_clk_req(&mut self) -> PWM0_CLK_REQ_W<4> {
        PWM0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 5 - PMC Clock Reuqired"]
    #[inline(always)]
    pub fn pmc_clk_req(&mut self) -> PMC_CLK_REQ_W<5> {
        PMC_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 6 - DMA Clock Reuqired"]
    #[inline(always)]
    pub fn dma_clk_req(&mut self) -> DMA_CLK_REQ_W<6> {
        DMA_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 7 - TFDP Clock Reuqired"]
    #[inline(always)]
    pub fn tfdp_clk_req(&mut self) -> TFDP_CLK_REQ_W<7> {
        TFDP_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 8 - PROCESSOR Clock Reuqired"]
    #[inline(always)]
    pub fn processor_clk_req(&mut self) -> PROCESSOR_CLK_REQ_W<8> {
        PROCESSOR_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 9 - WDT Clock Reuqired"]
    #[inline(always)]
    pub fn wdt_clk_req(&mut self) -> WDT_CLK_REQ_W<9> {
        WDT_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 10 - SMB0 Clock Reuqired"]
    #[inline(always)]
    pub fn smb0_clk_req(&mut self) -> SMB0_CLK_REQ_W<10> {
        SMB0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 11 - TACH1 Clock Reuqired"]
    #[inline(always)]
    pub fn tach1_clk_req(&mut self) -> TACH1_CLK_REQ_W<11> {
        TACH1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 20 - PWM1 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm1_clk_req(&mut self) -> PWM1_CLK_REQ_W<20> {
        PWM1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 21 - PWM2 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm2_clk_req(&mut self) -> PWM2_CLK_REQ_W<21> {
        PWM2_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 22 - PWM3 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm3_clk_req(&mut self) -> PWM3_CLK_REQ_W<22> {
        PWM3_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 23 - PWM4 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm4_clk_req(&mut self) -> PWM4_CLK_REQ_W<23> {
        PWM4_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 24 - PWM5 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm5_clk_req(&mut self) -> PWM5_CLK_REQ_W<24> {
        PWM5_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 29 - EC_REG_BANK Clock Reuqired"]
    #[inline(always)]
    pub fn ec_reg_bank_clk_req(&mut self) -> EC_REG_BANK_CLK_REQ_W<29> {
        EC_REG_BANK_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 30 - TIMER16_0 Clock Reuqired"]
    #[inline(always)]
    pub fn tmr16_0_clk_req(&mut self) -> TMR16_0_CLK_REQ_W<30> {
        TMR16_0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 31 - TIMER16_1 Clock Reuqired"]
    #[inline(always)]
    pub fn tmr16_1_clk_req(&mut self) -> TMR16_1_CLK_REQ_W<31> {
        TMR16_1_CLK_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Required 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_req_1](index.html) module"]
pub struct CLK_REQ_1_SPEC;
impl crate::RegisterSpec for CLK_REQ_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_req_1::R](R) reader structure"]
impl crate::Readable for CLK_REQ_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_req_1::W](W) writer structure"]
impl crate::Writable for CLK_REQ_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_REQ_1 to value 0"]
impl crate::Resettable for CLK_REQ_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
