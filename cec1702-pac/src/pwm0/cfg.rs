#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_EN` reader - When the PWM_EN is set to 0 the internal counters are reset and the internal state machine is set to the OFF state.\n In addition, the PWM_OUTPUT signal is set to the inactive state as determined by the Invert bit. The PWMx Counter ON Time Register\n and PWMx Counter OFF Time Register are not affected by the PWM_ENABLE bit and may be read and written while the PWM enable bit is 0.\n 1=Enabled (default); 0=Disabled (gates clocks to save power)."]
pub type PWM_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM_EN` writer - When the PWM_EN is set to 0 the internal counters are reset and the internal state machine is set to the OFF state.\n In addition, the PWM_OUTPUT signal is set to the inactive state as determined by the Invert bit. The PWMx Counter ON Time Register\n and PWMx Counter OFF Time Register are not affected by the PWM_ENABLE bit and may be read and written while the PWM enable bit is 0.\n 1=Enabled (default); 0=Disabled (gates clocks to save power)."]
pub type PWM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CLK_SEL` reader - This bit determines the clock source used by the PWM duty cycle and frequency control logic.\n 1=CLOCK_LOW\n 0=CLOCK_HIGH"]
pub type CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CLK_SEL` writer - This bit determines the clock source used by the PWM duty cycle and frequency control logic.\n 1=CLOCK_LOW\n 0=CLOCK_HIGH"]
pub type CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `INV` reader - 1= PWM_OUTPUT ON State is active low; 0=PWM_OUTPUT ON State is active high."]
pub type INV_R = crate::BitReader<bool>;
#[doc = "Field `INV` writer - 1= PWM_OUTPUT ON State is active low; 0=PWM_OUTPUT ON State is active high."]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CLK_PRE_DIV` reader - The Clock source for the 16-bit down counter (see PWMx Counter ON Time Register and PWMx Counter OFF Time Register)\n is determined by bit D1 of this register. The Clock source is then divided by the value of Pre-Divider+1 and the resulting\n signal determines the rate at which the down counter will be decremented. For example, a Pre-Divider value of 1 divides\n the input clock by 2 and a value of 2 divides the input clock by 3. A Pre-Divider of 0 will disable the Pre-Divider option."]
pub type CLK_PRE_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK_PRE_DIV` writer - The Clock source for the 16-bit down counter (see PWMx Counter ON Time Register and PWMx Counter OFF Time Register)\n is determined by bit D1 of this register. The Clock source is then divided by the value of Pre-Divider+1 and the resulting\n signal determines the rate at which the down counter will be decremented. For example, a Pre-Divider value of 1 divides\n the input clock by 2 and a value of 2 divides the input clock by 3. A Pre-Divider of 0 will disable the Pre-Divider option."]
pub type CLK_PRE_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - When the PWM_EN is set to 0 the internal counters are reset and the internal state machine is set to the OFF state.\n In addition, the PWM_OUTPUT signal is set to the inactive state as determined by the Invert bit. The PWMx Counter ON Time Register\n and PWMx Counter OFF Time Register are not affected by the PWM_ENABLE bit and may be read and written while the PWM enable bit is 0.\n 1=Enabled (default); 0=Disabled (gates clocks to save power)."]
    #[inline(always)]
    pub fn pwm_en(&self) -> PWM_EN_R {
        PWM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit determines the clock source used by the PWM duty cycle and frequency control logic.\n 1=CLOCK_LOW\n 0=CLOCK_HIGH"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1= PWM_OUTPUT ON State is active low; 0=PWM_OUTPUT ON State is active high."]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - The Clock source for the 16-bit down counter (see PWMx Counter ON Time Register and PWMx Counter OFF Time Register)\n is determined by bit D1 of this register. The Clock source is then divided by the value of Pre-Divider+1 and the resulting\n signal determines the rate at which the down counter will be decremented. For example, a Pre-Divider value of 1 divides\n the input clock by 2 and a value of 2 divides the input clock by 3. A Pre-Divider of 0 will disable the Pre-Divider option."]
    #[inline(always)]
    pub fn clk_pre_div(&self) -> CLK_PRE_DIV_R {
        CLK_PRE_DIV_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When the PWM_EN is set to 0 the internal counters are reset and the internal state machine is set to the OFF state.\n In addition, the PWM_OUTPUT signal is set to the inactive state as determined by the Invert bit. The PWMx Counter ON Time Register\n and PWMx Counter OFF Time Register are not affected by the PWM_ENABLE bit and may be read and written while the PWM enable bit is 0.\n 1=Enabled (default); 0=Disabled (gates clocks to save power)."]
    #[inline(always)]
    pub fn pwm_en(&mut self) -> PWM_EN_W<0> {
        PWM_EN_W::new(self)
    }
    #[doc = "Bit 1 - This bit determines the clock source used by the PWM duty cycle and frequency control logic.\n 1=CLOCK_LOW\n 0=CLOCK_HIGH"]
    #[inline(always)]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<1> {
        CLK_SEL_W::new(self)
    }
    #[doc = "Bit 2 - 1= PWM_OUTPUT ON State is active low; 0=PWM_OUTPUT ON State is active high."]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W<2> {
        INV_W::new(self)
    }
    #[doc = "Bits 3:6 - The Clock source for the 16-bit down counter (see PWMx Counter ON Time Register and PWMx Counter OFF Time Register)\n is determined by bit D1 of this register. The Clock source is then divided by the value of Pre-Divider+1 and the resulting\n signal determines the rate at which the down counter will be decremented. For example, a Pre-Divider value of 1 divides\n the input clock by 2 and a value of 2 divides the input clock by 3. A Pre-Divider of 0 will disable the Pre-Divider option."]
    #[inline(always)]
    pub fn clk_pre_div(&mut self) -> CLK_PRE_DIV_W<3> {
        CLK_PRE_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWMx CFGURATION REGISTER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
