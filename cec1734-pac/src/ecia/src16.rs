#[doc = "Register `SRC16` reader"]
pub struct R(crate::R<SRC16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC16` writer"]
pub struct W(crate::W<SRC16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC16_SPEC>;
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
impl From<crate::W<SRC16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKE_ERR` reader - PKE ERR"]
pub type PKE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `PKE_ERR` writer - PKE ERR"]
pub type PKE_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC16_SPEC, bool, O>;
#[doc = "Field `PKE_END` reader - PKE END"]
pub type PKE_END_R = crate::BitReader<bool>;
#[doc = "Field `PKE_END` writer - PKE END"]
pub type PKE_END_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC16_SPEC, bool, O>;
#[doc = "Field `RNG` reader - RNG"]
pub type RNG_R = crate::BitReader<bool>;
#[doc = "Field `RNG` writer - RNG"]
pub type RNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC16_SPEC, bool, O>;
#[doc = "Field `AES` reader - AES"]
pub type AES_R = crate::BitReader<bool>;
#[doc = "Field `AES` writer - AES"]
pub type AES_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC16_SPEC, bool, O>;
#[doc = "Field `HASH` reader - HASH"]
pub type HASH_R = crate::BitReader<bool>;
#[doc = "Field `HASH` writer - HASH"]
pub type HASH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC16_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PKE ERR"]
    #[inline(always)]
    pub fn pke_err(&self) -> PKE_ERR_R {
        PKE_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PKE END"]
    #[inline(always)]
    pub fn pke_end(&self) -> PKE_END_R {
        PKE_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AES"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HASH"]
    #[inline(always)]
    pub fn hash(&self) -> HASH_R {
        HASH_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PKE ERR"]
    #[inline(always)]
    pub fn pke_err(&mut self) -> PKE_ERR_W<0> {
        PKE_ERR_W::new(self)
    }
    #[doc = "Bit 1 - PKE END"]
    #[inline(always)]
    pub fn pke_end(&mut self) -> PKE_END_W<1> {
        PKE_END_W::new(self)
    }
    #[doc = "Bit 2 - RNG"]
    #[inline(always)]
    pub fn rng(&mut self) -> RNG_W<2> {
        RNG_W::new(self)
    }
    #[doc = "Bit 3 - AES"]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W<3> {
        AES_W::new(self)
    }
    #[doc = "Bit 4 - HASH"]
    #[inline(always)]
    pub fn hash(&mut self) -> HASH_W<4> {
        HASH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ16 SOURCE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src16](index.html) module"]
pub struct SRC16_SPEC;
impl crate::RegisterSpec for SRC16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src16::R](R) reader structure"]
impl crate::Readable for SRC16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src16::W](W) writer structure"]
impl crate::Writable for SRC16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRC16 to value 0"]
impl crate::Resettable for SRC16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
