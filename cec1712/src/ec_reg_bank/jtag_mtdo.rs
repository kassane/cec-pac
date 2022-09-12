#[doc = "Register `JTAG_MTDO` reader"]
pub struct R(crate::R<JTAG_MTDO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JTAG_MTDO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JTAG_MTDO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JTAG_MTDO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JTAG_MTDO` writer"]
pub struct W(crate::W<JTAG_MTDO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JTAG_MTDO_SPEC>;
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
impl From<crate::W<JTAG_MTDO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JTAG_MTDO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JTM_TDO` reader - When the JTAG Master Command Register is written, from 1 to 32 bits are shifted into this register, starting with bit 0,\n from the JTAG_TDO pin. Shifting is at the rate determined by the JTM_CLK field in the JTAG Master Configuration Register."]
pub type JTM_TDO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `JTM_TDO` writer - When the JTAG Master Command Register is written, from 1 to 32 bits are shifted into this register, starting with bit 0,\n from the JTAG_TDO pin. Shifting is at the rate determined by the JTM_CLK field in the JTAG Master Configuration Register."]
pub type JTM_TDO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JTAG_MTDO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - When the JTAG Master Command Register is written, from 1 to 32 bits are shifted into this register, starting with bit 0,\n from the JTAG_TDO pin. Shifting is at the rate determined by the JTM_CLK field in the JTAG Master Configuration Register."]
    #[inline(always)]
    pub fn jtm_tdo(&self) -> JTM_TDO_R {
        JTM_TDO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When the JTAG Master Command Register is written, from 1 to 32 bits are shifted into this register, starting with bit 0,\n from the JTAG_TDO pin. Shifting is at the rate determined by the JTM_CLK field in the JTAG Master Configuration Register."]
    #[inline(always)]
    pub fn jtm_tdo(&mut self) -> JTM_TDO_W<0> {
        JTM_TDO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JTAG Master TDO Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtag_mtdo](index.html) module"]
pub struct JTAG_MTDO_SPEC;
impl crate::RegisterSpec for JTAG_MTDO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jtag_mtdo::R](R) reader structure"]
impl crate::Readable for JTAG_MTDO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jtag_mtdo::W](W) writer structure"]
impl crate::Writable for JTAG_MTDO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JTAG_MTDO to value 0"]
impl crate::Resettable for JTAG_MTDO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
