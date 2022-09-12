#[doc = "Register `MIN_DRIVE` reader"]
pub struct R(crate::R<MIN_DRIVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIN_DRIVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIN_DRIVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIN_DRIVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIN_DRIVE` writer"]
pub struct W(crate::W<MIN_DRIVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIN_DRIVE_SPEC>;
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
impl From<crate::W<MIN_DRIVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIN_DRIVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIN_DRIVE` reader - The minimum drive setting."]
pub type MIN_DRIVE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN_DRIVE` writer - The minimum drive setting."]
pub type MIN_DRIVE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MIN_DRIVE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The minimum drive setting."]
    #[inline(always)]
    pub fn min_drive(&self) -> MIN_DRIVE_R {
        MIN_DRIVE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - The minimum drive setting."]
    #[inline(always)]
    pub fn min_drive(&mut self) -> MIN_DRIVE_W<0> {
        MIN_DRIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the minimum drive setting for the RPM based Fan Control Algorithm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [min_drive](index.html) module"]
pub struct MIN_DRIVE_SPEC;
impl crate::RegisterSpec for MIN_DRIVE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [min_drive::R](R) reader structure"]
impl crate::Readable for MIN_DRIVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [min_drive::W](W) writer structure"]
impl crate::Writable for MIN_DRIVE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIN_DRIVE to value 0x66"]
impl crate::Resettable for MIN_DRIVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x66
    }
}
