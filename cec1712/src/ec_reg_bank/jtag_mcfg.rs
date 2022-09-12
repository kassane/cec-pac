#[doc = "Register `JTAG_MCFG` reader"]
pub struct R(crate::R<JTAG_MCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JTAG_MCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JTAG_MCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JTAG_MCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JTAG_MCFG` writer"]
pub struct W(crate::W<JTAG_MCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JTAG_MCFG_SPEC>;
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
impl From<crate::W<JTAG_MCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JTAG_MCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JTM_CLK` reader - This field determines the JTAG Master clock rate, derived from the 48MHz master clock. 7=375KHz; 6=750KHz; 5=1.5Mhz; 4=3Mhz; 3=6Mhz; 2=12Mhz; 1=24MHz; 0=Reserved."]
pub type JTM_CLK_R = crate::FieldReader<u8, JTM_CLKSELECT_A>;
#[doc = "This field determines the JTAG Master clock rate, derived from the 48MHz master clock. 7=375KHz; 6=750KHz; 5=1.5Mhz; 4=3Mhz; 3=6Mhz; 2=12Mhz; 1=24MHz; 0=Reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JTM_CLKSELECT_A {
    #[doc = "7: 7=375KHz"]
    JTM_CLK_375KHZ = 7,
    #[doc = "6: 6=750KHz"]
    JTM_CLK_750KHZ = 6,
    #[doc = "5: 5=1.5Mhz"]
    JTM_CLK_1MHZ = 5,
    #[doc = "4: 4=3Mhz"]
    JTM_CLK_3MHZ = 4,
    #[doc = "3: 3=6Mhz"]
    JTM_CLK_6MHZ = 3,
    #[doc = "2: 2=12Mhz"]
    JTM_CLK_12MHZ = 2,
    #[doc = "1: 1=24MHz"]
    JTM_CLK_24MHZ = 1,
}
impl From<JTM_CLKSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: JTM_CLKSELECT_A) -> Self {
        variant as _
    }
}
impl JTM_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<JTM_CLKSELECT_A> {
        match self.bits {
            7 => Some(JTM_CLKSELECT_A::JTM_CLK_375KHZ),
            6 => Some(JTM_CLKSELECT_A::JTM_CLK_750KHZ),
            5 => Some(JTM_CLKSELECT_A::JTM_CLK_1MHZ),
            4 => Some(JTM_CLKSELECT_A::JTM_CLK_3MHZ),
            3 => Some(JTM_CLKSELECT_A::JTM_CLK_6MHZ),
            2 => Some(JTM_CLKSELECT_A::JTM_CLK_12MHZ),
            1 => Some(JTM_CLKSELECT_A::JTM_CLK_24MHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `JTM_CLK_375KHZ`"]
    #[inline(always)]
    pub fn is_jtm_clk_375khz(&self) -> bool {
        *self == JTM_CLKSELECT_A::JTM_CLK_375KHZ
    }
    #[doc = "Checks if the value of the field is `JTM_CLK_750KHZ`"]
    #[inline(always)]
    pub fn is_jtm_clk_750khz(&self) -> bool {
        *self == JTM_CLKSELECT_A::JTM_CLK_750KHZ
    }
    #[doc = "Checks if the value of the field is `JTM_CLK_1MHZ`"]
    #[inline(always)]
    pub fn is_jtm_clk_1mhz(&self) -> bool {
        *self == JTM_CLKSELECT_A::JTM_CLK_1MHZ
    }
    #[doc = "Checks if the value of the field is `JTM_CLK_3MHZ`"]
    #[inline(always)]
    pub fn is_jtm_clk_3mhz(&self) -> bool {
        *self == JTM_CLKSELECT_A::JTM_CLK_3MHZ
    }
    #[doc = "Checks if the value of the field is `JTM_CLK_6MHZ`"]
    #[inline(always)]
    pub fn is_jtm_clk_6mhz(&self) -> bool {
        *self == JTM_CLKSELECT_A::JTM_CLK_6MHZ
    }
    #[doc = "Checks if the value of the field is `JTM_CLK_12MHZ`"]
    #[inline(always)]
    pub fn is_jtm_clk_12mhz(&self) -> bool {
        *self == JTM_CLKSELECT_A::JTM_CLK_12MHZ
    }
    #[doc = "Checks if the value of the field is `JTM_CLK_24MHZ`"]
    #[inline(always)]
    pub fn is_jtm_clk_24mhz(&self) -> bool {
        *self == JTM_CLKSELECT_A::JTM_CLK_24MHZ
    }
}
#[doc = "Field `JTM_CLK` writer - This field determines the JTAG Master clock rate, derived from the 48MHz master clock. 7=375KHz; 6=750KHz; 5=1.5Mhz; 4=3Mhz; 3=6Mhz; 2=12Mhz; 1=24MHz; 0=Reserved."]
pub type JTM_CLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, JTAG_MCFG_SPEC, u8, JTM_CLKSELECT_A, 3, O>;
impl<'a, const O: u8> JTM_CLK_W<'a, O> {
    #[doc = "7=375KHz"]
    #[inline(always)]
    pub fn jtm_clk_375khz(self) -> &'a mut W {
        self.variant(JTM_CLKSELECT_A::JTM_CLK_375KHZ)
    }
    #[doc = "6=750KHz"]
    #[inline(always)]
    pub fn jtm_clk_750khz(self) -> &'a mut W {
        self.variant(JTM_CLKSELECT_A::JTM_CLK_750KHZ)
    }
    #[doc = "5=1.5Mhz"]
    #[inline(always)]
    pub fn jtm_clk_1mhz(self) -> &'a mut W {
        self.variant(JTM_CLKSELECT_A::JTM_CLK_1MHZ)
    }
    #[doc = "4=3Mhz"]
    #[inline(always)]
    pub fn jtm_clk_3mhz(self) -> &'a mut W {
        self.variant(JTM_CLKSELECT_A::JTM_CLK_3MHZ)
    }
    #[doc = "3=6Mhz"]
    #[inline(always)]
    pub fn jtm_clk_6mhz(self) -> &'a mut W {
        self.variant(JTM_CLKSELECT_A::JTM_CLK_6MHZ)
    }
    #[doc = "2=12Mhz"]
    #[inline(always)]
    pub fn jtm_clk_12mhz(self) -> &'a mut W {
        self.variant(JTM_CLKSELECT_A::JTM_CLK_12MHZ)
    }
    #[doc = "1=24MHz"]
    #[inline(always)]
    pub fn jtm_clk_24mhz(self) -> &'a mut W {
        self.variant(JTM_CLKSELECT_A::JTM_CLK_24MHZ)
    }
}
#[doc = "Field `MAS_SLV` reader - This bit controls the direction of the JTAG port. 1=The JTAG Port is configured as a Master\n 0=The JTAG Port is configures as a Slave."]
pub type MAS_SLV_R = crate::BitReader<bool>;
#[doc = "Field `MAS_SLV` writer - This bit controls the direction of the JTAG port. 1=The JTAG Port is configured as a Master\n 0=The JTAG Port is configures as a Slave."]
pub type MAS_SLV_W<'a, const O: u8> = crate::BitWriter<'a, u32, JTAG_MCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - This field determines the JTAG Master clock rate, derived from the 48MHz master clock. 7=375KHz; 6=750KHz; 5=1.5Mhz; 4=3Mhz; 3=6Mhz; 2=12Mhz; 1=24MHz; 0=Reserved."]
    #[inline(always)]
    pub fn jtm_clk(&self) -> JTM_CLK_R {
        JTM_CLK_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - This bit controls the direction of the JTAG port. 1=The JTAG Port is configured as a Master\n 0=The JTAG Port is configures as a Slave."]
    #[inline(always)]
    pub fn mas_slv(&self) -> MAS_SLV_R {
        MAS_SLV_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - This field determines the JTAG Master clock rate, derived from the 48MHz master clock. 7=375KHz; 6=750KHz; 5=1.5Mhz; 4=3Mhz; 3=6Mhz; 2=12Mhz; 1=24MHz; 0=Reserved."]
    #[inline(always)]
    pub fn jtm_clk(&mut self) -> JTM_CLK_W<0> {
        JTM_CLK_W::new(self)
    }
    #[doc = "Bit 3 - This bit controls the direction of the JTAG port. 1=The JTAG Port is configured as a Master\n 0=The JTAG Port is configures as a Slave."]
    #[inline(always)]
    pub fn mas_slv(&mut self) -> MAS_SLV_W<3> {
        MAS_SLV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JTAG Master Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtag_mcfg](index.html) module"]
pub struct JTAG_MCFG_SPEC;
impl crate::RegisterSpec for JTAG_MCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jtag_mcfg::R](R) reader structure"]
impl crate::Readable for JTAG_MCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jtag_mcfg::W](W) writer structure"]
impl crate::Writable for JTAG_MCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JTAG_MCFG to value 0"]
impl crate::Resettable for JTAG_MCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
