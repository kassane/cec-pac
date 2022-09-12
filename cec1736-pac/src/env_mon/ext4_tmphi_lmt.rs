#[doc = "Register `EXT4_TMPHI_LMT` reader"]
pub struct R(crate::R<EXT4_TMPHI_LMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT4_TMPHI_LMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT4_TMPHI_LMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT4_TMPHI_LMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT4_TMPHI_LMT` writer"]
pub struct W(crate::W<EXT4_TMPHI_LMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT4_TMPHI_LMT_SPEC>;
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
impl From<crate::W<EXT4_TMPHI_LMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT4_TMPHI_LMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMPHI4_LIMIT` reader - High limit for External Diode 4 Register"]
pub type TMPHI4_LIMIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMPHI4_LIMIT` writer - High limit for External Diode 4 Register"]
pub type TMPHI4_LIMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, EXT4_TMPHI_LMT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - High limit for External Diode 4 Register"]
    #[inline(always)]
    pub fn tmphi4_limit(&self) -> TMPHI4_LIMIT_R {
        TMPHI4_LIMIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - High limit for External Diode 4 Register"]
    #[inline(always)]
    pub fn tmphi4_limit(&mut self) -> TMPHI4_LIMIT_W<0> {
        TMPHI4_LIMIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High limit for External Diode 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext4_tmphi_lmt](index.html) module"]
pub struct EXT4_TMPHI_LMT_SPEC;
impl crate::RegisterSpec for EXT4_TMPHI_LMT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ext4_tmphi_lmt::R](R) reader structure"]
impl crate::Readable for EXT4_TMPHI_LMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext4_tmphi_lmt::W](W) writer structure"]
impl crate::Writable for EXT4_TMPHI_LMT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT4_TMPHI_LMT to value 0x7f"]
impl crate::Resettable for EXT4_TMPHI_LMT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
