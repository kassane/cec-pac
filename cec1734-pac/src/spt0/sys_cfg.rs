#[doc = "Register `SYS_CFG` reader"]
pub struct R(crate::R<SYS_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_CFG` writer"]
pub struct W(crate::W<SYS_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CFG_SPEC>;
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
impl From<crate::W<SYS_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFT_RST` reader - Soft reset for entire SPI Peripheral Target Block. This bit is self clearing."]
pub type SOFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `SOFT_RST` writer - Soft reset for entire SPI Peripheral Target Block. This bit is self clearing."]
pub type SOFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG_SPEC, bool, O>;
#[doc = "Field `LOCK_QUAD_SNGL_WRMOD` reader - Lock Quad / Single Write Mode bit, write access from SPI Master."]
pub type LOCK_QUAD_SNGL_WRMOD_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_QUAD_SNGL_WRMOD` writer - Lock Quad / Single Write Mode bit, write access from SPI Master."]
pub type LOCK_QUAD_SNGL_WRMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG_SPEC, bool, O>;
#[doc = "Field `LOCK_TAR_TIME` reader - Lock Tar Time bit, write access from SPI Master."]
pub type LOCK_TAR_TIME_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_TAR_TIME` writer - Lock Tar Time bit, write access from SPI Master."]
pub type LOCK_TAR_TIME_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG_SPEC, bool, O>;
#[doc = "Field `LOCK_WAIT_CYCL` reader - Lock Wait Cycle bits, write access from SPI Master."]
pub type LOCK_WAIT_CYCL_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_WAIT_CYCL` writer - Lock Wait Cycle bits, write access from SPI Master."]
pub type LOCK_WAIT_CYCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG_SPEC, bool, O>;
#[doc = "Field `LOCK_MEM_CFG` reader - Lock Memory Configuration register, write access from SPI Master."]
pub type LOCK_MEM_CFG_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_MEM_CFG` writer - Lock Memory Configuration register, write access from SPI Master."]
pub type LOCK_MEM_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG_SPEC, bool, O>;
#[doc = "Field `LOCK_SPIINT_EN` reader - Lock SPI Interrupt Enable register, write access from SPI Master."]
pub type LOCK_SPIINT_EN_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_SPIINT_EN` writer - Lock SPI Interrupt Enable register, write access from SPI Master."]
pub type LOCK_SPIINT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG_SPEC, bool, O>;
#[doc = "Field `LOCK_MEM_BAR0` reader - Lock Memory Bar 0 register, write access from SPI Master."]
pub type LOCK_MEM_BAR0_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_MEM_BAR0` writer - Lock Memory Bar 0 register, write access from SPI Master."]
pub type LOCK_MEM_BAR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG_SPEC, bool, O>;
#[doc = "Field `LOCK_MEM_BAR1` reader - Lock Memory Bar 1 register, write access from SPI Master."]
pub type LOCK_MEM_BAR1_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_MEM_BAR1` writer - Lock Memory Bar 1 register, write access from SPI Master."]
pub type LOCK_MEM_BAR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG_SPEC, bool, O>;
#[doc = "Field `LOCK_TEST_MODE` reader - Lock TEST Mode register, write access from SPI Master."]
pub type LOCK_TEST_MODE_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_TEST_MODE` writer - Lock TEST Mode register, write access from SPI Master."]
pub type LOCK_TEST_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG_SPEC, bool, O>;
#[doc = "Field `SPI_SLV_EN` reader - Enable / Disable SPI Peripheral Target Block. 0 = Disable SPI Peripheral Target module. 1 = Enable SPI Peripheral Target module."]
pub type SPI_SLV_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_EN` writer - Enable / Disable SPI Peripheral Target Block. 0 = Disable SPI Peripheral Target module. 1 = Enable SPI Peripheral Target module."]
pub type SPI_SLV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG_SPEC, bool, O>;
#[doc = "Field `MAS_ECREG` reader - Fixed in hardware to 1"]
pub type MAS_ECREG_R = crate::BitReader<bool>;
#[doc = "Field `MAS_ECREG` writer - Fixed in hardware to 1"]
pub type MAS_ECREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG_SPEC, bool, O>;
#[doc = "Field `SIM_EN` reader - Enable SPI Peripheral Target Simple Mode operation."]
pub type SIM_EN_R = crate::BitReader<bool>;
#[doc = "Field `SIM_EN` writer - Enable SPI Peripheral Target Simple Mode operation."]
pub type SIM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG_SPEC, bool, O>;
#[doc = "Field `ECDATL` reader - Notification to TX FIFO Engine that data is available for AHB Transfer. This register but is cleared by Hardware at the end of the transaction, with SPI_CS_N de-assertion. (R/WC)."]
pub type ECDATL_R = crate::BitReader<bool>;
#[doc = "Field `ECDATL` writer - Notification to TX FIFO Engine that data is available for AHB Transfer. This register but is cleared by Hardware at the end of the transaction, with SPI_CS_N de-assertion. (R/WC)."]
pub type ECDATL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Soft reset for entire SPI Peripheral Target Block. This bit is self clearing."]
    #[inline(always)]
    pub fn soft_rst(&self) -> SOFT_RST_R {
        SOFT_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Quad / Single Write Mode bit, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_quad_sngl_wrmod(&self) -> LOCK_QUAD_SNGL_WRMOD_R {
        LOCK_QUAD_SNGL_WRMOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock Tar Time bit, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_tar_time(&self) -> LOCK_TAR_TIME_R {
        LOCK_TAR_TIME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock Wait Cycle bits, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_wait_cycl(&self) -> LOCK_WAIT_CYCL_R {
        LOCK_WAIT_CYCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock Memory Configuration register, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_mem_cfg(&self) -> LOCK_MEM_CFG_R {
        LOCK_MEM_CFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lock SPI Interrupt Enable register, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_spiint_en(&self) -> LOCK_SPIINT_EN_R {
        LOCK_SPIINT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock Memory Bar 0 register, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_mem_bar0(&self) -> LOCK_MEM_BAR0_R {
        LOCK_MEM_BAR0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock Memory Bar 1 register, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_mem_bar1(&self) -> LOCK_MEM_BAR1_R {
        LOCK_MEM_BAR1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Lock TEST Mode register, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_test_mode(&self) -> LOCK_TEST_MODE_R {
        LOCK_TEST_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable / Disable SPI Peripheral Target Block. 0 = Disable SPI Peripheral Target module. 1 = Enable SPI Peripheral Target module."]
    #[inline(always)]
    pub fn spi_slv_en(&self) -> SPI_SLV_EN_R {
        SPI_SLV_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fixed in hardware to 1"]
    #[inline(always)]
    pub fn mas_ecreg(&self) -> MAS_ECREG_R {
        MAS_ECREG_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable SPI Peripheral Target Simple Mode operation."]
    #[inline(always)]
    pub fn sim_en(&self) -> SIM_EN_R {
        SIM_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Notification to TX FIFO Engine that data is available for AHB Transfer. This register but is cleared by Hardware at the end of the transaction, with SPI_CS_N de-assertion. (R/WC)."]
    #[inline(always)]
    pub fn ecdatl(&self) -> ECDATL_R {
        ECDATL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Soft reset for entire SPI Peripheral Target Block. This bit is self clearing."]
    #[inline(always)]
    pub fn soft_rst(&mut self) -> SOFT_RST_W<0> {
        SOFT_RST_W::new(self)
    }
    #[doc = "Bit 1 - Lock Quad / Single Write Mode bit, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_quad_sngl_wrmod(&mut self) -> LOCK_QUAD_SNGL_WRMOD_W<1> {
        LOCK_QUAD_SNGL_WRMOD_W::new(self)
    }
    #[doc = "Bit 2 - Lock Tar Time bit, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_tar_time(&mut self) -> LOCK_TAR_TIME_W<2> {
        LOCK_TAR_TIME_W::new(self)
    }
    #[doc = "Bit 3 - Lock Wait Cycle bits, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_wait_cycl(&mut self) -> LOCK_WAIT_CYCL_W<3> {
        LOCK_WAIT_CYCL_W::new(self)
    }
    #[doc = "Bit 4 - Lock Memory Configuration register, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_mem_cfg(&mut self) -> LOCK_MEM_CFG_W<4> {
        LOCK_MEM_CFG_W::new(self)
    }
    #[doc = "Bit 5 - Lock SPI Interrupt Enable register, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_spiint_en(&mut self) -> LOCK_SPIINT_EN_W<5> {
        LOCK_SPIINT_EN_W::new(self)
    }
    #[doc = "Bit 6 - Lock Memory Bar 0 register, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_mem_bar0(&mut self) -> LOCK_MEM_BAR0_W<6> {
        LOCK_MEM_BAR0_W::new(self)
    }
    #[doc = "Bit 7 - Lock Memory Bar 1 register, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_mem_bar1(&mut self) -> LOCK_MEM_BAR1_W<7> {
        LOCK_MEM_BAR1_W::new(self)
    }
    #[doc = "Bit 10 - Lock TEST Mode register, write access from SPI Master."]
    #[inline(always)]
    pub fn lock_test_mode(&mut self) -> LOCK_TEST_MODE_W<10> {
        LOCK_TEST_MODE_W::new(self)
    }
    #[doc = "Bit 16 - Enable / Disable SPI Peripheral Target Block. 0 = Disable SPI Peripheral Target module. 1 = Enable SPI Peripheral Target module."]
    #[inline(always)]
    pub fn spi_slv_en(&mut self) -> SPI_SLV_EN_W<16> {
        SPI_SLV_EN_W::new(self)
    }
    #[doc = "Bit 17 - Fixed in hardware to 1"]
    #[inline(always)]
    pub fn mas_ecreg(&mut self) -> MAS_ECREG_W<17> {
        MAS_ECREG_W::new(self)
    }
    #[doc = "Bit 18 - Enable SPI Peripheral Target Simple Mode operation."]
    #[inline(always)]
    pub fn sim_en(&mut self) -> SIM_EN_W<18> {
        SIM_EN_W::new(self)
    }
    #[doc = "Bit 19 - Notification to TX FIFO Engine that data is available for AHB Transfer. This register but is cleared by Hardware at the end of the transaction, with SPI_CS_N de-assertion. (R/WC)."]
    #[inline(always)]
    pub fn ecdatl(&mut self) -> ECDATL_W<19> {
        ECDATL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Peripheral Target System Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_cfg](index.html) module"]
pub struct SYS_CFG_SPEC;
impl crate::RegisterSpec for SYS_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_cfg::R](R) reader structure"]
impl crate::Readable for SYS_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_cfg::W](W) writer structure"]
impl crate::Writable for SYS_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_CFG to value 0x04c0"]
impl crate::Resettable for SYS_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04c0
    }
}
