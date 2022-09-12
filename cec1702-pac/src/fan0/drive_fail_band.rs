#[doc = "Register `DRIVE_FAIL_BAND` reader"]
pub struct R(crate::R<DRIVE_FAIL_BAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRIVE_FAIL_BAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRIVE_FAIL_BAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRIVE_FAIL_BAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRIVE_FAIL_BAND` writer"]
pub struct W(crate::W<DRIVE_FAIL_BAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRIVE_FAIL_BAND_SPEC>;
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
impl From<crate::W<DRIVE_FAIL_BAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRIVE_FAIL_BAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAN_DRIVE_FAIL_BAND` reader - The number of Tach counts used by the Fan Drive Fail detection circuitry."]
pub type FAN_DRIVE_FAIL_BAND_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FAN_DRIVE_FAIL_BAND` writer - The number of Tach counts used by the Fan Drive Fail detection circuitry."]
pub type FAN_DRIVE_FAIL_BAND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, DRIVE_FAIL_BAND_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 3:15 - The number of Tach counts used by the Fan Drive Fail detection circuitry."]
    #[inline(always)]
    pub fn fan_drive_fail_band(&self) -> FAN_DRIVE_FAIL_BAND_R {
        FAN_DRIVE_FAIL_BAND_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - The number of Tach counts used by the Fan Drive Fail detection circuitry."]
    #[inline(always)]
    pub fn fan_drive_fail_band(&mut self) -> FAN_DRIVE_FAIL_BAND_W<3> {
        FAN_DRIVE_FAIL_BAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The number of Tach counts used by the Fan Drive Fail detection circuitry\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drive_fail_band](index.html) module"]
pub struct DRIVE_FAIL_BAND_SPEC;
impl crate::RegisterSpec for DRIVE_FAIL_BAND_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [drive_fail_band::R](R) reader structure"]
impl crate::Readable for DRIVE_FAIL_BAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [drive_fail_band::W](W) writer structure"]
impl crate::Writable for DRIVE_FAIL_BAND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DRIVE_FAIL_BAND to value 0"]
impl crate::Resettable for DRIVE_FAIL_BAND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
