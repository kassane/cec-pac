#[doc = "Register `SPICFG2` reader"]
pub struct R(crate::R<SPICFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPICFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPICFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPICFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPICFG2` writer"]
pub struct W(crate::W<SPICFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPICFG2_SPEC>;
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
impl From<crate::W<SPICFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPICFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HRM` reader - Host Reset Mode. 0 = APn_RESET#, 1 = Pin."]
pub type HRM_R = crate::BitReader<bool>;
#[doc = "Field `HRM` writer - Host Reset Mode. 0 = APn_RESET#, 1 = Pin."]
pub type HRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPICFG2_SPEC, bool, O>;
#[doc = "Field `HRL` reader - HRM Bit Lock. 0 = Unlocked, 1 = Locked."]
pub type HRL_R = crate::BitReader<bool>;
#[doc = "Field `HRL` writer - HRM Bit Lock. 0 = Unlocked, 1 = Locked."]
pub type HRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPICFG2_SPEC, bool, O>;
#[doc = "Field `DIV` reader - Disable Inter Vention. 1 = IRQ-only Mode."]
pub type DIV_R = crate::BitReader<bool>;
#[doc = "Field `DIV` writer - Disable Inter Vention. 1 = IRQ-only Mode."]
pub type DIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPICFG2_SPEC, bool, O>;
#[doc = "Field `DIL` reader - DIV Bit Lock. 0 = Unlocked, 1 = Locked."]
pub type DIL_R = crate::BitReader<bool>;
#[doc = "Field `DIL` writer - DIV Bit Lock. 0 = Unlocked, 1 = Locked."]
pub type DIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPICFG2_SPEC, bool, O>;
#[doc = "Field `RIV` reader - Special Region InterVention Mode. If DIV=1 then RIV is ignored. If DIV=0 and RIV=1 then: Reads that are forbidden by the Runtime Region register set are only cancelled by gating off CSn# for that SPI command, and setting the IRQ. No system shutdown results, and subsequent legal Reads are allowed."]
pub type RIV_R = crate::BitReader<bool>;
#[doc = "Field `RIV` writer - Special Region InterVention Mode. If DIV=1 then RIV is ignored. If DIV=0 and RIV=1 then: Reads that are forbidden by the Runtime Region register set are only cancelled by gating off CSn# for that SPI command, and setting the IRQ. No system shutdown results, and subsequent legal Reads are allowed."]
pub type RIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPICFG2_SPEC, bool, O>;
#[doc = "Field `RIL` reader - RIV Bit Lock. 0 = Unlocked, 1 = Locked."]
pub type RIL_R = crate::BitReader<bool>;
#[doc = "Field `RIL` writer - RIV Bit Lock. 0 = Unlocked, 1 = Locked."]
pub type RIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPICFG2_SPEC, bool, O>;
#[doc = "Field `ALG` reader - Hash Algorithm Mode. 0 = SHA-384, 1 = Reserved"]
pub type ALG_R = crate::BitReader<bool>;
#[doc = "Field `ALG` writer - Hash Algorithm Mode. 0 = SHA-384, 1 = Reserved"]
pub type ALG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPICFG2_SPEC, bool, O>;
#[doc = "Field `ALL` reader - ALG Bit Lock. 0 = Unlocked, 1 = Locked."]
pub type ALL_R = crate::BitReader<bool>;
#[doc = "Field `ALL` writer - ALG Bit Lock. 0 = Unlocked, 1 = Locked."]
pub type ALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPICFG2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Host Reset Mode. 0 = APn_RESET#, 1 = Pin."]
    #[inline(always)]
    pub fn hrm(&self) -> HRM_R {
        HRM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HRM Bit Lock. 0 = Unlocked, 1 = Locked."]
    #[inline(always)]
    pub fn hrl(&self) -> HRL_R {
        HRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable Inter Vention. 1 = IRQ-only Mode."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIV Bit Lock. 0 = Unlocked, 1 = Locked."]
    #[inline(always)]
    pub fn dil(&self) -> DIL_R {
        DIL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Special Region InterVention Mode. If DIV=1 then RIV is ignored. If DIV=0 and RIV=1 then: Reads that are forbidden by the Runtime Region register set are only cancelled by gating off CSn# for that SPI command, and setting the IRQ. No system shutdown results, and subsequent legal Reads are allowed."]
    #[inline(always)]
    pub fn riv(&self) -> RIV_R {
        RIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RIV Bit Lock. 0 = Unlocked, 1 = Locked."]
    #[inline(always)]
    pub fn ril(&self) -> RIL_R {
        RIL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Hash Algorithm Mode. 0 = SHA-384, 1 = Reserved"]
    #[inline(always)]
    pub fn alg(&self) -> ALG_R {
        ALG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ALG Bit Lock. 0 = Unlocked, 1 = Locked."]
    #[inline(always)]
    pub fn all(&self) -> ALL_R {
        ALL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host Reset Mode. 0 = APn_RESET#, 1 = Pin."]
    #[inline(always)]
    pub fn hrm(&mut self) -> HRM_W<0> {
        HRM_W::new(self)
    }
    #[doc = "Bit 1 - HRM Bit Lock. 0 = Unlocked, 1 = Locked."]
    #[inline(always)]
    pub fn hrl(&mut self) -> HRL_W<1> {
        HRL_W::new(self)
    }
    #[doc = "Bit 8 - Disable Inter Vention. 1 = IRQ-only Mode."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<8> {
        DIV_W::new(self)
    }
    #[doc = "Bit 9 - DIV Bit Lock. 0 = Unlocked, 1 = Locked."]
    #[inline(always)]
    pub fn dil(&mut self) -> DIL_W<9> {
        DIL_W::new(self)
    }
    #[doc = "Bit 12 - Special Region InterVention Mode. If DIV=1 then RIV is ignored. If DIV=0 and RIV=1 then: Reads that are forbidden by the Runtime Region register set are only cancelled by gating off CSn# for that SPI command, and setting the IRQ. No system shutdown results, and subsequent legal Reads are allowed."]
    #[inline(always)]
    pub fn riv(&mut self) -> RIV_W<12> {
        RIV_W::new(self)
    }
    #[doc = "Bit 13 - RIV Bit Lock. 0 = Unlocked, 1 = Locked."]
    #[inline(always)]
    pub fn ril(&mut self) -> RIL_W<13> {
        RIL_W::new(self)
    }
    #[doc = "Bit 16 - Hash Algorithm Mode. 0 = SHA-384, 1 = Reserved"]
    #[inline(always)]
    pub fn alg(&mut self) -> ALG_W<16> {
        ALG_W::new(self)
    }
    #[doc = "Bit 17 - ALG Bit Lock. 0 = Unlocked, 1 = Locked."]
    #[inline(always)]
    pub fn all(&mut self) -> ALL_W<17> {
        ALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Monitor Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spicfg2](index.html) module"]
pub struct SPICFG2_SPEC;
impl crate::RegisterSpec for SPICFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spicfg2::R](R) reader structure"]
impl crate::Readable for SPICFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spicfg2::W](W) writer structure"]
impl crate::Writable for SPICFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPICFG2 to value 0"]
impl crate::Resettable for SPICFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
