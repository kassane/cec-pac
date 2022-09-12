#[doc = "Register `OWN_ADDR` reader"]
pub struct R(crate::R<OWN_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OWN_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OWN_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OWN_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OWN_ADDR` writer"]
pub struct W(crate::W<OWN_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OWN_ADDR_SPEC>;
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
impl From<crate::W<OWN_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OWN_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR1` reader - The Own Address 1 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
pub type ADDR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR1` writer - The Own Address 1 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
pub type ADDR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OWN_ADDR_SPEC, u8, u8, 7, O>;
#[doc = "Field `ADDR2` reader - The Own Address 2 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
pub type ADDR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR2` writer - The Own Address 2 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
pub type ADDR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OWN_ADDR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The Own Address 1 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The Own Address 2 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
    #[inline(always)]
    pub fn addr2(&self) -> ADDR2_R {
        ADDR2_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The Own Address 1 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
    #[inline(always)]
    pub fn addr1(&mut self) -> ADDR1_W<0> {
        ADDR1_W::new(self)
    }
    #[doc = "Bits 8:14 - The Own Address 2 bits configure one of the two addresses to which the SMB Controller Core will respond when addressed as a slave."]
    #[inline(always)]
    pub fn addr2(&mut self) -> ADDR2_W<8> {
        ADDR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Own Address Register\n Note that the Data Register and Own Address fields are offset by one bit, so that programming Own Address 1 with a value of 55h will result in the value AAh being recognized as the SMB Controller Core slave address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [own_addr](index.html) module"]
pub struct OWN_ADDR_SPEC;
impl crate::RegisterSpec for OWN_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [own_addr::R](R) reader structure"]
impl crate::Readable for OWN_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [own_addr::W](W) writer structure"]
impl crate::Writable for OWN_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OWN_ADDR to value 0"]
impl crate::Resettable for OWN_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
