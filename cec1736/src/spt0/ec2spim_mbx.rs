#[doc = "Register `EC2SPIM_MBX` reader"]
pub struct R(crate::R<EC2SPIM_MBX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EC2SPIM_MBX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EC2SPIM_MBX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EC2SPIM_MBX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EC2SPIM_MBX` writer"]
pub struct W(crate::W<EC2SPIM_MBX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EC2SPIM_MBX_SPEC>;
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
impl From<crate::W<EC2SPIM_MBX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EC2SPIM_MBX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EC2M` reader - Read only register for the Host. When data is written to this register the OBF Flag is set. Host can read the data and writes of 0xFFFF_FFFF will clear this register, also clearing the flag. Any form of read will clear the flag for this register"]
pub type EC2M_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EC2M` writer - Read only register for the Host. When data is written to this register the OBF Flag is set. Host can read the data and writes of 0xFFFF_FFFF will clear this register, also clearing the flag. Any form of read will clear the flag for this register"]
pub type EC2M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EC2SPIM_MBX_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Read only register for the Host. When data is written to this register the OBF Flag is set. Host can read the data and writes of 0xFFFF_FFFF will clear this register, also clearing the flag. Any form of read will clear the flag for this register"]
    #[inline(always)]
    pub fn ec2m(&self) -> EC2M_R {
        EC2M_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read only register for the Host. When data is written to this register the OBF Flag is set. Host can read the data and writes of 0xFFFF_FFFF will clear this register, also clearing the flag. Any form of read will clear the flag for this register"]
    #[inline(always)]
    pub fn ec2m(&mut self) -> EC2M_W<0> {
        EC2M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Peripheral Target Master to EC Mailbox Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec2spim_mbx](index.html) module"]
pub struct EC2SPIM_MBX_SPEC;
impl crate::RegisterSpec for EC2SPIM_MBX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ec2spim_mbx::R](R) reader structure"]
impl crate::Readable for EC2SPIM_MBX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ec2spim_mbx::W](W) writer structure"]
impl crate::Writable for EC2SPIM_MBX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EC2SPIM_MBX to value 0"]
impl crate::Resettable for EC2SPIM_MBX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
