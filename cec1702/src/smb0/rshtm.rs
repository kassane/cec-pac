#[doc = "Register `RSHTM` reader"]
pub struct R(crate::R<RSHTM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSHTM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSHTM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSHTM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSHTM` writer"]
pub struct W(crate::W<RSHTM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSHTM_SPEC>;
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
impl From<crate::W<RSHTM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSHTM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSHTM` reader - This is the value of the timing requirement tHd:Sta in the I2C specification for a repeated START bit. This is used to hold\n the clock until the Hold Time for the repeated Start Bit has been satisfied."]
pub type RSHTM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSHTM` writer - This is the value of the timing requirement tHd:Sta in the I2C specification for a repeated START bit. This is used to hold\n the clock until the Hold Time for the repeated Start Bit has been satisfied."]
pub type RSHTM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSHTM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This is the value of the timing requirement tHd:Sta in the I2C specification for a repeated START bit. This is used to hold\n the clock until the Hold Time for the repeated Start Bit has been satisfied."]
    #[inline(always)]
    pub fn rshtm(&self) -> RSHTM_R {
        RSHTM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This is the value of the timing requirement tHd:Sta in the I2C specification for a repeated START bit. This is used to hold\n the clock until the Hold Time for the repeated Start Bit has been satisfied."]
    #[inline(always)]
    pub fn rshtm(&mut self) -> RSHTM_W<0> {
        RSHTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Repeated Start Hold Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rshtm](index.html) module"]
pub struct RSHTM_SPEC;
impl crate::RegisterSpec for RSHTM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rshtm::R](R) reader structure"]
impl crate::Readable for RSHTM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rshtm::W](W) writer structure"]
impl crate::Writable for RSHTM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSHTM to value 0x4d"]
impl crate::Resettable for RSHTM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4d
    }
}
