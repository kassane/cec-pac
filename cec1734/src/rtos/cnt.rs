#[doc = "Register `CNT` reader"]
pub struct R(crate::R<CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT` writer"]
pub struct W(crate::W<CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_SPEC>;
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
impl From<crate::W<CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTR` reader - This register contains the current value of the RTOS Timer counter. This register should be read as a DWORD. There is no latching mechanism of the upper bytes implemented if the register is accessed as a byte or word. Reading the register with byte or word operations may give incorrect results."]
pub type CNTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNTR` writer - This register contains the current value of the RTOS Timer counter. This register should be read as a DWORD. There is no latching mechanism of the upper bytes implemented if the register is accessed as a byte or word. Reading the register with byte or word operations may give incorrect results."]
pub type CNTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register contains the current value of the RTOS Timer counter. This register should be read as a DWORD. There is no latching mechanism of the upper bytes implemented if the register is accessed as a byte or word. Reading the register with byte or word operations may give incorrect results."]
    #[inline(always)]
    pub fn cntr(&self) -> CNTR_R {
        CNTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register contains the current value of the RTOS Timer counter. This register should be read as a DWORD. There is no latching mechanism of the upper bytes implemented if the register is accessed as a byte or word. Reading the register with byte or word operations may give incorrect results."]
    #[inline(always)]
    pub fn cntr(&mut self) -> CNTR_W<0> {
        CNTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTOS Timer Count Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](index.html) module"]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt::R](R) reader structure"]
impl crate::Readable for CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt::W](W) writer structure"]
impl crate::Writable for CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
