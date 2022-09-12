#[doc = "Register `MTMON_BEGIN` reader"]
pub struct R(crate::R<MTMON_BEGIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTMON_BEGIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTMON_BEGIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTMON_BEGIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTMON_BEGIN` writer"]
pub struct W(crate::W<MTMON_BEGIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTMON_BEGIN_SPEC>;
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
impl From<crate::W<MTMON_BEGIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTMON_BEGIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BGN` reader - The 19 bits of base address within the designated SPI Flash, specifying the last aligned 8K block."]
pub type BGN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BGN` writer - The 19 bits of base address within the designated SPI Flash, specifying the last aligned 8K block."]
pub type BGN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTMON_BEGIN_SPEC, u32, u32, 19, O>;
#[doc = "Field `DV` reader - Flash Device number, applying to both Begin and End fields 0 = CS0#, 1 = CS1#"]
pub type DV_R = crate::BitReader<bool>;
#[doc = "Field `DV` writer - Flash Device number, applying to both Begin and End fields 0 = CS0#, 1 = CS1#"]
pub type DV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_BEGIN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:18 - The 19 bits of base address within the designated SPI Flash, specifying the last aligned 8K block."]
    #[inline(always)]
    pub fn bgn(&self) -> BGN_R {
        BGN_R::new((self.bits & 0x0007_ffff) as u32)
    }
    #[doc = "Bit 31 - Flash Device number, applying to both Begin and End fields 0 = CS0#, 1 = CS1#"]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18 - The 19 bits of base address within the designated SPI Flash, specifying the last aligned 8K block."]
    #[inline(always)]
    pub fn bgn(&mut self) -> BGN_W<0> {
        BGN_W::new(self)
    }
    #[doc = "Bit 31 - Flash Device number, applying to both Begin and End fields 0 = CS0#, 1 = CS1#"]
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
#[doc = "Match Monitor Region Begin Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtmon_begin](index.html) module"]
pub struct MTMON_BEGIN_SPEC;
impl crate::RegisterSpec for MTMON_BEGIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtmon_begin::R](R) reader structure"]
impl crate::Readable for MTMON_BEGIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtmon_begin::W](W) writer structure"]
impl crate::Writable for MTMON_BEGIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTMON_BEGIN to value 0"]
impl crate::Resettable for MTMON_BEGIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
