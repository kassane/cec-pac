#[doc = "Register `MUX_SEL` reader"]
pub struct R(crate::R<MUX_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX_SEL` writer"]
pub struct W(crate::W<MUX_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX_SEL_SPEC>;
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
impl From<crate::W<MUX_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP0` reader - Mux Select for Capture 0 register."]
pub type CAP0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP0` writer - Mux Select for Capture 0 register."]
pub type CAP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MUX_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CAP1` reader - Mux Select for Capture 1 register."]
pub type CAP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP1` writer - Mux Select for Capture 1 register."]
pub type CAP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MUX_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CAP2` reader - Mux Select for Capture 2 register."]
pub type CAP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP2` writer - Mux Select for Capture 2 register."]
pub type CAP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MUX_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CAP3` reader - Mux Select for Capture 3 register."]
pub type CAP3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP3` writer - Mux Select for Capture 3 register."]
pub type CAP3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MUX_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CAP4` reader - Mux Select for Capture 4 register."]
pub type CAP4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP4` writer - Mux Select for Capture 4 register."]
pub type CAP4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MUX_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CAP5` reader - Mux Select for Capture 5 register."]
pub type CAP5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP5` writer - Mux Select for Capture 5 register."]
pub type CAP5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MUX_SEL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Mux Select for Capture 0 register."]
    #[inline(always)]
    pub fn cap0(&self) -> CAP0_R {
        CAP0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Mux Select for Capture 1 register."]
    #[inline(always)]
    pub fn cap1(&self) -> CAP1_R {
        CAP1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Mux Select for Capture 2 register."]
    #[inline(always)]
    pub fn cap2(&self) -> CAP2_R {
        CAP2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Mux Select for Capture 3 register."]
    #[inline(always)]
    pub fn cap3(&self) -> CAP3_R {
        CAP3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Mux Select for Capture 4 register."]
    #[inline(always)]
    pub fn cap4(&self) -> CAP4_R {
        CAP4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Mux Select for Capture 5 register."]
    #[inline(always)]
    pub fn cap5(&self) -> CAP5_R {
        CAP5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Mux Select for Capture 0 register."]
    #[inline(always)]
    pub fn cap0(&mut self) -> CAP0_W<0> {
        CAP0_W::new(self)
    }
    #[doc = "Bits 4:7 - Mux Select for Capture 1 register."]
    #[inline(always)]
    pub fn cap1(&mut self) -> CAP1_W<4> {
        CAP1_W::new(self)
    }
    #[doc = "Bits 8:11 - Mux Select for Capture 2 register."]
    #[inline(always)]
    pub fn cap2(&mut self) -> CAP2_W<8> {
        CAP2_W::new(self)
    }
    #[doc = "Bits 12:15 - Mux Select for Capture 3 register."]
    #[inline(always)]
    pub fn cap3(&mut self) -> CAP3_W<12> {
        CAP3_W::new(self)
    }
    #[doc = "Bits 16:19 - Mux Select for Capture 4 register."]
    #[inline(always)]
    pub fn cap4(&mut self) -> CAP4_W<16> {
        CAP4_W::new(self)
    }
    #[doc = "Bits 20:23 - Mux Select for Capture 5 register."]
    #[inline(always)]
    pub fn cap5(&mut self) -> CAP5_W<20> {
        CAP5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register selects the pin mapping to the capture register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux_sel](index.html) module"]
pub struct MUX_SEL_SPEC;
impl crate::RegisterSpec for MUX_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mux_sel::R](R) reader structure"]
impl crate::Readable for MUX_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux_sel::W](W) writer structure"]
impl crate::Writable for MUX_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUX_SEL to value 0x0054_3210"]
impl crate::Resettable for MUX_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0054_3210
    }
}
