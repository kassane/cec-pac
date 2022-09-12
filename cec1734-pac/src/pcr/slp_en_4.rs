#[doc = "Register `SLP_EN_4` reader"]
pub struct R(crate::R<SLP_EN_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_EN_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_EN_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_EN_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_EN_4` writer"]
pub struct W(crate::W<SLP_EN_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_EN_4_SPEC>;
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
impl From<crate::W<SLP_EN_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_EN_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECMON0_SLP_EN` reader - SPI Monitor 0 Sleep Enable (SECMON0_SLP_EN)"]
pub type SECMON0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SECMON0_SLP_EN` writer - SPI Monitor 0 Sleep Enable (SECMON0_SLP_EN)"]
pub type SECMON0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `SECMON1_SLP_EN` reader - SPI Monitor 1 Sleep Enable (SECMON1_SLP_EN)"]
pub type SECMON1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SECMON1_SLP_EN` writer - SPI Monitor 1 Sleep Enable (SECMON1_SLP_EN)"]
pub type SECMON1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `RTOS_SLP_EN` reader - RTOS Sleep Enable (RTOS_SLP_EN)"]
pub type RTOS_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `RTOS_SLP_EN` writer - RTOS Sleep Enable (RTOS_SLP_EN)"]
pub type RTOS_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `QMSPI0_SLP_EN` reader - Quad Master SPI 0 Sleep Enable (QMSPI_1_SLP_EN)"]
pub type QMSPI0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `QMSPI0_SLP_EN` writer - Quad Master SPI 0 Sleep Enable (QMSPI_1_SLP_EN)"]
pub type QMSPI0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `UART0_SLP_EN` reader - UART0 Sleep Enable (UART_1_SLP_EN)"]
pub type UART0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART0_SLP_EN` writer - UART0 Sleep Enable (UART_1_SLP_EN)"]
pub type UART0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `SPIPER0_SLP_EN` reader - SPI Peropheral 0 Sleep Enable (SPIPER0_SLP_EN)"]
pub type SPIPER0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPIPER0_SLP_EN` writer - SPI Peropheral 0 Sleep Enable (SPIPER0_SLP_EN)"]
pub type SPIPER0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `SPIPER1_SLP_EN` reader - SPI Peropheral 1 Sleep Enable (SPIPER1_SLP_EN)"]
pub type SPIPER1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPIPER1_SLP_EN` writer - SPI Peropheral 1 Sleep Enable (SPIPER1_SLP_EN)"]
pub type SPIPER1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `QMSPI_1_SLP_EN` reader - QMSPI 1 Sleep Enable (QMSPI_1_SLP_EN)"]
pub type QMSPI_1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `QMSPI_1_SLP_EN` writer - QMSPI 1 Sleep Enable (QMSPI_1_SLP_EN)"]
pub type QMSPI_1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `VBAT_REG_SLP_EN` reader - VBAT REG Sleep Enable (VBAT_REG_SLP_EN)"]
pub type VBAT_REG_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `VBAT_REG_SLP_EN` writer - VBAT REG Sleep Enable (VBAT_REG_SLP_EN)"]
pub type VBAT_REG_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SPI Monitor 0 Sleep Enable (SECMON0_SLP_EN)"]
    #[inline(always)]
    pub fn secmon0_slp_en(&self) -> SECMON0_SLP_EN_R {
        SECMON0_SLP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Monitor 1 Sleep Enable (SECMON1_SLP_EN)"]
    #[inline(always)]
    pub fn secmon1_slp_en(&self) -> SECMON1_SLP_EN_R {
        SECMON1_SLP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - RTOS Sleep Enable (RTOS_SLP_EN)"]
    #[inline(always)]
    pub fn rtos_slp_en(&self) -> RTOS_SLP_EN_R {
        RTOS_SLP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Quad Master SPI 0 Sleep Enable (QMSPI_1_SLP_EN)"]
    #[inline(always)]
    pub fn qmspi0_slp_en(&self) -> QMSPI0_SLP_EN_R {
        QMSPI0_SLP_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART0 Sleep Enable (UART_1_SLP_EN)"]
    #[inline(always)]
    pub fn uart0_slp_en(&self) -> UART0_SLP_EN_R {
        UART0_SLP_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI Peropheral 0 Sleep Enable (SPIPER0_SLP_EN)"]
    #[inline(always)]
    pub fn spiper0_slp_en(&self) -> SPIPER0_SLP_EN_R {
        SPIPER0_SLP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - SPI Peropheral 1 Sleep Enable (SPIPER1_SLP_EN)"]
    #[inline(always)]
    pub fn spiper1_slp_en(&self) -> SPIPER1_SLP_EN_R {
        SPIPER1_SLP_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - QMSPI 1 Sleep Enable (QMSPI_1_SLP_EN)"]
    #[inline(always)]
    pub fn qmspi_1_slp_en(&self) -> QMSPI_1_SLP_EN_R {
        QMSPI_1_SLP_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - VBAT REG Sleep Enable (VBAT_REG_SLP_EN)"]
    #[inline(always)]
    pub fn vbat_reg_slp_en(&self) -> VBAT_REG_SLP_EN_R {
        VBAT_REG_SLP_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Monitor 0 Sleep Enable (SECMON0_SLP_EN)"]
    #[inline(always)]
    pub fn secmon0_slp_en(&mut self) -> SECMON0_SLP_EN_W<0> {
        SECMON0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 1 - SPI Monitor 1 Sleep Enable (SECMON1_SLP_EN)"]
    #[inline(always)]
    pub fn secmon1_slp_en(&mut self) -> SECMON1_SLP_EN_W<1> {
        SECMON1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 6 - RTOS Sleep Enable (RTOS_SLP_EN)"]
    #[inline(always)]
    pub fn rtos_slp_en(&mut self) -> RTOS_SLP_EN_W<6> {
        RTOS_SLP_EN_W::new(self)
    }
    #[doc = "Bit 8 - Quad Master SPI 0 Sleep Enable (QMSPI_1_SLP_EN)"]
    #[inline(always)]
    pub fn qmspi0_slp_en(&mut self) -> QMSPI0_SLP_EN_W<8> {
        QMSPI0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 9 - UART0 Sleep Enable (UART_1_SLP_EN)"]
    #[inline(always)]
    pub fn uart0_slp_en(&mut self) -> UART0_SLP_EN_W<9> {
        UART0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 16 - SPI Peropheral 0 Sleep Enable (SPIPER0_SLP_EN)"]
    #[inline(always)]
    pub fn spiper0_slp_en(&mut self) -> SPIPER0_SLP_EN_W<16> {
        SPIPER0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 21 - SPI Peropheral 1 Sleep Enable (SPIPER1_SLP_EN)"]
    #[inline(always)]
    pub fn spiper1_slp_en(&mut self) -> SPIPER1_SLP_EN_W<21> {
        SPIPER1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 22 - QMSPI 1 Sleep Enable (QMSPI_1_SLP_EN)"]
    #[inline(always)]
    pub fn qmspi_1_slp_en(&mut self) -> QMSPI_1_SLP_EN_W<22> {
        QMSPI_1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 23 - VBAT REG Sleep Enable (VBAT_REG_SLP_EN)"]
    #[inline(always)]
    pub fn vbat_reg_slp_en(&mut self) -> VBAT_REG_SLP_EN_W<23> {
        VBAT_REG_SLP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Enable 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_en_4](index.html) module"]
pub struct SLP_EN_4_SPEC;
impl crate::RegisterSpec for SLP_EN_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_en_4::R](R) reader structure"]
impl crate::Readable for SLP_EN_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_en_4::W](W) writer structure"]
impl crate::Writable for SLP_EN_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_EN_4 to value 0"]
impl crate::Resettable for SLP_EN_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
