#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDATE` reader - Determines the base time between fan driver updates. The Update Time, along with the Fan Step Register, is used to control\n the ramp rate of the drive response to provide a cleaner transition of the actual fan operation as the desired fan speed changes.\n 7=1600ms\n 6=1200ms\n 5=800ms\n 4=500ms\n 3=400ms\n 2=300ms\n 1=200ms\n 0=100ms\n Note: This ramp rate control applies for all changes to the active PWM output including when the RPM based Fan Speed Control Algorithm\n is disabled."]
pub type UPDATE_R = crate::FieldReader<u8, UPDATESELECT_A>;
#[doc = "Determines the base time between fan driver updates. The Update Time, along with the Fan Step Register, is used to control\n the ramp rate of the drive response to provide a cleaner transition of the actual fan operation as the desired fan speed changes.\n 7=1600ms\n 6=1200ms\n 5=800ms\n 4=500ms\n 3=400ms\n 2=300ms\n 1=200ms\n 0=100ms\n Note: This ramp rate control applies for all changes to the active PWM output including when the RPM based Fan Speed Control Algorithm\n is disabled.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UPDATESELECT_A {
    #[doc = "7: 7=1600ms"]
    BASE_TIME_1600MS = 7,
    #[doc = "6: 6=1200ms"]
    BASE_TIME_1200MS = 6,
    #[doc = "6: 5=800ms"]
    BASE_TIME_800MS = 6,
    #[doc = "5: 4=500ms"]
    BASE_TIME_500MS = 5,
    #[doc = "4: 3=400ms"]
    BASE_TIME_400MS = 4,
    #[doc = "3: 2=300ms"]
    BASE_TIME_300MS = 3,
    #[doc = "2: 1=200ms"]
    BASE_TIME_200MS = 2,
    #[doc = "1: 0=100ms"]
    BASE_TIME_100MS = 1,
}
impl From<UPDATESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: UPDATESELECT_A) -> Self {
        variant as _
    }
}
impl UPDATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDATESELECT_A {
        match self.bits {
            7 => UPDATESELECT_A::BASE_TIME_1600MS,
            6 => UPDATESELECT_A::BASE_TIME_1200MS,
            6 => UPDATESELECT_A::BASE_TIME_800MS,
            5 => UPDATESELECT_A::BASE_TIME_500MS,
            4 => UPDATESELECT_A::BASE_TIME_400MS,
            3 => UPDATESELECT_A::BASE_TIME_300MS,
            2 => UPDATESELECT_A::BASE_TIME_200MS,
            1 => UPDATESELECT_A::BASE_TIME_100MS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BASE_TIME_1600MS`"]
    #[inline(always)]
    pub fn is_base_time_1600ms(&self) -> bool {
        *self == UPDATESELECT_A::BASE_TIME_1600MS
    }
    #[doc = "Checks if the value of the field is `BASE_TIME_1200MS`"]
    #[inline(always)]
    pub fn is_base_time_1200ms(&self) -> bool {
        *self == UPDATESELECT_A::BASE_TIME_1200MS
    }
    #[doc = "Checks if the value of the field is `BASE_TIME_800MS`"]
    #[inline(always)]
    pub fn is_base_time_800ms(&self) -> bool {
        *self == UPDATESELECT_A::BASE_TIME_800MS
    }
    #[doc = "Checks if the value of the field is `BASE_TIME_500MS`"]
    #[inline(always)]
    pub fn is_base_time_500ms(&self) -> bool {
        *self == UPDATESELECT_A::BASE_TIME_500MS
    }
    #[doc = "Checks if the value of the field is `BASE_TIME_400MS`"]
    #[inline(always)]
    pub fn is_base_time_400ms(&self) -> bool {
        *self == UPDATESELECT_A::BASE_TIME_400MS
    }
    #[doc = "Checks if the value of the field is `BASE_TIME_300MS`"]
    #[inline(always)]
    pub fn is_base_time_300ms(&self) -> bool {
        *self == UPDATESELECT_A::BASE_TIME_300MS
    }
    #[doc = "Checks if the value of the field is `BASE_TIME_200MS`"]
    #[inline(always)]
    pub fn is_base_time_200ms(&self) -> bool {
        *self == UPDATESELECT_A::BASE_TIME_200MS
    }
    #[doc = "Checks if the value of the field is `BASE_TIME_100MS`"]
    #[inline(always)]
    pub fn is_base_time_100ms(&self) -> bool {
        *self == UPDATESELECT_A::BASE_TIME_100MS
    }
}
#[doc = "Field `UPDATE` writer - Determines the base time between fan driver updates. The Update Time, along with the Fan Step Register, is used to control\n the ramp rate of the drive response to provide a cleaner transition of the actual fan operation as the desired fan speed changes.\n 7=1600ms\n 6=1200ms\n 5=800ms\n 4=500ms\n 3=400ms\n 2=300ms\n 1=200ms\n 0=100ms\n Note: This ramp rate control applies for all changes to the active PWM output including when the RPM based Fan Speed Control Algorithm\n is disabled."]
pub type UPDATE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CFG_SPEC, u8, UPDATESELECT_A, 3, O>;
impl<'a, const O: u8> UPDATE_W<'a, O> {
    #[doc = "7=1600ms"]
    #[inline(always)]
    pub fn base_time_1600ms(self) -> &'a mut W {
        self.variant(UPDATESELECT_A::BASE_TIME_1600MS)
    }
    #[doc = "6=1200ms"]
    #[inline(always)]
    pub fn base_time_1200ms(self) -> &'a mut W {
        self.variant(UPDATESELECT_A::BASE_TIME_1200MS)
    }
    #[doc = "5=800ms"]
    #[inline(always)]
    pub fn base_time_800ms(self) -> &'a mut W {
        self.variant(UPDATESELECT_A::BASE_TIME_800MS)
    }
    #[doc = "4=500ms"]
    #[inline(always)]
    pub fn base_time_500ms(self) -> &'a mut W {
        self.variant(UPDATESELECT_A::BASE_TIME_500MS)
    }
    #[doc = "3=400ms"]
    #[inline(always)]
    pub fn base_time_400ms(self) -> &'a mut W {
        self.variant(UPDATESELECT_A::BASE_TIME_400MS)
    }
    #[doc = "2=300ms"]
    #[inline(always)]
    pub fn base_time_300ms(self) -> &'a mut W {
        self.variant(UPDATESELECT_A::BASE_TIME_300MS)
    }
    #[doc = "1=200ms"]
    #[inline(always)]
    pub fn base_time_200ms(self) -> &'a mut W {
        self.variant(UPDATESELECT_A::BASE_TIME_200MS)
    }
    #[doc = "0=100ms"]
    #[inline(always)]
    pub fn base_time_100ms(self) -> &'a mut W {
        self.variant(UPDATESELECT_A::BASE_TIME_100MS)
    }
}
#[doc = "Field `EDGES` reader - Determines the minimum number of edges that must be detected on the TACH signal to determine a single rotation. A typical\n fan measured 5 edges (for a 2-pole fan). Increasing the number of edges measured with respect to the number of poles of the fan will\n cause the TACH Reading registers to indicate a fan speed that is higher or lower than the actual speed. In order for the FSC Algorithm\n to operate correctly, the TACH Target must be updated by the user to accommodate this shift. The Effective Tach Multiplier is used as\n a direct multiplier term that is applied to the Actual RPM to achieve the Reported RPM. It should only be applied if the number of\n edges measured does not match the number of edges expected based on the number of poles of the fan (which is fixed for any given fan)."]
pub type EDGES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDGES` writer - Determines the minimum number of edges that must be detected on the TACH signal to determine a single rotation. A typical\n fan measured 5 edges (for a 2-pole fan). Increasing the number of edges measured with respect to the number of poles of the fan will\n cause the TACH Reading registers to indicate a fan speed that is higher or lower than the actual speed. In order for the FSC Algorithm\n to operate correctly, the TACH Target must be updated by the user to accommodate this shift. The Effective Tach Multiplier is used as\n a direct multiplier term that is applied to the Actual RPM to achieve the Reported RPM. It should only be applied if the number of\n edges measured does not match the number of edges expected based on the number of poles of the fan (which is fixed for any given fan)."]
pub type EDGES_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `RANGE` reader - Adjusts the range of reported and programmed tachometer reading values. The RANGE bits determine the weighting of all\n TACH values (including the Valid TACH Count, TACH Target, and TACH reading).\n 3=Reported Minimum RPM: 4000. Tach Count Multiplier: 1\n 2=Reported Minimum RPM: 2000. Tach Count Multiplier: 2\n 1=Reported Minimum RPM: 1000. Tach Count Multiplier: 4\n 0=Reported Minimum RPM: 500. Tach Count Multiplier: 8"]
pub type RANGE_R = crate::FieldReader<u8, RANGESELECT_A>;
#[doc = "Adjusts the range of reported and programmed tachometer reading values. The RANGE bits determine the weighting of all\n TACH values (including the Valid TACH Count, TACH Target, and TACH reading).\n 3=Reported Minimum RPM: 4000. Tach Count Multiplier: 1\n 2=Reported Minimum RPM: 2000. Tach Count Multiplier: 2\n 1=Reported Minimum RPM: 1000. Tach Count Multiplier: 4\n 0=Reported Minimum RPM: 500. Tach Count Multiplier: 8\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RANGESELECT_A {
    #[doc = "3: 3=Reported Minimum RPM: 4000. Tach Count Multiplier: 8"]
    TACH_COUNT_MULTIPLIER_8 = 3,
    #[doc = "2: 2=Reported Minimum RPM: 2000. Tach Count Multiplier: 4"]
    TACH_COUNT_MULTIPLIER_4 = 2,
    #[doc = "1: 1=Reported Minimum RPM: 1000. Tach Count Multiplier: 2"]
    TACH_COUNT_MULTIPLIER_2 = 1,
    #[doc = "0: 0=Reported Minimum RPM: 500. Tach Count Multiplier: 1"]
    TACH_COUNT_MULTIPLIER_1 = 0,
}
impl From<RANGESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGESELECT_A) -> Self {
        variant as _
    }
}
impl RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RANGESELECT_A {
        match self.bits {
            3 => RANGESELECT_A::TACH_COUNT_MULTIPLIER_8,
            2 => RANGESELECT_A::TACH_COUNT_MULTIPLIER_4,
            1 => RANGESELECT_A::TACH_COUNT_MULTIPLIER_2,
            0 => RANGESELECT_A::TACH_COUNT_MULTIPLIER_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TACH_COUNT_MULTIPLIER_8`"]
    #[inline(always)]
    pub fn is_tach_count_multiplier_8(&self) -> bool {
        *self == RANGESELECT_A::TACH_COUNT_MULTIPLIER_8
    }
    #[doc = "Checks if the value of the field is `TACH_COUNT_MULTIPLIER_4`"]
    #[inline(always)]
    pub fn is_tach_count_multiplier_4(&self) -> bool {
        *self == RANGESELECT_A::TACH_COUNT_MULTIPLIER_4
    }
    #[doc = "Checks if the value of the field is `TACH_COUNT_MULTIPLIER_2`"]
    #[inline(always)]
    pub fn is_tach_count_multiplier_2(&self) -> bool {
        *self == RANGESELECT_A::TACH_COUNT_MULTIPLIER_2
    }
    #[doc = "Checks if the value of the field is `TACH_COUNT_MULTIPLIER_1`"]
    #[inline(always)]
    pub fn is_tach_count_multiplier_1(&self) -> bool {
        *self == RANGESELECT_A::TACH_COUNT_MULTIPLIER_1
    }
}
#[doc = "Field `RANGE` writer - Adjusts the range of reported and programmed tachometer reading values. The RANGE bits determine the weighting of all\n TACH values (including the Valid TACH Count, TACH Target, and TACH reading).\n 3=Reported Minimum RPM: 4000. Tach Count Multiplier: 1\n 2=Reported Minimum RPM: 2000. Tach Count Multiplier: 2\n 1=Reported Minimum RPM: 1000. Tach Count Multiplier: 4\n 0=Reported Minimum RPM: 500. Tach Count Multiplier: 8"]
pub type RANGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CFG_SPEC, u8, RANGESELECT_A, 2, O>;
impl<'a, const O: u8> RANGE_W<'a, O> {
    #[doc = "3=Reported Minimum RPM: 4000. Tach Count Multiplier: 8"]
    #[inline(always)]
    pub fn tach_count_multiplier_8(self) -> &'a mut W {
        self.variant(RANGESELECT_A::TACH_COUNT_MULTIPLIER_8)
    }
    #[doc = "2=Reported Minimum RPM: 2000. Tach Count Multiplier: 4"]
    #[inline(always)]
    pub fn tach_count_multiplier_4(self) -> &'a mut W {
        self.variant(RANGESELECT_A::TACH_COUNT_MULTIPLIER_4)
    }
    #[doc = "1=Reported Minimum RPM: 1000. Tach Count Multiplier: 2"]
    #[inline(always)]
    pub fn tach_count_multiplier_2(self) -> &'a mut W {
        self.variant(RANGESELECT_A::TACH_COUNT_MULTIPLIER_2)
    }
    #[doc = "0=Reported Minimum RPM: 500. Tach Count Multiplier: 1"]
    #[inline(always)]
    pub fn tach_count_multiplier_1(self) -> &'a mut W {
        self.variant(RANGESELECT_A::TACH_COUNT_MULTIPLIER_1)
    }
}
#[doc = "Field `EN_ALGO` reader - Enables the RPM based Fan Control Algorithm.\n 1=The control circuitry is enabled and the Fan Driver output will be automatically updated to maintain the programmed fan speed\n as indicated by the TACH Target Register.\n 0=The control circuitry is disabled and the fan driver output is determined by the Fan Driver Setting Register."]
pub type EN_ALGO_R = crate::BitReader<bool>;
#[doc = "Field `EN_ALGO` writer - Enables the RPM based Fan Control Algorithm.\n 1=The control circuitry is enabled and the Fan Driver output will be automatically updated to maintain the programmed fan speed\n as indicated by the TACH Target Register.\n 0=The control circuitry is disabled and the fan driver output is determined by the Fan Driver Setting Register."]
pub type EN_ALGO_W<'a, const O: u8> = crate::BitWriter<'a, u16, CFG_SPEC, bool, O>;
#[doc = "Field `POLARITY` reader - Determines the polarity of the PWM driver. This does NOT affect the drive setting registers. A setting of 0% drive will\n still correspond to 0% drive independent of the polarity.\n 1 - The Polarity of the PWM driver is inverted. A drive setting of 00h will cause the output to be set at 100% duty cycle and\n a drive setting of FFh will cause the output to be set at 0% duty cycle.\n 0 - the Polarity of the PWM driver is normal. A drive setting of 00h will cause the output to be set at 0% duty cycle and\n a drive setting of FFh will cause the output to be set at 100% duty cycle."]
pub type POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `POLARITY` writer - Determines the polarity of the PWM driver. This does NOT affect the drive setting registers. A setting of 0% drive will\n still correspond to 0% drive independent of the polarity.\n 1 - The Polarity of the PWM driver is inverted. A drive setting of 00h will cause the output to be set at 100% duty cycle and\n a drive setting of FFh will cause the output to be set at 0% duty cycle.\n 0 - the Polarity of the PWM driver is normal. A drive setting of 00h will cause the output to be set at 0% duty cycle and\n a drive setting of FFh will cause the output to be set at 100% duty cycle."]
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u16, CFG_SPEC, bool, O>;
#[doc = "Field `ERR_RNG` reader - Control some of the advanced options that affect the error window. When the measured fan speed is within the programmed\n error window around the target speed, the fan drive setting is not updated. These bits only apply if the Fan Speed Control Algorithm is used.\n 3=200 RPM\n 2=100 RPM\n 1=50 RPM\n 0=0 RPM"]
pub type ERR_RNG_R = crate::FieldReader<u8, ERR_RNGSELECT_A>;
#[doc = "Control some of the advanced options that affect the error window. When the measured fan speed is within the programmed\n error window around the target speed, the fan drive setting is not updated. These bits only apply if the Fan Speed Control Algorithm is used.\n 3=200 RPM\n 2=100 RPM\n 1=50 RPM\n 0=0 RPM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ERR_RNGSELECT_A {
    #[doc = "3: 3=200 RPM"]
    RPM_200 = 3,
    #[doc = "2: 2=100 RPM"]
    RPM_100 = 2,
    #[doc = "1: 1=50 RPM"]
    RPM_50 = 1,
    #[doc = "0: 0=0 RPM"]
    RPM_0 = 0,
}
impl From<ERR_RNGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ERR_RNGSELECT_A) -> Self {
        variant as _
    }
}
impl ERR_RNG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_RNGSELECT_A {
        match self.bits {
            3 => ERR_RNGSELECT_A::RPM_200,
            2 => ERR_RNGSELECT_A::RPM_100,
            1 => ERR_RNGSELECT_A::RPM_50,
            0 => ERR_RNGSELECT_A::RPM_0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RPM_200`"]
    #[inline(always)]
    pub fn is_rpm_200(&self) -> bool {
        *self == ERR_RNGSELECT_A::RPM_200
    }
    #[doc = "Checks if the value of the field is `RPM_100`"]
    #[inline(always)]
    pub fn is_rpm_100(&self) -> bool {
        *self == ERR_RNGSELECT_A::RPM_100
    }
    #[doc = "Checks if the value of the field is `RPM_50`"]
    #[inline(always)]
    pub fn is_rpm_50(&self) -> bool {
        *self == ERR_RNGSELECT_A::RPM_50
    }
    #[doc = "Checks if the value of the field is `RPM_0`"]
    #[inline(always)]
    pub fn is_rpm_0(&self) -> bool {
        *self == ERR_RNGSELECT_A::RPM_0
    }
}
#[doc = "Field `ERR_RNG` writer - Control some of the advanced options that affect the error window. When the measured fan speed is within the programmed\n error window around the target speed, the fan drive setting is not updated. These bits only apply if the Fan Speed Control Algorithm is used.\n 3=200 RPM\n 2=100 RPM\n 1=50 RPM\n 0=0 RPM"]
pub type ERR_RNG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CFG_SPEC, u8, ERR_RNGSELECT_A, 2, O>;
impl<'a, const O: u8> ERR_RNG_W<'a, O> {
    #[doc = "3=200 RPM"]
    #[inline(always)]
    pub fn rpm_200(self) -> &'a mut W {
        self.variant(ERR_RNGSELECT_A::RPM_200)
    }
    #[doc = "2=100 RPM"]
    #[inline(always)]
    pub fn rpm_100(self) -> &'a mut W {
        self.variant(ERR_RNGSELECT_A::RPM_100)
    }
    #[doc = "1=50 RPM"]
    #[inline(always)]
    pub fn rpm_50(self) -> &'a mut W {
        self.variant(ERR_RNGSELECT_A::RPM_50)
    }
    #[doc = "0=0 RPM"]
    #[inline(always)]
    pub fn rpm_0(self) -> &'a mut W {
        self.variant(ERR_RNGSELECT_A::RPM_0)
    }
}
#[doc = "Field `DER_OPT` reader - Control some of the advanced options that affect the derivative portion of the RPM based fan control algorithm.\n These bits only apply if the Fan Speed Control Algorithm is used."]
pub type DER_OPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DER_OPT` writer - Control some of the advanced options that affect the derivative portion of the RPM based fan control algorithm.\n These bits only apply if the Fan Speed Control Algorithm is used."]
pub type DER_OPT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `DIS_GLITCH` reader - Disables the low pass glitch filter that removes high frequency noise injected on the TACH pin.\n 1 - The glitch filter is disabled.\n 0 - The glitch filter is enabled."]
pub type DIS_GLITCH_R = crate::BitReader<bool>;
#[doc = "Field `DIS_GLITCH` writer - Disables the low pass glitch filter that removes high frequency noise injected on the TACH pin.\n 1 - The glitch filter is disabled.\n 0 - The glitch filter is enabled."]
pub type DIS_GLITCH_W<'a, const O: u8> = crate::BitWriter<'a, u16, CFG_SPEC, bool, O>;
#[doc = "Field `EN_RRC` reader - Enables the ramp rate control circuitry during the Manual Mode of operation.\n 1=The ramp rate control circuitry for the Manual Mode of operation is enabled. The PWM setting will follow the ramp rate controls\n as determined by the Fan Step and Update Time settings. The maximum PWM step is capped at the Fan Step setting and is updated\n based on the Update Time as given by the field UPDATE.\n 0=The ramp rate control circuitry for the Manual Mode of operation is disabled. When the Fan Drive Setting values are changed\n and the RPM based Fan Control Algorithm is disabled, the fan driver will be set to the new setting immediately."]
pub type EN_RRC_R = crate::BitReader<bool>;
#[doc = "Field `EN_RRC` writer - Enables the ramp rate control circuitry during the Manual Mode of operation.\n 1=The ramp rate control circuitry for the Manual Mode of operation is enabled. The PWM setting will follow the ramp rate controls\n as determined by the Fan Step and Update Time settings. The maximum PWM step is capped at the Fan Step setting and is updated\n based on the Update Time as given by the field UPDATE.\n 0=The ramp rate control circuitry for the Manual Mode of operation is disabled. When the Fan Drive Setting values are changed\n and the RPM based Fan Control Algorithm is disabled, the fan driver will be set to the new setting immediately."]
pub type EN_RRC_W<'a, const O: u8> = crate::BitWriter<'a, u16, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Determines the base time between fan driver updates. The Update Time, along with the Fan Step Register, is used to control\n the ramp rate of the drive response to provide a cleaner transition of the actual fan operation as the desired fan speed changes.\n 7=1600ms\n 6=1200ms\n 5=800ms\n 4=500ms\n 3=400ms\n 2=300ms\n 1=200ms\n 0=100ms\n Note: This ramp rate control applies for all changes to the active PWM output including when the RPM based Fan Speed Control Algorithm\n is disabled."]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Determines the minimum number of edges that must be detected on the TACH signal to determine a single rotation. A typical\n fan measured 5 edges (for a 2-pole fan). Increasing the number of edges measured with respect to the number of poles of the fan will\n cause the TACH Reading registers to indicate a fan speed that is higher or lower than the actual speed. In order for the FSC Algorithm\n to operate correctly, the TACH Target must be updated by the user to accommodate this shift. The Effective Tach Multiplier is used as\n a direct multiplier term that is applied to the Actual RPM to achieve the Reported RPM. It should only be applied if the number of\n edges measured does not match the number of edges expected based on the number of poles of the fan (which is fixed for any given fan)."]
    #[inline(always)]
    pub fn edges(&self) -> EDGES_R {
        EDGES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Adjusts the range of reported and programmed tachometer reading values. The RANGE bits determine the weighting of all\n TACH values (including the Valid TACH Count, TACH Target, and TACH reading).\n 3=Reported Minimum RPM: 4000. Tach Count Multiplier: 1\n 2=Reported Minimum RPM: 2000. Tach Count Multiplier: 2\n 1=Reported Minimum RPM: 1000. Tach Count Multiplier: 4\n 0=Reported Minimum RPM: 500. Tach Count Multiplier: 8"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Enables the RPM based Fan Control Algorithm.\n 1=The control circuitry is enabled and the Fan Driver output will be automatically updated to maintain the programmed fan speed\n as indicated by the TACH Target Register.\n 0=The control circuitry is disabled and the fan driver output is determined by the Fan Driver Setting Register."]
    #[inline(always)]
    pub fn en_algo(&self) -> EN_ALGO_R {
        EN_ALGO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Determines the polarity of the PWM driver. This does NOT affect the drive setting registers. A setting of 0% drive will\n still correspond to 0% drive independent of the polarity.\n 1 - The Polarity of the PWM driver is inverted. A drive setting of 00h will cause the output to be set at 100% duty cycle and\n a drive setting of FFh will cause the output to be set at 0% duty cycle.\n 0 - the Polarity of the PWM driver is normal. A drive setting of 00h will cause the output to be set at 0% duty cycle and\n a drive setting of FFh will cause the output to be set at 100% duty cycle."]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Control some of the advanced options that affect the error window. When the measured fan speed is within the programmed\n error window around the target speed, the fan drive setting is not updated. These bits only apply if the Fan Speed Control Algorithm is used.\n 3=200 RPM\n 2=100 RPM\n 1=50 RPM\n 0=0 RPM"]
    #[inline(always)]
    pub fn err_rng(&self) -> ERR_RNG_R {
        ERR_RNG_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Control some of the advanced options that affect the derivative portion of the RPM based fan control algorithm.\n These bits only apply if the Fan Speed Control Algorithm is used."]
    #[inline(always)]
    pub fn der_opt(&self) -> DER_OPT_R {
        DER_OPT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Disables the low pass glitch filter that removes high frequency noise injected on the TACH pin.\n 1 - The glitch filter is disabled.\n 0 - The glitch filter is enabled."]
    #[inline(always)]
    pub fn dis_glitch(&self) -> DIS_GLITCH_R {
        DIS_GLITCH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables the ramp rate control circuitry during the Manual Mode of operation.\n 1=The ramp rate control circuitry for the Manual Mode of operation is enabled. The PWM setting will follow the ramp rate controls\n as determined by the Fan Step and Update Time settings. The maximum PWM step is capped at the Fan Step setting and is updated\n based on the Update Time as given by the field UPDATE.\n 0=The ramp rate control circuitry for the Manual Mode of operation is disabled. When the Fan Drive Setting values are changed\n and the RPM based Fan Control Algorithm is disabled, the fan driver will be set to the new setting immediately."]
    #[inline(always)]
    pub fn en_rrc(&self) -> EN_RRC_R {
        EN_RRC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Determines the base time between fan driver updates. The Update Time, along with the Fan Step Register, is used to control\n the ramp rate of the drive response to provide a cleaner transition of the actual fan operation as the desired fan speed changes.\n 7=1600ms\n 6=1200ms\n 5=800ms\n 4=500ms\n 3=400ms\n 2=300ms\n 1=200ms\n 0=100ms\n Note: This ramp rate control applies for all changes to the active PWM output including when the RPM based Fan Speed Control Algorithm\n is disabled."]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W<0> {
        UPDATE_W::new(self)
    }
    #[doc = "Bits 3:4 - Determines the minimum number of edges that must be detected on the TACH signal to determine a single rotation. A typical\n fan measured 5 edges (for a 2-pole fan). Increasing the number of edges measured with respect to the number of poles of the fan will\n cause the TACH Reading registers to indicate a fan speed that is higher or lower than the actual speed. In order for the FSC Algorithm\n to operate correctly, the TACH Target must be updated by the user to accommodate this shift. The Effective Tach Multiplier is used as\n a direct multiplier term that is applied to the Actual RPM to achieve the Reported RPM. It should only be applied if the number of\n edges measured does not match the number of edges expected based on the number of poles of the fan (which is fixed for any given fan)."]
    #[inline(always)]
    pub fn edges(&mut self) -> EDGES_W<3> {
        EDGES_W::new(self)
    }
    #[doc = "Bits 5:6 - Adjusts the range of reported and programmed tachometer reading values. The RANGE bits determine the weighting of all\n TACH values (including the Valid TACH Count, TACH Target, and TACH reading).\n 3=Reported Minimum RPM: 4000. Tach Count Multiplier: 1\n 2=Reported Minimum RPM: 2000. Tach Count Multiplier: 2\n 1=Reported Minimum RPM: 1000. Tach Count Multiplier: 4\n 0=Reported Minimum RPM: 500. Tach Count Multiplier: 8"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W<5> {
        RANGE_W::new(self)
    }
    #[doc = "Bit 7 - Enables the RPM based Fan Control Algorithm.\n 1=The control circuitry is enabled and the Fan Driver output will be automatically updated to maintain the programmed fan speed\n as indicated by the TACH Target Register.\n 0=The control circuitry is disabled and the fan driver output is determined by the Fan Driver Setting Register."]
    #[inline(always)]
    pub fn en_algo(&mut self) -> EN_ALGO_W<7> {
        EN_ALGO_W::new(self)
    }
    #[doc = "Bit 9 - Determines the polarity of the PWM driver. This does NOT affect the drive setting registers. A setting of 0% drive will\n still correspond to 0% drive independent of the polarity.\n 1 - The Polarity of the PWM driver is inverted. A drive setting of 00h will cause the output to be set at 100% duty cycle and\n a drive setting of FFh will cause the output to be set at 0% duty cycle.\n 0 - the Polarity of the PWM driver is normal. A drive setting of 00h will cause the output to be set at 0% duty cycle and\n a drive setting of FFh will cause the output to be set at 100% duty cycle."]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W<9> {
        POLARITY_W::new(self)
    }
    #[doc = "Bits 10:11 - Control some of the advanced options that affect the error window. When the measured fan speed is within the programmed\n error window around the target speed, the fan drive setting is not updated. These bits only apply if the Fan Speed Control Algorithm is used.\n 3=200 RPM\n 2=100 RPM\n 1=50 RPM\n 0=0 RPM"]
    #[inline(always)]
    pub fn err_rng(&mut self) -> ERR_RNG_W<10> {
        ERR_RNG_W::new(self)
    }
    #[doc = "Bits 12:13 - Control some of the advanced options that affect the derivative portion of the RPM based fan control algorithm.\n These bits only apply if the Fan Speed Control Algorithm is used."]
    #[inline(always)]
    pub fn der_opt(&mut self) -> DER_OPT_W<12> {
        DER_OPT_W::new(self)
    }
    #[doc = "Bit 14 - Disables the low pass glitch filter that removes high frequency noise injected on the TACH pin.\n 1 - The glitch filter is disabled.\n 0 - The glitch filter is enabled."]
    #[inline(always)]
    pub fn dis_glitch(&mut self) -> DIS_GLITCH_W<14> {
        DIS_GLITCH_W::new(self)
    }
    #[doc = "Bit 15 - Enables the ramp rate control circuitry during the Manual Mode of operation.\n 1=The ramp rate control circuitry for the Manual Mode of operation is enabled. The PWM setting will follow the ramp rate controls\n as determined by the Fan Step and Update Time settings. The maximum PWM step is capped at the Fan Step setting and is updated\n based on the Update Time as given by the field UPDATE.\n 0=The ramp rate control circuitry for the Manual Mode of operation is disabled. When the Fan Drive Setting values are changed\n and the RPM based Fan Control Algorithm is disabled, the fan driver will be set to the new setting immediately."]
    #[inline(always)]
    pub fn en_rrc(&mut self) -> EN_RRC_W<15> {
        EN_RRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Fan Configuration Register controls the general operation of the RPM based Fan Control Algorithm used by the fan driver.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0x342b"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x342b
    }
}
