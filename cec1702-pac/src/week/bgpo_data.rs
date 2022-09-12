#[doc = "Register `BGPO_DATA` reader"]
pub struct R(crate::R<BGPO_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGPO_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGPO_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGPO_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGPO_DATA` writer"]
pub struct W(crate::W<BGPO_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGPO_DATA_SPEC>;
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
impl From<crate::W<BGPO_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGPO_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BGPO` reader - Battery powered General Purpose Output. Each output pin may be individually configured to be either a VBAT-power BGPO or a VTR\n powered GPIO, based on the corresponding settings in the BGPO Power Register. Additionally, each output pin may be individually configured\n to reset to 0 on either RESET_VTR or RESET_SYS, based on the corresponding settings in the BGPO Reset Register.\n For each bit \\[i\\]
in the field: 1=BGPO\\[i\\]
output is high; 0=BGPO\\[i\\]
output is low."]
pub type BGPO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BGPO` writer - Battery powered General Purpose Output. Each output pin may be individually configured to be either a VBAT-power BGPO or a VTR\n powered GPIO, based on the corresponding settings in the BGPO Power Register. Additionally, each output pin may be individually configured\n to reset to 0 on either RESET_VTR or RESET_SYS, based on the corresponding settings in the BGPO Reset Register.\n For each bit \\[i\\]
in the field: 1=BGPO\\[i\\]
output is high; 0=BGPO\\[i\\]
output is low."]
pub type BGPO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGPO_DATA_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Battery powered General Purpose Output. Each output pin may be individually configured to be either a VBAT-power BGPO or a VTR\n powered GPIO, based on the corresponding settings in the BGPO Power Register. Additionally, each output pin may be individually configured\n to reset to 0 on either RESET_VTR or RESET_SYS, based on the corresponding settings in the BGPO Reset Register.\n For each bit \\[i\\]
in the field: 1=BGPO\\[i\\]
output is high; 0=BGPO\\[i\\]
output is low."]
    #[inline(always)]
    pub fn bgpo(&self) -> BGPO_R {
        BGPO_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Battery powered General Purpose Output. Each output pin may be individually configured to be either a VBAT-power BGPO or a VTR\n powered GPIO, based on the corresponding settings in the BGPO Power Register. Additionally, each output pin may be individually configured\n to reset to 0 on either RESET_VTR or RESET_SYS, based on the corresponding settings in the BGPO Reset Register.\n For each bit \\[i\\]
in the field: 1=BGPO\\[i\\]
output is high; 0=BGPO\\[i\\]
output is low."]
    #[inline(always)]
    pub fn bgpo(&mut self) -> BGPO_W<0> {
        BGPO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BGPO Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgpo_data](index.html) module"]
pub struct BGPO_DATA_SPEC;
impl crate::RegisterSpec for BGPO_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgpo_data::R](R) reader structure"]
impl crate::Readable for BGPO_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgpo_data::W](W) writer structure"]
impl crate::Writable for BGPO_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGPO_DATA to value 0"]
impl crate::Resettable for BGPO_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
