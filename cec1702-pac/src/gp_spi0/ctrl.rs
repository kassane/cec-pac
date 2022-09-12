#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSBF` reader - Least Significant Bit First\n 1= The data is transferred in LSB-first order.\n 0= The data is transferred in MSB-first order. (default)"]
pub type LSBF_R = crate::BitReader<bool>;
#[doc = "Field `LSBF` writer - Least Significant Bit First\n 1= The data is transferred in LSB-first order.\n 0= The data is transferred in MSB-first order. (default)"]
pub type LSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BIOEN` reader - Bidirectional Output Enable control.\n 1=The SPDOUT_Direction signal configures the SPDOUT signal as an output.\n 0=The SPDOUT_Direction signal configures the SPDOUT signal as an input."]
pub type BIOEN_R = crate::BitReader<bool>;
#[doc = "Field `BIOEN` writer - Bidirectional Output Enable control.\n 1=The SPDOUT_Direction signal configures the SPDOUT signal as an output.\n 0=The SPDOUT_Direction signal configures the SPDOUT signal as an input."]
pub type BIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SPDIN_SEL` reader - 3:2\\]
1xb=SPDIN1 and SPDIN2. Select this option for Dual Mode\n \\[3:2\\]
01b=SPDIN2 only. Select this option for Half Duplex\n \\[3:2\\]
00b=SPDIN1 only. Select this option for Full Duplex"]
pub type SPDIN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPDIN_SEL` writer - 3:2\\]
1xb=SPDIN1 and SPDIN2. Select this option for Dual Mode\n \\[3:2\\]
01b=SPDIN2 only. Select this option for Half Duplex\n \\[3:2\\]
00b=SPDIN1 only. Select this option for Full Duplex"]
pub type SPDIN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SOFT_RST` reader - Soft Reset is a self-clearing bit. Writing zero to this bit has no effect. \n Writing a one to this bit resets the entire SPI Interface, including all counters and registers back to their initial state."]
pub type SOFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `SOFT_RST` writer - Soft Reset is a self-clearing bit. Writing zero to this bit has no effect. \n Writing a one to this bit resets the entire SPI Interface, including all counters and registers back to their initial state."]
pub type SOFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `AUTO_RD` reader - Auto Read Enable.\n 1=A read of the SPI RX_DATA Register will clear both the RXBF status bit and the TXBE status bit\n 0=A read of the SPI RX_DATA Register will clear the RXBF status bit. The TXBE status bit will not be modified"]
pub type AUTO_RD_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_RD` writer - Auto Read Enable.\n 1=A read of the SPI RX_DATA Register will clear both the RXBF status bit and the TXBE status bit\n 0=A read of the SPI RX_DATA Register will clear the RXBF status bit. The TXBE status bit will not be modified"]
pub type AUTO_RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CE` reader - SPI Chip Select Enable.\n 1= SPI_CS# output signal is asserted, i.e., driven to logic '0' \n 0= SPI_CS# output signal is deasserted, i.e., driven to logic '1'"]
pub type CE_R = crate::BitReader<bool>;
#[doc = "Field `CE` writer - SPI Chip Select Enable.\n 1= SPI_CS# output signal is asserted, i.e., driven to logic '0' \n 0= SPI_CS# output signal is deasserted, i.e., driven to logic '1'"]
pub type CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Least Significant Bit First\n 1= The data is transferred in LSB-first order.\n 0= The data is transferred in MSB-first order. (default)"]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bidirectional Output Enable control.\n 1=The SPDOUT_Direction signal configures the SPDOUT signal as an output.\n 0=The SPDOUT_Direction signal configures the SPDOUT signal as an input."]
    #[inline(always)]
    pub fn bioen(&self) -> BIOEN_R {
        BIOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
1xb=SPDIN1 and SPDIN2. Select this option for Dual Mode\n \\[3:2\\]
01b=SPDIN2 only. Select this option for Half Duplex\n \\[3:2\\]
00b=SPDIN1 only. Select this option for Full Duplex"]
    #[inline(always)]
    pub fn spdin_sel(&self) -> SPDIN_SEL_R {
        SPDIN_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Soft Reset is a self-clearing bit. Writing zero to this bit has no effect. \n Writing a one to this bit resets the entire SPI Interface, including all counters and registers back to their initial state."]
    #[inline(always)]
    pub fn soft_rst(&self) -> SOFT_RST_R {
        SOFT_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Auto Read Enable.\n 1=A read of the SPI RX_DATA Register will clear both the RXBF status bit and the TXBE status bit\n 0=A read of the SPI RX_DATA Register will clear the RXBF status bit. The TXBE status bit will not be modified"]
    #[inline(always)]
    pub fn auto_rd(&self) -> AUTO_RD_R {
        AUTO_RD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Chip Select Enable.\n 1= SPI_CS# output signal is asserted, i.e., driven to logic '0' \n 0= SPI_CS# output signal is deasserted, i.e., driven to logic '1'"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Least Significant Bit First\n 1= The data is transferred in LSB-first order.\n 0= The data is transferred in MSB-first order. (default)"]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LSBF_W<0> {
        LSBF_W::new(self)
    }
    #[doc = "Bit 1 - Bidirectional Output Enable control.\n 1=The SPDOUT_Direction signal configures the SPDOUT signal as an output.\n 0=The SPDOUT_Direction signal configures the SPDOUT signal as an input."]
    #[inline(always)]
    pub fn bioen(&mut self) -> BIOEN_W<1> {
        BIOEN_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
1xb=SPDIN1 and SPDIN2. Select this option for Dual Mode\n \\[3:2\\]
01b=SPDIN2 only. Select this option for Half Duplex\n \\[3:2\\]
00b=SPDIN1 only. Select this option for Full Duplex"]
    #[inline(always)]
    pub fn spdin_sel(&mut self) -> SPDIN_SEL_W<2> {
        SPDIN_SEL_W::new(self)
    }
    #[doc = "Bit 4 - Soft Reset is a self-clearing bit. Writing zero to this bit has no effect. \n Writing a one to this bit resets the entire SPI Interface, including all counters and registers back to their initial state."]
    #[inline(always)]
    pub fn soft_rst(&mut self) -> SOFT_RST_W<4> {
        SOFT_RST_W::new(self)
    }
    #[doc = "Bit 5 - Auto Read Enable.\n 1=A read of the SPI RX_DATA Register will clear both the RXBF status bit and the TXBE status bit\n 0=A read of the SPI RX_DATA Register will clear the RXBF status bit. The TXBE status bit will not be modified"]
    #[inline(always)]
    pub fn auto_rd(&mut self) -> AUTO_RD_W<5> {
        AUTO_RD_W::new(self)
    }
    #[doc = "Bit 6 - SPI Chip Select Enable.\n 1= SPI_CS# output signal is asserted, i.e., driven to logic '0' \n 0= SPI_CS# output signal is deasserted, i.e., driven to logic '1'"]
    #[inline(always)]
    pub fn ce(&mut self) -> CE_W<6> {
        CE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x02"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
