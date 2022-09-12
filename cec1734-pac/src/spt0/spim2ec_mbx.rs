#[doc = "Register `SPIM2EC_MBX` reader"]
pub struct R(crate::R<SPIM2EC_MBX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIM2EC_MBX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIM2EC_MBX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIM2EC_MBX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIM2EC_MBX` writer"]
pub struct W(crate::W<SPIM2EC_MBX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIM2EC_MBX_SPEC>;
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
impl From<crate::W<SPIM2EC_MBX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIM2EC_MBX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M2EC` reader - Write only register for the Host. When data is written to this register the IBF Flag is set. EC can read the data and writes of 0xFFFF will clear this register. Any form of read will clear the flag for this register."]
pub type M2EC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `M2EC` writer - Write only register for the Host. When data is written to this register the IBF Flag is set. EC can read the data and writes of 0xFFFF will clear this register. Any form of read will clear the flag for this register."]
pub type M2EC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPIM2EC_MBX_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Write only register for the Host. When data is written to this register the IBF Flag is set. EC can read the data and writes of 0xFFFF will clear this register. Any form of read will clear the flag for this register."]
    #[inline(always)]
    pub fn m2ec(&self) -> M2EC_R {
        M2EC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write only register for the Host. When data is written to this register the IBF Flag is set. EC can read the data and writes of 0xFFFF will clear this register. Any form of read will clear the flag for this register."]
    #[inline(always)]
    pub fn m2ec(&mut self) -> M2EC_W<0> {
        M2EC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Peripheral Target Master to EC Mailbox Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spim2ec_mbx](index.html) module"]
pub struct SPIM2EC_MBX_SPEC;
impl crate::RegisterSpec for SPIM2EC_MBX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spim2ec_mbx::R](R) reader structure"]
impl crate::Readable for SPIM2EC_MBX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spim2ec_mbx::W](W) writer structure"]
impl crate::Writable for SPIM2EC_MBX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPIM2EC_MBX to value 0"]
impl crate::Resettable for SPIM2EC_MBX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
