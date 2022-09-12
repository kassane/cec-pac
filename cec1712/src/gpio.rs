#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - GPIO Pin Control Register"]
    pub ctrl0: [CTRL0; 8],
    #[doc = "0x20..0x40 - GPIO Pin Control Register"]
    pub ctrl1: [CTRL1; 8],
    #[doc = "0x40..0x60 - GPIO Pin Control Register"]
    pub ctrl2: [CTRL2; 8],
    #[doc = "0x60..0x80 - GPIO Pin Control Register"]
    pub ctrl3: [CTRL3; 8],
    #[doc = "0x80..0xa0 - GPIO Pin Control Register"]
    pub ctrl4: [CTRL4; 8],
    #[doc = "0xa0..0xc0 - GPIO Pin Control Register"]
    pub ctrl5: [CTRL5; 8],
    #[doc = "0xc0..0xe0 - GPIO Pin Control Register"]
    pub ctrl6: [CTRL6; 8],
    #[doc = "0xe0..0x100 - GPIO Pin Control Register"]
    pub ctrl7: [CTRL7; 8],
    #[doc = "0x100..0x120 - GPIO Pin Control Register"]
    pub ctrl10: [CTRL10; 8],
    #[doc = "0x120..0x140 - GPIO Pin Control Register"]
    pub ctrl11: [CTRL11; 8],
    #[doc = "0x140..0x160 - GPIO Pin Control Register"]
    pub ctrl12: [CTRL12; 8],
    #[doc = "0x160..0x180 - GPIO Pin Control Register"]
    pub ctrl13: [CTRL13; 8],
    #[doc = "0x180..0x1a0 - GPIO Pin Control Register"]
    pub ctrl14: [CTRL14; 8],
    #[doc = "0x1a0..0x1c0 - GPIO Pin Control Register"]
    pub ctrl15: [CTRL15; 8],
    #[doc = "0x1c0..0x1e0 - GPIO Pin Control Register"]
    pub ctrl16: [CTRL16; 8],
    #[doc = "0x1e0..0x200 - GPIO Pin Control Register"]
    pub ctrl17: [CTRL17; 8],
    #[doc = "0x200..0x220 - GPIO Pin Control Register"]
    pub ctrl20: [CTRL20; 8],
    #[doc = "0x220..0x240 - GPIO Pin Control Register"]
    pub ctrl21: [CTRL21; 8],
    #[doc = "0x240..0x260 - GPIO Pin Control Register"]
    pub ctrl22: [CTRL22; 8],
    #[doc = "0x260..0x280 - GPIO Pin Control Register"]
    pub ctrl23: [CTRL23; 8],
    #[doc = "0x280..0x2a0 - GPIO Pin Control Register"]
    pub ctrl24: [CTRL24; 8],
    #[doc = "0x2a0..0x2c0 - GPIO Pin Control Register"]
    pub ctrl25: [CTRL25; 8],
    #[doc = "0x2c0 - GPIO Pin Control Register"]
    pub ctrl26: [CTRL26; 1],
    _reserved23: [u8; 0x3c],
    #[doc = "0x300..0x318 - The GPIO Input Registers."]
    pub parin: [PARIN; 6],
    _reserved24: [u8; 0x68],
    #[doc = "0x380..0x398 - The GPIO Output Registers."]
    pub parout: [PAROUT; 6],
    _reserved25: [u8; 0x0168],
    #[doc = "0x500..0x520 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p0: [CTRL2P0; 8],
    #[doc = "0x520..0x540 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p1: [CTRL2P1; 8],
    #[doc = "0x540..0x560 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p2: [CTRL2P2; 8],
    #[doc = "0x560..0x580 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p3: [CTRL2P3; 8],
    #[doc = "0x580..0x5a0 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p4: [CTRL2P4; 8],
    #[doc = "0x5a0..0x5c0 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p5: [CTRL2P5; 8],
    #[doc = "0x5c0..0x5e0 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p6: [CTRL2P6; 8],
    #[doc = "0x5e0..0x600 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p7: [CTRL2P7; 8],
    #[doc = "0x600..0x620 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p10: [CTRL2P10; 8],
    #[doc = "0x620..0x640 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p11: [CTRL2P11; 8],
    #[doc = "0x640..0x660 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p12: [CTRL2P12; 8],
    #[doc = "0x660..0x680 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p13: [CTRL2P13; 8],
    #[doc = "0x680..0x6a0 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p14: [CTRL2P14; 8],
    #[doc = "0x6a0..0x6c0 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p15: [CTRL2P15; 8],
    #[doc = "0x6c0..0x6e0 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p16: [CTRL2P16; 8],
    #[doc = "0x6e0..0x700 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p17: [CTRL2P17; 8],
    #[doc = "0x700..0x720 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p20: [CTRL2P20; 8],
    #[doc = "0x720..0x740 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p21: [CTRL2P21; 8],
    #[doc = "0x740..0x760 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p22: [CTRL2P22; 8],
    #[doc = "0x760..0x780 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p23: [CTRL2P23; 8],
    #[doc = "0x780..0x7a0 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p24: [CTRL2P24; 8],
    #[doc = "0x7a0..0x7c0 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p25: [CTRL2P25; 8],
    #[doc = "0x7c0 - The GPIO PIN_CTRL2 Registers"]
    pub ctrl2p26: [CTRL2P26; 1],
}
#[doc = "CTRL0 (rw) register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl2;
#[doc = "CTRL3 (rw) register accessor: an alias for `Reg<CTRL3_SPEC>`"]
pub type CTRL3 = crate::Reg<ctrl3::CTRL3_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl3;
#[doc = "CTRL4 (rw) register accessor: an alias for `Reg<CTRL4_SPEC>`"]
pub type CTRL4 = crate::Reg<ctrl4::CTRL4_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl4;
#[doc = "CTRL5 (rw) register accessor: an alias for `Reg<CTRL5_SPEC>`"]
pub type CTRL5 = crate::Reg<ctrl5::CTRL5_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl5;
#[doc = "CTRL6 (rw) register accessor: an alias for `Reg<CTRL6_SPEC>`"]
pub type CTRL6 = crate::Reg<ctrl6::CTRL6_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl6;
#[doc = "CTRL7 (rw) register accessor: an alias for `Reg<CTRL7_SPEC>`"]
pub type CTRL7 = crate::Reg<ctrl7::CTRL7_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl7;
#[doc = "CTRL10 (rw) register accessor: an alias for `Reg<CTRL10_SPEC>`"]
pub type CTRL10 = crate::Reg<ctrl10::CTRL10_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl10;
#[doc = "CTRL11 (rw) register accessor: an alias for `Reg<CTRL11_SPEC>`"]
pub type CTRL11 = crate::Reg<ctrl11::CTRL11_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl11;
#[doc = "CTRL12 (rw) register accessor: an alias for `Reg<CTRL12_SPEC>`"]
pub type CTRL12 = crate::Reg<ctrl12::CTRL12_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl12;
#[doc = "CTRL13 (rw) register accessor: an alias for `Reg<CTRL13_SPEC>`"]
pub type CTRL13 = crate::Reg<ctrl13::CTRL13_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl13;
#[doc = "CTRL14 (rw) register accessor: an alias for `Reg<CTRL14_SPEC>`"]
pub type CTRL14 = crate::Reg<ctrl14::CTRL14_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl14;
#[doc = "CTRL15 (rw) register accessor: an alias for `Reg<CTRL15_SPEC>`"]
pub type CTRL15 = crate::Reg<ctrl15::CTRL15_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl15;
#[doc = "CTRL16 (rw) register accessor: an alias for `Reg<CTRL16_SPEC>`"]
pub type CTRL16 = crate::Reg<ctrl16::CTRL16_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl16;
#[doc = "CTRL17 (rw) register accessor: an alias for `Reg<CTRL17_SPEC>`"]
pub type CTRL17 = crate::Reg<ctrl17::CTRL17_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl17;
#[doc = "CTRL20 (rw) register accessor: an alias for `Reg<CTRL20_SPEC>`"]
pub type CTRL20 = crate::Reg<ctrl20::CTRL20_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl20;
#[doc = "CTRL21 (rw) register accessor: an alias for `Reg<CTRL21_SPEC>`"]
pub type CTRL21 = crate::Reg<ctrl21::CTRL21_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl21;
#[doc = "CTRL22 (rw) register accessor: an alias for `Reg<CTRL22_SPEC>`"]
pub type CTRL22 = crate::Reg<ctrl22::CTRL22_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl22;
#[doc = "CTRL23 (rw) register accessor: an alias for `Reg<CTRL23_SPEC>`"]
pub type CTRL23 = crate::Reg<ctrl23::CTRL23_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl23;
#[doc = "CTRL24 (rw) register accessor: an alias for `Reg<CTRL24_SPEC>`"]
pub type CTRL24 = crate::Reg<ctrl24::CTRL24_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl24;
#[doc = "CTRL25 (rw) register accessor: an alias for `Reg<CTRL25_SPEC>`"]
pub type CTRL25 = crate::Reg<ctrl25::CTRL25_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl25;
#[doc = "CTRL26 (rw) register accessor: an alias for `Reg<CTRL26_SPEC>`"]
pub type CTRL26 = crate::Reg<ctrl26::CTRL26_SPEC>;
#[doc = "GPIO Pin Control Register"]
pub mod ctrl26;
#[doc = "PARIN (rw) register accessor: an alias for `Reg<PARIN_SPEC>`"]
pub type PARIN = crate::Reg<parin::PARIN_SPEC>;
#[doc = "The GPIO Input Registers."]
pub mod parin;
#[doc = "PAROUT (rw) register accessor: an alias for `Reg<PAROUT_SPEC>`"]
pub type PAROUT = crate::Reg<parout::PAROUT_SPEC>;
#[doc = "The GPIO Output Registers."]
pub mod parout;
#[doc = "CTRL2P0 (rw) register accessor: an alias for `Reg<CTRL2P0_SPEC>`"]
pub type CTRL2P0 = crate::Reg<ctrl2p0::CTRL2P0_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p0;
#[doc = "CTRL2P1 (rw) register accessor: an alias for `Reg<CTRL2P1_SPEC>`"]
pub type CTRL2P1 = crate::Reg<ctrl2p1::CTRL2P1_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p1;
#[doc = "CTRL2P2 (rw) register accessor: an alias for `Reg<CTRL2P2_SPEC>`"]
pub type CTRL2P2 = crate::Reg<ctrl2p2::CTRL2P2_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p2;
#[doc = "CTRL2P3 (rw) register accessor: an alias for `Reg<CTRL2P3_SPEC>`"]
pub type CTRL2P3 = crate::Reg<ctrl2p3::CTRL2P3_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p3;
#[doc = "CTRL2P4 (rw) register accessor: an alias for `Reg<CTRL2P4_SPEC>`"]
pub type CTRL2P4 = crate::Reg<ctrl2p4::CTRL2P4_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p4;
#[doc = "CTRL2P5 (rw) register accessor: an alias for `Reg<CTRL2P5_SPEC>`"]
pub type CTRL2P5 = crate::Reg<ctrl2p5::CTRL2P5_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p5;
#[doc = "CTRL2P6 (rw) register accessor: an alias for `Reg<CTRL2P6_SPEC>`"]
pub type CTRL2P6 = crate::Reg<ctrl2p6::CTRL2P6_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p6;
#[doc = "CTRL2P7 (rw) register accessor: an alias for `Reg<CTRL2P7_SPEC>`"]
pub type CTRL2P7 = crate::Reg<ctrl2p7::CTRL2P7_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p7;
#[doc = "CTRL2P10 (rw) register accessor: an alias for `Reg<CTRL2P10_SPEC>`"]
pub type CTRL2P10 = crate::Reg<ctrl2p10::CTRL2P10_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p10;
#[doc = "CTRL2P11 (rw) register accessor: an alias for `Reg<CTRL2P11_SPEC>`"]
pub type CTRL2P11 = crate::Reg<ctrl2p11::CTRL2P11_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p11;
#[doc = "CTRL2P12 (rw) register accessor: an alias for `Reg<CTRL2P12_SPEC>`"]
pub type CTRL2P12 = crate::Reg<ctrl2p12::CTRL2P12_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p12;
#[doc = "CTRL2P13 (rw) register accessor: an alias for `Reg<CTRL2P13_SPEC>`"]
pub type CTRL2P13 = crate::Reg<ctrl2p13::CTRL2P13_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p13;
#[doc = "CTRL2P14 (rw) register accessor: an alias for `Reg<CTRL2P14_SPEC>`"]
pub type CTRL2P14 = crate::Reg<ctrl2p14::CTRL2P14_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p14;
#[doc = "CTRL2P15 (rw) register accessor: an alias for `Reg<CTRL2P15_SPEC>`"]
pub type CTRL2P15 = crate::Reg<ctrl2p15::CTRL2P15_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p15;
#[doc = "CTRL2P16 (rw) register accessor: an alias for `Reg<CTRL2P16_SPEC>`"]
pub type CTRL2P16 = crate::Reg<ctrl2p16::CTRL2P16_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p16;
#[doc = "CTRL2P17 (rw) register accessor: an alias for `Reg<CTRL2P17_SPEC>`"]
pub type CTRL2P17 = crate::Reg<ctrl2p17::CTRL2P17_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p17;
#[doc = "CTRL2P20 (rw) register accessor: an alias for `Reg<CTRL2P20_SPEC>`"]
pub type CTRL2P20 = crate::Reg<ctrl2p20::CTRL2P20_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p20;
#[doc = "CTRL2P21 (rw) register accessor: an alias for `Reg<CTRL2P21_SPEC>`"]
pub type CTRL2P21 = crate::Reg<ctrl2p21::CTRL2P21_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p21;
#[doc = "CTRL2P22 (rw) register accessor: an alias for `Reg<CTRL2P22_SPEC>`"]
pub type CTRL2P22 = crate::Reg<ctrl2p22::CTRL2P22_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p22;
#[doc = "CTRL2P23 (rw) register accessor: an alias for `Reg<CTRL2P23_SPEC>`"]
pub type CTRL2P23 = crate::Reg<ctrl2p23::CTRL2P23_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p23;
#[doc = "CTRL2P24 (rw) register accessor: an alias for `Reg<CTRL2P24_SPEC>`"]
pub type CTRL2P24 = crate::Reg<ctrl2p24::CTRL2P24_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p24;
#[doc = "CTRL2P25 (rw) register accessor: an alias for `Reg<CTRL2P25_SPEC>`"]
pub type CTRL2P25 = crate::Reg<ctrl2p25::CTRL2P25_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p25;
#[doc = "CTRL2P26 (rw) register accessor: an alias for `Reg<CTRL2P26_SPEC>`"]
pub type CTRL2P26 = crate::Reg<ctrl2p26::CTRL2P26_SPEC>;
#[doc = "The GPIO PIN_CTRL2 Registers"]
pub mod ctrl2p26;
