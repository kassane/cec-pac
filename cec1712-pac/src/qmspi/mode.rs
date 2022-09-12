#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT` reader - This bit is used to activate the QMSPI block.\n 1=Enabled. The block is fully operational\n 0=Disabled. Clocks are gated to conserve power and the output signals are set to their inactive state."]
pub type ACT_R = crate::BitReader<bool>;
#[doc = "Field `ACT` writer - This bit is used to activate the QMSPI block.\n 1=Enabled. The block is fully operational\n 0=Disabled. Clocks are gated to conserve power and the output signals are set to their inactive state."]
pub type ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `SOFT_RST` reader - Writing this bit with a 1 will reset the Quad SPI block. It is self-clearing."]
pub type SOFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `SOFT_RST` writer - Writing this bit with a 1 will reset the Quad SPI block. It is self-clearing."]
pub type SOFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `CPOL` reader - Polarity of the SPI clock line when there are no transactions in process. 1=SPI Clock starts High; 0=SPI Clock starts Low."]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - Polarity of the SPI clock line when there are no transactions in process. 1=SPI Clock starts High; 0=SPI Clock starts Low."]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `CHPA_MOSI` reader - Clock phase of the Master data out. Common SPI modes require this field to be programmed with the same value as CHPA_MISO in this register.\n e.g. Mode 0: CPOL=0; CHPA_MISO=0; CHPA_MOSI=0; Mode 3: CPOL=1; CHPA_MISO=1; CHPA_MOSI=1. See datasheet for alternate mode settings.\n If CPOL=1: 1=Data changes on the falling edge of the SPI clock; 0=Data changes on the rising edge of the SPI clock\n If CPOL=0: 1=Data changes on the rising edge of the SPI clock; 0=Data changes on the falling edge of the SPI clock"]
pub type CHPA_MOSI_R = crate::BitReader<bool>;
#[doc = "Field `CHPA_MOSI` writer - Clock phase of the Master data out. Common SPI modes require this field to be programmed with the same value as CHPA_MISO in this register.\n e.g. Mode 0: CPOL=0; CHPA_MISO=0; CHPA_MOSI=0; Mode 3: CPOL=1; CHPA_MISO=1; CHPA_MOSI=1. See datasheet for alternate mode settings.\n If CPOL=1: 1=Data changes on the falling edge of the SPI clock; 0=Data changes on the rising edge of the SPI clock\n If CPOL=0: 1=Data changes on the rising edge of the SPI clock; 0=Data changes on the falling edge of the SPI clock"]
pub type CHPA_MOSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `CHPA_MISO` reader - Clock phase of the Master data in. Common SPI modes require this field to be programmed with the same value as CHPA_MOSI in this register.\n e.g. Mode 0: CPOL=0; CHPA_MISO=0; CHPA_MOSI=0; Mode 3: CPOL=1; CHPA_MISO=1; CHPA_MOSI=1. See datasheet for alternate mode settings.\n If CPOL=1: 1=Data are captured on the rising edge of the SPI clock; 0=Data are captured on the falling edge of the SPI clock\n If CPOL=0: 1=Data are captured on the falling edge of the SPI clock; 0=Data are captured on the rising edge of the SPI clock"]
pub type CHPA_MISO_R = crate::BitReader<bool>;
#[doc = "Field `CHPA_MISO` writer - Clock phase of the Master data in. Common SPI modes require this field to be programmed with the same value as CHPA_MOSI in this register.\n e.g. Mode 0: CPOL=0; CHPA_MISO=0; CHPA_MOSI=0; Mode 3: CPOL=1; CHPA_MISO=1; CHPA_MOSI=1. See datasheet for alternate mode settings.\n If CPOL=1: 1=Data are captured on the rising edge of the SPI clock; 0=Data are captured on the falling edge of the SPI clock\n If CPOL=0: 1=Data are captured on the falling edge of the SPI clock; 0=Data are captured on the rising edge of the SPI clock"]
pub type CHPA_MISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `CLK_DIV` reader - The SPI clock divide in number of system clocks. A value of 1 divides the master clock by 1, a value of 255 divides the master clock by 255. A value of 0 divides the master clock by 256."]
pub type CLK_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLK_DIV` writer - The SPI clock divide in number of system clocks. A value of 1 divides the master clock by 1, a value of 255 divides the master clock by 255. A value of 0 divides the master clock by 256."]
pub type CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to activate the QMSPI block.\n 1=Enabled. The block is fully operational\n 0=Disabled. Clocks are gated to conserve power and the output signals are set to their inactive state."]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Writing this bit with a 1 will reset the Quad SPI block. It is self-clearing."]
    #[inline(always)]
    pub fn soft_rst(&self) -> SOFT_RST_R {
        SOFT_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Polarity of the SPI clock line when there are no transactions in process. 1=SPI Clock starts High; 0=SPI Clock starts Low."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase of the Master data out. Common SPI modes require this field to be programmed with the same value as CHPA_MISO in this register.\n e.g. Mode 0: CPOL=0; CHPA_MISO=0; CHPA_MOSI=0; Mode 3: CPOL=1; CHPA_MISO=1; CHPA_MOSI=1. See datasheet for alternate mode settings.\n If CPOL=1: 1=Data changes on the falling edge of the SPI clock; 0=Data changes on the rising edge of the SPI clock\n If CPOL=0: 1=Data changes on the rising edge of the SPI clock; 0=Data changes on the falling edge of the SPI clock"]
    #[inline(always)]
    pub fn chpa_mosi(&self) -> CHPA_MOSI_R {
        CHPA_MOSI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock phase of the Master data in. Common SPI modes require this field to be programmed with the same value as CHPA_MOSI in this register.\n e.g. Mode 0: CPOL=0; CHPA_MISO=0; CHPA_MOSI=0; Mode 3: CPOL=1; CHPA_MISO=1; CHPA_MOSI=1. See datasheet for alternate mode settings.\n If CPOL=1: 1=Data are captured on the rising edge of the SPI clock; 0=Data are captured on the falling edge of the SPI clock\n If CPOL=0: 1=Data are captured on the falling edge of the SPI clock; 0=Data are captured on the rising edge of the SPI clock"]
    #[inline(always)]
    pub fn chpa_miso(&self) -> CHPA_MISO_R {
        CHPA_MISO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:24 - The SPI clock divide in number of system clocks. A value of 1 divides the master clock by 1, a value of 255 divides the master clock by 255. A value of 0 divides the master clock by 256."]
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to activate the QMSPI block.\n 1=Enabled. The block is fully operational\n 0=Disabled. Clocks are gated to conserve power and the output signals are set to their inactive state."]
    #[inline(always)]
    pub fn act(&mut self) -> ACT_W<0> {
        ACT_W::new(self)
    }
    #[doc = "Bit 1 - Writing this bit with a 1 will reset the Quad SPI block. It is self-clearing."]
    #[inline(always)]
    pub fn soft_rst(&mut self) -> SOFT_RST_W<1> {
        SOFT_RST_W::new(self)
    }
    #[doc = "Bit 8 - Polarity of the SPI clock line when there are no transactions in process. 1=SPI Clock starts High; 0=SPI Clock starts Low."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<8> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 9 - Clock phase of the Master data out. Common SPI modes require this field to be programmed with the same value as CHPA_MISO in this register.\n e.g. Mode 0: CPOL=0; CHPA_MISO=0; CHPA_MOSI=0; Mode 3: CPOL=1; CHPA_MISO=1; CHPA_MOSI=1. See datasheet for alternate mode settings.\n If CPOL=1: 1=Data changes on the falling edge of the SPI clock; 0=Data changes on the rising edge of the SPI clock\n If CPOL=0: 1=Data changes on the rising edge of the SPI clock; 0=Data changes on the falling edge of the SPI clock"]
    #[inline(always)]
    pub fn chpa_mosi(&mut self) -> CHPA_MOSI_W<9> {
        CHPA_MOSI_W::new(self)
    }
    #[doc = "Bit 10 - Clock phase of the Master data in. Common SPI modes require this field to be programmed with the same value as CHPA_MOSI in this register.\n e.g. Mode 0: CPOL=0; CHPA_MISO=0; CHPA_MOSI=0; Mode 3: CPOL=1; CHPA_MISO=1; CHPA_MOSI=1. See datasheet for alternate mode settings.\n If CPOL=1: 1=Data are captured on the rising edge of the SPI clock; 0=Data are captured on the falling edge of the SPI clock\n If CPOL=0: 1=Data are captured on the falling edge of the SPI clock; 0=Data are captured on the rising edge of the SPI clock"]
    #[inline(always)]
    pub fn chpa_miso(&mut self) -> CHPA_MISO_W<10> {
        CHPA_MISO_W::new(self)
    }
    #[doc = "Bits 16:24 - The SPI clock divide in number of system clocks. A value of 1 divides the master clock by 1, a value of 255 divides the master clock by 255. A value of 0 divides the master clock by 256."]
    #[inline(always)]
    pub fn clk_div(&mut self) -> CLK_DIV_W<16> {
        CLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
