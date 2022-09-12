#[doc = "Register `INPUT_EN` reader"]
pub struct R(crate::R<INPUT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUT_EN` writer"]
pub struct W(crate::W<INPUT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUT_EN_SPEC>;
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
impl From<crate::W<INPUT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IE` reader - Input Enables for VCI_IN# signals. After changing the input enable for a VCI input, firmware should reset the input latch and clear\n any potential interrupt that may have been triggered by the input, as changing the enable may cause the internal status to change.\n For each bit in the field:\n 1=Enabled. The corresponding VCI_IN# input is not gated and toggling the pin will affect the VCI_OUT pin\n 0=Not Enabled. the corresponding VCI_IN# input does not affect the VCI_OUT pin, even if the input is '0'. Unless the corresponding bit\n in the VCI Buffer Enable Register is 1, latches are not asserted, even if the VCI_IN# pin is low, during a VBAT power transition"]
pub type IE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IE` writer - Input Enables for VCI_IN# signals. After changing the input enable for a VCI input, firmware should reset the input latch and clear\n any potential interrupt that may have been triggered by the input, as changing the enable may cause the internal status to change.\n For each bit in the field:\n 1=Enabled. The corresponding VCI_IN# input is not gated and toggling the pin will affect the VCI_OUT pin\n 0=Not Enabled. the corresponding VCI_IN# input does not affect the VCI_OUT pin, even if the input is '0'. Unless the corresponding bit\n in the VCI Buffer Enable Register is 1, latches are not asserted, even if the VCI_IN# pin is low, during a VBAT power transition"]
pub type IE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INPUT_EN_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input Enables for VCI_IN# signals. After changing the input enable for a VCI input, firmware should reset the input latch and clear\n any potential interrupt that may have been triggered by the input, as changing the enable may cause the internal status to change.\n For each bit in the field:\n 1=Enabled. The corresponding VCI_IN# input is not gated and toggling the pin will affect the VCI_OUT pin\n 0=Not Enabled. the corresponding VCI_IN# input does not affect the VCI_OUT pin, even if the input is '0'. Unless the corresponding bit\n in the VCI Buffer Enable Register is 1, latches are not asserted, even if the VCI_IN# pin is low, during a VBAT power transition"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input Enables for VCI_IN# signals. After changing the input enable for a VCI input, firmware should reset the input latch and clear\n any potential interrupt that may have been triggered by the input, as changing the enable may cause the internal status to change.\n For each bit in the field:\n 1=Enabled. The corresponding VCI_IN# input is not gated and toggling the pin will affect the VCI_OUT pin\n 0=Not Enabled. the corresponding VCI_IN# input does not affect the VCI_OUT pin, even if the input is '0'. Unless the corresponding bit\n in the VCI Buffer Enable Register is 1, latches are not asserted, even if the VCI_IN# pin is low, during a VBAT power transition"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<0> {
        IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VCI Input Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [input_en](index.html) module"]
pub struct INPUT_EN_SPEC;
impl crate::RegisterSpec for INPUT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [input_en::R](R) reader structure"]
impl crate::Readable for INPUT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [input_en::W](W) writer structure"]
impl crate::Writable for INPUT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INPUT_EN to value 0x0f"]
impl crate::Resettable for INPUT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
