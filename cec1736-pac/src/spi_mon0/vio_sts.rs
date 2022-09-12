#[doc = "Register `VIO_STS` reader"]
pub struct R(crate::R<VIO_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VIO_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VIO_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VIO_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VIO_STS` writer"]
pub struct W(crate::W<VIO_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VIO_STS_SPEC>;
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
impl From<crate::W<VIO_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VIO_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OP` reader - Opcode Violation"]
pub type OP_R = crate::BitReader<bool>;
#[doc = "Field `OP` writer - Opcode Violation"]
pub type OP_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIO_STS_SPEC, bool, O>;
#[doc = "Field `ROB` reader - Region Failure, Out of Bounds"]
pub type ROB_R = crate::BitReader<bool>;
#[doc = "Field `ROB` writer - Region Failure, Out of Bounds"]
pub type ROB_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIO_STS_SPEC, bool, O>;
#[doc = "Field `REG` reader - Runtime Region Violation"]
pub type REG_R = crate::BitReader<bool>;
#[doc = "Field `REG` writer - Runtime Region Violation"]
pub type REG_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIO_STS_SPEC, bool, O>;
#[doc = "Field `AWP` reader - Address Wrap"]
pub type AWP_R = crate::BitReader<bool>;
#[doc = "Field `AWP` writer - Address Wrap"]
pub type AWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIO_STS_SPEC, bool, O>;
#[doc = "Field `DV` reader - Device Number"]
pub type DV_R = crate::BitReader<bool>;
#[doc = "Field `DV` writer - Device Number"]
pub type DV_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIO_STS_SPEC, bool, O>;
#[doc = "Field `REGION` reader - Region Number"]
pub type REGION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGION` writer - Region Number"]
pub type REGION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VIO_STS_SPEC, u8, u8, 5, O>;
#[doc = "Field `RD` reader - Killed as a Read"]
pub type RD_R = crate::BitReader<bool>;
#[doc = "Field `RD` writer - Killed as a Read"]
pub type RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIO_STS_SPEC, bool, O>;
#[doc = "Field `PE` reader - Killed as a Program or Erase"]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `PE` writer - Killed as a Program or Erase"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIO_STS_SPEC, bool, O>;
#[doc = "Field `AM` reader - Flash Address Mode"]
pub type AM_R = crate::BitReader<bool>;
#[doc = "Field `AM` writer - Flash Address Mode"]
pub type AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIO_STS_SPEC, bool, O>;
#[doc = "Field `CLR` reader - Clear Register RW1C. This bit is auto clearing"]
pub type CLR_R = crate::BitReader<bool>;
#[doc = "Field `CLR` writer - Clear Register RW1C. This bit is auto clearing"]
pub type CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, VIO_STS_SPEC, bool, O>;
#[doc = "Field `OPCOD` reader - Flash Opcode"]
pub type OPCOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPCOD` writer - Flash Opcode"]
pub type OPCOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VIO_STS_SPEC, u8, u8, 8, O>;
#[doc = "Field `DAT` reader - SPI Data Byte"]
pub type DAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAT` writer - SPI Data Byte"]
pub type DAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VIO_STS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Opcode Violation"]
    #[inline(always)]
    pub fn op(&self) -> OP_R {
        OP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Region Failure, Out of Bounds"]
    #[inline(always)]
    pub fn rob(&self) -> ROB_R {
        ROB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Runtime Region Violation"]
    #[inline(always)]
    pub fn reg(&self) -> REG_R {
        REG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Address Wrap"]
    #[inline(always)]
    pub fn awp(&self) -> AWP_R {
        AWP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Device Number"]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Region Number"]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bit 12 - Killed as a Read"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Killed as a Program or Erase"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flash Address Mode"]
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
    pub fn dat(&self) -> DAT_R {
        DAT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Opcode Violation"]
    #[inline(always)]
    pub fn op(&mut self) -> OP_W<0> {
        OP_W::new(self)
    }
    #[doc = "Bit 2 - Region Failure, Out of Bounds"]
    #[inline(always)]
    pub fn rob(&mut self) -> ROB_W<2> {
        ROB_W::new(self)
    }
    #[doc = "Bit 3 - Runtime Region Violation"]
    #[inline(always)]
    pub fn reg(&mut self) -> REG_W<3> {
        REG_W::new(self)
    }
    #[doc = "Bit 5 - Address Wrap"]
    #[inline(always)]
    pub fn awp(&mut self) -> AWP_W<5> {
        AWP_W::new(self)
    }
    #[doc = "Bit 6 - Device Number"]
    #[inline(always)]
    pub fn dv(&mut self) -> DV_W<6> {
        DV_W::new(self)
    }
    #[doc = "Bits 7:11 - Region Number"]
    #[inline(always)]
    pub fn region(&mut self) -> REGION_W<7> {
        REGION_W::new(self)
    }
    #[doc = "Bit 12 - Killed as a Read"]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W<12> {
        RD_W::new(self)
    }
    #[doc = "Bit 13 - Killed as a Program or Erase"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<13> {
        PE_W::new(self)
    }
    #[doc = "Bit 14 - Flash Address Mode"]
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
    pub fn dat(&mut self) -> DAT_W<24> {
        DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Violation Log Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vio_sts](index.html) module"]
pub struct VIO_STS_SPEC;
impl crate::RegisterSpec for VIO_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vio_sts::R](R) reader structure"]
impl crate::Readable for VIO_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vio_sts::W](W) writer structure"]
impl crate::Writable for VIO_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VIO_STS to value 0"]
impl crate::Resettable for VIO_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
