#[doc = "Register `ADJ_CH4A` reader"]
pub struct R(crate::R<ADJ_CH4A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADJ_CH4A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADJ_CH4A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADJ_CH4A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADJ_CH4A` writer"]
pub struct W(crate::W<ADJ_CH4A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADJ_CH4A_SPEC>;
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
impl From<crate::W<ADJ_CH4A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADJ_CH4A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADJ_CH4A` reader - Contain EMC IP Trim Adjust values for External Channel 4A"]
pub type ADJ_CH4A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADJ_CH4A` writer - Contain EMC IP Trim Adjust values for External Channel 4A"]
pub type ADJ_CH4A_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADJ_CH4A_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Contain EMC IP Trim Adjust values for External Channel 4A"]
    #[inline(always)]
    pub fn adj_ch4a(&self) -> ADJ_CH4A_R {
        ADJ_CH4A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contain EMC IP Trim Adjust values for External Channel 4A"]
    #[inline(always)]
    pub fn adj_ch4a(&mut self) -> ADJ_CH4A_W<0> {
        ADJ_CH4A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Adjusted Channel 4A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adj_ch4a](index.html) module"]
pub struct ADJ_CH4A_SPEC;
impl crate::RegisterSpec for ADJ_CH4A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adj_ch4a::R](R) reader structure"]
impl crate::Readable for ADJ_CH4A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adj_ch4a::W](W) writer structure"]
impl crate::Writable for ADJ_CH4A_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADJ_CH4A to value 0"]
impl crate::Resettable for ADJ_CH4A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
