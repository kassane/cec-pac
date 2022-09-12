#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The ADC Control Register is used to control the behavior of the Analog to Digital Converter."]
    pub ctrl: CTRL,
    #[doc = "0x04 - The ADC Delay register determines the delay from setting Start_Repeat in the \n ADC Control Register and the start of a conversion cycle. This register also controls the interval between conversion cycles in repeat mode."]
    pub delay: DELAY,
    #[doc = "0x08 - The ADC Status Register indicates whether the ADC has completed a conversion cycle.\n All bits are cleared by being written with a 1. \n 0: conversion of the corresponding ADC channel is not complete\n 1: conversion of the corresponding ADC channel is complete"]
    pub chan_sts: CHAN_STS,
    #[doc = "0x0c - The ADC Single Register is used to control which ADC channel \n is captured during a Single-Sample conversion cycle initiated by the Start_Single bit in the ADC Control Register. \n APPLICATION NOTE: Do not change the bits in this register in the middle of a conversion cycle to insure proper operation.\n 0: single cycle conversions for this channel are disabled\n 1: single cycle conversions for this channel are enabled"]
    pub sng_en: SNG_EN,
    #[doc = "0x10 - The ADC Repeat Register is used to control which ADC channels \n are captured during a repeat conversion cycle initiated by the Start_Repeat bit in the ADC Control Register."]
    pub rept_en: REPT_EN,
    #[doc = "0x14..0x54 - All 16 ADC channels return their results into \n a 32-bit reading register. In each case the low 10 bits of the reading register\n return the result of the Analog to Digital conversion and the upper 22 bits return 0."]
    pub chan_rd: [CHAN_RD; 16],
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "The ADC Control Register is used to control the behavior of the Analog to Digital Converter."]
pub mod ctrl;
#[doc = "DELAY (rw) register accessor: an alias for `Reg<DELAY_SPEC>`"]
pub type DELAY = crate::Reg<delay::DELAY_SPEC>;
#[doc = "The ADC Delay register determines the delay from setting Start_Repeat in the \n ADC Control Register and the start of a conversion cycle. This register also controls the interval between conversion cycles in repeat mode."]
pub mod delay;
#[doc = "CHAN_STS (rw) register accessor: an alias for `Reg<CHAN_STS_SPEC>`"]
pub type CHAN_STS = crate::Reg<chan_sts::CHAN_STS_SPEC>;
#[doc = "The ADC Status Register indicates whether the ADC has completed a conversion cycle.\n All bits are cleared by being written with a 1. \n 0: conversion of the corresponding ADC channel is not complete\n 1: conversion of the corresponding ADC channel is complete"]
pub mod chan_sts;
#[doc = "SNG_EN (rw) register accessor: an alias for `Reg<SNG_EN_SPEC>`"]
pub type SNG_EN = crate::Reg<sng_en::SNG_EN_SPEC>;
#[doc = "The ADC Single Register is used to control which ADC channel \n is captured during a Single-Sample conversion cycle initiated by the Start_Single bit in the ADC Control Register. \n APPLICATION NOTE: Do not change the bits in this register in the middle of a conversion cycle to insure proper operation.\n 0: single cycle conversions for this channel are disabled\n 1: single cycle conversions for this channel are enabled"]
pub mod sng_en;
#[doc = "REPT_EN (rw) register accessor: an alias for `Reg<REPT_EN_SPEC>`"]
pub type REPT_EN = crate::Reg<rept_en::REPT_EN_SPEC>;
#[doc = "The ADC Repeat Register is used to control which ADC channels \n are captured during a repeat conversion cycle initiated by the Start_Repeat bit in the ADC Control Register."]
pub mod rept_en;
#[doc = "CHAN_RD (rw) register accessor: an alias for `Reg<CHAN_RD_SPEC>`"]
pub type CHAN_RD = crate::Reg<chan_rd::CHAN_RD_SPEC>;
#[doc = "All 16 ADC channels return their results into \n a 32-bit reading register. In each case the low 10 bits of the reading register\n return the result of the Analog to Digital conversion and the upper 22 bits return 0."]
pub mod chan_rd;
