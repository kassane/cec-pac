#[doc = "Register `FW_SCR2` reader"]
pub struct R(crate::R<FW_SCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FW_SCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FW_SCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FW_SCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FW_SCR2` writer"]
pub struct W(crate::W<FW_SCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FW_SCR2_SPEC>;
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
impl From<crate::W<FW_SCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FW_SCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCR2` reader - This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
pub type SCR2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCR2` writer - This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
pub type SCR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FW_SCR2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
    #[inline(always)]
    pub fn scr2(&self) -> SCR2_R {
        SCR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field has no functionality other than storage. This register is aliased to ESPI Config Scratch Register."]
    #[inline(always)]
    pub fn scr2(&mut self) -> SCR2_W<0> {
        SCR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOOT ROM Scratch 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fw_scr2](index.html) module"]
pub struct FW_SCR2_SPEC;
impl crate::RegisterSpec for FW_SCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fw_scr2::R](R) reader structure"]
impl crate::Readable for FW_SCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fw_scr2::W](W) writer structure"]
impl crate::Writable for FW_SCR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FW_SCR2 to value 0"]
impl crate::Resettable for FW_SCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
