#[doc = "Register `SLV_ADDR` reader"]
pub struct R(crate::R<SLV_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLV_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLV_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLV_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLV_ADDR` writer"]
pub struct W(crate::W<SLV_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLV_ADDR_SPEC>;
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
impl From<crate::W<SLV_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLV_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADDR` reader - This register stores value of address + LSB direction after the 8th clock of the Address Byte."]
pub type SADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SADDR` writer - This register stores value of address + LSB direction after the 8th clock of the Address Byte."]
pub type SADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SLV_ADDR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register stores value of address + LSB direction after the 8th clock of the Address Byte."]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register stores value of address + LSB direction after the 8th clock of the Address Byte."]
    #[inline(always)]
    pub fn saddr(&mut self) -> SADDR_W<0> {
        SADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the Slave Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slv_addr](index.html) module"]
pub struct SLV_ADDR_SPEC;
impl crate::RegisterSpec for SLV_ADDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [slv_addr::R](R) reader structure"]
impl crate::Readable for SLV_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slv_addr::W](W) writer structure"]
impl crate::Writable for SLV_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLV_ADDR to value 0"]
impl crate::Resettable for SLV_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
