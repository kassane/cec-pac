#[doc = "Register `PWR_RST_STS` reader"]
pub struct R(crate::R<PWR_RST_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_RST_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_RST_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_RST_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_RST_STS` writer"]
pub struct W(crate::W<PWR_RST_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_RST_STS_SPEC>;
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
impl From<crate::W<PWR_RST_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_RST_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCC_PWRGD_STS` reader - Indicates the status of VCC_PWRGD. 0 = PWRGD not asserted. 1 = PWRGD asserte."]
pub type VCC_PWRGD_STS_R = crate::BitReader<bool>;
#[doc = "Field `VCC_PWRGD_STS` writer - Indicates the status of VCC_PWRGD. 0 = PWRGD not asserted. 1 = PWRGD asserte."]
pub type VCC_PWRGD_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_RST_STS_SPEC, bool, O>;
#[doc = "Field `RST_H_STS` reader - Indicates the status of RESET_VCC. 0 = reset active. 1 = reset not active."]
pub type RST_H_STS_R = crate::BitReader<bool>;
#[doc = "Field `RST_H_STS` writer - Indicates the status of RESET_VCC. 0 = reset active. 1 = reset not active."]
pub type RST_H_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_RST_STS_SPEC, bool, O>;
#[doc = "Field `VBAT_RST_STS` reader - VBAT reset status 0 = No reset occurred while VTR was off or since the last time this bit was cleared. 1 = A reset occurred.(R/WC)"]
pub type VBAT_RST_STS_R = crate::BitReader<bool>;
#[doc = "Field `VBAT_RST_STS` writer - VBAT reset status 0 = No reset occurred while VTR was off or since the last time this bit was cleared. 1 = A reset occurred.(R/WC)"]
pub type VBAT_RST_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_RST_STS_SPEC, bool, O>;
#[doc = "Field `RST_VTR_STS` reader - Indicates the status of VTR_RESET.(R/WC)\n 0 = No reset occurred since the last time this bit was cleared.\n 1 = A reset occurred."]
pub type RST_VTR_STS_R = crate::BitReader<bool>;
#[doc = "Field `RST_VTR_STS` writer - Indicates the status of VTR_RESET.(R/WC)\n 0 = No reset occurred since the last time this bit was cleared.\n 1 = A reset occurred."]
pub type RST_VTR_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_RST_STS_SPEC, bool, O>;
#[doc = "Field `JTAG_RST_STS` reader - Indicates s RESET_SYS was triggered by a JTAG action.(R/WC)\n 0 = No JTAG reset occurred since the last time this bit was cleared.\n 1 = A reset occurred because of a JATAG command."]
pub type JTAG_RST_STS_R = crate::BitReader<bool>;
#[doc = "Field `JTAG_RST_STS` writer - Indicates s RESET_SYS was triggered by a JTAG action.(R/WC)\n 0 = No JTAG reset occurred since the last time this bit was cleared.\n 1 = A reset occurred because of a JATAG command."]
pub type JTAG_RST_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_RST_STS_SPEC, bool, O>;
#[doc = "Field `_32K_ACTIVE` reader - 32K_ACTIVE"]
pub type _32K_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `_32K_ACTIVE` writer - 32K_ACTIVE"]
pub type _32K_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_RST_STS_SPEC, bool, O>;
#[doc = "Field `PCICLK_ACTIVE` reader - PCICLK_ACTIVE"]
pub type PCICLK_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `PCICLK_ACTIVE` writer - PCICLK_ACTIVE"]
pub type PCICLK_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_RST_STS_SPEC, bool, O>;
#[doc = "Field `ESPI_CLK_ACTIVE` reader - ESPI_CLK_ACTIVE"]
pub type ESPI_CLK_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `ESPI_CLK_ACTIVE` writer - ESPI_CLK_ACTIVE"]
pub type ESPI_CLK_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_RST_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Indicates the status of VCC_PWRGD. 0 = PWRGD not asserted. 1 = PWRGD asserte."]
    #[inline(always)]
    pub fn vcc_pwrgd_sts(&self) -> VCC_PWRGD_STS_R {
        VCC_PWRGD_STS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates the status of RESET_VCC. 0 = reset active. 1 = reset not active."]
    #[inline(always)]
    pub fn rst_h_sts(&self) -> RST_H_STS_R {
        RST_H_STS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - VBAT reset status 0 = No reset occurred while VTR was off or since the last time this bit was cleared. 1 = A reset occurred.(R/WC)"]
    #[inline(always)]
    pub fn vbat_rst_sts(&self) -> VBAT_RST_STS_R {
        VBAT_RST_STS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates the status of VTR_RESET.(R/WC)\n 0 = No reset occurred since the last time this bit was cleared.\n 1 = A reset occurred."]
    #[inline(always)]
    pub fn rst_vtr_sts(&self) -> RST_VTR_STS_R {
        RST_VTR_STS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates s RESET_SYS was triggered by a JTAG action.(R/WC)\n 0 = No JTAG reset occurred since the last time this bit was cleared.\n 1 = A reset occurred because of a JATAG command."]
    #[inline(always)]
    pub fn jtag_rst_sts(&self) -> JTAG_RST_STS_R {
        JTAG_RST_STS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - 32K_ACTIVE"]
    #[inline(always)]
    pub fn _32k_active(&self) -> _32K_ACTIVE_R {
        _32K_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCICLK_ACTIVE"]
    #[inline(always)]
    pub fn pciclk_active(&self) -> PCICLK_ACTIVE_R {
        PCICLK_ACTIVE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ESPI_CLK_ACTIVE"]
    #[inline(always)]
    pub fn espi_clk_active(&self) -> ESPI_CLK_ACTIVE_R {
        ESPI_CLK_ACTIVE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Indicates the status of VCC_PWRGD. 0 = PWRGD not asserted. 1 = PWRGD asserte."]
    #[inline(always)]
    pub fn vcc_pwrgd_sts(&mut self) -> VCC_PWRGD_STS_W<2> {
        VCC_PWRGD_STS_W::new(self)
    }
    #[doc = "Bit 3 - Indicates the status of RESET_VCC. 0 = reset active. 1 = reset not active."]
    #[inline(always)]
    pub fn rst_h_sts(&mut self) -> RST_H_STS_W<3> {
        RST_H_STS_W::new(self)
    }
    #[doc = "Bit 5 - VBAT reset status 0 = No reset occurred while VTR was off or since the last time this bit was cleared. 1 = A reset occurred.(R/WC)"]
    #[inline(always)]
    pub fn vbat_rst_sts(&mut self) -> VBAT_RST_STS_W<5> {
        VBAT_RST_STS_W::new(self)
    }
    #[doc = "Bit 6 - Indicates the status of VTR_RESET.(R/WC)\n 0 = No reset occurred since the last time this bit was cleared.\n 1 = A reset occurred."]
    #[inline(always)]
    pub fn rst_vtr_sts(&mut self) -> RST_VTR_STS_W<6> {
        RST_VTR_STS_W::new(self)
    }
    #[doc = "Bit 7 - Indicates s RESET_SYS was triggered by a JTAG action.(R/WC)\n 0 = No JTAG reset occurred since the last time this bit was cleared.\n 1 = A reset occurred because of a JATAG command."]
    #[inline(always)]
    pub fn jtag_rst_sts(&mut self) -> JTAG_RST_STS_W<7> {
        JTAG_RST_STS_W::new(self)
    }
    #[doc = "Bit 10 - 32K_ACTIVE"]
    #[inline(always)]
    pub fn _32k_active(&mut self) -> _32K_ACTIVE_W<10> {
        _32K_ACTIVE_W::new(self)
    }
    #[doc = "Bit 11 - PCICLK_ACTIVE"]
    #[inline(always)]
    pub fn pciclk_active(&mut self) -> PCICLK_ACTIVE_W<11> {
        PCICLK_ACTIVE_W::new(self)
    }
    #[doc = "Bit 12 - ESPI_CLK_ACTIVE"]
    #[inline(always)]
    pub fn espi_clk_active(&mut self) -> ESPI_CLK_ACTIVE_W<12> {
        ESPI_CLK_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCR Power Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_rst_sts](index.html) module"]
pub struct PWR_RST_STS_SPEC;
impl crate::RegisterSpec for PWR_RST_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_rst_sts::R](R) reader structure"]
impl crate::Readable for PWR_RST_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_rst_sts::W](W) writer structure"]
impl crate::Writable for PWR_RST_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_RST_STS to value 0"]
impl crate::Resettable for PWR_RST_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
