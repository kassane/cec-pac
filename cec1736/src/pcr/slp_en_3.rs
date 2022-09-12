#[doc = "Register `SLP_EN_3` reader"]
pub struct R(crate::R<SLP_EN_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_EN_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_EN_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_EN_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_EN_3` writer"]
pub struct W(crate::W<SLP_EN_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_EN_3_SPEC>;
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
impl From<crate::W<SLP_EN_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_EN_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTM_0_SLP_EN` reader - Hibernation Timer 0 Sleep Enable (HTM_0_SLP_EN)"]
pub type HTM_0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `HTM_0_SLP_EN` writer - Hibernation Timer 0 Sleep Enable (HTM_0_SLP_EN)"]
pub type HTM_0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB1_SLP_EN` reader - SMB1 Sleep Enable (SMB1_SLP_EN)"]
pub type SMB1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB1_SLP_EN` writer - SMB1 Sleep Enable (SMB1_SLP_EN)"]
pub type SMB1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB2_SLP_EN` reader - SMB2 Sleep Enable (SMB2_SLP_EN)"]
pub type SMB2_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB2_SLP_EN` writer - SMB2 Sleep Enable (SMB2_SLP_EN)"]
pub type SMB2_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB3_SLP_EN` reader - SMB3 Sleep Enable (SMB3_SLP_EN)"]
pub type SMB3_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB3_SLP_EN` writer - SMB3 Sleep Enable (SMB3_SLP_EN)"]
pub type SMB3_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `LED0_SLP_EN` reader - LED0 Sleep Enable (LED0_SLP_EN)"]
pub type LED0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `LED0_SLP_EN` writer - LED0 Sleep Enable (LED0_SLP_EN)"]
pub type LED0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `LED1_SLP_EN` reader - LED1 Sleep Enable (LED1_SLP_EN)"]
pub type LED1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `LED1_SLP_EN` writer - LED1 Sleep Enable (LED1_SLP_EN)"]
pub type LED1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `SMB4_SLP_EN` reader - SMB4 Sleep Enable (SMB4_SLP_EN)"]
pub type SMB4_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SMB4_SLP_EN` writer - SMB4 Sleep Enable (SMB4_SLP_EN)"]
pub type SMB4_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `CRYPTO_SLP_EN` reader - CRYPTO Sleep Enable"]
pub type CRYPTO_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_SLP_EN` writer - CRYPTO Sleep Enable"]
pub type CRYPTO_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `HTM_1_SLP_EN` reader - Hibernation TIMER 1 Sleep Enable (HTM_1_SLP_EN)"]
pub type HTM_1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `HTM_1_SLP_EN` writer - Hibernation TIMER 1 Sleep Enable (HTM_1_SLP_EN)"]
pub type HTM_1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
#[doc = "Field `CCT_SLP_EN` reader - Capture Compare Timer Sleep Enable (CCTIMER_SLP_EN)"]
pub type CCT_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `CCT_SLP_EN` writer - Capture Compare Timer Sleep Enable (CCTIMER_SLP_EN)"]
pub type CCT_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 10 - Hibernation Timer 0 Sleep Enable (HTM_0_SLP_EN)"]
    #[inline(always)]
    pub fn htm_0_slp_en(&self) -> HTM_0_SLP_EN_R {
        HTM_0_SLP_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - SMB1 Sleep Enable (SMB1_SLP_EN)"]
    #[inline(always)]
    pub fn smb1_slp_en(&self) -> SMB1_SLP_EN_R {
        SMB1_SLP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SMB2 Sleep Enable (SMB2_SLP_EN)"]
    #[inline(always)]
    pub fn smb2_slp_en(&self) -> SMB2_SLP_EN_R {
        SMB2_SLP_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMB3 Sleep Enable (SMB3_SLP_EN)"]
    #[inline(always)]
    pub fn smb3_slp_en(&self) -> SMB3_SLP_EN_R {
        SMB3_SLP_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LED0 Sleep Enable (LED0_SLP_EN)"]
    #[inline(always)]
    pub fn led0_slp_en(&self) -> LED0_SLP_EN_R {
        LED0_SLP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LED1 Sleep Enable (LED1_SLP_EN)"]
    #[inline(always)]
    pub fn led1_slp_en(&self) -> LED1_SLP_EN_R {
        LED1_SLP_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - SMB4 Sleep Enable (SMB4_SLP_EN)"]
    #[inline(always)]
    pub fn smb4_slp_en(&self) -> SMB4_SLP_EN_R {
        SMB4_SLP_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 26 - CRYPTO Sleep Enable"]
    #[inline(always)]
    pub fn crypto_slp_en(&self) -> CRYPTO_SLP_EN_R {
        CRYPTO_SLP_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - Hibernation TIMER 1 Sleep Enable (HTM_1_SLP_EN)"]
    #[inline(always)]
    pub fn htm_1_slp_en(&self) -> HTM_1_SLP_EN_R {
        HTM_1_SLP_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Capture Compare Timer Sleep Enable (CCTIMER_SLP_EN)"]
    #[inline(always)]
    pub fn cct_slp_en(&self) -> CCT_SLP_EN_R {
        CCT_SLP_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Hibernation Timer 0 Sleep Enable (HTM_0_SLP_EN)"]
    #[inline(always)]
    pub fn htm_0_slp_en(&mut self) -> HTM_0_SLP_EN_W<10> {
        HTM_0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 13 - SMB1 Sleep Enable (SMB1_SLP_EN)"]
    #[inline(always)]
    pub fn smb1_slp_en(&mut self) -> SMB1_SLP_EN_W<13> {
        SMB1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 14 - SMB2 Sleep Enable (SMB2_SLP_EN)"]
    #[inline(always)]
    pub fn smb2_slp_en(&mut self) -> SMB2_SLP_EN_W<14> {
        SMB2_SLP_EN_W::new(self)
    }
    #[doc = "Bit 15 - SMB3 Sleep Enable (SMB3_SLP_EN)"]
    #[inline(always)]
    pub fn smb3_slp_en(&mut self) -> SMB3_SLP_EN_W<15> {
        SMB3_SLP_EN_W::new(self)
    }
    #[doc = "Bit 16 - LED0 Sleep Enable (LED0_SLP_EN)"]
    #[inline(always)]
    pub fn led0_slp_en(&mut self) -> LED0_SLP_EN_W<16> {
        LED0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 17 - LED1 Sleep Enable (LED1_SLP_EN)"]
    #[inline(always)]
    pub fn led1_slp_en(&mut self) -> LED1_SLP_EN_W<17> {
        LED1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 20 - SMB4 Sleep Enable (SMB4_SLP_EN)"]
    #[inline(always)]
    pub fn smb4_slp_en(&mut self) -> SMB4_SLP_EN_W<20> {
        SMB4_SLP_EN_W::new(self)
    }
    #[doc = "Bit 26 - CRYPTO Sleep Enable"]
    #[inline(always)]
    pub fn crypto_slp_en(&mut self) -> CRYPTO_SLP_EN_W<26> {
        CRYPTO_SLP_EN_W::new(self)
    }
    #[doc = "Bit 29 - Hibernation TIMER 1 Sleep Enable (HTM_1_SLP_EN)"]
    #[inline(always)]
    pub fn htm_1_slp_en(&mut self) -> HTM_1_SLP_EN_W<29> {
        HTM_1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 30 - Capture Compare Timer Sleep Enable (CCTIMER_SLP_EN)"]
    #[inline(always)]
    pub fn cct_slp_en(&mut self) -> CCT_SLP_EN_W<30> {
        CCT_SLP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Enable 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_en_3](index.html) module"]
pub struct SLP_EN_3_SPEC;
impl crate::RegisterSpec for SLP_EN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_en_3::R](R) reader structure"]
impl crate::Readable for SLP_EN_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_en_3::W](W) writer structure"]
impl crate::Writable for SLP_EN_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_EN_3 to value 0"]
impl crate::Resettable for SLP_EN_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
