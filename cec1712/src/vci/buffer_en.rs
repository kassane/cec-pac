#[doc = "Register `BUFFER_EN` reader"]
pub struct R(crate::R<BUFFER_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFFER_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFFER_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFFER_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUFFER_EN` writer"]
pub struct W(crate::W<BUFFER_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFFER_EN_SPEC>;
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
impl From<crate::W<BUFFER_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUFFER_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `V_BUF` reader - Input Buffer enable. After changing the buffer enable for a VCI input, firmware should reset the input latch and clear any\n potential interrupt that may have been triggered by the input, as changing the buffer may cause the internal status to change. This register\n has no effect when VTR is powered. When VTR is on, the input buffers are enabled only by the IE bit. For each bit in the field:\n 1=VCI_IN# input buffer enabled independent of the IE bit. The edge detection latches for this input are always enabled\n 0=VCI_IN# input buffer enabled by the IE bit. The edge detection latches are only enabled when the IE bit is 1 (default)."]
pub type V_BUF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `V_BUF` writer - Input Buffer enable. After changing the buffer enable for a VCI input, firmware should reset the input latch and clear any\n potential interrupt that may have been triggered by the input, as changing the buffer may cause the internal status to change. This register\n has no effect when VTR is powered. When VTR is on, the input buffers are enabled only by the IE bit. For each bit in the field:\n 1=VCI_IN# input buffer enabled independent of the IE bit. The edge detection latches for this input are always enabled\n 0=VCI_IN# input buffer enabled by the IE bit. The edge detection latches are only enabled when the IE bit is 1 (default)."]
pub type V_BUF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUFFER_EN_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Input Buffer enable. After changing the buffer enable for a VCI input, firmware should reset the input latch and clear any\n potential interrupt that may have been triggered by the input, as changing the buffer may cause the internal status to change. This register\n has no effect when VTR is powered. When VTR is on, the input buffers are enabled only by the IE bit. For each bit in the field:\n 1=VCI_IN# input buffer enabled independent of the IE bit. The edge detection latches for this input are always enabled\n 0=VCI_IN# input buffer enabled by the IE bit. The edge detection latches are only enabled when the IE bit is 1 (default)."]
    #[inline(always)]
    pub fn v_buf(&self) -> V_BUF_R {
        V_BUF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input Buffer enable. After changing the buffer enable for a VCI input, firmware should reset the input latch and clear any\n potential interrupt that may have been triggered by the input, as changing the buffer may cause the internal status to change. This register\n has no effect when VTR is powered. When VTR is on, the input buffers are enabled only by the IE bit. For each bit in the field:\n 1=VCI_IN# input buffer enabled independent of the IE bit. The edge detection latches for this input are always enabled\n 0=VCI_IN# input buffer enabled by the IE bit. The edge detection latches are only enabled when the IE bit is 1 (default)."]
    #[inline(always)]
    pub fn v_buf(&mut self) -> V_BUF_W<0> {
        V_BUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VCI Buffer Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buffer_en](index.html) module"]
pub struct BUFFER_EN_SPEC;
impl crate::RegisterSpec for BUFFER_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buffer_en::R](R) reader structure"]
impl crate::Readable for BUFFER_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buffer_en::W](W) writer structure"]
impl crate::Writable for BUFFER_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUFFER_EN to value 0"]
impl crate::Resettable for BUFFER_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
