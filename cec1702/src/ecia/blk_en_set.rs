#[doc = "Register `BLK_EN_SET` reader"]
pub struct R(crate::R<BLK_EN_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK_EN_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK_EN_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK_EN_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK_EN_SET` writer"]
pub struct W(crate::W<BLK_EN_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK_EN_SET_SPEC>;
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
impl From<crate::W<BLK_EN_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK_EN_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTOR_EN_SET` reader - Each GIRQx bit can be individually enabled to assert an interrupt event.\n Reads always return the current value of the internal GIRQX_ENABLE bit. The state of the GIRQX_ENABLE bit is determined by\n the corresponding GIRQX_ENABLE_SET bit and the GIRQX_ENABLE_ CLEAR bit. (0=disabled, 1=enabled) (R/WS)\n 1=Interrupts in the GIRQx Source Register may be enabled\n 0=No effect."]
pub type VTOR_EN_SET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VTOR_EN_SET` writer - Each GIRQx bit can be individually enabled to assert an interrupt event.\n Reads always return the current value of the internal GIRQX_ENABLE bit. The state of the GIRQX_ENABLE bit is determined by\n the corresponding GIRQX_ENABLE_SET bit and the GIRQX_ENABLE_ CLEAR bit. (0=disabled, 1=enabled) (R/WS)\n 1=Interrupts in the GIRQx Source Register may be enabled\n 0=No effect."]
pub type VTOR_EN_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLK_EN_SET_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 0:30 - Each GIRQx bit can be individually enabled to assert an interrupt event.\n Reads always return the current value of the internal GIRQX_ENABLE bit. The state of the GIRQX_ENABLE bit is determined by\n the corresponding GIRQX_ENABLE_SET bit and the GIRQX_ENABLE_ CLEAR bit. (0=disabled, 1=enabled) (R/WS)\n 1=Interrupts in the GIRQx Source Register may be enabled\n 0=No effect."]
    #[inline(always)]
    pub fn vtor_en_set(&self) -> VTOR_EN_SET_R {
        VTOR_EN_SET_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - Each GIRQx bit can be individually enabled to assert an interrupt event.\n Reads always return the current value of the internal GIRQX_ENABLE bit. The state of the GIRQX_ENABLE bit is determined by\n the corresponding GIRQX_ENABLE_SET bit and the GIRQX_ENABLE_ CLEAR bit. (0=disabled, 1=enabled) (R/WS)\n 1=Interrupts in the GIRQx Source Register may be enabled\n 0=No effect."]
    #[inline(always)]
    pub fn vtor_en_set(&mut self) -> VTOR_EN_SET_W<0> {
        VTOR_EN_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Enable Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk_en_set](index.html) module"]
pub struct BLK_EN_SET_SPEC;
impl crate::RegisterSpec for BLK_EN_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk_en_set::R](R) reader structure"]
impl crate::Readable for BLK_EN_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk_en_set::W](W) writer structure"]
impl crate::Writable for BLK_EN_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLK_EN_SET to value 0"]
impl crate::Resettable for BLK_EN_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
