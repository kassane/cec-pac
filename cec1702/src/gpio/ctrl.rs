#[doc = "Register `CTRL[%s]` reader"]
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
#[doc = "Register `CTRL[%s]` writer"]
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
#[doc = "Field `PU_PD` reader - These bits are used to enable an internal pull-up or pull-down resistor.\n 00 = None, 01 = Pull Up Enabled, 10 = Pull Down Enabled, 11 = None"]
pub type PU_PD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PU_PD` writer - These bits are used to enable an internal pull-up or pull-down resistor.\n 00 = None, 01 = Pull Up Enabled, 10 = Pull Down Enabled, 11 = None"]
pub type PU_PD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWR_GATING` reader - The GPIO pin will be tristated when the selected power well is off.\n 00 = VTR Power Rail, 01 = VCC Main Power Rail (as determined by the VCC_PWRGD input), 1x = Reserved"]
pub type PWR_GATING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_GATING` writer - The GPIO pin will be tristated when the selected power well is off.\n 00 = VTR Power Rail, 01 = VCC Main Power Rail (as determined by the VCC_PWRGD input), 1x = Reserved"]
pub type PWR_GATING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `INTR_DET` reader - When combined with the field INTERRUPT_DETECTION in this register, determines the interrupt capability of the GPIO input.\n 0 000 = Low Level Sensitive\n 0 001 = High Level Sensitive\n 0 100 = Interrupt events are disabled\n 1 101 = Rising Edge Triggered\n 1 110 = Falling Edge Triggered\n 1 111 = Either edge triggered"]
pub type INTR_DET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTR_DET` writer - When combined with the field INTERRUPT_DETECTION in this register, determines the interrupt capability of the GPIO input.\n 0 000 = Low Level Sensitive\n 0 001 = High Level Sensitive\n 0 100 = Interrupt events are disabled\n 1 101 = Rising Edge Triggered\n 1 110 = Falling Edge Triggered\n 1 111 = Either edge triggered"]
pub type INTR_DET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `EDGE_EN` reader - When combined with the field INTERRUPT_DETECTION in this register, determines the interrupt capability of the GPIO input.\n 0 = Edge detection disabled, 1 = Edge detection enabled"]
pub type EDGE_EN_R = crate::BitReader<bool>;
#[doc = "Field `EDGE_EN` writer - When combined with the field INTERRUPT_DETECTION in this register, determines the interrupt capability of the GPIO input.\n 0 = Edge detection disabled, 1 = Edge detection enabled"]
pub type EDGE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OUT_BUFF_TYPE` reader - Unless explicitly stated otherwise, pins with (I/O/OD) or (O/OD) in their buffer type column in the tables are\n compliant with the following Programmable OD/PP Multiplexing Design Rule: Each compliant pin has a programmable open drain/push-pull\n buffer controlled by the Output Buffer Type bit in the associated Pin Control Register. The state of this bit controls the mode of\n the interface buffer for all selected functions, including the GPIO function. 0 = Push-Pull, 1 = Open Drain"]
pub type OUT_BUFF_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `OUT_BUFF_TYPE` writer - Unless explicitly stated otherwise, pins with (I/O/OD) or (O/OD) in their buffer type column in the tables are\n compliant with the following Programmable OD/PP Multiplexing Design Rule: Each compliant pin has a programmable open drain/push-pull\n buffer controlled by the Output Buffer Type bit in the associated Pin Control Register. The state of this bit controls the mode of\n the interface buffer for all selected functions, including the GPIO function. 0 = Push-Pull, 1 = Open Drain"]
pub type OUT_BUFF_TYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `GPIO_DIR` reader - This bit controls the buffer direction only when the MUX_CONTROL field is '00' selecting the pin signal function to\n be GPIO. When the MUX_CONTROL field is greater than '00' (i.e., a non-GPIO signal function is selected) this bit has no affect\n and the selected signal function logic directly controls the pin direction. 0 = Input, 1 = Output"]
pub type GPIO_DIR_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_DIR` writer - This bit controls the buffer direction only when the MUX_CONTROL field is '00' selecting the pin signal function to\n be GPIO. When the MUX_CONTROL field is greater than '00' (i.e., a non-GPIO signal function is selected) this bit has no affect\n and the selected signal function logic directly controls the pin direction. 0 = Input, 1 = Output"]
pub type GPIO_DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `GPIO_OUT_SEL` reader - This control bit determines which register is used to update the data register for GPIO outputs.\n 0=GPIO output data for this GPIO come from the ALTERNATE_GPIO_DATA field of this register; writes to the bit representing\n this GPIO in the GPIO Output Register do not affect the GPIO; 1=GPIO output data for this GPIO come from the bit representing\n this GPIO in the GPIO Output Register; writes to the ALTERNATE_GPIO_DATA field of this register do not affect the GPIO."]
pub type GPIO_OUT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_OUT_SEL` writer - This control bit determines which register is used to update the data register for GPIO outputs.\n 0=GPIO output data for this GPIO come from the ALTERNATE_GPIO_DATA field of this register; writes to the bit representing\n this GPIO in the GPIO Output Register do not affect the GPIO; 1=GPIO output data for this GPIO come from the bit representing\n this GPIO in the GPIO Output Register; writes to the ALTERNATE_GPIO_DATA field of this register do not affect the GPIO."]
pub type GPIO_OUT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `POL` reader - When the Polarity bit is set to '1' and the MUX_CONTROL bits are greater than '00', the selected signal function outputs\n are inverted and Interrupt Detection sense is inverted. When the MUX_CONTROL field selects the GPIO signal function (Mux='00'), the\n Polarity bit does not effect the output. Regardless of the state of the MUX_CONTROL field and the Polarity bit, the state of the pin\n is always reported without inversion in the GPIO input register. 1=Inverted; 0=Non-inverted"]
pub type POL_R = crate::BitReader<bool>;
#[doc = "Field `POL` writer - When the Polarity bit is set to '1' and the MUX_CONTROL bits are greater than '00', the selected signal function outputs\n are inverted and Interrupt Detection sense is inverted. When the MUX_CONTROL field selects the GPIO signal function (Mux='00'), the\n Polarity bit does not effect the output. Regardless of the state of the MUX_CONTROL field and the Polarity bit, the state of the pin\n is always reported without inversion in the GPIO input register. 1=Inverted; 0=Non-inverted"]
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `MUX_CTRL` reader - This field determines the active signal function for a pin. 00 = GPIO Function Selected, 01 = Signal Function 1 Selected,\n 10 = Signal Function 2 Selected, 11 = Signal Function 3 Selected."]
pub type MUX_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUX_CTRL` writer - This field determines the active signal function for a pin. 00 = GPIO Function Selected, 01 = Signal Function 1 Selected,\n 10 = Signal Function 2 Selected, 11 = Signal Function 3 Selected."]
pub type MUX_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `ALT_GPIO_DATA` reader - Reads of this bit always return the last data written to the GPIO output data register bit; reads do not return the\n current output value of the GPIO pin if it is configured as an output. If the GPIO_OUTPUT_SEL T bit in this register is '1', then\n this bit is Read Only and the GPIO output data register bit is only written by the GPIO Output Register. If the GPIO_OUTPUT_SELECT\n bit in this register is '0', then this bit is R/W, and the bit corresponding to this GPIO in the GPIO Output Register is Read Only."]
pub type ALT_GPIO_DATA_R = crate::BitReader<bool>;
#[doc = "Field `ALT_GPIO_DATA` writer - Reads of this bit always return the last data written to the GPIO output data register bit; reads do not return the\n current output value of the GPIO pin if it is configured as an output. If the GPIO_OUTPUT_SEL T bit in this register is '1', then\n this bit is Read Only and the GPIO output data register bit is only written by the GPIO Output Register. If the GPIO_OUTPUT_SELECT\n bit in this register is '0', then this bit is R/W, and the bit corresponding to this GPIO in the GPIO Output Register is Read Only."]
pub type ALT_GPIO_DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `GPIO_INP` reader - Reads of this bit always return the state of GPIO input from the pad, independent of the Mux selection for the pin\n or the Direction. This bit is forced high when the selected power well is off as selected by the POWER_GATING field in this register."]
pub type GPIO_INP_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_INP` writer - Reads of this bit always return the state of GPIO input from the pad, independent of the Mux selection for the pin\n or the Direction. This bit is forced high when the selected power well is off as selected by the POWER_GATING field in this register."]
pub type GPIO_INP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - These bits are used to enable an internal pull-up or pull-down resistor.\n 00 = None, 01 = Pull Up Enabled, 10 = Pull Down Enabled, 11 = None"]
    #[inline(always)]
    pub fn pu_pd(&self) -> PU_PD_R {
        PU_PD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The GPIO pin will be tristated when the selected power well is off.\n 00 = VTR Power Rail, 01 = VCC Main Power Rail (as determined by the VCC_PWRGD input), 1x = Reserved"]
    #[inline(always)]
    pub fn pwr_gating(&self) -> PWR_GATING_R {
        PWR_GATING_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - When combined with the field INTERRUPT_DETECTION in this register, determines the interrupt capability of the GPIO input.\n 0 000 = Low Level Sensitive\n 0 001 = High Level Sensitive\n 0 100 = Interrupt events are disabled\n 1 101 = Rising Edge Triggered\n 1 110 = Falling Edge Triggered\n 1 111 = Either edge triggered"]
    #[inline(always)]
    pub fn intr_det(&self) -> INTR_DET_R {
        INTR_DET_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - When combined with the field INTERRUPT_DETECTION in this register, determines the interrupt capability of the GPIO input.\n 0 = Edge detection disabled, 1 = Edge detection enabled"]
    #[inline(always)]
    pub fn edge_en(&self) -> EDGE_EN_R {
        EDGE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Unless explicitly stated otherwise, pins with (I/O/OD) or (O/OD) in their buffer type column in the tables are\n compliant with the following Programmable OD/PP Multiplexing Design Rule: Each compliant pin has a programmable open drain/push-pull\n buffer controlled by the Output Buffer Type bit in the associated Pin Control Register. The state of this bit controls the mode of\n the interface buffer for all selected functions, including the GPIO function. 0 = Push-Pull, 1 = Open Drain"]
    #[inline(always)]
    pub fn out_buff_type(&self) -> OUT_BUFF_TYPE_R {
        OUT_BUFF_TYPE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit controls the buffer direction only when the MUX_CONTROL field is '00' selecting the pin signal function to\n be GPIO. When the MUX_CONTROL field is greater than '00' (i.e., a non-GPIO signal function is selected) this bit has no affect\n and the selected signal function logic directly controls the pin direction. 0 = Input, 1 = Output"]
    #[inline(always)]
    pub fn gpio_dir(&self) -> GPIO_DIR_R {
        GPIO_DIR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This control bit determines which register is used to update the data register for GPIO outputs.\n 0=GPIO output data for this GPIO come from the ALTERNATE_GPIO_DATA field of this register; writes to the bit representing\n this GPIO in the GPIO Output Register do not affect the GPIO; 1=GPIO output data for this GPIO come from the bit representing\n this GPIO in the GPIO Output Register; writes to the ALTERNATE_GPIO_DATA field of this register do not affect the GPIO."]
    #[inline(always)]
    pub fn gpio_out_sel(&self) -> GPIO_OUT_SEL_R {
        GPIO_OUT_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When the Polarity bit is set to '1' and the MUX_CONTROL bits are greater than '00', the selected signal function outputs\n are inverted and Interrupt Detection sense is inverted. When the MUX_CONTROL field selects the GPIO signal function (Mux='00'), the\n Polarity bit does not effect the output. Regardless of the state of the MUX_CONTROL field and the Polarity bit, the state of the pin\n is always reported without inversion in the GPIO input register. 1=Inverted; 0=Non-inverted"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - This field determines the active signal function for a pin. 00 = GPIO Function Selected, 01 = Signal Function 1 Selected,\n 10 = Signal Function 2 Selected, 11 = Signal Function 3 Selected."]
    #[inline(always)]
    pub fn mux_ctrl(&self) -> MUX_CTRL_R {
        MUX_CTRL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Reads of this bit always return the last data written to the GPIO output data register bit; reads do not return the\n current output value of the GPIO pin if it is configured as an output. If the GPIO_OUTPUT_SEL T bit in this register is '1', then\n this bit is Read Only and the GPIO output data register bit is only written by the GPIO Output Register. If the GPIO_OUTPUT_SELECT\n bit in this register is '0', then this bit is R/W, and the bit corresponding to this GPIO in the GPIO Output Register is Read Only."]
    #[inline(always)]
    pub fn alt_gpio_data(&self) -> ALT_GPIO_DATA_R {
        ALT_GPIO_DATA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Reads of this bit always return the state of GPIO input from the pad, independent of the Mux selection for the pin\n or the Direction. This bit is forced high when the selected power well is off as selected by the POWER_GATING field in this register."]
    #[inline(always)]
    pub fn gpio_inp(&self) -> GPIO_INP_R {
        GPIO_INP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - These bits are used to enable an internal pull-up or pull-down resistor.\n 00 = None, 01 = Pull Up Enabled, 10 = Pull Down Enabled, 11 = None"]
    #[inline(always)]
    pub fn pu_pd(&mut self) -> PU_PD_W<0> {
        PU_PD_W::new(self)
    }
    #[doc = "Bits 2:3 - The GPIO pin will be tristated when the selected power well is off.\n 00 = VTR Power Rail, 01 = VCC Main Power Rail (as determined by the VCC_PWRGD input), 1x = Reserved"]
    #[inline(always)]
    pub fn pwr_gating(&mut self) -> PWR_GATING_W<2> {
        PWR_GATING_W::new(self)
    }
    #[doc = "Bits 4:6 - When combined with the field INTERRUPT_DETECTION in this register, determines the interrupt capability of the GPIO input.\n 0 000 = Low Level Sensitive\n 0 001 = High Level Sensitive\n 0 100 = Interrupt events are disabled\n 1 101 = Rising Edge Triggered\n 1 110 = Falling Edge Triggered\n 1 111 = Either edge triggered"]
    #[inline(always)]
    pub fn intr_det(&mut self) -> INTR_DET_W<4> {
        INTR_DET_W::new(self)
    }
    #[doc = "Bit 7 - When combined with the field INTERRUPT_DETECTION in this register, determines the interrupt capability of the GPIO input.\n 0 = Edge detection disabled, 1 = Edge detection enabled"]
    #[inline(always)]
    pub fn edge_en(&mut self) -> EDGE_EN_W<7> {
        EDGE_EN_W::new(self)
    }
    #[doc = "Bit 8 - Unless explicitly stated otherwise, pins with (I/O/OD) or (O/OD) in their buffer type column in the tables are\n compliant with the following Programmable OD/PP Multiplexing Design Rule: Each compliant pin has a programmable open drain/push-pull\n buffer controlled by the Output Buffer Type bit in the associated Pin Control Register. The state of this bit controls the mode of\n the interface buffer for all selected functions, including the GPIO function. 0 = Push-Pull, 1 = Open Drain"]
    #[inline(always)]
    pub fn out_buff_type(&mut self) -> OUT_BUFF_TYPE_W<8> {
        OUT_BUFF_TYPE_W::new(self)
    }
    #[doc = "Bit 9 - This bit controls the buffer direction only when the MUX_CONTROL field is '00' selecting the pin signal function to\n be GPIO. When the MUX_CONTROL field is greater than '00' (i.e., a non-GPIO signal function is selected) this bit has no affect\n and the selected signal function logic directly controls the pin direction. 0 = Input, 1 = Output"]
    #[inline(always)]
    pub fn gpio_dir(&mut self) -> GPIO_DIR_W<9> {
        GPIO_DIR_W::new(self)
    }
    #[doc = "Bit 10 - This control bit determines which register is used to update the data register for GPIO outputs.\n 0=GPIO output data for this GPIO come from the ALTERNATE_GPIO_DATA field of this register; writes to the bit representing\n this GPIO in the GPIO Output Register do not affect the GPIO; 1=GPIO output data for this GPIO come from the bit representing\n this GPIO in the GPIO Output Register; writes to the ALTERNATE_GPIO_DATA field of this register do not affect the GPIO."]
    #[inline(always)]
    pub fn gpio_out_sel(&mut self) -> GPIO_OUT_SEL_W<10> {
        GPIO_OUT_SEL_W::new(self)
    }
    #[doc = "Bit 11 - When the Polarity bit is set to '1' and the MUX_CONTROL bits are greater than '00', the selected signal function outputs\n are inverted and Interrupt Detection sense is inverted. When the MUX_CONTROL field selects the GPIO signal function (Mux='00'), the\n Polarity bit does not effect the output. Regardless of the state of the MUX_CONTROL field and the Polarity bit, the state of the pin\n is always reported without inversion in the GPIO input register. 1=Inverted; 0=Non-inverted"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<11> {
        POL_W::new(self)
    }
    #[doc = "Bits 12:13 - This field determines the active signal function for a pin. 00 = GPIO Function Selected, 01 = Signal Function 1 Selected,\n 10 = Signal Function 2 Selected, 11 = Signal Function 3 Selected."]
    #[inline(always)]
    pub fn mux_ctrl(&mut self) -> MUX_CTRL_W<12> {
        MUX_CTRL_W::new(self)
    }
    #[doc = "Bit 16 - Reads of this bit always return the last data written to the GPIO output data register bit; reads do not return the\n current output value of the GPIO pin if it is configured as an output. If the GPIO_OUTPUT_SEL T bit in this register is '1', then\n this bit is Read Only and the GPIO output data register bit is only written by the GPIO Output Register. If the GPIO_OUTPUT_SELECT\n bit in this register is '0', then this bit is R/W, and the bit corresponding to this GPIO in the GPIO Output Register is Read Only."]
    #[inline(always)]
    pub fn alt_gpio_data(&mut self) -> ALT_GPIO_DATA_W<16> {
        ALT_GPIO_DATA_W::new(self)
    }
    #[doc = "Bit 24 - Reads of this bit always return the state of GPIO input from the pad, independent of the Mux selection for the pin\n or the Direction. This bit is forced high when the selected power well is off as selected by the POWER_GATING field in this register."]
    #[inline(always)]
    pub fn gpio_inp(&mut self) -> GPIO_INP_W<24> {
        GPIO_INP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL[%s]
to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
