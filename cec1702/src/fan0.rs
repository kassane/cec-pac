#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The Fan Driver Setting used to control the output of the Fan Driver."]
    pub set: SET,
    #[doc = "0x02 - The Fan Configuration Register controls the general operation of the RPM based Fan Control Algorithm used by the fan driver."]
    pub cfg: CFG,
    #[doc = "0x04 - PWM Divide"]
    pub pwm_div: PWM_DIV,
    #[doc = "0x05 - Gain Register stores the gain terms used by the proportional and integral portions of the RPM based Fan Control Algorithm."]
    pub gain: GAIN,
    #[doc = "0x06 - The Fan Spin Up Configuration Register controls the settings of Spin Up Routine."]
    pub spin_up_cfg: SPIN_UP_CFG,
    #[doc = "0x07 - FAN_STEP The Fan Step value represents the maximum step size the fan driver will take between update times"]
    pub step: STEP,
    #[doc = "0x08 - the minimum drive setting for the RPM based Fan Control Algorithm."]
    pub min_drive: MIN_DRIVE,
    #[doc = "0x09 - The maximum TACH Reading Register value to indicate that the fan is spinning properly."]
    pub val_tach_cnt: VAL_TACH_CNT,
    #[doc = "0x0a - The number of Tach counts used by the Fan Drive Fail detection circuitry"]
    pub drive_fail_band: DRIVE_FAIL_BAND,
    #[doc = "0x0c - The target tachometer value."]
    pub tach_tgt: TACH_TGT,
    #[doc = "0x0e - 15:3\\]
The current tachometer reading value."]
    pub tach_rd: TACH_RD,
    #[doc = "0x10 - 1:0\\]
Determines the frequency range of the PWM fan driver"]
    pub driv_base_freq: DRIV_BASE_FREQ,
    #[doc = "0x11 - The bits in this register are routed to interrupts."]
    pub sts: STS,
}
#[doc = "SET (rw) register accessor: an alias for `Reg<SET_SPEC>`"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = "The Fan Driver Setting used to control the output of the Fan Driver."]
pub mod set;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "The Fan Configuration Register controls the general operation of the RPM based Fan Control Algorithm used by the fan driver."]
pub mod cfg;
#[doc = "PWM_DIV (rw) register accessor: an alias for `Reg<PWM_DIV_SPEC>`"]
pub type PWM_DIV = crate::Reg<pwm_div::PWM_DIV_SPEC>;
#[doc = "PWM Divide"]
pub mod pwm_div;
#[doc = "GAIN (rw) register accessor: an alias for `Reg<GAIN_SPEC>`"]
pub type GAIN = crate::Reg<gain::GAIN_SPEC>;
#[doc = "Gain Register stores the gain terms used by the proportional and integral portions of the RPM based Fan Control Algorithm."]
pub mod gain;
#[doc = "SPIN_UP_CFG (rw) register accessor: an alias for `Reg<SPIN_UP_CFG_SPEC>`"]
pub type SPIN_UP_CFG = crate::Reg<spin_up_cfg::SPIN_UP_CFG_SPEC>;
#[doc = "The Fan Spin Up Configuration Register controls the settings of Spin Up Routine."]
pub mod spin_up_cfg;
#[doc = "STEP (rw) register accessor: an alias for `Reg<STEP_SPEC>`"]
pub type STEP = crate::Reg<step::STEP_SPEC>;
#[doc = "FAN_STEP The Fan Step value represents the maximum step size the fan driver will take between update times"]
pub mod step;
#[doc = "MIN_DRIVE (rw) register accessor: an alias for `Reg<MIN_DRIVE_SPEC>`"]
pub type MIN_DRIVE = crate::Reg<min_drive::MIN_DRIVE_SPEC>;
#[doc = "the minimum drive setting for the RPM based Fan Control Algorithm."]
pub mod min_drive;
#[doc = "VAL_TACH_CNT (rw) register accessor: an alias for `Reg<VAL_TACH_CNT_SPEC>`"]
pub type VAL_TACH_CNT = crate::Reg<val_tach_cnt::VAL_TACH_CNT_SPEC>;
#[doc = "The maximum TACH Reading Register value to indicate that the fan is spinning properly."]
pub mod val_tach_cnt;
#[doc = "DRIVE_FAIL_BAND (rw) register accessor: an alias for `Reg<DRIVE_FAIL_BAND_SPEC>`"]
pub type DRIVE_FAIL_BAND = crate::Reg<drive_fail_band::DRIVE_FAIL_BAND_SPEC>;
#[doc = "The number of Tach counts used by the Fan Drive Fail detection circuitry"]
pub mod drive_fail_band;
#[doc = "TACH_TGT (rw) register accessor: an alias for `Reg<TACH_TGT_SPEC>`"]
pub type TACH_TGT = crate::Reg<tach_tgt::TACH_TGT_SPEC>;
#[doc = "The target tachometer value."]
pub mod tach_tgt;
#[doc = "TACH_RD (rw) register accessor: an alias for `Reg<TACH_RD_SPEC>`"]
pub type TACH_RD = crate::Reg<tach_rd::TACH_RD_SPEC>;
#[doc = "15:3\\]
The current tachometer reading value."]
pub mod tach_rd;
#[doc = "DRIV_BASE_FREQ (rw) register accessor: an alias for `Reg<DRIV_BASE_FREQ_SPEC>`"]
pub type DRIV_BASE_FREQ = crate::Reg<driv_base_freq::DRIV_BASE_FREQ_SPEC>;
#[doc = "1:0\\]
Determines the frequency range of the PWM fan driver"]
pub mod driv_base_freq;
#[doc = "STS (rw) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "The bits in this register are routed to interrupts."]
pub mod sts;
