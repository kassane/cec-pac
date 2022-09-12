#[doc = "Register `GPIO_BANK_PWR` reader"]
pub struct R(crate::R<GPIO_BANK_PWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_BANK_PWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_BANK_PWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_BANK_PWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_BANK_PWR` writer"]
pub struct W(crate::W<GPIO_BANK_PWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_BANK_PWR_SPEC>;
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
impl From<crate::W<GPIO_BANK_PWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_BANK_PWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTR_LVL1` reader - Voltage value on VTR1. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software.\n It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7.\n 1=VTR1 is powered by 3.3V\n 0=VTR1 is powered by 1.8V."]
pub type VTR_LVL1_R = crate::BitReader<bool>;
#[doc = "Field `VTR_LVL1` writer - Voltage value on VTR1. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software.\n It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7.\n 1=VTR1 is powered by 3.3V\n 0=VTR1 is powered by 1.8V."]
pub type VTR_LVL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_BANK_PWR_SPEC, bool, O>;
#[doc = "Field `VTR_LVL2` reader - Voltage value on VTR2. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software.\n It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7.\n 1=VTR2 is powered by 3.3V\n 0=VTR2 is powered by 1.8V."]
pub type VTR_LVL2_R = crate::BitReader<bool>;
#[doc = "Field `VTR_LVL2` writer - Voltage value on VTR2. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software.\n It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7.\n 1=VTR2 is powered by 3.3V\n 0=VTR2 is powered by 1.8V."]
pub type VTR_LVL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_BANK_PWR_SPEC, bool, O>;
#[doc = "Field `VTR_LVL3` reader - Voltage value on VTR3. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software.\n It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7.\n 1=VTR3 is powered by 3.3V\n 0=VTR3 is powered by 1.8V."]
pub type VTR_LVL3_R = crate::BitReader<bool>;
#[doc = "Field `VTR_LVL3` writer - Voltage value on VTR3. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software.\n It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7.\n 1=VTR3 is powered by 3.3V\n 0=VTR3 is powered by 1.8V."]
pub type VTR_LVL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_BANK_PWR_SPEC, bool, O>;
#[doc = "Field `GPIO_BANK_PWR_LOCK` reader - GPIO Bank Power Lock. 0: VTR_LEVEL bits\\[2:0\\]
and GPIO Bank Power Lock bit are R/W\n 1 = VTR_LEVEL bits\\[2:0\\]
and GPIO Bank Power Lock bit are Read Only."]
pub type GPIO_BANK_PWR_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_BANK_PWR_LOCK` writer - GPIO Bank Power Lock. 0: VTR_LEVEL bits\\[2:0\\]
and GPIO Bank Power Lock bit are R/W\n 1 = VTR_LEVEL bits\\[2:0\\]
and GPIO Bank Power Lock bit are Read Only."]
pub type GPIO_BANK_PWR_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_BANK_PWR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Voltage value on VTR1. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software.\n It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7.\n 1=VTR1 is powered by 3.3V\n 0=VTR1 is powered by 1.8V."]
    #[inline(always)]
    pub fn vtr_lvl1(&self) -> VTR_LVL1_R {
        VTR_LVL1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage value on VTR2. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software.\n It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7.\n 1=VTR2 is powered by 3.3V\n 0=VTR2 is powered by 1.8V."]
    #[inline(always)]
    pub fn vtr_lvl2(&self) -> VTR_LVL2_R {
        VTR_LVL2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage value on VTR3. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software.\n It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7.\n 1=VTR3 is powered by 3.3V\n 0=VTR3 is powered by 1.8V."]
    #[inline(always)]
    pub fn vtr_lvl3(&self) -> VTR_LVL3_R {
        VTR_LVL3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO Bank Power Lock. 0: VTR_LEVEL bits\\[2:0\\]
and GPIO Bank Power Lock bit are R/W\n 1 = VTR_LEVEL bits\\[2:0\\]
and GPIO Bank Power Lock bit are Read Only."]
    #[inline(always)]
    pub fn gpio_bank_pwr_lock(&self) -> GPIO_BANK_PWR_LOCK_R {
        GPIO_BANK_PWR_LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage value on VTR1. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software.\n It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7.\n 1=VTR1 is powered by 3.3V\n 0=VTR1 is powered by 1.8V."]
    #[inline(always)]
    pub fn vtr_lvl1(&mut self) -> VTR_LVL1_W<0> {
        VTR_LVL1_W::new(self)
    }
    #[doc = "Bit 1 - Voltage value on VTR2. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software.\n It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7.\n 1=VTR2 is powered by 3.3V\n 0=VTR2 is powered by 1.8V."]
    #[inline(always)]
    pub fn vtr_lvl2(&mut self) -> VTR_LVL2_W<1> {
        VTR_LVL2_W::new(self)
    }
    #[doc = "Bit 2 - Voltage value on VTR3. This bit is set by hardware after a VTR Power On Reset, but may be overridden by software.\n It must be set by software if the VTR power rail is not active when RESET_SYS is de-asserted. Write access is determined by bit 7.\n 1=VTR3 is powered by 3.3V\n 0=VTR3 is powered by 1.8V."]
    #[inline(always)]
    pub fn vtr_lvl3(&mut self) -> VTR_LVL3_W<2> {
        VTR_LVL3_W::new(self)
    }
    #[doc = "Bit 7 - GPIO Bank Power Lock. 0: VTR_LEVEL bits\\[2:0\\]
and GPIO Bank Power Lock bit are R/W\n 1 = VTR_LEVEL bits\\[2:0\\]
and GPIO Bank Power Lock bit are Read Only."]
    #[inline(always)]
    pub fn gpio_bank_pwr_lock(&mut self) -> GPIO_BANK_PWR_LOCK_W<7> {
        GPIO_BANK_PWR_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Bank Power Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_bank_pwr](index.html) module"]
pub struct GPIO_BANK_PWR_SPEC;
impl crate::RegisterSpec for GPIO_BANK_PWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_bank_pwr::R](R) reader structure"]
impl crate::Readable for GPIO_BANK_PWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_bank_pwr::W](W) writer structure"]
impl crate::Writable for GPIO_BANK_PWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_BANK_PWR to value 0"]
impl crate::Resettable for GPIO_BANK_PWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
