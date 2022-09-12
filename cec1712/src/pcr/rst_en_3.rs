#[doc = "Register `RST_EN_3` reader"]
pub struct R(crate::R<RST_EN_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_EN_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_EN_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_EN_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST_EN_3` writer"]
pub struct W(crate::W<RST_EN_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_EN_3_SPEC>;
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
impl From<crate::W<RST_EN_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_EN_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_RST_EN` reader - ADC Reset Enable (ADC_RST_EN)"]
pub type ADC_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC_RST_EN` writer - ADC Reset Enable (ADC_RST_EN)"]
pub type ADC_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `PS2_0_RST_EN` reader - PS2_0 Reset Enable (PS2_0_RST_EN)"]
pub type PS2_0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `PS2_0_RST_EN` writer - PS2_0 Reset Enable (PS2_0_RST_EN)"]
pub type PS2_0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `HTM_0_RST_EN` reader - Hibernation TIMER 0 Reset Enable (HTM_0_RST_EN)"]
pub type HTM_0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `HTM_0_RST_EN` writer - Hibernation TIMER 0 Reset Enable (HTM_0_RST_EN)"]
pub type HTM_0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB1_RST_EN` reader - SMB1 Reset Enable (SMB1_RST_EN)"]
pub type SMB1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB1_RST_EN` writer - SMB1 Reset Enable (SMB1_RST_EN)"]
pub type SMB1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB2_RST_EN` reader - SMB2 Reset Enable (SMB2_RST_EN)"]
pub type SMB2_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB2_RST_EN` writer - SMB2 Reset Enable (SMB2_RST_EN)"]
pub type SMB2_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB3_RST_EN` reader - SMB3 Reset Enable (SMB3_RST_EN)"]
pub type SMB3_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB3_RST_EN` writer - SMB3 Reset Enable (SMB3_RST_EN)"]
pub type SMB3_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `LED0_RST_EN` reader - LED0 Reset Enable (LED0_RST_EN)"]
pub type LED0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `LED0_RST_EN` writer - LED0 Reset Enable (LED0_RST_EN)"]
pub type LED0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB_4_RST_EN` reader - SMB 4 Reset Enable (SMB_4_RST_EN)"]
pub type SMB_4_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB_4_RST_EN` writer - SMB 4 Reset Enable (SMB_4_RST_EN)"]
pub type SMB_4_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `TMR32_0_RST_EN` reader - TIMER32_0 Reset Enable (TIMER32_0_RST_EN)"]
pub type TMR32_0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR32_0_RST_EN` writer - TIMER32_0 Reset Enable (TIMER32_0_RST_EN)"]
pub type TMR32_0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `TMR32_1_RST_EN` reader - TIMER32_1 Reset Enable (TIMER32_1_RST_EN)"]
pub type TMR32_1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `TMR32_1_RST_EN` writer - TIMER32_1 Reset Enable (TIMER32_1_RST_EN)"]
pub type TMR32_1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `PKE_RST_EN` reader - PKE Reset Enable"]
pub type PKE_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `PKE_RST_EN` writer - PKE Reset Enable"]
pub type PKE_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `RNG_RST_EN` reader - RNG Reset Enable"]
pub type RNG_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `RNG_RST_EN` writer - RNG Reset Enable"]
pub type RNG_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `AES_HASH_RST_EN` reader - AES_HASH Reset Enable"]
pub type AES_HASH_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `AES_HASH_RST_EN` writer - AES_HASH Reset Enable"]
pub type AES_HASH_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `HTM_1_RST_EN` reader - Hibernation TIMER 1 Reset Enable (HTM_1_RST_EN)"]
pub type HTM_1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `HTM_1_RST_EN` writer - Hibernation TIMER 1 Reset Enable (HTM_1_RST_EN)"]
pub type HTM_1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
#[doc = "Field `CCTIMER_RST_EN` reader - Capture Compare Timer Reset Enable (CCTIMER_RST_EN)"]
pub type CCTIMER_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `CCTIMER_RST_EN` writer - Capture Compare Timer Reset Enable (CCTIMER_RST_EN)"]
pub type CCTIMER_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - ADC Reset Enable (ADC_RST_EN)"]
    #[inline(always)]
    pub fn adc_rst_en(&self) -> ADC_RST_EN_R {
        ADC_RST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - PS2_0 Reset Enable (PS2_0_RST_EN)"]
    #[inline(always)]
    pub fn ps2_0_rst_en(&self) -> PS2_0_RST_EN_R {
        PS2_0_RST_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Hibernation TIMER 0 Reset Enable (HTM_0_RST_EN)"]
    #[inline(always)]
    pub fn htm_0_rst_en(&self) -> HTM_0_RST_EN_R {
        HTM_0_RST_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - SMB1 Reset Enable (SMB1_RST_EN)"]
    #[inline(always)]
    pub fn smb1_rst_en(&self) -> SMB1_RST_EN_R {
        SMB1_RST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SMB2 Reset Enable (SMB2_RST_EN)"]
    #[inline(always)]
    pub fn smb2_rst_en(&self) -> SMB2_RST_EN_R {
        SMB2_RST_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMB3 Reset Enable (SMB3_RST_EN)"]
    #[inline(always)]
    pub fn smb3_rst_en(&self) -> SMB3_RST_EN_R {
        SMB3_RST_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LED0 Reset Enable (LED0_RST_EN)"]
    #[inline(always)]
    pub fn led0_rst_en(&self) -> LED0_RST_EN_R {
        LED0_RST_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - SMB 4 Reset Enable (SMB_4_RST_EN)"]
    #[inline(always)]
    pub fn smb_4_rst_en(&self) -> SMB_4_RST_EN_R {
        SMB_4_RST_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - TIMER32_0 Reset Enable (TIMER32_0_RST_EN)"]
    #[inline(always)]
    pub fn tmr32_0_rst_en(&self) -> TMR32_0_RST_EN_R {
        TMR32_0_RST_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TIMER32_1 Reset Enable (TIMER32_1_RST_EN)"]
    #[inline(always)]
    pub fn tmr32_1_rst_en(&self) -> TMR32_1_RST_EN_R {
        TMR32_1_RST_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - PKE Reset Enable"]
    #[inline(always)]
    pub fn pke_rst_en(&self) -> PKE_RST_EN_R {
        PKE_RST_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RNG Reset Enable"]
    #[inline(always)]
    pub fn rng_rst_en(&self) -> RNG_RST_EN_R {
        RNG_RST_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - AES_HASH Reset Enable"]
    #[inline(always)]
    pub fn aes_hash_rst_en(&self) -> AES_HASH_RST_EN_R {
        AES_HASH_RST_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Hibernation TIMER 1 Reset Enable (HTM_1_RST_EN)"]
    #[inline(always)]
    pub fn htm_1_rst_en(&self) -> HTM_1_RST_EN_R {
        HTM_1_RST_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Capture Compare Timer Reset Enable (CCTIMER_RST_EN)"]
    #[inline(always)]
    pub fn cctimer_rst_en(&self) -> CCTIMER_RST_EN_R {
        CCTIMER_RST_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ADC Reset Enable (ADC_RST_EN)"]
    #[inline(always)]
    pub fn adc_rst_en(&mut self) -> ADC_RST_EN_W<3> {
        ADC_RST_EN_W::new(self)
    }
    #[doc = "Bit 5 - PS2_0 Reset Enable (PS2_0_RST_EN)"]
    #[inline(always)]
    pub fn ps2_0_rst_en(&mut self) -> PS2_0_RST_EN_W<5> {
        PS2_0_RST_EN_W::new(self)
    }
    #[doc = "Bit 10 - Hibernation TIMER 0 Reset Enable (HTM_0_RST_EN)"]
    #[inline(always)]
    pub fn htm_0_rst_en(&mut self) -> HTM_0_RST_EN_W<10> {
        HTM_0_RST_EN_W::new(self)
    }
    #[doc = "Bit 13 - SMB1 Reset Enable (SMB1_RST_EN)"]
    #[inline(always)]
    pub fn smb1_rst_en(&mut self) -> SMB1_RST_EN_W<13> {
        SMB1_RST_EN_W::new(self)
    }
    #[doc = "Bit 14 - SMB2 Reset Enable (SMB2_RST_EN)"]
    #[inline(always)]
    pub fn smb2_rst_en(&mut self) -> SMB2_RST_EN_W<14> {
        SMB2_RST_EN_W::new(self)
    }
    #[doc = "Bit 15 - SMB3 Reset Enable (SMB3_RST_EN)"]
    #[inline(always)]
    pub fn smb3_rst_en(&mut self) -> SMB3_RST_EN_W<15> {
        SMB3_RST_EN_W::new(self)
    }
    #[doc = "Bit 16 - LED0 Reset Enable (LED0_RST_EN)"]
    #[inline(always)]
    pub fn led0_rst_en(&mut self) -> LED0_RST_EN_W<16> {
        LED0_RST_EN_W::new(self)
    }
    #[doc = "Bit 20 - SMB 4 Reset Enable (SMB_4_RST_EN)"]
    #[inline(always)]
    pub fn smb_4_rst_en(&mut self) -> SMB_4_RST_EN_W<20> {
        SMB_4_RST_EN_W::new(self)
    }
    #[doc = "Bit 23 - TIMER32_0 Reset Enable (TIMER32_0_RST_EN)"]
    #[inline(always)]
    pub fn tmr32_0_rst_en(&mut self) -> TMR32_0_RST_EN_W<23> {
        TMR32_0_RST_EN_W::new(self)
    }
    #[doc = "Bit 24 - TIMER32_1 Reset Enable (TIMER32_1_RST_EN)"]
    #[inline(always)]
    pub fn tmr32_1_rst_en(&mut self) -> TMR32_1_RST_EN_W<24> {
        TMR32_1_RST_EN_W::new(self)
    }
    #[doc = "Bit 26 - PKE Reset Enable"]
    #[inline(always)]
    pub fn pke_rst_en(&mut self) -> PKE_RST_EN_W<26> {
        PKE_RST_EN_W::new(self)
    }
    #[doc = "Bit 27 - RNG Reset Enable"]
    #[inline(always)]
    pub fn rng_rst_en(&mut self) -> RNG_RST_EN_W<27> {
        RNG_RST_EN_W::new(self)
    }
    #[doc = "Bit 28 - AES_HASH Reset Enable"]
    #[inline(always)]
    pub fn aes_hash_rst_en(&mut self) -> AES_HASH_RST_EN_W<28> {
        AES_HASH_RST_EN_W::new(self)
    }
    #[doc = "Bit 29 - Hibernation TIMER 1 Reset Enable (HTM_1_RST_EN)"]
    #[inline(always)]
    pub fn htm_1_rst_en(&mut self) -> HTM_1_RST_EN_W<29> {
        HTM_1_RST_EN_W::new(self)
    }
    #[doc = "Bit 30 - Capture Compare Timer Reset Enable (CCTIMER_RST_EN)"]
    #[inline(always)]
    pub fn cctimer_rst_en(&mut self) -> CCTIMER_RST_EN_W<30> {
        CCTIMER_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Enable 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_en_3](index.html) module"]
pub struct RST_EN_3_SPEC;
impl crate::RegisterSpec for RST_EN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst_en_3::R](R) reader structure"]
impl crate::Readable for RST_EN_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_en_3::W](W) writer structure"]
impl crate::Writable for RST_EN_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RST_EN_3 to value 0"]
impl crate::Resettable for RST_EN_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
