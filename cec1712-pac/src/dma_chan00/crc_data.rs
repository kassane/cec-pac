#[doc = "Register `CRC_DATA` reader"]
pub struct R(crate::R<CRC_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_DATA` writer"]
pub struct W(crate::W<CRC_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_DATA_SPEC>;
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
impl From<crate::W<CRC_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC` reader - Writes to this register initialize the CRC generator. Reads from this register return the output of the\n CRC that is calculated from the data transferred by DMA Channel N. The output of the CRC generator is bit-reversed\n and inverted on reads, as required by the CRC-32-IEEE definition. A CRC can be accumulated across multiple DMA transactions\n on Channel N. If it is necessary to save the intermediate CRC value, the result of the read of this register must be\n bit-reversed and inverted before being written back to this register."]
pub type CRC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRC` writer - Writes to this register initialize the CRC generator. Reads from this register return the output of the\n CRC that is calculated from the data transferred by DMA Channel N. The output of the CRC generator is bit-reversed\n and inverted on reads, as required by the CRC-32-IEEE definition. A CRC can be accumulated across multiple DMA transactions\n on Channel N. If it is necessary to save the intermediate CRC value, the result of the read of this register must be\n bit-reversed and inverted before being written back to this register."]
pub type CRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRC_DATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Writes to this register initialize the CRC generator. Reads from this register return the output of the\n CRC that is calculated from the data transferred by DMA Channel N. The output of the CRC generator is bit-reversed\n and inverted on reads, as required by the CRC-32-IEEE definition. A CRC can be accumulated across multiple DMA transactions\n on Channel N. If it is necessary to save the intermediate CRC value, the result of the read of this register must be\n bit-reversed and inverted before being written back to this register."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Writes to this register initialize the CRC generator. Reads from this register return the output of the\n CRC that is calculated from the data transferred by DMA Channel N. The output of the CRC generator is bit-reversed\n and inverted on reads, as required by the CRC-32-IEEE definition. A CRC can be accumulated across multiple DMA transactions\n on Channel N. If it is necessary to save the intermediate CRC value, the result of the read of this register must be\n bit-reversed and inverted before being written back to this register."]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W<0> {
        CRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA CHANNEL N CRC DATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_data](index.html) module"]
pub struct CRC_DATA_SPEC;
impl crate::RegisterSpec for CRC_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_data::R](R) reader structure"]
impl crate::Readable for CRC_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_data::W](W) writer structure"]
impl crate::Writable for CRC_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC_DATA to value 0"]
impl crate::Resettable for CRC_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
