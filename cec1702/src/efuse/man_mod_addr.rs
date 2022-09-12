#[doc = "Register `MAN_MOD_ADDR` reader"]
pub struct R(crate::R<MAN_MOD_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAN_MOD_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAN_MOD_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAN_MOD_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAN_MOD_ADDR` writer"]
pub struct W(crate::W<MAN_MOD_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAN_MOD_ADDR_SPEC>;
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
impl From<crate::W<MAN_MOD_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAN_MOD_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP_ADDR_LO` reader - Manual mode address, selecting the bit address within a 1K bit block."]
pub type IP_ADDR_LO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IP_ADDR_LO` writer - Manual mode address, selecting the bit address within a 1K bit block."]
pub type IP_ADDR_LO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAN_MOD_ADDR_SPEC, u16, u16, 10, O>;
#[doc = "Field `IP_ADDR_HI` reader - Manual mode address, selecting a 1K bit block of eFuse data"]
pub type IP_ADDR_HI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_ADDR_HI` writer - Manual mode address, selecting a 1K bit block of eFuse data"]
pub type IP_ADDR_HI_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAN_MOD_ADDR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:9 - Manual mode address, selecting the bit address within a 1K bit block."]
    #[inline(always)]
    pub fn ip_addr_lo(&self) -> IP_ADDR_LO_R {
        IP_ADDR_LO_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:11 - Manual mode address, selecting a 1K bit block of eFuse data"]
    #[inline(always)]
    pub fn ip_addr_hi(&self) -> IP_ADDR_HI_R {
        IP_ADDR_HI_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Manual mode address, selecting the bit address within a 1K bit block."]
    #[inline(always)]
    pub fn ip_addr_lo(&mut self) -> IP_ADDR_LO_W<0> {
        IP_ADDR_LO_W::new(self)
    }
    #[doc = "Bits 10:11 - Manual mode address, selecting a 1K bit block of eFuse data"]
    #[inline(always)]
    pub fn ip_addr_hi(&mut self) -> IP_ADDR_HI_W<10> {
        IP_ADDR_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MANUAL MODE ADDRESS REGISTER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man_mod_addr](index.html) module"]
pub struct MAN_MOD_ADDR_SPEC;
impl crate::RegisterSpec for MAN_MOD_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [man_mod_addr::R](R) reader structure"]
impl crate::Readable for MAN_MOD_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [man_mod_addr::W](W) writer structure"]
impl crate::Writable for MAN_MOD_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAN_MOD_ADDR to value 0"]
impl crate::Resettable for MAN_MOD_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
