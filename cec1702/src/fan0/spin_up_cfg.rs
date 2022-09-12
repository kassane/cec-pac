#[doc = "Register `SPIN_UP_CFG` reader"]
pub struct R(crate::R<SPIN_UP_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIN_UP_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIN_UP_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIN_UP_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIN_UP_CFG` writer"]
pub struct W(crate::W<SPIN_UP_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIN_UP_CFG_SPEC>;
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
impl From<crate::W<SPIN_UP_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIN_UP_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPINUP_TIME` reader - Determines the maximum Spin Time that the Spin Up Routine will run for. If a valid tachometer measurement is not\n detected before the Spin Time has elapsed, an interrupt will be generated. When the RPM based Fan Control Algorithm is active,\n the fan driver will attempt to re-start the fan immediately after the end of the last spin up attempt.\n 3=2 seconds\n 2=1 second\n 1=500 ms\n 0=250 ms"]
pub type SPINUP_TIME_R = crate::FieldReader<u8, SPINUP_TIMESELECT_A>;
#[doc = "Determines the maximum Spin Time that the Spin Up Routine will run for. If a valid tachometer measurement is not\n detected before the Spin Time has elapsed, an interrupt will be generated. When the RPM based Fan Control Algorithm is active,\n the fan driver will attempt to re-start the fan immediately after the end of the last spin up attempt.\n 3=2 seconds\n 2=1 second\n 1=500 ms\n 0=250 ms\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPINUP_TIMESELECT_A {
    #[doc = "3: 3=2 seconds"]
    SPINUP_2_SEC = 3,
    #[doc = "2: 2=1 second"]
    SPINUP_1_SEC = 2,
    #[doc = "1: 1=500 ms"]
    SPINUP_500_MSEC = 1,
    #[doc = "0: 0=250 ms"]
    SPINUP_250_MSEC = 0,
}
impl From<SPINUP_TIMESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SPINUP_TIMESELECT_A) -> Self {
        variant as _
    }
}
impl SPINUP_TIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPINUP_TIMESELECT_A {
        match self.bits {
            3 => SPINUP_TIMESELECT_A::SPINUP_2_SEC,
            2 => SPINUP_TIMESELECT_A::SPINUP_1_SEC,
            1 => SPINUP_TIMESELECT_A::SPINUP_500_MSEC,
            0 => SPINUP_TIMESELECT_A::SPINUP_250_MSEC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPINUP_2_SEC`"]
    #[inline(always)]
    pub fn is_spinup_2_sec(&self) -> bool {
        *self == SPINUP_TIMESELECT_A::SPINUP_2_SEC
    }
    #[doc = "Checks if the value of the field is `SPINUP_1_SEC`"]
    #[inline(always)]
    pub fn is_spinup_1_sec(&self) -> bool {
        *self == SPINUP_TIMESELECT_A::SPINUP_1_SEC
    }
    #[doc = "Checks if the value of the field is `SPINUP_500_MSEC`"]
    #[inline(always)]
    pub fn is_spinup_500_msec(&self) -> bool {
        *self == SPINUP_TIMESELECT_A::SPINUP_500_MSEC
    }
    #[doc = "Checks if the value of the field is `SPINUP_250_MSEC`"]
    #[inline(always)]
    pub fn is_spinup_250_msec(&self) -> bool {
        *self == SPINUP_TIMESELECT_A::SPINUP_250_MSEC
    }
}
#[doc = "Field `SPINUP_TIME` writer - Determines the maximum Spin Time that the Spin Up Routine will run for. If a valid tachometer measurement is not\n detected before the Spin Time has elapsed, an interrupt will be generated. When the RPM based Fan Control Algorithm is active,\n the fan driver will attempt to re-start the fan immediately after the end of the last spin up attempt.\n 3=2 seconds\n 2=1 second\n 1=500 ms\n 0=250 ms"]
pub type SPINUP_TIME_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, SPIN_UP_CFG_SPEC, u8, SPINUP_TIMESELECT_A, 2, O>;
impl<'a, const O: u8> SPINUP_TIME_W<'a, O> {
    #[doc = "3=2 seconds"]
    #[inline(always)]
    pub fn spinup_2_sec(self) -> &'a mut W {
        self.variant(SPINUP_TIMESELECT_A::SPINUP_2_SEC)
    }
    #[doc = "2=1 second"]
    #[inline(always)]
    pub fn spinup_1_sec(self) -> &'a mut W {
        self.variant(SPINUP_TIMESELECT_A::SPINUP_1_SEC)
    }
    #[doc = "1=500 ms"]
    #[inline(always)]
    pub fn spinup_500_msec(self) -> &'a mut W {
        self.variant(SPINUP_TIMESELECT_A::SPINUP_500_MSEC)
    }
    #[doc = "0=250 ms"]
    #[inline(always)]
    pub fn spinup_250_msec(self) -> &'a mut W {
        self.variant(SPINUP_TIMESELECT_A::SPINUP_250_MSEC)
    }
}
#[doc = "Field `SPIN_LVL` reader - Determines the final drive level that is used by the Spin Up Routine.\n 7=65%\n 6=60%\n 5=55%\n 4=50%\n 3=45%\n 2=40%\n 1=35%\n 0=30%"]
pub type SPIN_LVL_R = crate::FieldReader<u8, SPIN_LVLSELECT_A>;
#[doc = "Determines the final drive level that is used by the Spin Up Routine.\n 7=65%\n 6=60%\n 5=55%\n 4=50%\n 3=45%\n 2=40%\n 1=35%\n 0=30%\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPIN_LVLSELECT_A {
    #[doc = "7: 7=65%"]
    SPIN_LVL_65 = 7,
    #[doc = "6: 6=60%"]
    SPIN_LVL_60 = 6,
    #[doc = "5: 5=55%"]
    SPIN_LVL_55 = 5,
    #[doc = "4: 4=50%"]
    SPIN_LVL_50 = 4,
    #[doc = "3: 3=45%"]
    SPIN_LVL_45 = 3,
    #[doc = "2: 2=40%"]
    SPIN_LVL_40 = 2,
    #[doc = "1: 1=35%"]
    SPIN_LVL_35 = 1,
    #[doc = "0: 0=30%"]
    SPIN_LVL_30 = 0,
}
impl From<SPIN_LVLSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SPIN_LVLSELECT_A) -> Self {
        variant as _
    }
}
impl SPIN_LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIN_LVLSELECT_A {
        match self.bits {
            7 => SPIN_LVLSELECT_A::SPIN_LVL_65,
            6 => SPIN_LVLSELECT_A::SPIN_LVL_60,
            5 => SPIN_LVLSELECT_A::SPIN_LVL_55,
            4 => SPIN_LVLSELECT_A::SPIN_LVL_50,
            3 => SPIN_LVLSELECT_A::SPIN_LVL_45,
            2 => SPIN_LVLSELECT_A::SPIN_LVL_40,
            1 => SPIN_LVLSELECT_A::SPIN_LVL_35,
            0 => SPIN_LVLSELECT_A::SPIN_LVL_30,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPIN_LVL_65`"]
    #[inline(always)]
    pub fn is_spin_lvl_65(&self) -> bool {
        *self == SPIN_LVLSELECT_A::SPIN_LVL_65
    }
    #[doc = "Checks if the value of the field is `SPIN_LVL_60`"]
    #[inline(always)]
    pub fn is_spin_lvl_60(&self) -> bool {
        *self == SPIN_LVLSELECT_A::SPIN_LVL_60
    }
    #[doc = "Checks if the value of the field is `SPIN_LVL_55`"]
    #[inline(always)]
    pub fn is_spin_lvl_55(&self) -> bool {
        *self == SPIN_LVLSELECT_A::SPIN_LVL_55
    }
    #[doc = "Checks if the value of the field is `SPIN_LVL_50`"]
    #[inline(always)]
    pub fn is_spin_lvl_50(&self) -> bool {
        *self == SPIN_LVLSELECT_A::SPIN_LVL_50
    }
    #[doc = "Checks if the value of the field is `SPIN_LVL_45`"]
    #[inline(always)]
    pub fn is_spin_lvl_45(&self) -> bool {
        *self == SPIN_LVLSELECT_A::SPIN_LVL_45
    }
    #[doc = "Checks if the value of the field is `SPIN_LVL_40`"]
    #[inline(always)]
    pub fn is_spin_lvl_40(&self) -> bool {
        *self == SPIN_LVLSELECT_A::SPIN_LVL_40
    }
    #[doc = "Checks if the value of the field is `SPIN_LVL_35`"]
    #[inline(always)]
    pub fn is_spin_lvl_35(&self) -> bool {
        *self == SPIN_LVLSELECT_A::SPIN_LVL_35
    }
    #[doc = "Checks if the value of the field is `SPIN_LVL_30`"]
    #[inline(always)]
    pub fn is_spin_lvl_30(&self) -> bool {
        *self == SPIN_LVLSELECT_A::SPIN_LVL_30
    }
}
#[doc = "Field `SPIN_LVL` writer - Determines the final drive level that is used by the Spin Up Routine.\n 7=65%\n 6=60%\n 5=55%\n 4=50%\n 3=45%\n 2=40%\n 1=35%\n 0=30%"]
pub type SPIN_LVL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, SPIN_UP_CFG_SPEC, u8, SPIN_LVLSELECT_A, 3, O>;
impl<'a, const O: u8> SPIN_LVL_W<'a, O> {
    #[doc = "7=65%"]
    #[inline(always)]
    pub fn spin_lvl_65(self) -> &'a mut W {
        self.variant(SPIN_LVLSELECT_A::SPIN_LVL_65)
    }
    #[doc = "6=60%"]
    #[inline(always)]
    pub fn spin_lvl_60(self) -> &'a mut W {
        self.variant(SPIN_LVLSELECT_A::SPIN_LVL_60)
    }
    #[doc = "5=55%"]
    #[inline(always)]
    pub fn spin_lvl_55(self) -> &'a mut W {
        self.variant(SPIN_LVLSELECT_A::SPIN_LVL_55)
    }
    #[doc = "4=50%"]
    #[inline(always)]
    pub fn spin_lvl_50(self) -> &'a mut W {
        self.variant(SPIN_LVLSELECT_A::SPIN_LVL_50)
    }
    #[doc = "3=45%"]
    #[inline(always)]
    pub fn spin_lvl_45(self) -> &'a mut W {
        self.variant(SPIN_LVLSELECT_A::SPIN_LVL_45)
    }
    #[doc = "2=40%"]
    #[inline(always)]
    pub fn spin_lvl_40(self) -> &'a mut W {
        self.variant(SPIN_LVLSELECT_A::SPIN_LVL_40)
    }
    #[doc = "1=35%"]
    #[inline(always)]
    pub fn spin_lvl_35(self) -> &'a mut W {
        self.variant(SPIN_LVLSELECT_A::SPIN_LVL_35)
    }
    #[doc = "0=30%"]
    #[inline(always)]
    pub fn spin_lvl_30(self) -> &'a mut W {
        self.variant(SPIN_LVLSELECT_A::SPIN_LVL_30)
    }
}
#[doc = "Field `NOKICK` reader - Determines if the Spin Up Routine will drive the fan to 100% duty cycle for 1/4 of the programmed spin up time before\n driving it at the programmed level.\n 1=The Spin Up Routine will not drive the PWM to 100%. It will set the drive at the programmed spin level for the entire duration of\n the programmed spin up time\n 0=The Spin Up Routine will drive the PWM to 100% for 1/4 of the programmed spin up time before reverting to the programmed spin level."]
pub type NOKICK_R = crate::BitReader<bool>;
#[doc = "Field `NOKICK` writer - Determines if the Spin Up Routine will drive the fan to 100% duty cycle for 1/4 of the programmed spin up time before\n driving it at the programmed level.\n 1=The Spin Up Routine will not drive the PWM to 100%. It will set the drive at the programmed spin level for the entire duration of\n the programmed spin up time\n 0=The Spin Up Routine will drive the PWM to 100% for 1/4 of the programmed spin up time before reverting to the programmed spin level."]
pub type NOKICK_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPIN_UP_CFG_SPEC, bool, O>;
#[doc = "Field `DRIVE_FAIL_CNT` reader - Determines how many update cycles are used for the Drive Fail detection function. This circuitry determines whether the\n fan can be driven to the desired Tach target. These settings only apply if the Fan Speed Control Algorithm is enabled.\n 3=Drive Fail detection circuitry will count for 64 update periods\n 2=Drive Fail detection circuitry will count for 32 update periods\n 1=Drive Fail detection circuitry will count for 16 update periods\n 0=Drive Fail detection circuitry is disabled."]
pub type DRIVE_FAIL_CNT_R = crate::FieldReader<u8, DRIVE_FAIL_CNTSELECT_A>;
#[doc = "Determines how many update cycles are used for the Drive Fail detection function. This circuitry determines whether the\n fan can be driven to the desired Tach target. These settings only apply if the Fan Speed Control Algorithm is enabled.\n 3=Drive Fail detection circuitry will count for 64 update periods\n 2=Drive Fail detection circuitry will count for 32 update periods\n 1=Drive Fail detection circuitry will count for 16 update periods\n 0=Drive Fail detection circuitry is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRIVE_FAIL_CNTSELECT_A {
    #[doc = "3: 3=Drive Fail detection circuitry will count for 64 update periods"]
    DRIVE_FAIL_CNT_64_PERIOD = 3,
    #[doc = "2: 2=Drive Fail detection circuitry will count for 32 update periods"]
    DRIVE_FAIL_CNT_32_PERIOD = 2,
    #[doc = "1: 1=Drive Fail detection circuitry will count for 16 update periods"]
    DRIVE_FAIL_CNT_16_PERIOD = 1,
    #[doc = "0: 0=Drive Fail detection circuitry is disabled."]
    DRIVE_FAIL_CNT_DIS = 0,
}
impl From<DRIVE_FAIL_CNTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVE_FAIL_CNTSELECT_A) -> Self {
        variant as _
    }
}
impl DRIVE_FAIL_CNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVE_FAIL_CNTSELECT_A {
        match self.bits {
            3 => DRIVE_FAIL_CNTSELECT_A::DRIVE_FAIL_CNT_64_PERIOD,
            2 => DRIVE_FAIL_CNTSELECT_A::DRIVE_FAIL_CNT_32_PERIOD,
            1 => DRIVE_FAIL_CNTSELECT_A::DRIVE_FAIL_CNT_16_PERIOD,
            0 => DRIVE_FAIL_CNTSELECT_A::DRIVE_FAIL_CNT_DIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DRIVE_FAIL_CNT_64_PERIOD`"]
    #[inline(always)]
    pub fn is_drive_fail_cnt_64_period(&self) -> bool {
        *self == DRIVE_FAIL_CNTSELECT_A::DRIVE_FAIL_CNT_64_PERIOD
    }
    #[doc = "Checks if the value of the field is `DRIVE_FAIL_CNT_32_PERIOD`"]
    #[inline(always)]
    pub fn is_drive_fail_cnt_32_period(&self) -> bool {
        *self == DRIVE_FAIL_CNTSELECT_A::DRIVE_FAIL_CNT_32_PERIOD
    }
    #[doc = "Checks if the value of the field is `DRIVE_FAIL_CNT_16_PERIOD`"]
    #[inline(always)]
    pub fn is_drive_fail_cnt_16_period(&self) -> bool {
        *self == DRIVE_FAIL_CNTSELECT_A::DRIVE_FAIL_CNT_16_PERIOD
    }
    #[doc = "Checks if the value of the field is `DRIVE_FAIL_CNT_DIS`"]
    #[inline(always)]
    pub fn is_drive_fail_cnt_dis(&self) -> bool {
        *self == DRIVE_FAIL_CNTSELECT_A::DRIVE_FAIL_CNT_DIS
    }
}
#[doc = "Field `DRIVE_FAIL_CNT` writer - Determines how many update cycles are used for the Drive Fail detection function. This circuitry determines whether the\n fan can be driven to the desired Tach target. These settings only apply if the Fan Speed Control Algorithm is enabled.\n 3=Drive Fail detection circuitry will count for 64 update periods\n 2=Drive Fail detection circuitry will count for 32 update periods\n 1=Drive Fail detection circuitry will count for 16 update periods\n 0=Drive Fail detection circuitry is disabled."]
pub type DRIVE_FAIL_CNT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, SPIN_UP_CFG_SPEC, u8, DRIVE_FAIL_CNTSELECT_A, 2, O>;
impl<'a, const O: u8> DRIVE_FAIL_CNT_W<'a, O> {
    #[doc = "3=Drive Fail detection circuitry will count for 64 update periods"]
    #[inline(always)]
    pub fn drive_fail_cnt_64_period(self) -> &'a mut W {
        self.variant(DRIVE_FAIL_CNTSELECT_A::DRIVE_FAIL_CNT_64_PERIOD)
    }
    #[doc = "2=Drive Fail detection circuitry will count for 32 update periods"]
    #[inline(always)]
    pub fn drive_fail_cnt_32_period(self) -> &'a mut W {
        self.variant(DRIVE_FAIL_CNTSELECT_A::DRIVE_FAIL_CNT_32_PERIOD)
    }
    #[doc = "1=Drive Fail detection circuitry will count for 16 update periods"]
    #[inline(always)]
    pub fn drive_fail_cnt_16_period(self) -> &'a mut W {
        self.variant(DRIVE_FAIL_CNTSELECT_A::DRIVE_FAIL_CNT_16_PERIOD)
    }
    #[doc = "0=Drive Fail detection circuitry is disabled."]
    #[inline(always)]
    pub fn drive_fail_cnt_dis(self) -> &'a mut W {
        self.variant(DRIVE_FAIL_CNTSELECT_A::DRIVE_FAIL_CNT_DIS)
    }
}
impl R {
    #[doc = "Bits 0:1 - Determines the maximum Spin Time that the Spin Up Routine will run for. If a valid tachometer measurement is not\n detected before the Spin Time has elapsed, an interrupt will be generated. When the RPM based Fan Control Algorithm is active,\n the fan driver will attempt to re-start the fan immediately after the end of the last spin up attempt.\n 3=2 seconds\n 2=1 second\n 1=500 ms\n 0=250 ms"]
    #[inline(always)]
    pub fn spinup_time(&self) -> SPINUP_TIME_R {
        SPINUP_TIME_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Determines the final drive level that is used by the Spin Up Routine.\n 7=65%\n 6=60%\n 5=55%\n 4=50%\n 3=45%\n 2=40%\n 1=35%\n 0=30%"]
    #[inline(always)]
    pub fn spin_lvl(&self) -> SPIN_LVL_R {
        SPIN_LVL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Determines if the Spin Up Routine will drive the fan to 100% duty cycle for 1/4 of the programmed spin up time before\n driving it at the programmed level.\n 1=The Spin Up Routine will not drive the PWM to 100%. It will set the drive at the programmed spin level for the entire duration of\n the programmed spin up time\n 0=The Spin Up Routine will drive the PWM to 100% for 1/4 of the programmed spin up time before reverting to the programmed spin level."]
    #[inline(always)]
    pub fn nokick(&self) -> NOKICK_R {
        NOKICK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Determines how many update cycles are used for the Drive Fail detection function. This circuitry determines whether the\n fan can be driven to the desired Tach target. These settings only apply if the Fan Speed Control Algorithm is enabled.\n 3=Drive Fail detection circuitry will count for 64 update periods\n 2=Drive Fail detection circuitry will count for 32 update periods\n 1=Drive Fail detection circuitry will count for 16 update periods\n 0=Drive Fail detection circuitry is disabled."]
    #[inline(always)]
    pub fn drive_fail_cnt(&self) -> DRIVE_FAIL_CNT_R {
        DRIVE_FAIL_CNT_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Determines the maximum Spin Time that the Spin Up Routine will run for. If a valid tachometer measurement is not\n detected before the Spin Time has elapsed, an interrupt will be generated. When the RPM based Fan Control Algorithm is active,\n the fan driver will attempt to re-start the fan immediately after the end of the last spin up attempt.\n 3=2 seconds\n 2=1 second\n 1=500 ms\n 0=250 ms"]
    #[inline(always)]
    pub fn spinup_time(&mut self) -> SPINUP_TIME_W<0> {
        SPINUP_TIME_W::new(self)
    }
    #[doc = "Bits 2:4 - Determines the final drive level that is used by the Spin Up Routine.\n 7=65%\n 6=60%\n 5=55%\n 4=50%\n 3=45%\n 2=40%\n 1=35%\n 0=30%"]
    #[inline(always)]
    pub fn spin_lvl(&mut self) -> SPIN_LVL_W<2> {
        SPIN_LVL_W::new(self)
    }
    #[doc = "Bit 5 - Determines if the Spin Up Routine will drive the fan to 100% duty cycle for 1/4 of the programmed spin up time before\n driving it at the programmed level.\n 1=The Spin Up Routine will not drive the PWM to 100%. It will set the drive at the programmed spin level for the entire duration of\n the programmed spin up time\n 0=The Spin Up Routine will drive the PWM to 100% for 1/4 of the programmed spin up time before reverting to the programmed spin level."]
    #[inline(always)]
    pub fn nokick(&mut self) -> NOKICK_W<5> {
        NOKICK_W::new(self)
    }
    #[doc = "Bits 6:7 - Determines how many update cycles are used for the Drive Fail detection function. This circuitry determines whether the\n fan can be driven to the desired Tach target. These settings only apply if the Fan Speed Control Algorithm is enabled.\n 3=Drive Fail detection circuitry will count for 64 update periods\n 2=Drive Fail detection circuitry will count for 32 update periods\n 1=Drive Fail detection circuitry will count for 16 update periods\n 0=Drive Fail detection circuitry is disabled."]
    #[inline(always)]
    pub fn drive_fail_cnt(&mut self) -> DRIVE_FAIL_CNT_W<6> {
        DRIVE_FAIL_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Fan Spin Up Configuration Register controls the settings of Spin Up Routine.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spin_up_cfg](index.html) module"]
pub struct SPIN_UP_CFG_SPEC;
impl crate::RegisterSpec for SPIN_UP_CFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spin_up_cfg::R](R) reader structure"]
impl crate::Readable for SPIN_UP_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spin_up_cfg::W](W) writer structure"]
impl crate::Writable for SPIN_UP_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPIN_UP_CFG to value 0x19"]
impl crate::Resettable for SPIN_UP_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x19
    }
}
