#[doc = "Register `LM_BEGIN` reader"]
pub struct R(crate::R<LM_BEGIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LM_BEGIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LM_BEGIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LM_BEGIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LM_BEGIN` writer"]
pub struct W(crate::W<LM_BEGIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LM_BEGIN_SPEC>;
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
impl From<crate::W<LM_BEGIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LM_BEGIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BADDR` reader - A byte address within the designated Flash, specifying the first byte of the load image."]
pub type BADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BADDR` writer - A byte address within the designated Flash, specifying the first byte of the load image."]
pub type BADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LM_BEGIN_SPEC, u32, u32, 31, O>;
#[doc = "Field `DV` reader - Flash Device number, applying to both Begin and End fields. 0 = CS0#, 1 = CS1#"]
pub type DV_R = crate::BitReader<bool>;
#[doc = "Field `DV` writer - Flash Device number, applying to both Begin and End fields. 0 = CS0#, 1 = CS1#"]
pub type DV_W<'a, const O: u8> = crate::BitWriter<'a, u32, LM_BEGIN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - A byte address within the designated Flash, specifying the first byte of the load image."]
    #[inline(always)]
    pub fn baddr(&self) -> BADDR_R {
        BADDR_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Flash Device number, applying to both Begin and End fields. 0 = CS0#, 1 = CS1#"]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - A byte address within the designated Flash, specifying the first byte of the load image."]
    #[inline(always)]
    pub fn baddr(&mut self) -> BADDR_W<0> {
        BADDR_W::new(self)
    }
    #[doc = "Bit 31 - Flash Device number, applying to both Begin and End fields. 0 = CS0#, 1 = CS1#"]
    #[inline(always)]
    pub fn dv(&mut self) -> DV_W<31> {
        DV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Loadtime Monitor Channel Begin Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lm_begin](index.html) module"]
pub struct LM_BEGIN_SPEC;
impl crate::RegisterSpec for LM_BEGIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lm_begin::R](R) reader structure"]
impl crate::Readable for LM_BEGIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lm_begin::W](W) writer structure"]
impl crate::Writable for LM_BEGIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LM_BEGIN to value 0"]
impl crate::Resettable for LM_BEGIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
