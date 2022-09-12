#[doc = "Register `VREF_CHAN` reader"]
pub struct R(crate::R<VREF_CHAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREF_CHAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREF_CHAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREF_CHAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREF_CHAN` writer"]
pub struct W(crate::W<VREF_CHAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREF_CHAN_SPEC>;
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
impl From<crate::W<VREF_CHAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREF_CHAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL0` reader - These bits define the reference voltage for Channel 0.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL0` writer - These bits define the reference voltage for Channel 0.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL1` reader - These bits define the reference voltage for Channel 1.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL1` writer - These bits define the reference voltage for Channel 1.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL2` reader - These bits define the reference voltage for Channel 2.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL2` writer - These bits define the reference voltage for Channel 2.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL3` reader - These bits define the reference voltage for Channel 3.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL3` writer - These bits define the reference voltage for Channel 3.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL4` reader - These bits define the reference voltage for Channel 4.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL4` writer - These bits define the reference voltage for Channel 4.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL5` reader - These bits define the reference voltage for Channel 5.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL5` writer - These bits define the reference voltage for Channel 5.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL6` reader - These bits define the reference voltage for Channel 6.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL6` writer - These bits define the reference voltage for Channel 6.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL7` reader - These bits define the reference voltage for Channel 7.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL7` writer - These bits define the reference voltage for Channel 7.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL8` reader - These bits define the reference voltage for Channel 8.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL8` writer - These bits define the reference voltage for Channel 8.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL9` reader - These bits define the reference voltage for Channel 9.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL9` writer - These bits define the reference voltage for Channel 9.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL10` reader - These bits define the reference voltage for Channel 10.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL10` writer - These bits define the reference voltage for Channel 10.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL11` reader - These bits define the reference voltage for Channel 11.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL11` writer - These bits define the reference voltage for Channel 11.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL12` reader - These bits define the reference voltage for Channel 12.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL12` writer - These bits define the reference voltage for Channel 12.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL13` reader - These bits define the reference voltage for Channel 13.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL13` writer - These bits define the reference voltage for Channel 13.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL14` reader - These bits define the reference voltage for Channel 14.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL14` writer - These bits define the reference voltage for Channel 14.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEL15` reader - These bits define the reference voltage for Channel 15.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL15` writer - These bits define the reference voltage for Channel 15.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
pub type SEL15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CHAN_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - These bits define the reference voltage for Channel 0.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel0(&self) -> SEL0_R {
        SEL0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - These bits define the reference voltage for Channel 1.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel1(&self) -> SEL1_R {
        SEL1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - These bits define the reference voltage for Channel 2.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel2(&self) -> SEL2_R {
        SEL2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - These bits define the reference voltage for Channel 3.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel3(&self) -> SEL3_R {
        SEL3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - These bits define the reference voltage for Channel 4.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel4(&self) -> SEL4_R {
        SEL4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - These bits define the reference voltage for Channel 5.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel5(&self) -> SEL5_R {
        SEL5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - These bits define the reference voltage for Channel 6.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel6(&self) -> SEL6_R {
        SEL6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - These bits define the reference voltage for Channel 7.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel7(&self) -> SEL7_R {
        SEL7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - These bits define the reference voltage for Channel 8.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL8_R {
        SEL8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - These bits define the reference voltage for Channel 9.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel9(&self) -> SEL9_R {
        SEL9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - These bits define the reference voltage for Channel 10.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel10(&self) -> SEL10_R {
        SEL10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - These bits define the reference voltage for Channel 11.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel11(&self) -> SEL11_R {
        SEL11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - These bits define the reference voltage for Channel 12.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL12_R {
        SEL12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - These bits define the reference voltage for Channel 13.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel13(&self) -> SEL13_R {
        SEL13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - These bits define the reference voltage for Channel 14.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel14(&self) -> SEL14_R {
        SEL14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - These bits define the reference voltage for Channel 15.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel15(&self) -> SEL15_R {
        SEL15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - These bits define the reference voltage for Channel 0.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel0(&mut self) -> SEL0_W<0> {
        SEL0_W::new(self)
    }
    #[doc = "Bits 2:3 - These bits define the reference voltage for Channel 1.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel1(&mut self) -> SEL1_W<2> {
        SEL1_W::new(self)
    }
    #[doc = "Bits 4:5 - These bits define the reference voltage for Channel 2.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel2(&mut self) -> SEL2_W<4> {
        SEL2_W::new(self)
    }
    #[doc = "Bits 6:7 - These bits define the reference voltage for Channel 3.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel3(&mut self) -> SEL3_W<6> {
        SEL3_W::new(self)
    }
    #[doc = "Bits 8:9 - These bits define the reference voltage for Channel 4.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel4(&mut self) -> SEL4_W<8> {
        SEL4_W::new(self)
    }
    #[doc = "Bits 10:11 - These bits define the reference voltage for Channel 5.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel5(&mut self) -> SEL5_W<10> {
        SEL5_W::new(self)
    }
    #[doc = "Bits 12:13 - These bits define the reference voltage for Channel 6.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel6(&mut self) -> SEL6_W<12> {
        SEL6_W::new(self)
    }
    #[doc = "Bits 14:15 - These bits define the reference voltage for Channel 7.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel7(&mut self) -> SEL7_W<14> {
        SEL7_W::new(self)
    }
    #[doc = "Bits 16:17 - These bits define the reference voltage for Channel 8.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel8(&mut self) -> SEL8_W<16> {
        SEL8_W::new(self)
    }
    #[doc = "Bits 18:19 - These bits define the reference voltage for Channel 9.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel9(&mut self) -> SEL9_W<18> {
        SEL9_W::new(self)
    }
    #[doc = "Bits 20:21 - These bits define the reference voltage for Channel 10.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel10(&mut self) -> SEL10_W<20> {
        SEL10_W::new(self)
    }
    #[doc = "Bits 22:23 - These bits define the reference voltage for Channel 11.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel11(&mut self) -> SEL11_W<22> {
        SEL11_W::new(self)
    }
    #[doc = "Bits 24:25 - These bits define the reference voltage for Channel 12.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel12(&mut self) -> SEL12_W<24> {
        SEL12_W::new(self)
    }
    #[doc = "Bits 26:27 - These bits define the reference voltage for Channel 13.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel13(&mut self) -> SEL13_W<26> {
        SEL13_W::new(self)
    }
    #[doc = "Bits 28:29 - These bits define the reference voltage for Channel 14.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel14(&mut self) -> SEL14_W<28> {
        SEL14_W::new(self)
    }
    #[doc = "Bits 30:31 - These bits define the reference voltage for Channel 15.\n 0h= VREF0\n 1h= VREF1\n 2h= Reserved\n 3h= Reserved\n"]
    #[inline(always)]
    pub fn sel15(&mut self) -> SEL15_W<30> {
        SEL15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The ADC Channel Register is used to configure the reference voltage to the clock timing.\n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref_chan](index.html) module"]
pub struct VREF_CHAN_SPEC;
impl crate::RegisterSpec for VREF_CHAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vref_chan::R](R) reader structure"]
impl crate::Readable for VREF_CHAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vref_chan::W](W) writer structure"]
impl crate::Writable for VREF_CHAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREF_CHAN to value 0"]
impl crate::Resettable for VREF_CHAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
