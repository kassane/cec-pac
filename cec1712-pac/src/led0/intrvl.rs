#[doc = "Register `INTRVL` reader"]
pub struct R(crate::R<INTRVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTRVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTRVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTRVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTRVL` writer"]
pub struct W(crate::W<INTRVL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTRVL_SPEC>;
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
impl From<crate::W<INTRVL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTRVL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I0` reader - The number of PWM periods between updates to current duty cycle when the segment index is equal to 000b."]
pub type I0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I0` writer - The number of PWM periods between updates to current duty cycle when the segment index is equal to 000b."]
pub type I0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTRVL_SPEC, u8, u8, 4, O>;
#[doc = "Field `I1` reader - The number of PWM periods between updates to current duty cycle when the segment index is equal to 001b."]
pub type I1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I1` writer - The number of PWM periods between updates to current duty cycle when the segment index is equal to 001b."]
pub type I1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTRVL_SPEC, u8, u8, 4, O>;
#[doc = "Field `I2` reader - The number of PWM periods between updates to current duty cycle when the segment index is equal to 010b."]
pub type I2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2` writer - The number of PWM periods between updates to current duty cycle when the segment index is equal to 010b."]
pub type I2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTRVL_SPEC, u8, u8, 4, O>;
#[doc = "Field `I3` reader - The number of PWM periods between updates to current duty cycle when the segment index is equal to 011b."]
pub type I3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I3` writer - The number of PWM periods between updates to current duty cycle when the segment index is equal to 011b."]
pub type I3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTRVL_SPEC, u8, u8, 4, O>;
#[doc = "Field `I4` reader - The number of PWM periods between updates to current duty cycle when the segment index is equal to 100b."]
pub type I4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I4` writer - The number of PWM periods between updates to current duty cycle when the segment index is equal to 100b."]
pub type I4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTRVL_SPEC, u8, u8, 4, O>;
#[doc = "Field `I5` reader - The number of PWM periods between updates to current duty cycle when the segment index is equal to 101b."]
pub type I5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I5` writer - The number of PWM periods between updates to current duty cycle when the segment index is equal to 101b."]
pub type I5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTRVL_SPEC, u8, u8, 4, O>;
#[doc = "Field `I6` reader - The number of PWM periods between updates to current duty cycle when the segment index is equal to 110b."]
pub type I6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I6` writer - The number of PWM periods between updates to current duty cycle when the segment index is equal to 110b."]
pub type I6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTRVL_SPEC, u8, u8, 4, O>;
#[doc = "Field `I7` reader - The number of PWM periods between updates to current duty cycle when the segment index is equal to 111b."]
pub type I7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I7` writer - The number of PWM periods between updates to current duty cycle when the segment index is equal to 111b."]
pub type I7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTRVL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 000b."]
    #[inline(always)]
    pub fn i0(&self) -> I0_R {
        I0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 001b."]
    #[inline(always)]
    pub fn i1(&self) -> I1_R {
        I1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 010b."]
    #[inline(always)]
    pub fn i2(&self) -> I2_R {
        I2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 011b."]
    #[inline(always)]
    pub fn i3(&self) -> I3_R {
        I3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 100b."]
    #[inline(always)]
    pub fn i4(&self) -> I4_R {
        I4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 101b."]
    #[inline(always)]
    pub fn i5(&self) -> I5_R {
        I5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 110b."]
    #[inline(always)]
    pub fn i6(&self) -> I6_R {
        I6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 111b."]
    #[inline(always)]
    pub fn i7(&self) -> I7_R {
        I7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 000b."]
    #[inline(always)]
    pub fn i0(&mut self) -> I0_W<0> {
        I0_W::new(self)
    }
    #[doc = "Bits 4:7 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 001b."]
    #[inline(always)]
    pub fn i1(&mut self) -> I1_W<4> {
        I1_W::new(self)
    }
    #[doc = "Bits 8:11 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 010b."]
    #[inline(always)]
    pub fn i2(&mut self) -> I2_W<8> {
        I2_W::new(self)
    }
    #[doc = "Bits 12:15 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 011b."]
    #[inline(always)]
    pub fn i3(&mut self) -> I3_W<12> {
        I3_W::new(self)
    }
    #[doc = "Bits 16:19 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 100b."]
    #[inline(always)]
    pub fn i4(&mut self) -> I4_W<16> {
        I4_W::new(self)
    }
    #[doc = "Bits 20:23 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 101b."]
    #[inline(always)]
    pub fn i5(&mut self) -> I5_W<20> {
        I5_W::new(self)
    }
    #[doc = "Bits 24:27 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 110b."]
    #[inline(always)]
    pub fn i6(&mut self) -> I6_W<24> {
        I6_W::new(self)
    }
    #[doc = "Bits 28:31 - The number of PWM periods between updates to current duty cycle when the segment index is equal to 111b."]
    #[inline(always)]
    pub fn i7(&mut self) -> I7_W<28> {
        I7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LED Update Interval\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intrvl](index.html) module"]
pub struct INTRVL_SPEC;
impl crate::RegisterSpec for INTRVL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intrvl::R](R) reader structure"]
impl crate::Readable for INTRVL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intrvl::W](W) writer structure"]
impl crate::Writable for INTRVL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTRVL to value 0"]
impl crate::Resettable for INTRVL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
