#[doc = "Register `CLK_REQ_3` reader"]
pub struct R(crate::R<CLK_REQ_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_REQ_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_REQ_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_REQ_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_REQ_3` writer"]
pub struct W(crate::W<CLK_REQ_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_REQ_3_SPEC>;
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
impl From<crate::W<CLK_REQ_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_REQ_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_CLK_REQ` reader - ADC Clock Required (ADC_CLK_REQ)"]
pub type ADC_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `ADC_CLK_REQ` writer - ADC Clock Required (ADC_CLK_REQ)"]
pub type ADC_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `PS2_0_CLK_REQ` reader - PS2_0 Clock Required (PS2_0_CLK_REQ)"]
pub type PS2_0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PS2_0_CLK_REQ` writer - PS2_0 Clock Required (PS2_0_CLK_REQ)"]
pub type PS2_0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `HTM0_CLK_REQ` reader - Hibernation TIMER 0 Clock Required (HTM_0_CLK_REQ)"]
pub type HTM0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `HTM0_CLK_REQ` writer - Hibernation TIMER 0 Clock Required (HTM_0_CLK_REQ)"]
pub type HTM0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `SMB1_CLK_REQ` reader - SMB1 Clock Required (SMB1_CLK_REQ)"]
pub type SMB1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SMB1_CLK_REQ` writer - SMB1 Clock Required (SMB1_CLK_REQ)"]
pub type SMB1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `SMB2_CLK_REQ` reader - SMB2 Clock Required (SMB2_CLK_REQ)"]
pub type SMB2_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SMB2_CLK_REQ` writer - SMB2 Clock Required (SMB2_CLK_REQ)"]
pub type SMB2_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `SMB3_CLK_REQ` reader - SMB3 Clock Required (SMB3_CLK_REQ)"]
pub type SMB3_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SMB3_CLK_REQ` writer - SMB3 Clock Required (SMB3_CLK_REQ)"]
pub type SMB3_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `LED0_CLK_REQ` reader - LED0 Clock Required (LED0_CLK_REQ)"]
pub type LED0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `LED0_CLK_REQ` writer - LED0 Clock Required (LED0_CLK_REQ)"]
pub type LED0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `SMB_4_CLK_REQ` reader - SMB 4 Clock Required (SMB_4_CLK_REQ)"]
pub type SMB_4_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SMB_4_CLK_REQ` writer - SMB 4 Clock Required (SMB_4_CLK_REQ)"]
pub type SMB_4_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `TMR32_0_CLK_REQ` reader - TIMER32_0 Clock Required (TIMER32_0_CLK_REQ)"]
pub type TMR32_0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TMR32_0_CLK_REQ` writer - TIMER32_0 Clock Required (TIMER32_0_CLK_REQ)"]
pub type TMR32_0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `TMR32_1_CLK_REQ` reader - TIMER32_1 Clock Required (TIMER32_1_CLK_REQ)"]
pub type TMR32_1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TMR32_1_CLK_REQ` writer - TIMER32_1 Clock Required (TIMER32_1_CLK_REQ)"]
pub type TMR32_1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `PKE_CLK_REQ` reader - PKE Clock Required"]
pub type PKE_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PKE_CLK_REQ` writer - PKE Clock Required"]
pub type PKE_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `RNG_CLK_REQ` reader - RNG Clock Required"]
pub type RNG_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `RNG_CLK_REQ` writer - RNG Clock Required"]
pub type RNG_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `AES_HASH_CLK_REQ` reader - AES_HASH Clock Required"]
pub type AES_HASH_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `AES_HASH_CLK_REQ` writer - AES_HASH Clock Required"]
pub type AES_HASH_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `HTM_1_CLK_REQ` reader - Hibernation TIMER 1 Clock Required (HTM_1_CLK_REQ)"]
pub type HTM_1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `HTM_1_CLK_REQ` writer - Hibernation TIMER 1 Clock Required (HTM_1_CLK_REQ)"]
pub type HTM_1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
#[doc = "Field `CCTIMER_CLK_REQ` reader - Capture Compare Timer Clock Required (CCTIMER_CLK_REQ)"]
pub type CCTIMER_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `CCTIMER_CLK_REQ` writer - Capture Compare Timer Clock Required (CCTIMER_CLK_REQ)"]
pub type CCTIMER_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - ADC Clock Required (ADC_CLK_REQ)"]
    #[inline(always)]
    pub fn adc_clk_req(&self) -> ADC_CLK_REQ_R {
        ADC_CLK_REQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - PS2_0 Clock Required (PS2_0_CLK_REQ)"]
    #[inline(always)]
    pub fn ps2_0_clk_req(&self) -> PS2_0_CLK_REQ_R {
        PS2_0_CLK_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Hibernation TIMER 0 Clock Required (HTM_0_CLK_REQ)"]
    #[inline(always)]
    pub fn htm0_clk_req(&self) -> HTM0_CLK_REQ_R {
        HTM0_CLK_REQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - SMB1 Clock Required (SMB1_CLK_REQ)"]
    #[inline(always)]
    pub fn smb1_clk_req(&self) -> SMB1_CLK_REQ_R {
        SMB1_CLK_REQ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SMB2 Clock Required (SMB2_CLK_REQ)"]
    #[inline(always)]
    pub fn smb2_clk_req(&self) -> SMB2_CLK_REQ_R {
        SMB2_CLK_REQ_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMB3 Clock Required (SMB3_CLK_REQ)"]
    #[inline(always)]
    pub fn smb3_clk_req(&self) -> SMB3_CLK_REQ_R {
        SMB3_CLK_REQ_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LED0 Clock Required (LED0_CLK_REQ)"]
    #[inline(always)]
    pub fn led0_clk_req(&self) -> LED0_CLK_REQ_R {
        LED0_CLK_REQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - SMB 4 Clock Required (SMB_4_CLK_REQ)"]
    #[inline(always)]
    pub fn smb_4_clk_req(&self) -> SMB_4_CLK_REQ_R {
        SMB_4_CLK_REQ_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - TIMER32_0 Clock Required (TIMER32_0_CLK_REQ)"]
    #[inline(always)]
    pub fn tmr32_0_clk_req(&self) -> TMR32_0_CLK_REQ_R {
        TMR32_0_CLK_REQ_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TIMER32_1 Clock Required (TIMER32_1_CLK_REQ)"]
    #[inline(always)]
    pub fn tmr32_1_clk_req(&self) -> TMR32_1_CLK_REQ_R {
        TMR32_1_CLK_REQ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - PKE Clock Required"]
    #[inline(always)]
    pub fn pke_clk_req(&self) -> PKE_CLK_REQ_R {
        PKE_CLK_REQ_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RNG Clock Required"]
    #[inline(always)]
    pub fn rng_clk_req(&self) -> RNG_CLK_REQ_R {
        RNG_CLK_REQ_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - AES_HASH Clock Required"]
    #[inline(always)]
    pub fn aes_hash_clk_req(&self) -> AES_HASH_CLK_REQ_R {
        AES_HASH_CLK_REQ_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Hibernation TIMER 1 Clock Required (HTM_1_CLK_REQ)"]
    #[inline(always)]
    pub fn htm_1_clk_req(&self) -> HTM_1_CLK_REQ_R {
        HTM_1_CLK_REQ_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Capture Compare Timer Clock Required (CCTIMER_CLK_REQ)"]
    #[inline(always)]
    pub fn cctimer_clk_req(&self) -> CCTIMER_CLK_REQ_R {
        CCTIMER_CLK_REQ_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ADC Clock Required (ADC_CLK_REQ)"]
    #[inline(always)]
    pub fn adc_clk_req(&mut self) -> ADC_CLK_REQ_W<3> {
        ADC_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 5 - PS2_0 Clock Required (PS2_0_CLK_REQ)"]
    #[inline(always)]
    pub fn ps2_0_clk_req(&mut self) -> PS2_0_CLK_REQ_W<5> {
        PS2_0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 10 - Hibernation TIMER 0 Clock Required (HTM_0_CLK_REQ)"]
    #[inline(always)]
    pub fn htm0_clk_req(&mut self) -> HTM0_CLK_REQ_W<10> {
        HTM0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 13 - SMB1 Clock Required (SMB1_CLK_REQ)"]
    #[inline(always)]
    pub fn smb1_clk_req(&mut self) -> SMB1_CLK_REQ_W<13> {
        SMB1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 14 - SMB2 Clock Required (SMB2_CLK_REQ)"]
    #[inline(always)]
    pub fn smb2_clk_req(&mut self) -> SMB2_CLK_REQ_W<14> {
        SMB2_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 15 - SMB3 Clock Required (SMB3_CLK_REQ)"]
    #[inline(always)]
    pub fn smb3_clk_req(&mut self) -> SMB3_CLK_REQ_W<15> {
        SMB3_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 16 - LED0 Clock Required (LED0_CLK_REQ)"]
    #[inline(always)]
    pub fn led0_clk_req(&mut self) -> LED0_CLK_REQ_W<16> {
        LED0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 20 - SMB 4 Clock Required (SMB_4_CLK_REQ)"]
    #[inline(always)]
    pub fn smb_4_clk_req(&mut self) -> SMB_4_CLK_REQ_W<20> {
        SMB_4_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 23 - TIMER32_0 Clock Required (TIMER32_0_CLK_REQ)"]
    #[inline(always)]
    pub fn tmr32_0_clk_req(&mut self) -> TMR32_0_CLK_REQ_W<23> {
        TMR32_0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 24 - TIMER32_1 Clock Required (TIMER32_1_CLK_REQ)"]
    #[inline(always)]
    pub fn tmr32_1_clk_req(&mut self) -> TMR32_1_CLK_REQ_W<24> {
        TMR32_1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 26 - PKE Clock Required"]
    #[inline(always)]
    pub fn pke_clk_req(&mut self) -> PKE_CLK_REQ_W<26> {
        PKE_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 27 - RNG Clock Required"]
    #[inline(always)]
    pub fn rng_clk_req(&mut self) -> RNG_CLK_REQ_W<27> {
        RNG_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 28 - AES_HASH Clock Required"]
    #[inline(always)]
    pub fn aes_hash_clk_req(&mut self) -> AES_HASH_CLK_REQ_W<28> {
        AES_HASH_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 29 - Hibernation TIMER 1 Clock Required (HTM_1_CLK_REQ)"]
    #[inline(always)]
    pub fn htm_1_clk_req(&mut self) -> HTM_1_CLK_REQ_W<29> {
        HTM_1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 30 - Capture Compare Timer Clock Required (CCTIMER_CLK_REQ)"]
    #[inline(always)]
    pub fn cctimer_clk_req(&mut self) -> CCTIMER_CLK_REQ_W<30> {
        CCTIMER_CLK_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Required 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_req_3](index.html) module"]
pub struct CLK_REQ_3_SPEC;
impl crate::RegisterSpec for CLK_REQ_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_req_3::R](R) reader structure"]
impl crate::Readable for CLK_REQ_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_req_3::W](W) writer structure"]
impl crate::Writable for CLK_REQ_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_REQ_3 to value 0"]
impl crate::Resettable for CLK_REQ_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
