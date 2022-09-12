#[doc = "Register `BLK_EN_CLR` reader"]
pub struct R(crate::R<BLK_EN_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK_EN_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK_EN_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK_EN_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK_EN_CLR` writer"]
pub struct W(crate::W<BLK_EN_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK_EN_CLR_SPEC>;
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
impl From<crate::W<BLK_EN_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK_EN_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTOR_EN_CLR` reader - Each GIRQx bit can be individually disabled to inhibit an interrupt event.\n Reads always return the current value of the internal GIRQX_ENABLE bit. The state of the GIRQX_ENABLE bit is determined by\n the corresponding GIRQX_ENABLE_SET bit and the GIRQX_ENABLE_ CLEAR bit. (0=disabled, 1=enabled) (R/WC)\n 1=All interrupts in the GIRQx Source Register are disabled\n 0=No effect."]
pub type VTOR_EN_CLR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VTOR_EN_CLR` writer - Each GIRQx bit can be individually disabled to inhibit an interrupt event.\n Reads always return the current value of the internal GIRQX_ENABLE bit. The state of the GIRQX_ENABLE bit is determined by\n the corresponding GIRQX_ENABLE_SET bit and the GIRQX_ENABLE_ CLEAR bit. (0=disabled, 1=enabled) (R/WC)\n 1=All interrupts in the GIRQx Source Register are disabled\n 0=No effect."]
pub type VTOR_EN_CLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLK_EN_CLR_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 0:30 - Each GIRQx bit can be individually disabled to inhibit an interrupt event.\n Reads always return the current value of the internal GIRQX_ENABLE bit. The state of the GIRQX_ENABLE bit is determined by\n the corresponding GIRQX_ENABLE_SET bit and the GIRQX_ENABLE_ CLEAR bit. (0=disabled, 1=enabled) (R/WC)\n 1=All interrupts in the GIRQx Source Register are disabled\n 0=No effect."]
    #[inline(always)]
    pub fn vtor_en_clr(&self) -> VTOR_EN_CLR_R {
        VTOR_EN_CLR_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - Each GIRQx bit can be individually disabled to inhibit an interrupt event.\n Reads always return the current value of the internal GIRQX_ENABLE bit. The state of the GIRQX_ENABLE bit is determined by\n the corresponding GIRQX_ENABLE_SET bit and the GIRQX_ENABLE_ CLEAR bit. (0=disabled, 1=enabled) (R/WC)\n 1=All interrupts in the GIRQx Source Register are disabled\n 0=No effect."]
    #[inline(always)]
    pub fn vtor_en_clr(&mut self) -> VTOR_EN_CLR_W<0> {
        VTOR_EN_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Enable Clear Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk_en_clr](index.html) module"]
pub struct BLK_EN_CLR_SPEC;
impl crate::RegisterSpec for BLK_EN_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk_en_clr::R](R) reader structure"]
impl crate::Readable for BLK_EN_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk_en_clr::W](W) writer structure"]
impl crate::Writable for BLK_EN_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLK_EN_CLR to value 0"]
impl crate::Resettable for BLK_EN_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
