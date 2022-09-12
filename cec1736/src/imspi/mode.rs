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
#[doc = "Field `ACTIVATE` reader - This bit enables the controller. 1=The controller is enabled; 0=The controller is disabled and placed in its lowest power state."]
pub type ACTIVATE_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVATE` writer - This bit enables the controller. 1=The controller is enabled; 0=The controller is disabled and placed in its lowest power state."]
pub type ACTIVATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `SOFT_RESET` reader - A write of '1b' to this bit resets the controller. This bit is self-clearing."]
pub type SOFT_RESET_R = crate::BitReader<bool>;
#[doc = "Field `SOFT_RESET` writer - A write of '1b' to this bit resets the controller. This bit is self-clearing."]
pub type SOFT_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `DLY2_SUSB` reader - This bit is routed to the DLY2_SUSB# pin function."]
pub type DLY2_SUSB_R = crate::BitReader<bool>;
#[doc = "Field `DLY2_SUSB` writer - This bit is routed to the DLY2_SUSB# pin function."]
pub type DLY2_SUSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `CPOL` reader - This bit corresponds to the Polarity control for the underlying SPI controller. It describes the default state of the SPI Clock signal. 1=The clock starts in a high state; 0=The clock starts in a low state."]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - This bit corresponds to the Polarity control for the underlying SPI controller. It describes the default state of the SPI Clock signal. 1=The clock starts in a high state; 0=The clock starts in a low state."]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `CPHA_MOSI` reader - This field is the CPHA field of the underlying SPI controller which affects only the MOSI Data. This field changes determines the clock edge on which data are sent, in combination with the CPOL field. 1=If CPOL=0, data sent on Rising Edge; if CPOL=1, data sent on Falling Edge ; 0=If CPOL=0, data sent on Falling Edge; if CPOL=1, data sent on Rising Edge."]
pub type CPHA_MOSI_R = crate::BitReader<bool>;
#[doc = "Field `CPHA_MOSI` writer - This field is the CPHA field of the underlying SPI controller which affects only the MOSI Data. This field changes determines the clock edge on which data are sent, in combination with the CPOL field. 1=If CPOL=0, data sent on Rising Edge; if CPOL=1, data sent on Falling Edge ; 0=If CPOL=0, data sent on Falling Edge; if CPOL=1, data sent on Rising Edge."]
pub type CPHA_MOSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `CPHA_MISO` reader - This field is the CPHA field of the underlying SPI controller which affects only the MISO Data. This field changes determines the clock edge on which data are captured, in combination with the CPOL field. For standard SPI Modes, this must be programmed with the same value as CPHA_MOSI. 1=If CPOL=0, data captured on Falling Edge; if CPOL=1, data captured on Rising Edge 0=If CPOL=0, data captured on Rising Edge; if CPOL=1, data captured on Falling Edge."]
pub type CPHA_MISO_R = crate::BitReader<bool>;
#[doc = "Field `CPHA_MISO` writer - This field is the CPHA field of the underlying SPI controller which affects only the MISO Data. This field changes determines the clock edge on which data are captured, in combination with the CPOL field. For standard SPI Modes, this must be programmed with the same value as CPHA_MOSI. 1=If CPOL=0, data captured on Falling Edge; if CPOL=1, data captured on Rising Edge 0=If CPOL=0, data captured on Rising Edge; if CPOL=1, data captured on Falling Edge."]
pub type CPHA_MISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `CLOCK_DIVIDE` reader - This SPI clock divide in terms of the number of system clocks. 255:1=The SPI clock period is equal to this number of system clocks. 0=The SPI clock period is equal to 256 system clocks."]
pub type CLOCK_DIVIDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_DIVIDE` writer - This SPI clock divide in terms of the number of system clocks. 255:1=The SPI clock period is equal to this number of system clocks. 0=The SPI clock period is equal to 256 system clocks."]
pub type CLOCK_DIVIDE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 8, O>;
#[doc = "Field `IF_MODE` reader - This field sets the interface mode for the SPI controller. 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single Mode."]
pub type IF_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IF_MODE` writer - This field sets the interface mode for the SPI controller. 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single Mode."]
pub type IF_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - This bit enables the controller. 1=The controller is enabled; 0=The controller is disabled and placed in its lowest power state."]
    #[inline(always)]
    pub fn activate(&self) -> ACTIVATE_R {
        ACTIVATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A write of '1b' to this bit resets the controller. This bit is self-clearing."]
    #[inline(always)]
    pub fn soft_reset(&self) -> SOFT_RESET_R {
        SOFT_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is routed to the DLY2_SUSB# pin function."]
    #[inline(always)]
    pub fn dly2_susb(&self) -> DLY2_SUSB_R {
        DLY2_SUSB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit corresponds to the Polarity control for the underlying SPI controller. It describes the default state of the SPI Clock signal. 1=The clock starts in a high state; 0=The clock starts in a low state."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This field is the CPHA field of the underlying SPI controller which affects only the MOSI Data. This field changes determines the clock edge on which data are sent, in combination with the CPOL field. 1=If CPOL=0, data sent on Rising Edge; if CPOL=1, data sent on Falling Edge ; 0=If CPOL=0, data sent on Falling Edge; if CPOL=1, data sent on Rising Edge."]
    #[inline(always)]
    pub fn cpha_mosi(&self) -> CPHA_MOSI_R {
        CPHA_MOSI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This field is the CPHA field of the underlying SPI controller which affects only the MISO Data. This field changes determines the clock edge on which data are captured, in combination with the CPOL field. For standard SPI Modes, this must be programmed with the same value as CPHA_MOSI. 1=If CPOL=0, data captured on Falling Edge; if CPOL=1, data captured on Rising Edge 0=If CPOL=0, data captured on Rising Edge; if CPOL=1, data captured on Falling Edge."]
    #[inline(always)]
    pub fn cpha_miso(&self) -> CPHA_MISO_R {
        CPHA_MISO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:23 - This SPI clock divide in terms of the number of system clocks. 255:1=The SPI clock period is equal to this number of system clocks. 0=The SPI clock period is equal to 256 system clocks."]
    #[inline(always)]
    pub fn clock_divide(&self) -> CLOCK_DIVIDE_R {
        CLOCK_DIVIDE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - This field sets the interface mode for the SPI controller. 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single Mode."]
    #[inline(always)]
    pub fn if_mode(&self) -> IF_MODE_R {
        IF_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables the controller. 1=The controller is enabled; 0=The controller is disabled and placed in its lowest power state."]
    #[inline(always)]
    pub fn activate(&mut self) -> ACTIVATE_W<0> {
        ACTIVATE_W::new(self)
    }
    #[doc = "Bit 1 - A write of '1b' to this bit resets the controller. This bit is self-clearing."]
    #[inline(always)]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W<1> {
        SOFT_RESET_W::new(self)
    }
    #[doc = "Bit 2 - This bit is routed to the DLY2_SUSB# pin function."]
    #[inline(always)]
    pub fn dly2_susb(&mut self) -> DLY2_SUSB_W<2> {
        DLY2_SUSB_W::new(self)
    }
    #[doc = "Bit 8 - This bit corresponds to the Polarity control for the underlying SPI controller. It describes the default state of the SPI Clock signal. 1=The clock starts in a high state; 0=The clock starts in a low state."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<8> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 9 - This field is the CPHA field of the underlying SPI controller which affects only the MOSI Data. This field changes determines the clock edge on which data are sent, in combination with the CPOL field. 1=If CPOL=0, data sent on Rising Edge; if CPOL=1, data sent on Falling Edge ; 0=If CPOL=0, data sent on Falling Edge; if CPOL=1, data sent on Rising Edge."]
    #[inline(always)]
    pub fn cpha_mosi(&mut self) -> CPHA_MOSI_W<9> {
        CPHA_MOSI_W::new(self)
    }
    #[doc = "Bit 10 - This field is the CPHA field of the underlying SPI controller which affects only the MISO Data. This field changes determines the clock edge on which data are captured, in combination with the CPOL field. For standard SPI Modes, this must be programmed with the same value as CPHA_MOSI. 1=If CPOL=0, data captured on Falling Edge; if CPOL=1, data captured on Rising Edge 0=If CPOL=0, data captured on Rising Edge; if CPOL=1, data captured on Falling Edge."]
    #[inline(always)]
    pub fn cpha_miso(&mut self) -> CPHA_MISO_W<10> {
        CPHA_MISO_W::new(self)
    }
    #[doc = "Bits 16:23 - This SPI clock divide in terms of the number of system clocks. 255:1=The SPI clock period is equal to this number of system clocks. 0=The SPI clock period is equal to 256 system clocks."]
    #[inline(always)]
    pub fn clock_divide(&mut self) -> CLOCK_DIVIDE_W<16> {
        CLOCK_DIVIDE_W::new(self)
    }
    #[doc = "Bits 24:25 - This field sets the interface mode for the SPI controller. 3=Reserved; 2=Quad Mode; 1=Dual Mode; 0=Single Mode."]
    #[inline(always)]
    pub fn if_mode(&mut self) -> IF_MODE_W<24> {
        IF_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IMSPI Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
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
