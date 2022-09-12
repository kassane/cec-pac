#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This field determines both the frequency and duty cycle of the PWM signal. Setting this field to a value of n will cause the On time of the PWM to be n+1 cycles of the PWM Clock Source. When this field is set to zero and the PWMX_COUNTER_OFF_TIME is not set to zero, the PWM_OUTPUT is held low (Full Off)."]
    pub cnt_on: CNT_ON,
    #[doc = "0x04 - This field determine both the frequency and duty cycle of the PWM signal. Setting this field to a value of n will cause the Off time of the PWM to be n+1 cycles of the PWM Clock Source. When this field is set to zero, the PWM_OUTPUT is held high (Full On)."]
    pub cnt_off: CNT_OFF,
    #[doc = "0x08 - PWMx CONFIGURATION REGISTER"]
    pub cfg: CFG,
}
#[doc = "CNT_ON (rw) register accessor: an alias for `Reg<CNT_ON_SPEC>`"]
pub type CNT_ON = crate::Reg<cnt_on::CNT_ON_SPEC>;
#[doc = "This field determines both the frequency and duty cycle of the PWM signal. Setting this field to a value of n will cause the On time of the PWM to be n+1 cycles of the PWM Clock Source. When this field is set to zero and the PWMX_COUNTER_OFF_TIME is not set to zero, the PWM_OUTPUT is held low (Full Off)."]
pub mod cnt_on;
#[doc = "CNT_OFF (rw) register accessor: an alias for `Reg<CNT_OFF_SPEC>`"]
pub type CNT_OFF = crate::Reg<cnt_off::CNT_OFF_SPEC>;
#[doc = "This field determine both the frequency and duty cycle of the PWM signal. Setting this field to a value of n will cause the Off time of the PWM to be n+1 cycles of the PWM Clock Source. When this field is set to zero, the PWM_OUTPUT is held high (Full On)."]
pub mod cnt_off;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "PWMx CONFIGURATION REGISTER"]
pub mod cfg;
