#[doc = "Register `STEP` reader"]
pub struct R(crate::R<STEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STEP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STEP` writer"]
pub struct W(crate::W<STEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STEP_SPEC>;
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
impl From<crate::W<STEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S0` reader - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 000."]
pub type S0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S0` writer - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 000."]
pub type S0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STEP_SPEC, u8, u8, 4, O>;
#[doc = "Field `S1` reader - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 001."]
pub type S1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S1` writer - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 001."]
pub type S1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STEP_SPEC, u8, u8, 4, O>;
#[doc = "Field `S2` reader - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 010."]
pub type S2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S2` writer - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 010."]
pub type S2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STEP_SPEC, u8, u8, 4, O>;
#[doc = "Field `S3` reader - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 011."]
pub type S3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S3` writer - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 011."]
pub type S3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STEP_SPEC, u8, u8, 4, O>;
#[doc = "Field `S4` reader - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 100."]
pub type S4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S4` writer - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 100."]
pub type S4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STEP_SPEC, u8, u8, 4, O>;
#[doc = "Field `S5` reader - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 101"]
pub type S5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S5` writer - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 101"]
pub type S5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STEP_SPEC, u8, u8, 4, O>;
#[doc = "Field `S6` reader - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 110."]
pub type S6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S6` writer - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 110."]
pub type S6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STEP_SPEC, u8, u8, 4, O>;
#[doc = "Field `S7` reader - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 111."]
pub type S7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S7` writer - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 111."]
pub type S7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STEP_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 000."]
    #[inline(always)]
    pub fn s0(&self) -> S0_R {
        S0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 001."]
    #[inline(always)]
    pub fn s1(&self) -> S1_R {
        S1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 010."]
    #[inline(always)]
    pub fn s2(&self) -> S2_R {
        S2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 011."]
    #[inline(always)]
    pub fn s3(&self) -> S3_R {
        S3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 100."]
    #[inline(always)]
    pub fn s4(&self) -> S4_R {
        S4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 101"]
    #[inline(always)]
    pub fn s5(&self) -> S5_R {
        S5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 110."]
    #[inline(always)]
    pub fn s6(&self) -> S6_R {
        S6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 111."]
    #[inline(always)]
    pub fn s7(&self) -> S7_R {
        S7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 000."]
    #[inline(always)]
    pub fn s0(&mut self) -> S0_W<0> {
        S0_W::new(self)
    }
    #[doc = "Bits 4:7 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 001."]
    #[inline(always)]
    pub fn s1(&mut self) -> S1_W<4> {
        S1_W::new(self)
    }
    #[doc = "Bits 8:11 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 010."]
    #[inline(always)]
    pub fn s2(&mut self) -> S2_W<8> {
        S2_W::new(self)
    }
    #[doc = "Bits 12:15 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 011."]
    #[inline(always)]
    pub fn s3(&mut self) -> S3_W<12> {
        S3_W::new(self)
    }
    #[doc = "Bits 16:19 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 100."]
    #[inline(always)]
    pub fn s4(&mut self) -> S4_W<16> {
        S4_W::new(self)
    }
    #[doc = "Bits 20:23 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 101"]
    #[inline(always)]
    pub fn s5(&mut self) -> S5_W<20> {
        S5_W::new(self)
    }
    #[doc = "Bits 24:27 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 110."]
    #[inline(always)]
    pub fn s6(&mut self) -> S6_W<24> {
        S6_W::new(self)
    }
    #[doc = "Bits 28:31 - Amount the current duty cycle is adjusted at the end of every PWM period when the segment index is equal to 111."]
    #[inline(always)]
    pub fn s7(&mut self) -> S7_W<28> {
        S7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register has eight segment fields which provide the amount the current duty cycle is adjusted at the end of every PWM period. Segment field selection is decoded based on the segment index. The segment index equation utilized depends on the SYMMETRY bit in the LED Configuration Register Register) . In Symmetric Mode the Segment_Index\\[2:0\\]
= Duty Cycle Bits\\[7:5\\]
. In Asymmetric Mode the Segment_Index\\[2:0\\]
is the bit concatenation of following: Segment_Index\\[2\\]
= (FALLING RAMP TIME in Figure 30-3, Clipping Example) and Segment_Index\\[1:0\\]
= Duty Cycle Bits\\[7:6\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [step](index.html) module"]
pub struct STEP_SPEC;
impl crate::RegisterSpec for STEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [step::R](R) reader structure"]
impl crate::Readable for STEP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [step::W](W) writer structure"]
impl crate::Writable for STEP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STEP to value 0"]
impl crate::Resettable for STEP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
