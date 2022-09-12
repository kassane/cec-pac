#[doc = "Register `LCK_STRT` reader"]
pub struct R(crate::R<LCK_STRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCK_STRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCK_STRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCK_STRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCK_STRT` writer"]
pub struct W(crate::W<LCK_STRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCK_STRT_SPEC>;
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
impl From<crate::W<LCK_STRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCK_STRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCOMP2_EN` reader - Enables the software lock and monitoring functionality"]
pub type BCOMP2_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCOMP2_EN` writer - Enables the software lock and monitoring functionality"]
pub type BCOMP2_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LCK_STRT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Enables the software lock and monitoring functionality"]
    #[inline(always)]
    pub fn bcomp2_en(&self) -> BCOMP2_EN_R {
        BCOMP2_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enables the software lock and monitoring functionality"]
    #[inline(always)]
    pub fn bcomp2_en(&mut self) -> BCOMP2_EN_W<0> {
        BCOMP2_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock Start Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lck_strt](index.html) module"]
pub struct LCK_STRT_SPEC;
impl crate::RegisterSpec for LCK_STRT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lck_strt::R](R) reader structure"]
impl crate::Readable for LCK_STRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lck_strt::W](W) writer structure"]
impl crate::Writable for LCK_STRT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCK_STRT to value 0x01"]
impl crate::Resettable for LCK_STRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
