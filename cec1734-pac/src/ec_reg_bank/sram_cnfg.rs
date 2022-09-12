#[doc = "Register `SRAM_CNFG` reader"]
pub struct R(crate::R<SRAM_CNFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CNFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CNFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CNFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_CNFG` writer"]
pub struct W(crate::W<SRAM_CNFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CNFG_SPEC>;
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
impl From<crate::W<SRAM_CNFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CNFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_SIZE` reader - SRAM Configuration Register: 0: 384KB (352k Code, 32k Data) 1: 320kB (288k Code, 32k Data) 2: 256kB (224k Code, 32k Data) 3: Illegal 256kB (224k Code, 32k Data)"]
pub type SRAM_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRAM_SIZE` writer - SRAM Configuration Register: 0: 384KB (352k Code, 32k Data) 1: 320kB (288k Code, 32k Data) 2: 256kB (224k Code, 32k Data) 3: Illegal 256kB (224k Code, 32k Data)"]
pub type SRAM_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SRAM_CNFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - SRAM Configuration Register: 0: 384KB (352k Code, 32k Data) 1: 320kB (288k Code, 32k Data) 2: 256kB (224k Code, 32k Data) 3: Illegal 256kB (224k Code, 32k Data)"]
    #[inline(always)]
    pub fn sram_size(&self) -> SRAM_SIZE_R {
        SRAM_SIZE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SRAM Configuration Register: 0: 384KB (352k Code, 32k Data) 1: 320kB (288k Code, 32k Data) 2: 256kB (224k Code, 32k Data) 3: Illegal 256kB (224k Code, 32k Data)"]
    #[inline(always)]
    pub fn sram_size(&mut self) -> SRAM_SIZE_W<0> {
        SRAM_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_cnfg](index.html) module"]
pub struct SRAM_CNFG_SPEC;
impl crate::RegisterSpec for SRAM_CNFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sram_cnfg::R](R) reader structure"]
impl crate::Readable for SRAM_CNFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_cnfg::W](W) writer structure"]
impl crate::Writable for SRAM_CNFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM_CNFG to value 0"]
impl crate::Resettable for SRAM_CNFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
