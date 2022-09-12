#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GIRQ\\[%s\\]"]
    pub girq: crate::ArrayProxy<GIRQ, 19, 0x14>,
    _reserved1: [u8; 0x0200],
    #[doc = "0x200 - Block Enable Set Register"]
    pub blk_en_set: BLK_EN_SET,
    #[doc = "0x204 - Block Enable Clear Register."]
    pub blk_en_clr: BLK_EN_CLR,
    #[doc = "0x208 - Block IRQ Vector Register"]
    pub blk_irq_vtor: BLK_IRQ_VTOR,
}
#[doc = "GIRQ\\[%s\\]"]
pub use girq::GIRQ;
#[doc = r"Cluster"]
#[doc = "GIRQ\\[%s\\]"]
pub mod girq;
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
