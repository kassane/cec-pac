#[doc = "Register `CLK_CTRL` reader"]
pub struct R(crate::R<CLK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CTRL` writer"]
pub struct W(crate::W<CLK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CTRL_SPEC>;
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
impl From<crate::W<CLK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCLKPH` reader - 1=Valid data is clocked out on the first SPI_CLK edge on SPDOUT signal. The slave device should sample this data on the second and \n following even SPI_CLK edges (i.e., sample data on falling edge) 0=Valid data is clocked out on the SPDOUT signal prior to the first SPI_CLK edge. \n The slave device should sample this data on the first and following odd SPI_CLK edges (i.e., sample data on rising edge)"]
pub type TCLKPH_R = crate::BitReader<bool>;
#[doc = "Field `TCLKPH` writer - 1=Valid data is clocked out on the first SPI_CLK edge on SPDOUT signal. The slave device should sample this data on the second and \n following even SPI_CLK edges (i.e., sample data on falling edge) 0=Valid data is clocked out on the SPDOUT signal prior to the first SPI_CLK edge. \n The slave device should sample this data on the first and following odd SPI_CLK edges (i.e., sample data on rising edge)"]
pub type TCLKPH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL_SPEC, bool, O>;
#[doc = "Field `RCLKPH` reader - 1=Valid data on SPDIN signal is expected after the first SPI_CLK edge. This data is sampled on the second and \n following even SPI_CLK edges (i.e., sample data on falling edge) 0=Valid data is expected on the SPDIN signal on the first SPI_CLK edge. \n This data is sampled on the first and following odd SPI_-CLK edges (i.e., sample data on rising edge)"]
pub type RCLKPH_R = crate::BitReader<bool>;
#[doc = "Field `RCLKPH` writer - 1=Valid data on SPDIN signal is expected after the first SPI_CLK edge. This data is sampled on the second and \n following even SPI_CLK edges (i.e., sample data on falling edge) 0=Valid data is expected on the SPDIN signal on the first SPI_CLK edge. \n This data is sampled on the first and following odd SPI_-CLK edges (i.e., sample data on rising edge)"]
pub type RCLKPH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL_SPEC, bool, O>;
#[doc = "Field `CLKPOL` reader - 1=The SPI_CLK signal is high when the interface is idle and the first clock edge is a falling edge\n 0=The SPI_CLK is low when the interface is idle and the first clock edge is a rising edge"]
pub type CLKPOL_R = crate::BitReader<bool>;
#[doc = "Field `CLKPOL` writer - 1=The SPI_CLK signal is high when the interface is idle and the first clock edge is a falling edge\n 0=The SPI_CLK is low when the interface is idle and the first clock edge is a rising edge"]
pub type CLKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL_SPEC, bool, O>;
#[doc = "Field `CLKSRC` reader - 1=2MHz, 0=48 MHz Ring Oscillator"]
pub type CLKSRC_R = crate::BitReader<bool>;
#[doc = "Field `CLKSRC` writer - 1=2MHz, 0=48 MHz Ring Oscillator"]
pub type CLKSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 1=Valid data is clocked out on the first SPI_CLK edge on SPDOUT signal. The slave device should sample this data on the second and \n following even SPI_CLK edges (i.e., sample data on falling edge) 0=Valid data is clocked out on the SPDOUT signal prior to the first SPI_CLK edge. \n The slave device should sample this data on the first and following odd SPI_CLK edges (i.e., sample data on rising edge)"]
    #[inline(always)]
    pub fn tclkph(&self) -> TCLKPH_R {
        TCLKPH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1=Valid data on SPDIN signal is expected after the first SPI_CLK edge. This data is sampled on the second and \n following even SPI_CLK edges (i.e., sample data on falling edge) 0=Valid data is expected on the SPDIN signal on the first SPI_CLK edge. \n This data is sampled on the first and following odd SPI_-CLK edges (i.e., sample data on rising edge)"]
    #[inline(always)]
    pub fn rclkph(&self) -> RCLKPH_R {
        RCLKPH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1=The SPI_CLK signal is high when the interface is idle and the first clock edge is a falling edge\n 0=The SPI_CLK is low when the interface is idle and the first clock edge is a rising edge"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 1=2MHz, 0=48 MHz Ring Oscillator"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1=Valid data is clocked out on the first SPI_CLK edge on SPDOUT signal. The slave device should sample this data on the second and \n following even SPI_CLK edges (i.e., sample data on falling edge) 0=Valid data is clocked out on the SPDOUT signal prior to the first SPI_CLK edge. \n The slave device should sample this data on the first and following odd SPI_CLK edges (i.e., sample data on rising edge)"]
    #[inline(always)]
    pub fn tclkph(&mut self) -> TCLKPH_W<0> {
        TCLKPH_W::new(self)
    }
    #[doc = "Bit 1 - 1=Valid data on SPDIN signal is expected after the first SPI_CLK edge. This data is sampled on the second and \n following even SPI_CLK edges (i.e., sample data on falling edge) 0=Valid data is expected on the SPDIN signal on the first SPI_CLK edge. \n This data is sampled on the first and following odd SPI_-CLK edges (i.e., sample data on rising edge)"]
    #[inline(always)]
    pub fn rclkph(&mut self) -> RCLKPH_W<1> {
        RCLKPH_W::new(self)
    }
    #[doc = "Bit 2 - 1=The SPI_CLK signal is high when the interface is idle and the first clock edge is a falling edge\n 0=The SPI_CLK is low when the interface is idle and the first clock edge is a rising edge"]
    #[inline(always)]
    pub fn clkpol(&mut self) -> CLKPOL_W<2> {
        CLKPOL_W::new(self)
    }
    #[doc = "Bit 4 - 1=2MHz, 0=48 MHz Ring Oscillator"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W<4> {
        CLKSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Clock Control. This register should not be changed during an active SPI transaction.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl](index.html) module"]
pub struct CLK_CTRL_SPEC;
impl crate::RegisterSpec for CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ctrl::R](R) reader structure"]
impl crate::Readable for CLK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ctrl::W](W) writer structure"]
impl crate::Writable for CLK_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CTRL to value 0"]
impl crate::Resettable for CLK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
