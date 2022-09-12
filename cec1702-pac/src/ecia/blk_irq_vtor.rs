#[doc = "Register `BLK_IRQ_VTOR` reader"]
pub struct R(crate::R<BLK_IRQ_VTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK_IRQ_VTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK_IRQ_VTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK_IRQ_VTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VTOR` reader - Each bit in this field reports the status of the group GIRQ interrupt assertion to the NVIC. If the GIRQx interrupt\n is disabled as a group, by the Block Enable Clear Register, then the corresponding bit will be '0'b and no interrupt will be asserted."]
pub type VTOR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - Each bit in this field reports the status of the group GIRQ interrupt assertion to the NVIC. If the GIRQx interrupt\n is disabled as a group, by the Block Enable Clear Register, then the corresponding bit will be '0'b and no interrupt will be asserted."]
    #[inline(always)]
    pub fn vtor(&self) -> VTOR_R {
        VTOR_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "Block IRQ Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk_irq_vtor](index.html) module"]
pub struct BLK_IRQ_VTOR_SPEC;
impl crate::RegisterSpec for BLK_IRQ_VTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk_irq_vtor::R](R) reader structure"]
impl crate::Readable for BLK_IRQ_VTOR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK_IRQ_VTOR to value 0"]
impl crate::Resettable for BLK_IRQ_VTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
