#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LED Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - LED Limits This register may be written at any time. Values written into the register are held in an holding register, which is transferred into the actual register at the end of a PWM period. The two byte fields may be written independently. Reads of this register return the current contents and not the value of the holding register."]
    pub limit: LIMIT,
    #[doc = "0x08 - LED Delay"]
    pub dly: DLY,
    #[doc = "0x0c - This register has eight segment fields which provide the amount the current duty cycle is adjusted at the end of every PWM period. Segment field selection is decoded based on the segment index. The segment index equation utilized depends on the SYMMETRY bit in the LED Configuration Register Register) . In Symmetric Mode the Segment_Index\\[2:0\\]
= Duty Cycle Bits\\[7:5\\]\n . In Asymmetric Mode the Segment_Index\\[2:0\\]
is the bit concatenation of following: Segment_Index\\[2\\]
= (FALLING RAMP TIME in Figure 30-3, Clipping Example) and Segment_Index\\[1:0\\]
= Duty Cycle Bits\\[7:6\\]."]
    pub step: STEP,
    #[doc = "0x10 - LED Update Interval"]
    pub intrvl: INTRVL,
    #[doc = "0x14 - LED Output Delay"]
    pub outdly: OUTDLY,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "LED Configuration"]
pub mod cfg;
#[doc = "LIMIT (rw) register accessor: an alias for `Reg<LIMIT_SPEC>`"]
pub type LIMIT = crate::Reg<limit::LIMIT_SPEC>;
#[doc = "LED Limits This register may be written at any time. Values written into the register are held in an holding register, which is transferred into the actual register at the end of a PWM period. The two byte fields may be written independently. Reads of this register return the current contents and not the value of the holding register."]
pub mod limit;
#[doc = "DLY (rw) register accessor: an alias for `Reg<DLY_SPEC>`"]
pub type DLY = crate::Reg<dly::DLY_SPEC>;
#[doc = "LED Delay"]
pub mod dly;
#[doc = "STEP (rw) register accessor: an alias for `Reg<STEP_SPEC>`"]
pub type STEP = crate::Reg<step::STEP_SPEC>;
#[doc = "This register has eight segment fields which provide the amount the current duty cycle is adjusted at the end of every PWM period. Segment field selection is decoded based on the segment index. The segment index equation utilized depends on the SYMMETRY bit in the LED Configuration Register Register) . In Symmetric Mode the Segment_Index\\[2:0\\]
= Duty Cycle Bits\\[7:5\\]\n . In Asymmetric Mode the Segment_Index\\[2:0\\]
is the bit concatenation of following: Segment_Index\\[2\\]
= (FALLING RAMP TIME in Figure 30-3, Clipping Example) and Segment_Index\\[1:0\\]
= Duty Cycle Bits\\[7:6\\]."]
pub mod step;
#[doc = "INTRVL (rw) register accessor: an alias for `Reg<INTRVL_SPEC>`"]
pub type INTRVL = crate::Reg<intrvl::INTRVL_SPEC>;
#[doc = "LED Update Interval"]
pub mod intrvl;
#[doc = "OUTDLY (rw) register accessor: an alias for `Reg<OUTDLY_SPEC>`"]
pub type OUTDLY = crate::Reg<outdly::OUTDLY_SPEC>;
#[doc = "LED Output Delay"]
pub mod outdly;
