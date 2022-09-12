#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GIRQ8 SOURCE"]
    pub src8: SRC8,
    #[doc = "0x04 - GIRQ8 ENABLE SET"]
    pub en_set8: EN_SET8,
    #[doc = "0x08 - GIRQ8 RESULT"]
    pub result8: RESULT8,
    #[doc = "0x0c - GIRQ8 ENABLE CLEAR"]
    pub en_clr8: EN_CLR8,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - GIRQ9 SOURCE"]
    pub src9: SRC9,
    #[doc = "0x18 - GIRQ9 ENABLE SET"]
    pub en_set9: EN_SET9,
    #[doc = "0x1c - GIRQ9 RESULT"]
    pub result9: RESULT9,
    #[doc = "0x20 - GIRQ9 ENABLE CLEAR"]
    pub en_clr9: EN_CLR9,
    _reserved8: [u8; 0x04],
    #[doc = "0x28 - GIRQ10 SOURCE"]
    pub src10: SRC10,
    #[doc = "0x2c - GIRQ10 ENABLE SET"]
    pub en_set10: EN_SET10,
    #[doc = "0x30 - GIRQ10 RESULT"]
    pub result10: RESULT10,
    #[doc = "0x34 - GIRQ10 ENABLE CLEAR"]
    pub en_clr10: EN_CLR10,
    _reserved12: [u8; 0x04],
    #[doc = "0x3c - GIRQ11 SOURCE"]
    pub src11: SRC11,
    #[doc = "0x40 - GIRQ11 ENABLE SET"]
    pub en_set11: EN_SET11,
    #[doc = "0x44 - GIRQ11 RESULT"]
    pub result11: RESULT11,
    #[doc = "0x48 - GIRQ11 ENABLE CLEAR"]
    pub en_clr11: EN_CLR11,
    _reserved16: [u8; 0x04],
    #[doc = "0x50 - GIRQ12 SOURCE"]
    pub src12: SRC12,
    #[doc = "0x54 - GIRQ12 ENABLE SET"]
    pub en_set12: EN_SET12,
    #[doc = "0x58 - GIRQ12 RESULT"]
    pub result12: RESULT12,
    #[doc = "0x5c - GIRQ12 ENABLE CLEAR"]
    pub en_clr12: EN_CLR12,
    _reserved20: [u8; 0x04],
    #[doc = "0x64 - GIRQ13 SOURCE"]
    pub src13: SRC13,
    #[doc = "0x68 - GIRQ13 ENABLE SET"]
    pub en_set13: EN_SET13,
    #[doc = "0x6c - GIRQ13 RESULT"]
    pub result13: RESULT13,
    #[doc = "0x70 - GIRQ13 ENABLE CLEAR"]
    pub en_clr13: EN_CLR13,
    _reserved24: [u8; 0x04],
    #[doc = "0x78 - GIRQ14 SOURCE"]
    pub src14: SRC14,
    #[doc = "0x7c - GIRQ14 ENABLE SET"]
    pub en_set14: EN_SET14,
    #[doc = "0x80 - GIRQ14 RESULT"]
    pub result14: RESULT14,
    #[doc = "0x84 - GIRQ14 ENABLE CLEAR"]
    pub en_clr14: EN_CLR14,
    _reserved28: [u8; 0x04],
    #[doc = "0x8c - GIRQ15 SOURCE"]
    pub src15: SRC15,
    #[doc = "0x90 - GIRQ15 ENABLE SET"]
    pub en_set15: EN_SET15,
    #[doc = "0x94 - GIRQ15 RESULT"]
    pub result15: RESULT15,
    #[doc = "0x98 - GIRQ15 ENABLE CLEAR"]
    pub en_clr15: EN_CLR15,
    _reserved32: [u8; 0x04],
    #[doc = "0xa0 - GIRQ16 SOURCE"]
    pub src16: SRC16,
    #[doc = "0xa4 - GIRQ16 ENABLE SET"]
    pub en_set16: EN_SET16,
    #[doc = "0xa8 - GIRQ16 RESULT"]
    pub result16: RESULT16,
    #[doc = "0xac - GIRQ16 ENABLE CLEAR"]
    pub en_clr16: EN_CLR16,
    _reserved36: [u8; 0x04],
    #[doc = "0xb4 - GIRQ17 SOURCE"]
    pub src17: SRC17,
    #[doc = "0xb8 - GIRQ17 ENABLE SET"]
    pub en_set17: EN_SET17,
    #[doc = "0xbc - GIRQ17 RESULT"]
    pub result17: RESULT17,
    #[doc = "0xc0 - GIRQ17 ENABLE CLEAR"]
    pub en_clr17: EN_CLR17,
    _reserved40: [u8; 0x04],
    #[doc = "0xc8 - GIRQ18 SOURCE"]
    pub src18: SRC18,
    #[doc = "0xcc - GIRQ18 ENABLE SET"]
    pub en_set18: EN_SET18,
    #[doc = "0xd0 - GIRQ18 RESULT"]
    pub result18: RESULT18,
    #[doc = "0xd4 - GIRQ18 ENABLE CLEAR"]
    pub en_clr18: EN_CLR18,
    _reserved44: [u8; 0x04],
    #[doc = "0xdc - GIRQ19 SOURCE"]
    pub src19: SRC19,
    #[doc = "0xe0 - GIRQ19 ENABLE SET"]
    pub en_set19: EN_SET19,
    #[doc = "0xe4 - GIRQ19 RESULT"]
    pub result19: RESULT19,
    #[doc = "0xe8 - GIRQ19 ENABLE CLEAR"]
    pub en_clr19: EN_CLR19,
    _reserved48: [u8; 0x04],
    #[doc = "0xf0 - GIRQ20 SOURCE"]
    pub src20: SRC20,
    #[doc = "0xf4 - GIRQ20 ENABLE SET"]
    pub en_set20: EN_SET20,
    #[doc = "0xf8 - GIRQ20 RESULT"]
    pub result20: RESULT20,
    #[doc = "0xfc - GIRQ20 ENABLE CLEAR"]
    pub en_clr20: EN_CLR20,
    _reserved52: [u8; 0x04],
    #[doc = "0x104 - GIRQ21 SOURCE"]
    pub src21: SRC21,
    #[doc = "0x108 - GIRQ21 ENABLE SET"]
    pub en_set21: EN_SET21,
    #[doc = "0x10c - GIRQ21 RESULT"]
    pub result21: RESULT21,
    #[doc = "0x110 - GIRQ21 ENABLE CLEAR"]
    pub en_clr21: EN_CLR21,
    _reserved56: [u8; 0x04],
    #[doc = "0x118 - GIRQ22 SOURCE"]
    pub src22: SRC22,
    #[doc = "0x11c - GIRQ22 ENABLE SET"]
    pub en_set22: EN_SET22,
    #[doc = "0x120 - GIRQ22 RESULT"]
    pub result22: RESULT22,
    #[doc = "0x124 - GIRQ22 ENABLE CLEAR"]
    pub en_clr22: EN_CLR22,
    _reserved60: [u8; 0x04],
    #[doc = "0x12c - GIRQ23 SOURCE"]
    pub src23: SRC23,
    #[doc = "0x130 - GIRQ23 ENABLE SET"]
    pub en_set23: EN_SET23,
    #[doc = "0x134 - GIRQ23 RESULT"]
    pub result23: RESULT23,
    #[doc = "0x138 - GIRQ23 ENABLE CLEAR"]
    pub en_clr23: EN_CLR23,
    _reserved64: [u8; 0x04],
    #[doc = "0x140 - GIRQ24 SOURCE"]
    pub src24: SRC24,
    #[doc = "0x144 - GIRQ24 ENABLE SET"]
    pub en_set24: EN_SET24,
    #[doc = "0x148 - GIRQ24 RESULT"]
    pub result24: RESULT24,
    #[doc = "0x14c - GIRQ24 ENABLE CLEAR"]
    pub en_clr24: EN_CLR24,
    _reserved68: [u8; 0x04],
    #[doc = "0x154 - GIRQ25 SOURCE"]
    pub src25: SRC25,
    #[doc = "0x158 - GIRQ25 ENABLE SET"]
    pub en_set25: EN_SET25,
    #[doc = "0x15c - GIRQ25 RESULT"]
    pub result25: RESULT25,
    #[doc = "0x160 - GIRQ25 ENABLE CLEAR"]
    pub en_clr25: EN_CLR25,
    _reserved72: [u8; 0x04],
    #[doc = "0x168 - GIRQ26 SOURCE"]
    pub src26: SRC26,
    #[doc = "0x16c - GIRQ26 ENABLE SET"]
    pub en_set26: EN_SET26,
    #[doc = "0x170 - GIRQ26 RESULT"]
    pub result26: RESULT26,
    #[doc = "0x174 - GIRQ26 ENABLE CLEAR"]
    pub en_clr26: EN_CLR26,
    _reserved76: [u8; 0x88],
    #[doc = "0x200 - Block Enable Set Register"]
    pub blk_en_set: BLK_EN_SET,
    #[doc = "0x204 - Block Enable Clear Register."]
    pub blk_en_clr: BLK_EN_CLR,
    #[doc = "0x208 - Block IRQ Vector Register"]
    pub blk_irq_vtor: BLK_IRQ_VTOR,
}
#[doc = "SRC8 (rw) register accessor: an alias for `Reg<SRC8_SPEC>`"]
pub type SRC8 = crate::Reg<src8::SRC8_SPEC>;
#[doc = "GIRQ8 SOURCE"]
pub mod src8;
#[doc = "EN_SET8 (rw) register accessor: an alias for `Reg<EN_SET8_SPEC>`"]
pub type EN_SET8 = crate::Reg<en_set8::EN_SET8_SPEC>;
#[doc = "GIRQ8 ENABLE SET"]
pub mod en_set8;
#[doc = "RESULT8 (r) register accessor: an alias for `Reg<RESULT8_SPEC>`"]
pub type RESULT8 = crate::Reg<result8::RESULT8_SPEC>;
#[doc = "GIRQ8 RESULT"]
pub mod result8;
#[doc = "EN_CLR8 (rw) register accessor: an alias for `Reg<EN_CLR8_SPEC>`"]
pub type EN_CLR8 = crate::Reg<en_clr8::EN_CLR8_SPEC>;
#[doc = "GIRQ8 ENABLE CLEAR"]
pub mod en_clr8;
#[doc = "SRC9 (rw) register accessor: an alias for `Reg<SRC9_SPEC>`"]
pub type SRC9 = crate::Reg<src9::SRC9_SPEC>;
#[doc = "GIRQ9 SOURCE"]
pub mod src9;
#[doc = "EN_SET9 (rw) register accessor: an alias for `Reg<EN_SET9_SPEC>`"]
pub type EN_SET9 = crate::Reg<en_set9::EN_SET9_SPEC>;
#[doc = "GIRQ9 ENABLE SET"]
pub mod en_set9;
#[doc = "RESULT9 (r) register accessor: an alias for `Reg<RESULT9_SPEC>`"]
pub type RESULT9 = crate::Reg<result9::RESULT9_SPEC>;
#[doc = "GIRQ9 RESULT"]
pub mod result9;
#[doc = "EN_CLR9 (rw) register accessor: an alias for `Reg<EN_CLR9_SPEC>`"]
pub type EN_CLR9 = crate::Reg<en_clr9::EN_CLR9_SPEC>;
#[doc = "GIRQ9 ENABLE CLEAR"]
pub mod en_clr9;
#[doc = "SRC10 (rw) register accessor: an alias for `Reg<SRC10_SPEC>`"]
pub type SRC10 = crate::Reg<src10::SRC10_SPEC>;
#[doc = "GIRQ10 SOURCE"]
pub mod src10;
#[doc = "EN_SET10 (rw) register accessor: an alias for `Reg<EN_SET10_SPEC>`"]
pub type EN_SET10 = crate::Reg<en_set10::EN_SET10_SPEC>;
#[doc = "GIRQ10 ENABLE SET"]
pub mod en_set10;
#[doc = "RESULT10 (r) register accessor: an alias for `Reg<RESULT10_SPEC>`"]
pub type RESULT10 = crate::Reg<result10::RESULT10_SPEC>;
#[doc = "GIRQ10 RESULT"]
pub mod result10;
#[doc = "EN_CLR10 (rw) register accessor: an alias for `Reg<EN_CLR10_SPEC>`"]
pub type EN_CLR10 = crate::Reg<en_clr10::EN_CLR10_SPEC>;
#[doc = "GIRQ10 ENABLE CLEAR"]
pub mod en_clr10;
#[doc = "SRC11 (rw) register accessor: an alias for `Reg<SRC11_SPEC>`"]
pub type SRC11 = crate::Reg<src11::SRC11_SPEC>;
#[doc = "GIRQ11 SOURCE"]
pub mod src11;
#[doc = "EN_SET11 (rw) register accessor: an alias for `Reg<EN_SET11_SPEC>`"]
pub type EN_SET11 = crate::Reg<en_set11::EN_SET11_SPEC>;
#[doc = "GIRQ11 ENABLE SET"]
pub mod en_set11;
#[doc = "RESULT11 (r) register accessor: an alias for `Reg<RESULT11_SPEC>`"]
pub type RESULT11 = crate::Reg<result11::RESULT11_SPEC>;
#[doc = "GIRQ11 RESULT"]
pub mod result11;
#[doc = "EN_CLR11 (rw) register accessor: an alias for `Reg<EN_CLR11_SPEC>`"]
pub type EN_CLR11 = crate::Reg<en_clr11::EN_CLR11_SPEC>;
#[doc = "GIRQ11 ENABLE CLEAR"]
pub mod en_clr11;
#[doc = "SRC12 (rw) register accessor: an alias for `Reg<SRC12_SPEC>`"]
pub type SRC12 = crate::Reg<src12::SRC12_SPEC>;
#[doc = "GIRQ12 SOURCE"]
pub mod src12;
#[doc = "EN_SET12 (rw) register accessor: an alias for `Reg<EN_SET12_SPEC>`"]
pub type EN_SET12 = crate::Reg<en_set12::EN_SET12_SPEC>;
#[doc = "GIRQ12 ENABLE SET"]
pub mod en_set12;
#[doc = "RESULT12 (r) register accessor: an alias for `Reg<RESULT12_SPEC>`"]
pub type RESULT12 = crate::Reg<result12::RESULT12_SPEC>;
#[doc = "GIRQ12 RESULT"]
pub mod result12;
#[doc = "EN_CLR12 (rw) register accessor: an alias for `Reg<EN_CLR12_SPEC>`"]
pub type EN_CLR12 = crate::Reg<en_clr12::EN_CLR12_SPEC>;
#[doc = "GIRQ12 ENABLE CLEAR"]
pub mod en_clr12;
#[doc = "SRC13 (rw) register accessor: an alias for `Reg<SRC13_SPEC>`"]
pub type SRC13 = crate::Reg<src13::SRC13_SPEC>;
#[doc = "GIRQ13 SOURCE"]
pub mod src13;
#[doc = "EN_SET13 (rw) register accessor: an alias for `Reg<EN_SET13_SPEC>`"]
pub type EN_SET13 = crate::Reg<en_set13::EN_SET13_SPEC>;
#[doc = "GIRQ13 ENABLE SET"]
pub mod en_set13;
#[doc = "RESULT13 (r) register accessor: an alias for `Reg<RESULT13_SPEC>`"]
pub type RESULT13 = crate::Reg<result13::RESULT13_SPEC>;
#[doc = "GIRQ13 RESULT"]
pub mod result13;
#[doc = "EN_CLR13 (rw) register accessor: an alias for `Reg<EN_CLR13_SPEC>`"]
pub type EN_CLR13 = crate::Reg<en_clr13::EN_CLR13_SPEC>;
#[doc = "GIRQ13 ENABLE CLEAR"]
pub mod en_clr13;
#[doc = "SRC14 (rw) register accessor: an alias for `Reg<SRC14_SPEC>`"]
pub type SRC14 = crate::Reg<src14::SRC14_SPEC>;
#[doc = "GIRQ14 SOURCE"]
pub mod src14;
#[doc = "EN_SET14 (rw) register accessor: an alias for `Reg<EN_SET14_SPEC>`"]
pub type EN_SET14 = crate::Reg<en_set14::EN_SET14_SPEC>;
#[doc = "GIRQ14 ENABLE SET"]
pub mod en_set14;
#[doc = "RESULT14 (r) register accessor: an alias for `Reg<RESULT14_SPEC>`"]
pub type RESULT14 = crate::Reg<result14::RESULT14_SPEC>;
#[doc = "GIRQ14 RESULT"]
pub mod result14;
#[doc = "EN_CLR14 (rw) register accessor: an alias for `Reg<EN_CLR14_SPEC>`"]
pub type EN_CLR14 = crate::Reg<en_clr14::EN_CLR14_SPEC>;
#[doc = "GIRQ14 ENABLE CLEAR"]
pub mod en_clr14;
#[doc = "SRC15 (rw) register accessor: an alias for `Reg<SRC15_SPEC>`"]
pub type SRC15 = crate::Reg<src15::SRC15_SPEC>;
#[doc = "GIRQ15 SOURCE"]
pub mod src15;
#[doc = "EN_SET15 (rw) register accessor: an alias for `Reg<EN_SET15_SPEC>`"]
pub type EN_SET15 = crate::Reg<en_set15::EN_SET15_SPEC>;
#[doc = "GIRQ15 ENABLE SET"]
pub mod en_set15;
#[doc = "RESULT15 (r) register accessor: an alias for `Reg<RESULT15_SPEC>`"]
pub type RESULT15 = crate::Reg<result15::RESULT15_SPEC>;
#[doc = "GIRQ15 RESULT"]
pub mod result15;
#[doc = "EN_CLR15 (rw) register accessor: an alias for `Reg<EN_CLR15_SPEC>`"]
pub type EN_CLR15 = crate::Reg<en_clr15::EN_CLR15_SPEC>;
#[doc = "GIRQ15 ENABLE CLEAR"]
pub mod en_clr15;
#[doc = "SRC16 (rw) register accessor: an alias for `Reg<SRC16_SPEC>`"]
pub type SRC16 = crate::Reg<src16::SRC16_SPEC>;
#[doc = "GIRQ16 SOURCE"]
pub mod src16;
#[doc = "EN_SET16 (rw) register accessor: an alias for `Reg<EN_SET16_SPEC>`"]
pub type EN_SET16 = crate::Reg<en_set16::EN_SET16_SPEC>;
#[doc = "GIRQ16 ENABLE SET"]
pub mod en_set16;
#[doc = "RESULT16 (r) register accessor: an alias for `Reg<RESULT16_SPEC>`"]
pub type RESULT16 = crate::Reg<result16::RESULT16_SPEC>;
#[doc = "GIRQ16 RESULT"]
pub mod result16;
#[doc = "EN_CLR16 (rw) register accessor: an alias for `Reg<EN_CLR16_SPEC>`"]
pub type EN_CLR16 = crate::Reg<en_clr16::EN_CLR16_SPEC>;
#[doc = "GIRQ16 ENABLE CLEAR"]
pub mod en_clr16;
#[doc = "SRC17 (rw) register accessor: an alias for `Reg<SRC17_SPEC>`"]
pub type SRC17 = crate::Reg<src17::SRC17_SPEC>;
#[doc = "GIRQ17 SOURCE"]
pub mod src17;
#[doc = "EN_SET17 (rw) register accessor: an alias for `Reg<EN_SET17_SPEC>`"]
pub type EN_SET17 = crate::Reg<en_set17::EN_SET17_SPEC>;
#[doc = "GIRQ17 ENABLE SET"]
pub mod en_set17;
#[doc = "RESULT17 (r) register accessor: an alias for `Reg<RESULT17_SPEC>`"]
pub type RESULT17 = crate::Reg<result17::RESULT17_SPEC>;
#[doc = "GIRQ17 RESULT"]
pub mod result17;
#[doc = "EN_CLR17 (rw) register accessor: an alias for `Reg<EN_CLR17_SPEC>`"]
pub type EN_CLR17 = crate::Reg<en_clr17::EN_CLR17_SPEC>;
#[doc = "GIRQ17 ENABLE CLEAR"]
pub mod en_clr17;
#[doc = "SRC18 (rw) register accessor: an alias for `Reg<SRC18_SPEC>`"]
pub type SRC18 = crate::Reg<src18::SRC18_SPEC>;
#[doc = "GIRQ18 SOURCE"]
pub mod src18;
#[doc = "EN_SET18 (rw) register accessor: an alias for `Reg<EN_SET18_SPEC>`"]
pub type EN_SET18 = crate::Reg<en_set18::EN_SET18_SPEC>;
#[doc = "GIRQ18 ENABLE SET"]
pub mod en_set18;
#[doc = "RESULT18 (r) register accessor: an alias for `Reg<RESULT18_SPEC>`"]
pub type RESULT18 = crate::Reg<result18::RESULT18_SPEC>;
#[doc = "GIRQ18 RESULT"]
pub mod result18;
#[doc = "EN_CLR18 (rw) register accessor: an alias for `Reg<EN_CLR18_SPEC>`"]
pub type EN_CLR18 = crate::Reg<en_clr18::EN_CLR18_SPEC>;
#[doc = "GIRQ18 ENABLE CLEAR"]
pub mod en_clr18;
#[doc = "SRC19 (rw) register accessor: an alias for `Reg<SRC19_SPEC>`"]
pub type SRC19 = crate::Reg<src19::SRC19_SPEC>;
#[doc = "GIRQ19 SOURCE"]
pub mod src19;
#[doc = "EN_SET19 (rw) register accessor: an alias for `Reg<EN_SET19_SPEC>`"]
pub type EN_SET19 = crate::Reg<en_set19::EN_SET19_SPEC>;
#[doc = "GIRQ19 ENABLE SET"]
pub mod en_set19;
#[doc = "RESULT19 (r) register accessor: an alias for `Reg<RESULT19_SPEC>`"]
pub type RESULT19 = crate::Reg<result19::RESULT19_SPEC>;
#[doc = "GIRQ19 RESULT"]
pub mod result19;
#[doc = "EN_CLR19 (rw) register accessor: an alias for `Reg<EN_CLR19_SPEC>`"]
pub type EN_CLR19 = crate::Reg<en_clr19::EN_CLR19_SPEC>;
#[doc = "GIRQ19 ENABLE CLEAR"]
pub mod en_clr19;
#[doc = "SRC20 (rw) register accessor: an alias for `Reg<SRC20_SPEC>`"]
pub type SRC20 = crate::Reg<src20::SRC20_SPEC>;
#[doc = "GIRQ20 SOURCE"]
pub mod src20;
#[doc = "EN_SET20 (rw) register accessor: an alias for `Reg<EN_SET20_SPEC>`"]
pub type EN_SET20 = crate::Reg<en_set20::EN_SET20_SPEC>;
#[doc = "GIRQ20 ENABLE SET"]
pub mod en_set20;
#[doc = "RESULT20 (r) register accessor: an alias for `Reg<RESULT20_SPEC>`"]
pub type RESULT20 = crate::Reg<result20::RESULT20_SPEC>;
#[doc = "GIRQ20 RESULT"]
pub mod result20;
#[doc = "EN_CLR20 (rw) register accessor: an alias for `Reg<EN_CLR20_SPEC>`"]
pub type EN_CLR20 = crate::Reg<en_clr20::EN_CLR20_SPEC>;
#[doc = "GIRQ20 ENABLE CLEAR"]
pub mod en_clr20;
#[doc = "SRC21 (rw) register accessor: an alias for `Reg<SRC21_SPEC>`"]
pub type SRC21 = crate::Reg<src21::SRC21_SPEC>;
#[doc = "GIRQ21 SOURCE"]
pub mod src21;
#[doc = "EN_SET21 (rw) register accessor: an alias for `Reg<EN_SET21_SPEC>`"]
pub type EN_SET21 = crate::Reg<en_set21::EN_SET21_SPEC>;
#[doc = "GIRQ21 ENABLE SET"]
pub mod en_set21;
#[doc = "RESULT21 (r) register accessor: an alias for `Reg<RESULT21_SPEC>`"]
pub type RESULT21 = crate::Reg<result21::RESULT21_SPEC>;
#[doc = "GIRQ21 RESULT"]
pub mod result21;
#[doc = "EN_CLR21 (rw) register accessor: an alias for `Reg<EN_CLR21_SPEC>`"]
pub type EN_CLR21 = crate::Reg<en_clr21::EN_CLR21_SPEC>;
#[doc = "GIRQ21 ENABLE CLEAR"]
pub mod en_clr21;
#[doc = "SRC22 (rw) register accessor: an alias for `Reg<SRC22_SPEC>`"]
pub type SRC22 = crate::Reg<src22::SRC22_SPEC>;
#[doc = "GIRQ22 SOURCE"]
pub mod src22;
#[doc = "EN_SET22 (rw) register accessor: an alias for `Reg<EN_SET22_SPEC>`"]
pub type EN_SET22 = crate::Reg<en_set22::EN_SET22_SPEC>;
#[doc = "GIRQ22 ENABLE SET"]
pub mod en_set22;
#[doc = "RESULT22 (r) register accessor: an alias for `Reg<RESULT22_SPEC>`"]
pub type RESULT22 = crate::Reg<result22::RESULT22_SPEC>;
#[doc = "GIRQ22 RESULT"]
pub mod result22;
#[doc = "EN_CLR22 (rw) register accessor: an alias for `Reg<EN_CLR22_SPEC>`"]
pub type EN_CLR22 = crate::Reg<en_clr22::EN_CLR22_SPEC>;
#[doc = "GIRQ22 ENABLE CLEAR"]
pub mod en_clr22;
#[doc = "SRC23 (rw) register accessor: an alias for `Reg<SRC23_SPEC>`"]
pub type SRC23 = crate::Reg<src23::SRC23_SPEC>;
#[doc = "GIRQ23 SOURCE"]
pub mod src23;
#[doc = "EN_SET23 (rw) register accessor: an alias for `Reg<EN_SET23_SPEC>`"]
pub type EN_SET23 = crate::Reg<en_set23::EN_SET23_SPEC>;
#[doc = "GIRQ23 ENABLE SET"]
pub mod en_set23;
#[doc = "RESULT23 (r) register accessor: an alias for `Reg<RESULT23_SPEC>`"]
pub type RESULT23 = crate::Reg<result23::RESULT23_SPEC>;
#[doc = "GIRQ23 RESULT"]
pub mod result23;
#[doc = "EN_CLR23 (rw) register accessor: an alias for `Reg<EN_CLR23_SPEC>`"]
pub type EN_CLR23 = crate::Reg<en_clr23::EN_CLR23_SPEC>;
#[doc = "GIRQ23 ENABLE CLEAR"]
pub mod en_clr23;
#[doc = "SRC24 (rw) register accessor: an alias for `Reg<SRC24_SPEC>`"]
pub type SRC24 = crate::Reg<src24::SRC24_SPEC>;
#[doc = "GIRQ24 SOURCE"]
pub mod src24;
#[doc = "EN_SET24 (rw) register accessor: an alias for `Reg<EN_SET24_SPEC>`"]
pub type EN_SET24 = crate::Reg<en_set24::EN_SET24_SPEC>;
#[doc = "GIRQ24 ENABLE SET"]
pub mod en_set24;
#[doc = "RESULT24 (r) register accessor: an alias for `Reg<RESULT24_SPEC>`"]
pub type RESULT24 = crate::Reg<result24::RESULT24_SPEC>;
#[doc = "GIRQ24 RESULT"]
pub mod result24;
#[doc = "EN_CLR24 (rw) register accessor: an alias for `Reg<EN_CLR24_SPEC>`"]
pub type EN_CLR24 = crate::Reg<en_clr24::EN_CLR24_SPEC>;
#[doc = "GIRQ24 ENABLE CLEAR"]
pub mod en_clr24;
#[doc = "SRC25 (rw) register accessor: an alias for `Reg<SRC25_SPEC>`"]
pub type SRC25 = crate::Reg<src25::SRC25_SPEC>;
#[doc = "GIRQ25 SOURCE"]
pub mod src25;
#[doc = "EN_SET25 (rw) register accessor: an alias for `Reg<EN_SET25_SPEC>`"]
pub type EN_SET25 = crate::Reg<en_set25::EN_SET25_SPEC>;
#[doc = "GIRQ25 ENABLE SET"]
pub mod en_set25;
#[doc = "RESULT25 (r) register accessor: an alias for `Reg<RESULT25_SPEC>`"]
pub type RESULT25 = crate::Reg<result25::RESULT25_SPEC>;
#[doc = "GIRQ25 RESULT"]
pub mod result25;
#[doc = "EN_CLR25 (rw) register accessor: an alias for `Reg<EN_CLR25_SPEC>`"]
pub type EN_CLR25 = crate::Reg<en_clr25::EN_CLR25_SPEC>;
#[doc = "GIRQ25 ENABLE CLEAR"]
pub mod en_clr25;
#[doc = "SRC26 (rw) register accessor: an alias for `Reg<SRC26_SPEC>`"]
pub type SRC26 = crate::Reg<src26::SRC26_SPEC>;
#[doc = "GIRQ26 SOURCE"]
pub mod src26;
#[doc = "EN_SET26 (rw) register accessor: an alias for `Reg<EN_SET26_SPEC>`"]
pub type EN_SET26 = crate::Reg<en_set26::EN_SET26_SPEC>;
#[doc = "GIRQ26 ENABLE SET"]
pub mod en_set26;
#[doc = "RESULT26 (r) register accessor: an alias for `Reg<RESULT26_SPEC>`"]
pub type RESULT26 = crate::Reg<result26::RESULT26_SPEC>;
#[doc = "GIRQ26 RESULT"]
pub mod result26;
#[doc = "EN_CLR26 (rw) register accessor: an alias for `Reg<EN_CLR26_SPEC>`"]
pub type EN_CLR26 = crate::Reg<en_clr26::EN_CLR26_SPEC>;
#[doc = "GIRQ26 ENABLE CLEAR"]
pub mod en_clr26;
#[doc = "BLK_EN_SET (rw) register accessor: an alias for `Reg<BLK_EN_SET_SPEC>`"]
pub type BLK_EN_SET = crate::Reg<blk_en_set::BLK_EN_SET_SPEC>;
#[doc = "Block Enable Set Register"]
pub mod blk_en_set;
#[doc = "BLK_EN_CLR (rw) register accessor: an alias for `Reg<BLK_EN_CLR_SPEC>`"]
pub type BLK_EN_CLR = crate::Reg<blk_en_clr::BLK_EN_CLR_SPEC>;
#[doc = "Block Enable Clear Register."]
pub mod blk_en_clr;
#[doc = "BLK_IRQ_VTOR (r) register accessor: an alias for `Reg<BLK_IRQ_VTOR_SPEC>`"]
pub type BLK_IRQ_VTOR = crate::Reg<blk_irq_vtor::BLK_IRQ_VTOR_SPEC>;
#[doc = "Block IRQ Vector Register"]
pub mod blk_irq_vtor;
