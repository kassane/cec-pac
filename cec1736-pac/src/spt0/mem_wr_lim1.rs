#[doc = "Register `MEM_WR_LIM1` reader"]
pub struct R(crate::R<MEM_WR_LIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_WR_LIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_WR_LIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_WR_LIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_WR_LIM1` writer"]
pub struct W(crate::W<MEM_WR_LIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_WR_LIM1_SPEC>;
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
impl From<crate::W<MEM_WR_LIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_WR_LIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LMT1` reader - Write Limit for Region 1."]
pub type LMT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LMT1` writer - Write Limit for Region 1."]
pub type LMT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEM_WR_LIM1_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Write Limit for Region 1."]
    #[inline(always)]
    pub fn lmt1(&self) -> LMT1_R {
        LMT1_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Write Limit for Region 1."]
    #[inline(always)]
    pub fn lmt1(&mut self) -> LMT1_W<0> {
        LMT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Peripheral Target Memory Write LIMIT 1 Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_wr_lim1](index.html) module"]
pub struct MEM_WR_LIM1_SPEC;
impl crate::RegisterSpec for MEM_WR_LIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_wr_lim1::R](R) reader structure"]
impl crate::Readable for MEM_WR_LIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_wr_lim1::W](W) writer structure"]
impl crate::Writable for MEM_WR_LIM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_WR_LIM1 to value 0"]
impl crate::Resettable for MEM_WR_LIM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
