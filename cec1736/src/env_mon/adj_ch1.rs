#[doc = "Register `ADJ_CH1` reader"]
pub struct R(crate::R<ADJ_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADJ_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADJ_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADJ_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADJ_CH1` writer"]
pub struct W(crate::W<ADJ_CH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADJ_CH1_SPEC>;
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
impl From<crate::W<ADJ_CH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADJ_CH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADJ_CH1` reader - Contain EMC IP Trim Adjust values for External Channel 1"]
pub type ADJ_CH1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADJ_CH1` writer - Contain EMC IP Trim Adjust values for External Channel 1"]
pub type ADJ_CH1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADJ_CH1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Contain EMC IP Trim Adjust values for External Channel 1"]
    #[inline(always)]
    pub fn adj_ch1(&self) -> ADJ_CH1_R {
        ADJ_CH1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contain EMC IP Trim Adjust values for External Channel 1"]
    #[inline(always)]
    pub fn adj_ch1(&mut self) -> ADJ_CH1_W<0> {
        ADJ_CH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Adjusted Channel 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adj_ch1](index.html) module"]
pub struct ADJ_CH1_SPEC;
impl crate::RegisterSpec for ADJ_CH1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adj_ch1::R](R) reader structure"]
impl crate::Readable for ADJ_CH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adj_ch1::W](W) writer structure"]
impl crate::Writable for ADJ_CH1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADJ_CH1 to value 0"]
impl crate::Resettable for ADJ_CH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
