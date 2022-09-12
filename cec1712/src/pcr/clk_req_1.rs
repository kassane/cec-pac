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
#[doc = "Field `INT_CLK_REQ` reader - Interrupt Clock Required"]
pub type INT_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `INT_CLK_REQ` writer - Interrupt Clock Required"]
pub type INT_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `TACH0_CLK_REQ` reader - TACH0 Clock Required (TACH0_CLK_REQ)"]
pub type TACH0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TACH0_CLK_REQ` writer - TACH0 Clock Required (TACH0_CLK_REQ)"]
pub type TACH0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PWM0_CLK_REQ` reader - PWM0 Clock Required (PWM0_CLK_REQ)"]
pub type PWM0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PWM0_CLK_REQ` writer - PWM0 Clock Required (PWM0_CLK_REQ)"]
pub type PWM0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PMC_CLK_REQ` reader - PMC Clock Required (PMC_CLK_REQ)"]
pub type PMC_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PMC_CLK_REQ` writer - PMC Clock Required (PMC_CLK_REQ)"]
pub type PMC_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `DMA_CLK_REQ` reader - DMA Clock Required (DMA_CLK_REQ)"]
pub type DMA_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CLK_REQ` writer - DMA Clock Required (DMA_CLK_REQ)"]
pub type DMA_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `TFDP_CLK_REQ` reader - TFDP Clock Required (TFDP_CLK_REQ)"]
pub type TFDP_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TFDP_CLK_REQ` writer - TFDP Clock Required (TFDP_CLK_REQ)"]
pub type TFDP_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PROC_CLK_REQ` reader - PROCESSOR Clock Required (PROCESSOR_CLK_REQ)"]
pub type PROC_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PROC_CLK_REQ` writer - PROCESSOR Clock Required (PROCESSOR_CLK_REQ)"]
pub type PROC_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `WDT_CLK_REQ` reader - WDT Clock Required (WDT_CLK_REQ)"]
pub type WDT_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `WDT_CLK_REQ` writer - WDT Clock Required (WDT_CLK_REQ)"]
pub type WDT_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `SMB0_CLK_REQ` reader - SMB0 Clock Required (SMB0_CLK_REQ)"]
pub type SMB0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SMB0_CLK_REQ` writer - SMB0 Clock Required (SMB0_CLK_REQ)"]
pub type SMB0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `TACH2_CLK_REQ` reader - TACH2 Clock Required (TACH2_CLK_REQ)"]
pub type TACH2_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TACH2_CLK_REQ` writer - TACH2 Clock Required (TACH2_CLK_REQ)"]
pub type TACH2_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PWM1_CLK_REQ` reader - PWM1 Clock Required (PWM1_CLK_REQ)"]
pub type PWM1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PWM1_CLK_REQ` writer - PWM1 Clock Required (PWM1_CLK_REQ)"]
pub type PWM1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PWM4_CLK_REQ` reader - PWM4 Clock Required (PWM4_CLK_REQ)"]
pub type PWM4_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PWM4_CLK_REQ` writer - PWM4 Clock Required (PWM4_CLK_REQ)"]
pub type PWM4_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PWM6_CLK_REQ` reader - PWM6 Clock Required (PWM6_CLK_REQ)"]
pub type PWM6_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PWM6_CLK_REQ` writer - PWM6 Clock Required (PWM6_CLK_REQ)"]
pub type PWM6_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `PWM7_CLK_REQ` reader - PWM7 Clock Required (PWM7_CLK_REQ)"]
pub type PWM7_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PWM7_CLK_REQ` writer - PWM7 Clock Required (PWM7_CLK_REQ)"]
pub type PWM7_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `EC_REG_BANK_CLK_REQ` reader - EC_REG_BANK Clock Required (EC_REG_BANK_CLK_REQ)"]
pub type EC_REG_BANK_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `EC_REG_BANK_CLK_REQ` writer - EC_REG_BANK Clock Required (EC_REG_BANK_CLK_REQ)"]
pub type EC_REG_BANK_CLK_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `TMR16_0_CLK_REQ` reader - TIMER16_0 Clock Required (TIMER16_0_CLK_REQ)"]
pub type TMR16_0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TMR16_0_CLK_REQ` writer - TIMER16_0 Clock Required (TIMER16_0_CLK_REQ)"]
pub type TMR16_0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
#[doc = "Field `TMR16_1_CLK_REQ` reader - TIMER16_1 Clock Required (TIMER16_1_CLK_REQ)"]
pub type TMR16_1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TMR16_1_CLK_REQ` writer - TIMER16_1 Clock Required (TIMER16_1_CLK_REQ)"]
pub type TMR16_1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Clock Required"]
    #[inline(always)]
    pub fn int_clk_req(&self) -> INT_CLK_REQ_R {
        INT_CLK_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TACH0 Clock Required (TACH0_CLK_REQ)"]
    #[inline(always)]
    pub fn tach0_clk_req(&self) -> TACH0_CLK_REQ_R {
        TACH0_CLK_REQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM0 Clock Required (PWM0_CLK_REQ)"]
    #[inline(always)]
    pub fn pwm0_clk_req(&self) -> PWM0_CLK_REQ_R {
        PWM0_CLK_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PMC Clock Required (PMC_CLK_REQ)"]
    #[inline(always)]
    pub fn pmc_clk_req(&self) -> PMC_CLK_REQ_R {
        PMC_CLK_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Clock Required (DMA_CLK_REQ)"]
    #[inline(always)]
    pub fn dma_clk_req(&self) -> DMA_CLK_REQ_R {
        DMA_CLK_REQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TFDP Clock Required (TFDP_CLK_REQ)"]
    #[inline(always)]
    pub fn tfdp_clk_req(&self) -> TFDP_CLK_REQ_R {
        TFDP_CLK_REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PROCESSOR Clock Required (PROCESSOR_CLK_REQ)"]
    #[inline(always)]
    pub fn proc_clk_req(&self) -> PROC_CLK_REQ_R {
        PROC_CLK_REQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WDT Clock Required (WDT_CLK_REQ)"]
    #[inline(always)]
    pub fn wdt_clk_req(&self) -> WDT_CLK_REQ_R {
        WDT_CLK_REQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SMB0 Clock Required (SMB0_CLK_REQ)"]
    #[inline(always)]
    pub fn smb0_clk_req(&self) -> SMB0_CLK_REQ_R {
        SMB0_CLK_REQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - TACH2 Clock Required (TACH2_CLK_REQ)"]
    #[inline(always)]
    pub fn tach2_clk_req(&self) -> TACH2_CLK_REQ_R {
        TACH2_CLK_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM1 Clock Required (PWM1_CLK_REQ)"]
    #[inline(always)]
    pub fn pwm1_clk_req(&self) -> PWM1_CLK_REQ_R {
        PWM1_CLK_REQ_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - PWM4 Clock Required (PWM4_CLK_REQ)"]
    #[inline(always)]
    pub fn pwm4_clk_req(&self) -> PWM4_CLK_REQ_R {
        PWM4_CLK_REQ_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - PWM6 Clock Required (PWM6_CLK_REQ)"]
    #[inline(always)]
    pub fn pwm6_clk_req(&self) -> PWM6_CLK_REQ_R {
        PWM6_CLK_REQ_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PWM7 Clock Required (PWM7_CLK_REQ)"]
    #[inline(always)]
    pub fn pwm7_clk_req(&self) -> PWM7_CLK_REQ_R {
        PWM7_CLK_REQ_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - EC_REG_BANK Clock Required (EC_REG_BANK_CLK_REQ)"]
    #[inline(always)]
    pub fn ec_reg_bank_clk_req(&self) -> EC_REG_BANK_CLK_REQ_R {
        EC_REG_BANK_CLK_REQ_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TIMER16_0 Clock Required (TIMER16_0_CLK_REQ)"]
    #[inline(always)]
    pub fn tmr16_0_clk_req(&self) -> TMR16_0_CLK_REQ_R {
        TMR16_0_CLK_REQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TIMER16_1 Clock Required (TIMER16_1_CLK_REQ)"]
    #[inline(always)]
    pub fn tmr16_1_clk_req(&self) -> TMR16_1_CLK_REQ_R {
        TMR16_1_CLK_REQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Clock Required"]
    #[inline(always)]
    pub fn int_clk_req(&mut self) -> INT_CLK_REQ_W<0> {
        INT_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 2 - TACH0 Clock Required (TACH0_CLK_REQ)"]
    #[inline(always)]
    pub fn tach0_clk_req(&mut self) -> TACH0_CLK_REQ_W<2> {
        TACH0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 4 - PWM0 Clock Required (PWM0_CLK_REQ)"]
    #[inline(always)]
    pub fn pwm0_clk_req(&mut self) -> PWM0_CLK_REQ_W<4> {
        PWM0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 5 - PMC Clock Required (PMC_CLK_REQ)"]
    #[inline(always)]
    pub fn pmc_clk_req(&mut self) -> PMC_CLK_REQ_W<5> {
        PMC_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 6 - DMA Clock Required (DMA_CLK_REQ)"]
    #[inline(always)]
    pub fn dma_clk_req(&mut self) -> DMA_CLK_REQ_W<6> {
        DMA_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 7 - TFDP Clock Required (TFDP_CLK_REQ)"]
    #[inline(always)]
    pub fn tfdp_clk_req(&mut self) -> TFDP_CLK_REQ_W<7> {
        TFDP_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 8 - PROCESSOR Clock Required (PROCESSOR_CLK_REQ)"]
    #[inline(always)]
    pub fn proc_clk_req(&mut self) -> PROC_CLK_REQ_W<8> {
        PROC_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 9 - WDT Clock Required (WDT_CLK_REQ)"]
    #[inline(always)]
    pub fn wdt_clk_req(&mut self) -> WDT_CLK_REQ_W<9> {
        WDT_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 10 - SMB0 Clock Required (SMB0_CLK_REQ)"]
    #[inline(always)]
    pub fn smb0_clk_req(&mut self) -> SMB0_CLK_REQ_W<10> {
        SMB0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 12 - TACH2 Clock Required (TACH2_CLK_REQ)"]
    #[inline(always)]
    pub fn tach2_clk_req(&mut self) -> TACH2_CLK_REQ_W<12> {
        TACH2_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 20 - PWM1 Clock Required (PWM1_CLK_REQ)"]
    #[inline(always)]
    pub fn pwm1_clk_req(&mut self) -> PWM1_CLK_REQ_W<20> {
        PWM1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 23 - PWM4 Clock Required (PWM4_CLK_REQ)"]
    #[inline(always)]
    pub fn pwm4_clk_req(&mut self) -> PWM4_CLK_REQ_W<23> {
        PWM4_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 25 - PWM6 Clock Required (PWM6_CLK_REQ)"]
    #[inline(always)]
    pub fn pwm6_clk_req(&mut self) -> PWM6_CLK_REQ_W<25> {
        PWM6_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 26 - PWM7 Clock Required (PWM7_CLK_REQ)"]
    #[inline(always)]
    pub fn pwm7_clk_req(&mut self) -> PWM7_CLK_REQ_W<26> {
        PWM7_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 29 - EC_REG_BANK Clock Required (EC_REG_BANK_CLK_REQ)"]
    #[inline(always)]
    pub fn ec_reg_bank_clk_req(&mut self) -> EC_REG_BANK_CLK_REQ_W<29> {
        EC_REG_BANK_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 30 - TIMER16_0 Clock Required (TIMER16_0_CLK_REQ)"]
    #[inline(always)]
    pub fn tmr16_0_clk_req(&mut self) -> TMR16_0_CLK_REQ_W<30> {
        TMR16_0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 31 - TIMER16_1 Clock Required (TIMER16_1_CLK_REQ)"]
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
