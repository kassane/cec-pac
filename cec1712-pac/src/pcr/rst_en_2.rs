#[doc = "Register `RST_EN_2` reader"]
pub struct R(crate::R<RST_EN_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_EN_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_EN_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_EN_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST_EN_2` writer"]
pub struct W(crate::W<RST_EN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_EN_2_SPEC>;
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
impl From<crate::W<RST_EN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_EN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMAP_RST_EN` reader - IMAP Reset Enable (IMAP_RST_EN)"]
pub type IMAP_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `IMAP_RST_EN` writer - IMAP Reset Enable (IMAP_RST_EN)"]
pub type IMAP_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `UART0_RST_EN` reader - UART 0 Reset Enable"]
pub type UART0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART0_RST_EN` writer - UART 0 Reset Enable"]
pub type UART0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `UART1_RST_EN` reader - UART 1 Reset Enable"]
pub type UART1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART1_RST_EN` writer - UART 1 Reset Enable"]
pub type UART1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `ACPI_EC0_RST_EN` reader - ACPI EC 0 Reset Enable (ACPI_EC_0_RST_EN)"]
pub type ACPI_EC0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `ACPI_EC0_RST_EN` writer - ACPI EC 0 Reset Enable (ACPI_EC_0_RST_EN)"]
pub type ACPI_EC0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `ACPI_EC1_RST_EN` reader - ACPI EC 1 Reset Enable (ACPI_EC_1_RST_EN)"]
pub type ACPI_EC1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `ACPI_EC1_RST_EN` writer - ACPI EC 1 Reset Enable (ACPI_EC_1_RST_EN)"]
pub type ACPI_EC1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `ACPI_PM1_RST_EN` reader - ACPI PM1 Reset Enable (ACPI_PM1_RST_EN)"]
pub type ACPI_PM1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `ACPI_PM1_RST_EN` writer - ACPI PM1 Reset Enable (ACPI_PM1_RST_EN)"]
pub type ACPI_PM1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `MBX_RST_EN` reader - Mailbox Reset Enable (MBX_RST_EN)"]
pub type MBX_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `MBX_RST_EN` writer - Mailbox Reset Enable (MBX_RST_EN)"]
pub type MBX_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `RTC_RST_EN` reader - RTC Reset Enable (RTC_RST_EN)"]
pub type RTC_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `RTC_RST_EN` writer - RTC Reset Enable (RTC_RST_EN)"]
pub type RTC_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `SCRATCH_32_RST_EN` reader - SCRATCH 32 Reset Enable"]
pub type SCRATCH_32_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCRATCH_32_RST_EN` writer - SCRATCH 32 Reset Enable"]
pub type SCRATCH_32_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `ACPI_EC_2_RST_EN` reader - ACPI EC 2 Reset Enable (ACPI_EC_2_RST_EN)"]
pub type ACPI_EC_2_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `ACPI_EC_2_RST_EN` writer - ACPI EC 2 Reset Enable (ACPI_EC_2_RST_EN)"]
pub type ACPI_EC_2_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `ACPI_EC_3_RST_EN` reader - ACPI EC 3 Reset Enable (ACPI_EC_3_RST_EN)"]
pub type ACPI_EC_3_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `ACPI_EC_3_RST_EN` writer - ACPI EC 3 Reset Enable (ACPI_EC_3_RST_EN)"]
pub type ACPI_EC_3_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `PORT_80_0_RST_EN` reader - Port 80 0 Reset Enable"]
pub type PORT_80_0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `PORT_80_0_RST_EN` writer - Port 80 0 Reset Enable"]
pub type PORT_80_0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
#[doc = "Field `PORT_80_1_RST_EN` reader - Port 80 1 Reset Enable"]
pub type PORT_80_1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `PORT_80_1_RST_EN` writer - Port 80 1 Reset Enable"]
pub type PORT_80_1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IMAP Reset Enable (IMAP_RST_EN)"]
    #[inline(always)]
    pub fn imap_rst_en(&self) -> IMAP_RST_EN_R {
        IMAP_RST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART 0 Reset Enable"]
    #[inline(always)]
    pub fn uart0_rst_en(&self) -> UART0_RST_EN_R {
        UART0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART 1 Reset Enable"]
    #[inline(always)]
    pub fn uart1_rst_en(&self) -> UART1_RST_EN_R {
        UART1_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 13 - ACPI EC 0 Reset Enable (ACPI_EC_0_RST_EN)"]
    #[inline(always)]
    pub fn acpi_ec0_rst_en(&self) -> ACPI_EC0_RST_EN_R {
        ACPI_EC0_RST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ACPI EC 1 Reset Enable (ACPI_EC_1_RST_EN)"]
    #[inline(always)]
    pub fn acpi_ec1_rst_en(&self) -> ACPI_EC1_RST_EN_R {
        ACPI_EC1_RST_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ACPI PM1 Reset Enable (ACPI_PM1_RST_EN)"]
    #[inline(always)]
    pub fn acpi_pm1_rst_en(&self) -> ACPI_PM1_RST_EN_R {
        ACPI_PM1_RST_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Mailbox Reset Enable (MBX_RST_EN)"]
    #[inline(always)]
    pub fn mbx_rst_en(&self) -> MBX_RST_EN_R {
        MBX_RST_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RTC Reset Enable (RTC_RST_EN)"]
    #[inline(always)]
    pub fn rtc_rst_en(&self) -> RTC_RST_EN_R {
        RTC_RST_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SCRATCH 32 Reset Enable"]
    #[inline(always)]
    pub fn scratch_32_rst_en(&self) -> SCRATCH_32_RST_EN_R {
        SCRATCH_32_RST_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ACPI EC 2 Reset Enable (ACPI_EC_2_RST_EN)"]
    #[inline(always)]
    pub fn acpi_ec_2_rst_en(&self) -> ACPI_EC_2_RST_EN_R {
        ACPI_EC_2_RST_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ACPI EC 3 Reset Enable (ACPI_EC_3_RST_EN)"]
    #[inline(always)]
    pub fn acpi_ec_3_rst_en(&self) -> ACPI_EC_3_RST_EN_R {
        ACPI_EC_3_RST_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - Port 80 0 Reset Enable"]
    #[inline(always)]
    pub fn port_80_0_rst_en(&self) -> PORT_80_0_RST_EN_R {
        PORT_80_0_RST_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port 80 1 Reset Enable"]
    #[inline(always)]
    pub fn port_80_1_rst_en(&self) -> PORT_80_1_RST_EN_R {
        PORT_80_1_RST_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IMAP Reset Enable (IMAP_RST_EN)"]
    #[inline(always)]
    pub fn imap_rst_en(&mut self) -> IMAP_RST_EN_W<0> {
        IMAP_RST_EN_W::new(self)
    }
    #[doc = "Bit 1 - UART 0 Reset Enable"]
    #[inline(always)]
    pub fn uart0_rst_en(&mut self) -> UART0_RST_EN_W<1> {
        UART0_RST_EN_W::new(self)
    }
    #[doc = "Bit 2 - UART 1 Reset Enable"]
    #[inline(always)]
    pub fn uart1_rst_en(&mut self) -> UART1_RST_EN_W<2> {
        UART1_RST_EN_W::new(self)
    }
    #[doc = "Bit 13 - ACPI EC 0 Reset Enable (ACPI_EC_0_RST_EN)"]
    #[inline(always)]
    pub fn acpi_ec0_rst_en(&mut self) -> ACPI_EC0_RST_EN_W<13> {
        ACPI_EC0_RST_EN_W::new(self)
    }
    #[doc = "Bit 14 - ACPI EC 1 Reset Enable (ACPI_EC_1_RST_EN)"]
    #[inline(always)]
    pub fn acpi_ec1_rst_en(&mut self) -> ACPI_EC1_RST_EN_W<14> {
        ACPI_EC1_RST_EN_W::new(self)
    }
    #[doc = "Bit 15 - ACPI PM1 Reset Enable (ACPI_PM1_RST_EN)"]
    #[inline(always)]
    pub fn acpi_pm1_rst_en(&mut self) -> ACPI_PM1_RST_EN_W<15> {
        ACPI_PM1_RST_EN_W::new(self)
    }
    #[doc = "Bit 17 - Mailbox Reset Enable (MBX_RST_EN)"]
    #[inline(always)]
    pub fn mbx_rst_en(&mut self) -> MBX_RST_EN_W<17> {
        MBX_RST_EN_W::new(self)
    }
    #[doc = "Bit 18 - RTC Reset Enable (RTC_RST_EN)"]
    #[inline(always)]
    pub fn rtc_rst_en(&mut self) -> RTC_RST_EN_W<18> {
        RTC_RST_EN_W::new(self)
    }
    #[doc = "Bit 20 - SCRATCH 32 Reset Enable"]
    #[inline(always)]
    pub fn scratch_32_rst_en(&mut self) -> SCRATCH_32_RST_EN_W<20> {
        SCRATCH_32_RST_EN_W::new(self)
    }
    #[doc = "Bit 21 - ACPI EC 2 Reset Enable (ACPI_EC_2_RST_EN)"]
    #[inline(always)]
    pub fn acpi_ec_2_rst_en(&mut self) -> ACPI_EC_2_RST_EN_W<21> {
        ACPI_EC_2_RST_EN_W::new(self)
    }
    #[doc = "Bit 22 - ACPI EC 3 Reset Enable (ACPI_EC_3_RST_EN)"]
    #[inline(always)]
    pub fn acpi_ec_3_rst_en(&mut self) -> ACPI_EC_3_RST_EN_W<22> {
        ACPI_EC_3_RST_EN_W::new(self)
    }
    #[doc = "Bit 25 - Port 80 0 Reset Enable"]
    #[inline(always)]
    pub fn port_80_0_rst_en(&mut self) -> PORT_80_0_RST_EN_W<25> {
        PORT_80_0_RST_EN_W::new(self)
    }
    #[doc = "Bit 26 - Port 80 1 Reset Enable"]
    #[inline(always)]
    pub fn port_80_1_rst_en(&mut self) -> PORT_80_1_RST_EN_W<26> {
        PORT_80_1_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Enable 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_en_2](index.html) module"]
pub struct RST_EN_2_SPEC;
impl crate::RegisterSpec for RST_EN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst_en_2::R](R) reader structure"]
impl crate::Readable for RST_EN_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_en_2::W](W) writer structure"]
impl crate::Writable for RST_EN_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RST_EN_2 to value 0"]
impl crate::Resettable for RST_EN_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
