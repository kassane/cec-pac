#[doc = "Register `PFRS` reader"]
pub struct R(crate::R<PFRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFRS` writer"]
pub struct W(crate::W<PFRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFRS_SPEC>;
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
impl From<crate::W<PFRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DET_32KHZ` reader - Detect 32KHz Clock Input (R/WC)"]
pub type DET_32KHZ_R = crate::BitReader<bool>;
#[doc = "Field `DET_32KHZ` writer - Detect 32KHz Clock Input (R/WC)"]
pub type DET_32KHZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFRS_SPEC, bool, O>;
#[doc = "Field `SFT_RST` reader - This bit is set to '1b' if a was triggered by an assertion of the SOFT_SYS_RESET bit in the System Reset Register. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
pub type SFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `SFT_RST` writer - This bit is set to '1b' if a was triggered by an assertion of the SOFT_SYS_RESET bit in the System Reset Register. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
pub type SFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFRS_SPEC, bool, O>;
#[doc = "Field `RSTI` reader - This bit is set to '1b' if a RESET_SYS was triggered by a low signal on the RESETI# input pin. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
pub type RSTI_R = crate::BitReader<bool>;
#[doc = "Field `RSTI` writer - This bit is set to '1b' if a RESET_SYS was triggered by a low signal on the RESETI# input pin. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
pub type RSTI_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFRS_SPEC, bool, O>;
#[doc = "Field `WDT_EVT` reader - This bit is set to '1b' if a RESET_SYS was triggered by a Watchdog Timer event. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
pub type WDT_EVT_R = crate::BitReader<bool>;
#[doc = "Field `WDT_EVT` writer - This bit is set to '1b' if a RESET_SYS was triggered by a Watchdog Timer event. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
pub type WDT_EVT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFRS_SPEC, bool, O>;
#[doc = "Field `SYS_RSTREQ` reader - This bit is set to '1b' if a RESET_SYS was triggered by an ARM SYSRESETREQ event. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
pub type SYS_RSTREQ_R = crate::BitReader<bool>;
#[doc = "Field `SYS_RSTREQ` writer - This bit is set to '1b' if a RESET_SYS was triggered by an ARM SYSRESETREQ event. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
pub type SYS_RSTREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFRS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Detect 32KHz Clock Input (R/WC)"]
    #[inline(always)]
    pub fn det_32khz(&self) -> DET_32KHZ_R {
        DET_32KHZ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set to '1b' if a was triggered by an assertion of the SOFT_SYS_RESET bit in the System Reset Register. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
    #[inline(always)]
    pub fn sft_rst(&self) -> SFT_RST_R {
        SFT_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is set to '1b' if a RESET_SYS was triggered by a low signal on the RESETI# input pin. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
    #[inline(always)]
    pub fn rsti(&self) -> RSTI_R {
        RSTI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set to '1b' if a RESET_SYS was triggered by a Watchdog Timer event. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
    #[inline(always)]
    pub fn wdt_evt(&self) -> WDT_EVT_R {
        WDT_EVT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is set to '1b' if a RESET_SYS was triggered by an ARM SYSRESETREQ event. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
    #[inline(always)]
    pub fn sys_rstreq(&self) -> SYS_RSTREQ_R {
        SYS_RSTREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Detect 32KHz Clock Input (R/WC)"]
    #[inline(always)]
    pub fn det_32khz(&mut self) -> DET_32KHZ_W<0> {
        DET_32KHZ_W::new(self)
    }
    #[doc = "Bit 2 - This bit is set to '1b' if a was triggered by an assertion of the SOFT_SYS_RESET bit in the System Reset Register. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
    #[inline(always)]
    pub fn sft_rst(&mut self) -> SFT_RST_W<2> {
        SFT_RST_W::new(self)
    }
    #[doc = "Bit 4 - This bit is set to '1b' if a RESET_SYS was triggered by a low signal on the RESETI# input pin. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
    #[inline(always)]
    pub fn rsti(&mut self) -> RSTI_W<4> {
        RSTI_W::new(self)
    }
    #[doc = "Bit 5 - This bit is set to '1b' if a RESET_SYS was triggered by a Watchdog Timer event. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
    #[inline(always)]
    pub fn wdt_evt(&mut self) -> WDT_EVT_W<5> {
        WDT_EVT_W::new(self)
    }
    #[doc = "Bit 6 - This bit is set to '1b' if a RESET_SYS was triggered by an ARM SYSRESETREQ event. This bit is cleared to '0b' when written with a '1b'; writes of a '0b' have no effect. (R/WC)"]
    #[inline(always)]
    pub fn sys_rstreq(&mut self) -> SYS_RSTREQ_W<6> {
        SYS_RSTREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Power-Fail and Reset Status Register collects and retains the VBAT RST and WDT event status when VCC1 is unpowered.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfrs](index.html) module"]
pub struct PFRS_SPEC;
impl crate::RegisterSpec for PFRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfrs::R](R) reader structure"]
impl crate::Readable for PFRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfrs::W](W) writer structure"]
impl crate::Writable for PFRS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PFRS to value 0x80"]
impl crate::Resettable for PFRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
