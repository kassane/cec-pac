#[doc = "Register `CTRL15[%s]` reader"]
pub struct R(crate::R<CTRL15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL15[%s]` writer"]
pub struct W(crate::W<CTRL15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL15_SPEC>;
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
impl From<crate::W<CTRL15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU_PD` reader - These bits are used to enable an internal pull-up or pull-down resistor."]
pub type PU_PD_R = crate::FieldReader<u8, PU_PDSELECT_A>;
#[doc = "These bits are used to enable an internal pull-up or pull-down resistor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PU_PDSELECT_A {
    #[doc = "0: None. Pin tristates when no active driver is present on the pin."]
    NONE = 0,
    #[doc = "1: Pull Up Enabled"]
    UP = 1,
    #[doc = "2: Pull Down Enabled"]
    DOWN = 2,
    #[doc = "3: Repeater mode. Pin is kept at previous voltage level when no active driver is present on the pin."]
    REPEATER = 3,
}
impl From<PU_PDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PU_PDSELECT_A) -> Self {
        variant as _
    }
}
impl PU_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PU_PDSELECT_A {
        match self.bits {
            0 => PU_PDSELECT_A::NONE,
            1 => PU_PDSELECT_A::UP,
            2 => PU_PDSELECT_A::DOWN,
            3 => PU_PDSELECT_A::REPEATER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PU_PDSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == PU_PDSELECT_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == PU_PDSELECT_A::DOWN
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PU_PDSELECT_A::REPEATER
    }
}
#[doc = "Field `PU_PD` writer - These bits are used to enable an internal pull-up or pull-down resistor."]
pub type PU_PD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL15_SPEC, u8, PU_PDSELECT_A, 2, O>;
impl<'a, const O: u8> PU_PD_W<'a, O> {
    #[doc = "None. Pin tristates when no active driver is present on the pin."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PU_PDSELECT_A::NONE)
    }
    #[doc = "Pull Up Enabled"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(PU_PDSELECT_A::UP)
    }
    #[doc = "Pull Down Enabled"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(PU_PDSELECT_A::DOWN)
    }
    #[doc = "Repeater mode. Pin is kept at previous voltage level when no active driver is present on the pin."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(PU_PDSELECT_A::REPEATER)
    }
}
#[doc = "Field `PWR_GATING` reader - The GPIO pin will be tristated when the selected power well is off."]
pub type PWR_GATING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_GATING` writer - The GPIO pin will be tristated when the selected power well is off."]
pub type PWR_GATING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL15_SPEC, u8, u8, 2, O>;
#[doc = "Field `INTR_DET` reader - Determines the interrupt capability of the GPIO input."]
pub type INTR_DET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTR_DET` writer - Determines the interrupt capability of the GPIO input."]
pub type INTR_DET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL15_SPEC, u8, u8, 3, O>;
#[doc = "Field `EDGE_EN` reader - Determines the interrupt capability of the GPIO input."]
pub type EDGE_EN_R = crate::BitReader<bool>;
#[doc = "Field `EDGE_EN` writer - Determines the interrupt capability of the GPIO input."]
pub type EDGE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL15_SPEC, bool, O>;
#[doc = "Field `OUT_BUFF_TYPE` reader - 0 = Push-Pull, 1 = Open Drain"]
pub type OUT_BUFF_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `OUT_BUFF_TYPE` writer - 0 = Push-Pull, 1 = Open Drain"]
pub type OUT_BUFF_TYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL15_SPEC, bool, O>;
#[doc = "Field `GPIO_DIR` reader - Buffer direction when GPIO selected by pin mux 0 = Input, 1 = Output"]
pub type GPIO_DIR_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_DIR` writer - Buffer direction when GPIO selected by pin mux 0 = Input, 1 = Output"]
pub type GPIO_DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL15_SPEC, bool, O>;
#[doc = "Field `GPIO_OUT_SEL` reader - GPIO outputs registe select.0=GPIO ALTERNATE_GPIO_DATA 1=GPIO Output Register."]
pub type GPIO_OUT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_OUT_SEL` writer - GPIO outputs registe select.0=GPIO ALTERNATE_GPIO_DATA 1=GPIO Output Register."]
pub type GPIO_OUT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL15_SPEC, bool, O>;
#[doc = "Field `POL` reader - 1=Inverted; 0=Non-inverted"]
pub type POL_R = crate::BitReader<bool>;
#[doc = "Field `POL` writer - 1=Inverted; 0=Non-inverted"]
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL15_SPEC, bool, O>;
#[doc = "Field `MUX_CTRL` reader - 00 = GPIO Function, 01 = Function 1, 10 = Function 2, 11 = Function 3."]
pub type MUX_CTRL_R = crate::FieldReader<u8, MUX_CTRLSELECT_A>;
#[doc = "00 = GPIO Function, 01 = Function 1, 10 = Function 2, 11 = Function 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_CTRLSELECT_A {
    #[doc = "0: GPIO function selected"]
    GPIO = 0,
    #[doc = "1: Signal function 1 selected"]
    FUNC1 = 1,
    #[doc = "2: Signal function 2 selected"]
    FUNC2 = 2,
    #[doc = "3: Signal function 3 selected"]
    FUNC3 = 3,
}
impl From<MUX_CTRLSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_CTRLSELECT_A) -> Self {
        variant as _
    }
}
impl MUX_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUX_CTRLSELECT_A> {
        match self.bits {
            0 => Some(MUX_CTRLSELECT_A::GPIO),
            1 => Some(MUX_CTRLSELECT_A::FUNC1),
            2 => Some(MUX_CTRLSELECT_A::FUNC2),
            3 => Some(MUX_CTRLSELECT_A::FUNC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == MUX_CTRLSELECT_A::GPIO
    }
    #[doc = "Checks if the value of the field is `FUNC1`"]
    #[inline(always)]
    pub fn is_func1(&self) -> bool {
        *self == MUX_CTRLSELECT_A::FUNC1
    }
    #[doc = "Checks if the value of the field is `FUNC2`"]
    #[inline(always)]
    pub fn is_func2(&self) -> bool {
        *self == MUX_CTRLSELECT_A::FUNC2
    }
    #[doc = "Checks if the value of the field is `FUNC3`"]
    #[inline(always)]
    pub fn is_func3(&self) -> bool {
        *self == MUX_CTRLSELECT_A::FUNC3
    }
}
#[doc = "Field `MUX_CTRL` writer - 00 = GPIO Function, 01 = Function 1, 10 = Function 2, 11 = Function 3."]
pub type MUX_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL15_SPEC, u8, MUX_CTRLSELECT_A, 3, O>;
impl<'a, const O: u8> MUX_CTRL_W<'a, O> {
    #[doc = "GPIO function selected"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(MUX_CTRLSELECT_A::GPIO)
    }
    #[doc = "Signal function 1 selected"]
    #[inline(always)]
    pub fn func1(self) -> &'a mut W {
        self.variant(MUX_CTRLSELECT_A::FUNC1)
    }
    #[doc = "Signal function 2 selected"]
    #[inline(always)]
    pub fn func2(self) -> &'a mut W {
        self.variant(MUX_CTRLSELECT_A::FUNC2)
    }
    #[doc = "Signal function 3 selected"]
    #[inline(always)]
    pub fn func3(self) -> &'a mut W {
        self.variant(MUX_CTRLSELECT_A::FUNC3)
    }
}
#[doc = "Field `INP_DIS` reader - GPIO input disable"]
pub type INP_DIS_R = crate::BitReader<bool>;
#[doc = "Field `INP_DIS` writer - GPIO input disable"]
pub type INP_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL15_SPEC, bool, O>;
#[doc = "Field `ALT_GPIO_DATA` reader - GPIO Alternate Data Register."]
pub type ALT_GPIO_DATA_R = crate::BitReader<bool>;
#[doc = "Field `ALT_GPIO_DATA` writer - GPIO Alternate Data Register."]
pub type ALT_GPIO_DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL15_SPEC, bool, O>;
#[doc = "Field `GPIO_INP` reader - GPIO input from pin, independent of the Mux selection for the pin or the Direction."]
pub type GPIO_INP_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_INP` writer - GPIO input from pin, independent of the Mux selection for the pin or the Direction."]
pub type GPIO_INP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL15_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - These bits are used to enable an internal pull-up or pull-down resistor."]
    #[inline(always)]
    pub fn pu_pd(&self) -> PU_PD_R {
        PU_PD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The GPIO pin will be tristated when the selected power well is off."]
    #[inline(always)]
    pub fn pwr_gating(&self) -> PWR_GATING_R {
        PWR_GATING_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Determines the interrupt capability of the GPIO input."]
    #[inline(always)]
    pub fn intr_det(&self) -> INTR_DET_R {
        INTR_DET_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Determines the interrupt capability of the GPIO input."]
    #[inline(always)]
    pub fn edge_en(&self) -> EDGE_EN_R {
        EDGE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0 = Push-Pull, 1 = Open Drain"]
    #[inline(always)]
    pub fn out_buff_type(&self) -> OUT_BUFF_TYPE_R {
        OUT_BUFF_TYPE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Buffer direction when GPIO selected by pin mux 0 = Input, 1 = Output"]
    #[inline(always)]
    pub fn gpio_dir(&self) -> GPIO_DIR_R {
        GPIO_DIR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO outputs registe select.0=GPIO ALTERNATE_GPIO_DATA 1=GPIO Output Register."]
    #[inline(always)]
    pub fn gpio_out_sel(&self) -> GPIO_OUT_SEL_R {
        GPIO_OUT_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1=Inverted; 0=Non-inverted"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - 00 = GPIO Function, 01 = Function 1, 10 = Function 2, 11 = Function 3."]
    #[inline(always)]
    pub fn mux_ctrl(&self) -> MUX_CTRL_R {
        MUX_CTRL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - GPIO input disable"]
    #[inline(always)]
    pub fn inp_dis(&self) -> INP_DIS_R {
        INP_DIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO Alternate Data Register."]
    #[inline(always)]
    pub fn alt_gpio_data(&self) -> ALT_GPIO_DATA_R {
        ALT_GPIO_DATA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO input from pin, independent of the Mux selection for the pin or the Direction."]
    #[inline(always)]
    pub fn gpio_inp(&self) -> GPIO_INP_R {
        GPIO_INP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - These bits are used to enable an internal pull-up or pull-down resistor."]
    #[inline(always)]
    pub fn pu_pd(&mut self) -> PU_PD_W<0> {
        PU_PD_W::new(self)
    }
    #[doc = "Bits 2:3 - The GPIO pin will be tristated when the selected power well is off."]
    #[inline(always)]
    pub fn pwr_gating(&mut self) -> PWR_GATING_W<2> {
        PWR_GATING_W::new(self)
    }
    #[doc = "Bits 4:6 - Determines the interrupt capability of the GPIO input."]
    #[inline(always)]
    pub fn intr_det(&mut self) -> INTR_DET_W<4> {
        INTR_DET_W::new(self)
    }
    #[doc = "Bit 7 - Determines the interrupt capability of the GPIO input."]
    #[inline(always)]
    pub fn edge_en(&mut self) -> EDGE_EN_W<7> {
        EDGE_EN_W::new(self)
    }
    #[doc = "Bit 8 - 0 = Push-Pull, 1 = Open Drain"]
    #[inline(always)]
    pub fn out_buff_type(&mut self) -> OUT_BUFF_TYPE_W<8> {
        OUT_BUFF_TYPE_W::new(self)
    }
    #[doc = "Bit 9 - Buffer direction when GPIO selected by pin mux 0 = Input, 1 = Output"]
    #[inline(always)]
    pub fn gpio_dir(&mut self) -> GPIO_DIR_W<9> {
        GPIO_DIR_W::new(self)
    }
    #[doc = "Bit 10 - GPIO outputs registe select.0=GPIO ALTERNATE_GPIO_DATA 1=GPIO Output Register."]
    #[inline(always)]
    pub fn gpio_out_sel(&mut self) -> GPIO_OUT_SEL_W<10> {
        GPIO_OUT_SEL_W::new(self)
    }
    #[doc = "Bit 11 - 1=Inverted; 0=Non-inverted"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<11> {
        POL_W::new(self)
    }
    #[doc = "Bits 12:14 - 00 = GPIO Function, 01 = Function 1, 10 = Function 2, 11 = Function 3."]
    #[inline(always)]
    pub fn mux_ctrl(&mut self) -> MUX_CTRL_W<12> {
        MUX_CTRL_W::new(self)
    }
    #[doc = "Bit 15 - GPIO input disable"]
    #[inline(always)]
    pub fn inp_dis(&mut self) -> INP_DIS_W<15> {
        INP_DIS_W::new(self)
    }
    #[doc = "Bit 16 - GPIO Alternate Data Register."]
    #[inline(always)]
    pub fn alt_gpio_data(&mut self) -> ALT_GPIO_DATA_W<16> {
        ALT_GPIO_DATA_W::new(self)
    }
    #[doc = "Bit 24 - GPIO input from pin, independent of the Mux selection for the pin or the Direction."]
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
#[doc = "GPIO Pin Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl15](index.html) module"]
pub struct CTRL15_SPEC;
impl crate::RegisterSpec for CTRL15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl15::R](R) reader structure"]
impl crate::Readable for CTRL15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl15::W](W) writer structure"]
impl crate::Writable for CTRL15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL15[%s]
to value 0x8040"]
impl crate::Resettable for CTRL15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8040
    }
}
