#[doc = "Register `CRYPTO_SRST` reader"]
pub struct R(crate::R<CRYPTO_SRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTO_SRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTO_SRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTO_SRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTO_SRST` writer"]
pub struct W(crate::W<CRYPTO_SRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_SRST_SPEC>;
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
impl From<crate::W<CRYPTO_SRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_SRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNG_SFT_RST` reader - When this bit is asserted 1, the Random Number Generator block is reset."]
pub type RNG_SFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `RNG_SFT_RST` writer - When this bit is asserted 1, the Random Number Generator block is reset."]
pub type RNG_SFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYPTO_SRST_SPEC, bool, O>;
#[doc = "Field `PUB_KEY_SFT_RST` reader - When this bit is asserted 1, the Public Key block is reset."]
pub type PUB_KEY_SFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `PUB_KEY_SFT_RST` writer - When this bit is asserted 1, the Public Key block is reset."]
pub type PUB_KEY_SFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYPTO_SRST_SPEC, bool, O>;
#[doc = "Field `AES_HASH_SFT_RST` reader - When this bit is asserted 1, the AES and Hash blocks are reset."]
pub type AES_HASH_SFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `AES_HASH_SFT_RST` writer - When this bit is asserted 1, the AES and Hash blocks are reset."]
pub type AES_HASH_SFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYPTO_SRST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When this bit is asserted 1, the Random Number Generator block is reset."]
    #[inline(always)]
    pub fn rng_sft_rst(&self) -> RNG_SFT_RST_R {
        RNG_SFT_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is asserted 1, the Public Key block is reset."]
    #[inline(always)]
    pub fn pub_key_sft_rst(&self) -> PUB_KEY_SFT_RST_R {
        PUB_KEY_SFT_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is asserted 1, the AES and Hash blocks are reset."]
    #[inline(always)]
    pub fn aes_hash_sft_rst(&self) -> AES_HASH_SFT_RST_R {
        AES_HASH_SFT_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When this bit is asserted 1, the Random Number Generator block is reset."]
    #[inline(always)]
    pub fn rng_sft_rst(&mut self) -> RNG_SFT_RST_W<0> {
        RNG_SFT_RST_W::new(self)
    }
    #[doc = "Bit 1 - When this bit is asserted 1, the Public Key block is reset."]
    #[inline(always)]
    pub fn pub_key_sft_rst(&mut self) -> PUB_KEY_SFT_RST_W<1> {
        PUB_KEY_SFT_RST_W::new(self)
    }
    #[doc = "Bit 2 - When this bit is asserted 1, the AES and Hash blocks are reset."]
    #[inline(always)]
    pub fn aes_hash_sft_rst(&mut self) -> AES_HASH_SFT_RST_W<2> {
        AES_HASH_SFT_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Shutdown Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_srst](index.html) module"]
pub struct CRYPTO_SRST_SPEC;
impl crate::RegisterSpec for CRYPTO_SRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypto_srst::R](R) reader structure"]
impl crate::Readable for CRYPTO_SRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypto_srst::W](W) writer structure"]
impl crate::Writable for CRYPTO_SRST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYPTO_SRST to value 0"]
impl crate::Resettable for CRYPTO_SRST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
