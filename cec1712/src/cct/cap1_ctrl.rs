#[doc = "Register `CAP1_CTRL` reader"]
pub struct R(crate::R<CAP1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP1_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP1_CTRL` writer"]
pub struct W(crate::W<CAP1_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP1_CTRL_SPEC>;
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
impl From<crate::W<CAP1_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP1_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_EDGE4` reader - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 4."]
pub type CAP_EDGE4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP_EDGE4` writer - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 4."]
pub type CAP_EDGE4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP1_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `FILTER_BYP4` reader - This bit enables bypassing the input noise filter for Capture Register 4, so that the input signal goes directly into the timer."]
pub type FILTER_BYP4_R = crate::BitReader<bool>;
#[doc = "Field `FILTER_BYP4` writer - This bit enables bypassing the input noise filter for Capture Register 4, so that the input signal goes directly into the timer."]
pub type FILTER_BYP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAP1_CTRL_SPEC, bool, O>;
#[doc = "Field `FCLK_SEL4` reader - This 3-bit field sets the clock source for the input filter for Capture Register 4."]
pub type FCLK_SEL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCLK_SEL4` writer - This 3-bit field sets the clock source for the input filter for Capture Register 4."]
pub type FCLK_SEL4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP1_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CAP_EDGE5` reader - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 5."]
pub type CAP_EDGE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP_EDGE5` writer - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 5."]
pub type CAP_EDGE5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP1_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `FILTER_BYP5` reader - This bit enables bypassing the input noise filter for Capture Register 5, so that the input signal goes directly into the timer."]
pub type FILTER_BYP5_R = crate::BitReader<bool>;
#[doc = "Field `FILTER_BYP5` writer - This bit enables bypassing the input noise filter for Capture Register 5, so that the input signal goes directly into the timer."]
pub type FILTER_BYP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAP1_CTRL_SPEC, bool, O>;
#[doc = "Field `FCLK_SEL5` reader - This 3-bit field sets the clock source for the input filter for Capture Register 5."]
pub type FCLK_SEL5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCLK_SEL5` writer - This 3-bit field sets the clock source for the input filter for Capture Register 5."]
pub type FCLK_SEL5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP1_CTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 4."]
    #[inline(always)]
    pub fn cap_edge4(&self) -> CAP_EDGE4_R {
        CAP_EDGE4_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This bit enables bypassing the input noise filter for Capture Register 4, so that the input signal goes directly into the timer."]
    #[inline(always)]
    pub fn filter_byp4(&self) -> FILTER_BYP4_R {
        FILTER_BYP4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:7 - This 3-bit field sets the clock source for the input filter for Capture Register 4."]
    #[inline(always)]
    pub fn fclk_sel4(&self) -> FCLK_SEL4_R {
        FCLK_SEL4_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 5."]
    #[inline(always)]
    pub fn cap_edge5(&self) -> CAP_EDGE5_R {
        CAP_EDGE5_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - This bit enables bypassing the input noise filter for Capture Register 5, so that the input signal goes directly into the timer."]
    #[inline(always)]
    pub fn filter_byp5(&self) -> FILTER_BYP5_R {
        FILTER_BYP5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:15 - This 3-bit field sets the clock source for the input filter for Capture Register 5."]
    #[inline(always)]
    pub fn fclk_sel5(&self) -> FCLK_SEL5_R {
        FCLK_SEL5_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 4."]
    #[inline(always)]
    pub fn cap_edge4(&mut self) -> CAP_EDGE4_W<0> {
        CAP_EDGE4_W::new(self)
    }
    #[doc = "Bit 2 - This bit enables bypassing the input noise filter for Capture Register 4, so that the input signal goes directly into the timer."]
    #[inline(always)]
    pub fn filter_byp4(&mut self) -> FILTER_BYP4_W<2> {
        FILTER_BYP4_W::new(self)
    }
    #[doc = "Bits 5:7 - This 3-bit field sets the clock source for the input filter for Capture Register 4."]
    #[inline(always)]
    pub fn fclk_sel4(&mut self) -> FCLK_SEL4_W<5> {
        FCLK_SEL4_W::new(self)
    }
    #[doc = "Bits 8:9 - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 5."]
    #[inline(always)]
    pub fn cap_edge5(&mut self) -> CAP_EDGE5_W<8> {
        CAP_EDGE5_W::new(self)
    }
    #[doc = "Bit 10 - This bit enables bypassing the input noise filter for Capture Register 5, so that the input signal goes directly into the timer."]
    #[inline(always)]
    pub fn filter_byp5(&mut self) -> FILTER_BYP5_W<10> {
        FILTER_BYP5_W::new(self)
    }
    #[doc = "Bits 13:15 - This 3-bit field sets the clock source for the input filter for Capture Register 5."]
    #[inline(always)]
    pub fn fclk_sel5(&mut self) -> FCLK_SEL5_W<13> {
        FCLK_SEL5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure capture and compare timers 4-5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap1_ctrl](index.html) module"]
pub struct CAP1_CTRL_SPEC;
impl crate::RegisterSpec for CAP1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap1_ctrl::R](R) reader structure"]
impl crate::Readable for CAP1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap1_ctrl::W](W) writer structure"]
impl crate::Writable for CAP1_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP1_CTRL to value 0"]
impl crate::Resettable for CAP1_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
