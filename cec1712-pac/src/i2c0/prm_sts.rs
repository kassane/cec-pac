#[doc = "Register `PRM_STS` reader"]
pub struct R(crate::R<PRM_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRM_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRM_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRM_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRM_STS` writer"]
pub struct W(crate::W<PRM_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRM_STS_SPEC>;
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
impl From<crate::W<PRM_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRM_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR_INTR` reader - This is the Promiscuous Address Status interrupt. This bit is functional only in Promiscuous mode. This bit is set \n on the 8th clock of the I2C Address. This bit will hold the Clock line low till this register bit is cleared. In other words, \n Software should read the address program ACK/NAK and clear this bit for the ACK/NAK clock to appear on the I2C clock (SCL) line. \n This is Write 1 to clear. R/W1C."]
pub type ADDR_INTR_R = crate::BitReader<bool>;
#[doc = "Field `ADDR_INTR` writer - This is the Promiscuous Address Status interrupt. This bit is functional only in Promiscuous mode. This bit is set \n on the 8th clock of the I2C Address. This bit will hold the Clock line low till this register bit is cleared. In other words, \n Software should read the address program ACK/NAK and clear this bit for the ACK/NAK clock to appear on the I2C clock (SCL) line. \n This is Write 1 to clear. R/W1C."]
pub type ADDR_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRM_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This is the Promiscuous Address Status interrupt. This bit is functional only in Promiscuous mode. This bit is set \n on the 8th clock of the I2C Address. This bit will hold the Clock line low till this register bit is cleared. In other words, \n Software should read the address program ACK/NAK and clear this bit for the ACK/NAK clock to appear on the I2C clock (SCL) line. \n This is Write 1 to clear. R/W1C."]
    #[inline(always)]
    pub fn addr_intr(&self) -> ADDR_INTR_R {
        ADDR_INTR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is the Promiscuous Address Status interrupt. This bit is functional only in Promiscuous mode. This bit is set \n on the 8th clock of the I2C Address. This bit will hold the Clock line low till this register bit is cleared. In other words, \n Software should read the address program ACK/NAK and clear this bit for the ACK/NAK clock to appear on the I2C clock (SCL) line. \n This is Write 1 to clear. R/W1C."]
    #[inline(always)]
    pub fn addr_intr(&mut self) -> ADDR_INTR_W<0> {
        ADDR_INTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the Promiscuous Interrupt Register. This register bit will be functional only in Promiscuous mode.\n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prm_sts](index.html) module"]
pub struct PRM_STS_SPEC;
impl crate::RegisterSpec for PRM_STS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [prm_sts::R](R) reader structure"]
impl crate::Readable for PRM_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prm_sts::W](W) writer structure"]
impl crate::Writable for PRM_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRM_STS to value 0"]
impl crate::Resettable for PRM_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
