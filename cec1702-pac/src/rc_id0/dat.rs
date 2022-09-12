#[doc = "Register `DAT` reader"]
pub struct R(crate::R<DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAT` writer"]
pub struct W(crate::W<DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAT_SPEC>;
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
impl From<crate::W<DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCID_DATA` reader - Reads of this register provide the result of an RC_ID measurement."]
pub type RCID_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RCID_DATA` writer - Reads of this register provide the result of an RC_ID measurement."]
pub type RCID_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Reads of this register provide the result of an RC_ID measurement."]
    #[inline(always)]
    pub fn rcid_data(&self) -> RCID_DATA_R {
        RCID_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Reads of this register provide the result of an RC_ID measurement."]
    #[inline(always)]
    pub fn rcid_data(&mut self) -> RCID_DATA_W<0> {
        RCID_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reads of this register provide the result of an RC_ID measurement.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dat](index.html) module"]
pub struct DAT_SPEC;
impl crate::RegisterSpec for DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dat::R](R) reader structure"]
impl crate::Readable for DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dat::W](W) writer structure"]
impl crate::Writable for DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAT to value 0"]
impl crate::Resettable for DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
