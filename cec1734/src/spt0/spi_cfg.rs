#[doc = "Register `SPI_CFG` reader"]
pub struct R(crate::R<SPI_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CFG` writer"]
pub struct W(crate::W<SPI_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CFG_SPEC>;
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
impl From<crate::W<SPI_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SNG_QUD_SEL` reader - This field defines the Single / Quad Wire mode of operation for SPI Peripheral Target block. 0 = Single Wire Slave SPI block operation. 1 = Quad Wire Slave SPI block operation."]
pub type SNG_QUD_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SNG_QUD_SEL` writer - This field defines the Single / Quad Wire mode of operation for SPI Peripheral Target block. 0 = Single Wire Slave SPI block operation. 1 = Quad Wire Slave SPI block operation."]
pub type SNG_QUD_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CFG_SPEC, bool, O>;
#[doc = "Field `TAR_TIM_SEL` reader - Turn Around Time select for Quad wire mode. 0h = 1 cycle. 1h = 2 cycles. 2h = 4 cycles. 3h = 8 cycles. Other values are reserved."]
pub type TAR_TIM_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAR_TIM_SEL` writer - Turn Around Time select for Quad wire mode. 0h = 1 cycle. 1h = 2 cycles. 2h = 4 cycles. 3h = 8 cycles. Other values are reserved."]
pub type TAR_TIM_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `WAIT_TIME` reader - These bits set the amount of wait time in cycles before transmitting data back to master. During this wait time status bits will be transmitted"]
pub type WAIT_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAIT_TIME` writer - These bits set the amount of wait time in cycles before transmitting data back to master. During this wait time status bits will be transmitted"]
pub type WAIT_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - This field defines the Single / Quad Wire mode of operation for SPI Peripheral Target block. 0 = Single Wire Slave SPI block operation. 1 = Quad Wire Slave SPI block operation."]
    #[inline(always)]
    pub fn sng_qud_sel(&self) -> SNG_QUD_SEL_R {
        SNG_QUD_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Turn Around Time select for Quad wire mode. 0h = 1 cycle. 1h = 2 cycles. 2h = 4 cycles. 3h = 8 cycles. Other values are reserved."]
    #[inline(always)]
    pub fn tar_tim_sel(&self) -> TAR_TIM_SEL_R {
        TAR_TIM_SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:23 - These bits set the amount of wait time in cycles before transmitting data back to master. During this wait time status bits will be transmitted"]
    #[inline(always)]
    pub fn wait_time(&self) -> WAIT_TIME_R {
        WAIT_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This field defines the Single / Quad Wire mode of operation for SPI Peripheral Target block. 0 = Single Wire Slave SPI block operation. 1 = Quad Wire Slave SPI block operation."]
    #[inline(always)]
    pub fn sng_qud_sel(&mut self) -> SNG_QUD_SEL_W<0> {
        SNG_QUD_SEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Turn Around Time select for Quad wire mode. 0h = 1 cycle. 1h = 2 cycles. 2h = 4 cycles. 3h = 8 cycles. Other values are reserved."]
    #[inline(always)]
    pub fn tar_tim_sel(&mut self) -> TAR_TIM_SEL_W<8> {
        TAR_TIM_SEL_W::new(self)
    }
    #[doc = "Bits 16:23 - These bits set the amount of wait time in cycles before transmitting data back to master. During this wait time status bits will be transmitted"]
    #[inline(always)]
    pub fn wait_time(&mut self) -> WAIT_TIME_W<16> {
        WAIT_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Peripheral Target Communication Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cfg](index.html) module"]
pub struct SPI_CFG_SPEC;
impl crate::RegisterSpec for SPI_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_cfg::R](R) reader structure"]
impl crate::Readable for SPI_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_cfg::W](W) writer structure"]
impl crate::Writable for SPI_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CFG to value 0x0004_0000"]
impl crate::Resettable for SPI_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0000
    }
}
