#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Block enable: 1=block is enabled for operation; 0=block is disabled and in lowest power state."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Block enable: 1=block is enabled for operation; 0=block is disabled and in lowest power state."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `RST` reader - Block reset: 1=Block is reset; 0=Normal operation. This bit self-clears and always reads back 0."]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - Block reset: 1=Block is reset; 0=Normal operation. This bit self-clears and always reads back 0."]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `EXT_PGM` reader - External programming enable: 1=eFUSE programming is done via external pin interface 0=Manual/Normal mode.\n eFUSE programming is done via this block's register set."]
pub type EXT_PGM_R = crate::BitReader<bool>;
#[doc = "Field `EXT_PGM` writer - External programming enable: 1=eFUSE programming is done via external pin interface 0=Manual/Normal mode.\n eFUSE programming is done via this block's register set."]
pub type EXT_PGM_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `FSOURCE_EN_PRGM` reader - FSOURCE pin enable for programming: 1=FSOURCE switch logic connects eFUSE FSOURCE pin to a power pad for PROGRAM mode.\n Only set this bit when FSOURCE_EN_READ bit is already 0 to avoid shorting the power pad to ground;\n 0=FSOURCE switch logic isolates eFUSE FSOURCE pin from power pad."]
pub type FSOURCE_EN_PRGM_R = crate::BitReader<bool>;
#[doc = "Field `FSOURCE_EN_PRGM` writer - FSOURCE pin enable for programming: 1=FSOURCE switch logic connects eFUSE FSOURCE pin to a power pad for PROGRAM mode.\n Only set this bit when FSOURCE_EN_READ bit is already 0 to avoid shorting the power pad to ground;\n 0=FSOURCE switch logic isolates eFUSE FSOURCE pin from power pad."]
pub type FSOURCE_EN_PRGM_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `FSOURCE_EN_READ` reader - FSOURCE pin enable for reading: 1=FSOURCE switch logic connects eFUSE FSOURCE pin to a power pad for read mode.\n Only set this bit when FSOURCE_EN_PRGM bit is already 0 to avoid shorting the power pad to ground;\n 0=FSOURCE switch logic isolates eFUSE FSOURCE pin from ground."]
pub type FSOURCE_EN_READ_R = crate::BitReader<bool>;
#[doc = "Field `FSOURCE_EN_READ` writer - FSOURCE pin enable for reading: 1=FSOURCE switch logic connects eFUSE FSOURCE pin to a power pad for read mode.\n Only set this bit when FSOURCE_EN_PRGM bit is already 0 to avoid shorting the power pad to ground;\n 0=FSOURCE switch logic isolates eFUSE FSOURCE pin from ground."]
pub type FSOURCE_EN_READ_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Block enable: 1=block is enabled for operation; 0=block is disabled and in lowest power state."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block reset: 1=Block is reset; 0=Normal operation. This bit self-clears and always reads back 0."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External programming enable: 1=eFUSE programming is done via external pin interface 0=Manual/Normal mode.\n eFUSE programming is done via this block's register set."]
    #[inline(always)]
    pub fn ext_pgm(&self) -> EXT_PGM_R {
        EXT_PGM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FSOURCE pin enable for programming: 1=FSOURCE switch logic connects eFUSE FSOURCE pin to a power pad for PROGRAM mode.\n Only set this bit when FSOURCE_EN_READ bit is already 0 to avoid shorting the power pad to ground;\n 0=FSOURCE switch logic isolates eFUSE FSOURCE pin from power pad."]
    #[inline(always)]
    pub fn fsource_en_prgm(&self) -> FSOURCE_EN_PRGM_R {
        FSOURCE_EN_PRGM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FSOURCE pin enable for reading: 1=FSOURCE switch logic connects eFUSE FSOURCE pin to a power pad for read mode.\n Only set this bit when FSOURCE_EN_PRGM bit is already 0 to avoid shorting the power pad to ground;\n 0=FSOURCE switch logic isolates eFUSE FSOURCE pin from ground."]
    #[inline(always)]
    pub fn fsource_en_read(&self) -> FSOURCE_EN_READ_R {
        FSOURCE_EN_READ_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Block enable: 1=block is enabled for operation; 0=block is disabled and in lowest power state."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Block reset: 1=Block is reset; 0=Normal operation. This bit self-clears and always reads back 0."]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<1> {
        RST_W::new(self)
    }
    #[doc = "Bit 2 - External programming enable: 1=eFUSE programming is done via external pin interface 0=Manual/Normal mode.\n eFUSE programming is done via this block's register set."]
    #[inline(always)]
    pub fn ext_pgm(&mut self) -> EXT_PGM_W<2> {
        EXT_PGM_W::new(self)
    }
    #[doc = "Bit 3 - FSOURCE pin enable for programming: 1=FSOURCE switch logic connects eFUSE FSOURCE pin to a power pad for PROGRAM mode.\n Only set this bit when FSOURCE_EN_READ bit is already 0 to avoid shorting the power pad to ground;\n 0=FSOURCE switch logic isolates eFUSE FSOURCE pin from power pad."]
    #[inline(always)]
    pub fn fsource_en_prgm(&mut self) -> FSOURCE_EN_PRGM_W<3> {
        FSOURCE_EN_PRGM_W::new(self)
    }
    #[doc = "Bit 4 - FSOURCE pin enable for reading: 1=FSOURCE switch logic connects eFUSE FSOURCE pin to a power pad for read mode.\n Only set this bit when FSOURCE_EN_PRGM bit is already 0 to avoid shorting the power pad to ground;\n 0=FSOURCE switch logic isolates eFUSE FSOURCE pin from ground."]
    #[inline(always)]
    pub fn fsource_en_read(&mut self) -> FSOURCE_EN_READ_W<4> {
        FSOURCE_EN_READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eFUSE CTRL Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x10"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
