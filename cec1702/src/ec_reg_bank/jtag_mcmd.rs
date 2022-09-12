#[doc = "Register `JTAG_MCMD` reader"]
pub struct R(crate::R<JTAG_MCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JTAG_MCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JTAG_MCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JTAG_MCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JTAG_MCMD` writer"]
pub struct W(crate::W<JTAG_MCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JTAG_MCMD_SPEC>;
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
impl From<crate::W<JTAG_MCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JTAG_MCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JTM_COUNT` reader - If the JTAG Port is configured as a Master, writing this register starts clocking and shifting on the JTAG port. The JTAG\n Master port will shift JTM_COUNT+1 times, so writing a '0h' will shift 1 bit, and writing '31h' will shift 32 bits. The signal JTAG_CLK\n will cycle JTM_COUNT+1 times. The contents of the JTAG Master TMS Register and the JTAG Master TDI Register will be shifted out on\n the falling edge of JTAG_CLK and the.JTAG Master TDO Register will get shifted in on the rising edge of JTAG_CLK.\n If the JTAG Port is configured as a Slave, writing this register has no effect."]
pub type JTM_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JTM_COUNT` writer - If the JTAG Port is configured as a Master, writing this register starts clocking and shifting on the JTAG port. The JTAG\n Master port will shift JTM_COUNT+1 times, so writing a '0h' will shift 1 bit, and writing '31h' will shift 32 bits. The signal JTAG_CLK\n will cycle JTM_COUNT+1 times. The contents of the JTAG Master TMS Register and the JTAG Master TDI Register will be shifted out on\n the falling edge of JTAG_CLK and the.JTAG Master TDO Register will get shifted in on the rising edge of JTAG_CLK.\n If the JTAG Port is configured as a Slave, writing this register has no effect."]
pub type JTM_COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JTAG_MCMD_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - If the JTAG Port is configured as a Master, writing this register starts clocking and shifting on the JTAG port. The JTAG\n Master port will shift JTM_COUNT+1 times, so writing a '0h' will shift 1 bit, and writing '31h' will shift 32 bits. The signal JTAG_CLK\n will cycle JTM_COUNT+1 times. The contents of the JTAG Master TMS Register and the JTAG Master TDI Register will be shifted out on\n the falling edge of JTAG_CLK and the.JTAG Master TDO Register will get shifted in on the rising edge of JTAG_CLK.\n If the JTAG Port is configured as a Slave, writing this register has no effect."]
    #[inline(always)]
    pub fn jtm_count(&self) -> JTM_COUNT_R {
        JTM_COUNT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - If the JTAG Port is configured as a Master, writing this register starts clocking and shifting on the JTAG port. The JTAG\n Master port will shift JTM_COUNT+1 times, so writing a '0h' will shift 1 bit, and writing '31h' will shift 32 bits. The signal JTAG_CLK\n will cycle JTM_COUNT+1 times. The contents of the JTAG Master TMS Register and the JTAG Master TDI Register will be shifted out on\n the falling edge of JTAG_CLK and the.JTAG Master TDO Register will get shifted in on the rising edge of JTAG_CLK.\n If the JTAG Port is configured as a Slave, writing this register has no effect."]
    #[inline(always)]
    pub fn jtm_count(&mut self) -> JTM_COUNT_W<0> {
        JTM_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JTAG Master Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtag_mcmd](index.html) module"]
pub struct JTAG_MCMD_SPEC;
impl crate::RegisterSpec for JTAG_MCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jtag_mcmd::R](R) reader structure"]
impl crate::Readable for JTAG_MCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jtag_mcmd::W](W) writer structure"]
impl crate::Writable for JTAG_MCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JTAG_MCMD to value 0"]
impl crate::Resettable for JTAG_MCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
