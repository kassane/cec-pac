#[doc = "Register `FW_SCR1` reader"]
pub struct R(crate::R<FW_SCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FW_SCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FW_SCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FW_SCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FW_SCR1` writer"]
pub struct W(crate::W<FW_SCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FW_SCR1_SPEC>;
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
impl From<crate::W<FW_SCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FW_SCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCR1` reader - This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
pub type SCR1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCR1` writer - This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
pub type SCR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FW_SCR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
    #[inline(always)]
    pub fn scr1(&self) -> SCR1_R {
        SCR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
    #[inline(always)]
    pub fn scr1(&mut self) -> SCR1_W<0> {
        SCR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOOT ROM Scratch 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fw_scr1](index.html) module"]
pub struct FW_SCR1_SPEC;
impl crate::RegisterSpec for FW_SCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fw_scr1::R](R) reader structure"]
impl crate::Readable for FW_SCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fw_scr1::W](W) writer structure"]
impl crate::Writable for FW_SCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FW_SCR1 to value 0"]
impl crate::Resettable for FW_SCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
