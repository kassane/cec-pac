#[doc = "Register `MTMON_VIOSTS` reader"]
pub struct R(crate::R<MTMON_VIOSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTMON_VIOSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTMON_VIOSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTMON_VIOSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTMON_VIOSTS` writer"]
pub struct W(crate::W<MTMON_VIOSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTMON_VIOSTS_SPEC>;
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
impl From<crate::W<MTMON_VIOSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTMON_VIOSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTO` reader - Match Monitor Timeout. This is read-Only bit."]
pub type MTO_R = crate::BitReader<bool>;
#[doc = "Field `MTO` writer - Match Monitor Timeout. This is read-Only bit."]
pub type MTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_VIOSTS_SPEC, bool, O>;
#[doc = "Field `DV` reader - Device Number. This is read-Only bit. 0 = CS0, 1 = CS1."]
pub type DV_R = crate::BitReader<bool>;
#[doc = "Field `DV` writer - Device Number. This is read-Only bit. 0 = CS0, 1 = CS1."]
pub type DV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_VIOSTS_SPEC, bool, O>;
#[doc = "Field `RGN` reader - Region. This is read-Only bit. Shows which of 8 Match regions \\[7:0\\]
got the mismatch."]
pub type RGN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RGN` writer - Region. This is read-Only bit. Shows which of 8 Match regions \\[7:0\\]
got the mismatch."]
pub type RGN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTMON_VIOSTS_SPEC, u8, u8, 5, O>;
#[doc = "Field `AM` reader - Flash Address Mode 0 = 3-byte, 1 = 4-byte"]
pub type AM_R = crate::BitReader<bool>;
#[doc = "Field `AM` writer - Flash Address Mode 0 = 3-byte, 1 = 4-byte"]
pub type AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_VIOSTS_SPEC, bool, O>;
#[doc = "Field `CLR` reader - Clear Register RW1C. This bit is auto clearing"]
pub type CLR_R = crate::BitReader<bool>;
#[doc = "Field `CLR` writer - Clear Register RW1C. This bit is auto clearing"]
pub type CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_VIOSTS_SPEC, bool, O>;
#[doc = "Field `OPCOD` reader - Flash Opcode"]
pub type OPCOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPCOD` writer - Flash Opcode"]
pub type OPCOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTMON_VIOSTS_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA` reader - SPI Data Byte"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - SPI Data Byte"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTMON_VIOSTS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 4 - Match Monitor Timeout. This is read-Only bit."]
    #[inline(always)]
    pub fn mto(&self) -> MTO_R {
        MTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Device Number. This is read-Only bit. 0 = CS0, 1 = CS1."]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Region. This is read-Only bit. Shows which of 8 Match regions \\[7:0\\]
got the mismatch."]
    #[inline(always)]
    pub fn rgn(&self) -> RGN_R {
        RGN_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Flash Address Mode 0 = 3-byte, 1 = 4-byte"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clear Register RW1C. This bit is auto clearing"]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Flash Opcode"]
    #[inline(always)]
    pub fn opcod(&self) -> OPCOD_R {
        OPCOD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SPI Data Byte"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Match Monitor Timeout. This is read-Only bit."]
    #[inline(always)]
    pub fn mto(&mut self) -> MTO_W<4> {
        MTO_W::new(self)
    }
    #[doc = "Bit 6 - Device Number. This is read-Only bit. 0 = CS0, 1 = CS1."]
    #[inline(always)]
    pub fn dv(&mut self) -> DV_W<6> {
        DV_W::new(self)
    }
    #[doc = "Bits 7:11 - Region. This is read-Only bit. Shows which of 8 Match regions \\[7:0\\]
got the mismatch."]
    #[inline(always)]
    pub fn rgn(&mut self) -> RGN_W<7> {
        RGN_W::new(self)
    }
    #[doc = "Bit 14 - Flash Address Mode 0 = 3-byte, 1 = 4-byte"]
    #[inline(always)]
    pub fn am(&mut self) -> AM_W<14> {
        AM_W::new(self)
    }
    #[doc = "Bit 15 - Clear Register RW1C. This bit is auto clearing"]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W<15> {
        CLR_W::new(self)
    }
    #[doc = "Bits 16:23 - Flash Opcode"]
    #[inline(always)]
    pub fn opcod(&mut self) -> OPCOD_W<16> {
        OPCOD_W::new(self)
    }
    #[doc = "Bits 24:31 - SPI Data Byte"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<24> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Match Monitor Violation Log Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtmon_viosts](index.html) module"]
pub struct MTMON_VIOSTS_SPEC;
impl crate::RegisterSpec for MTMON_VIOSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtmon_viosts::R](R) reader structure"]
impl crate::Readable for MTMON_VIOSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtmon_viosts::W](W) writer structure"]
impl crate::Writable for MTMON_VIOSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTMON_VIOSTS to value 0"]
impl crate::Resettable for MTMON_VIOSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
