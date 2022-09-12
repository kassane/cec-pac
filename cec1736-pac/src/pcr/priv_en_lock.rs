#[doc = "Register `PRIV_EN_LOCK` reader"]
pub struct R(crate::R<PRIV_EN_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIV_EN_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIV_EN_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIV_EN_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIV_EN_LOCK` writer"]
pub struct W(crate::W<PRIV_EN_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIV_EN_LOCK_SPEC>;
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
impl From<crate::W<PRIV_EN_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIV_EN_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK_EN` reader - Peripheral Privilege Lock Register. 1=Locked, 0=Unlocked. Locks Itself and CHIP_PRIV_EN, EC_PRIV_EN, EC_PRIV_EN2, EC_PRIV_EN3, HOST_PRIV_EN registers"]
pub type LOCK_EN_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_EN` writer - Peripheral Privilege Lock Register. 1=Locked, 0=Unlocked. Locks Itself and CHIP_PRIV_EN, EC_PRIV_EN, EC_PRIV_EN2, EC_PRIV_EN3, HOST_PRIV_EN registers"]
pub type LOCK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIV_EN_LOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Peripheral Privilege Lock Register. 1=Locked, 0=Unlocked. Locks Itself and CHIP_PRIV_EN, EC_PRIV_EN, EC_PRIV_EN2, EC_PRIV_EN3, HOST_PRIV_EN registers"]
    #[inline(always)]
    pub fn lock_en(&self) -> LOCK_EN_R {
        LOCK_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral Privilege Lock Register. 1=Locked, 0=Unlocked. Locks Itself and CHIP_PRIV_EN, EC_PRIV_EN, EC_PRIV_EN2, EC_PRIV_EN3, HOST_PRIV_EN registers"]
    #[inline(always)]
    pub fn lock_en(&mut self) -> LOCK_EN_W<0> {
        LOCK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Privilege Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [priv_en_lock](index.html) module"]
pub struct PRIV_EN_LOCK_SPEC;
impl crate::RegisterSpec for PRIV_EN_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [priv_en_lock::R](R) reader structure"]
impl crate::Readable for PRIV_EN_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [priv_en_lock::W](W) writer structure"]
impl crate::Writable for PRIV_EN_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRIV_EN_LOCK to value 0"]
impl crate::Resettable for PRIV_EN_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
