#[doc = "Register `CAP0_CTRL` reader"]
pub struct R(crate::R<CAP0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP0_CTRL` writer"]
pub struct W(crate::W<CAP0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP0_CTRL_SPEC>;
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
impl From<crate::W<CAP0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_EDGE0` reader - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 0."]
pub type CAP_EDGE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP_EDGE0` writer - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 0."]
pub type CAP_EDGE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP0_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `FILTER_BYP0` reader - This bit enables bypassing the input noise filter for Capture Register 0, so that the input signal goes directly into the timer."]
pub type FILTER_BYP0_R = crate::BitReader<bool>;
#[doc = "Field `FILTER_BYP0` writer - This bit enables bypassing the input noise filter for Capture Register 0, so that the input signal goes directly into the timer."]
pub type FILTER_BYP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAP0_CTRL_SPEC, bool, O>;
#[doc = "Field `FCLK_SEL0` reader - This 3-bit field sets the clock source for the input filter for Capture Register 0."]
pub type FCLK_SEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCLK_SEL0` writer - This 3-bit field sets the clock source for the input filter for Capture Register 0."]
pub type FCLK_SEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP0_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CAP_EDGE1` reader - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 1."]
pub type CAP_EDGE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP_EDGE1` writer - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 1."]
pub type CAP_EDGE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP0_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `FILTER_BYP1` reader - This bit enables bypassing the input noise filter for Capture Register 1, so that the input signal goes directly into the timer."]
pub type FILTER_BYP1_R = crate::BitReader<bool>;
#[doc = "Field `FILTER_BYP1` writer - This bit enables bypassing the input noise filter for Capture Register 1, so that the input signal goes directly into the timer."]
pub type FILTER_BYP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAP0_CTRL_SPEC, bool, O>;
#[doc = "Field `FCLK_SEL1` reader - This 3-bit field sets the clock source for the input filter for Capture Register 1."]
pub type FCLK_SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCLK_SEL1` writer - This 3-bit field sets the clock source for the input filter for Capture Register 1."]
pub type FCLK_SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP0_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CAP_EDGE2` reader - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 2."]
pub type CAP_EDGE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP_EDGE2` writer - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 2."]
pub type CAP_EDGE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP0_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `FILTER_BYP2` reader - This bit enables bypassing the input noise filter for Capture Register 2, so that the input signal goes directly into the timer."]
pub type FILTER_BYP2_R = crate::BitReader<bool>;
#[doc = "Field `FILTER_BYP2` writer - This bit enables bypassing the input noise filter for Capture Register 2, so that the input signal goes directly into the timer."]
pub type FILTER_BYP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAP0_CTRL_SPEC, bool, O>;
#[doc = "Field `FCLK_SEL2` reader - This 3-bit field sets the clock source for the input filter for Capture Register 2."]
pub type FCLK_SEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCLK_SEL2` writer - This 3-bit field sets the clock source for the input filter for Capture Register 2."]
pub type FCLK_SEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP0_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CAP_EDGE3` reader - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 3."]
pub type CAP_EDGE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP_EDGE3` writer - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 3."]
pub type CAP_EDGE3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP0_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `FILTER_BYP3` reader - This bit enables bypassing the input noise filter for Capture Register 3, so that the input signal goes directly into the timer."]
pub type FILTER_BYP3_R = crate::BitReader<bool>;
#[doc = "Field `FILTER_BYP3` writer - This bit enables bypassing the input noise filter for Capture Register 3, so that the input signal goes directly into the timer."]
pub type FILTER_BYP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAP0_CTRL_SPEC, bool, O>;
#[doc = "Field `FCLK_SEL3` reader - This 3-bit field sets the clock source for the input filter for Capture Register 3."]
pub type FCLK_SEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCLK_SEL3` writer - This 3-bit field sets the clock source for the input filter for Capture Register 3."]
pub type FCLK_SEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP0_CTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 0."]
    #[inline(always)]
    pub fn cap_edge0(&self) -> CAP_EDGE0_R {
        CAP_EDGE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This bit enables bypassing the input noise filter for Capture Register 0, so that the input signal goes directly into the timer."]
    #[inline(always)]
    pub fn filter_byp0(&self) -> FILTER_BYP0_R {
        FILTER_BYP0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:7 - This 3-bit field sets the clock source for the input filter for Capture Register 0."]
    #[inline(always)]
    pub fn fclk_sel0(&self) -> FCLK_SEL0_R {
        FCLK_SEL0_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 1."]
    #[inline(always)]
    pub fn cap_edge1(&self) -> CAP_EDGE1_R {
        CAP_EDGE1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - This bit enables bypassing the input noise filter for Capture Register 1, so that the input signal goes directly into the timer."]
    #[inline(always)]
    pub fn filter_byp1(&self) -> FILTER_BYP1_R {
        FILTER_BYP1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:15 - This 3-bit field sets the clock source for the input filter for Capture Register 1."]
    #[inline(always)]
    pub fn fclk_sel1(&self) -> FCLK_SEL1_R {
        FCLK_SEL1_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 2."]
    #[inline(always)]
    pub fn cap_edge2(&self) -> CAP_EDGE2_R {
        CAP_EDGE2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - This bit enables bypassing the input noise filter for Capture Register 2, so that the input signal goes directly into the timer."]
    #[inline(always)]
    pub fn filter_byp2(&self) -> FILTER_BYP2_R {
        FILTER_BYP2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 21:23 - This 3-bit field sets the clock source for the input filter for Capture Register 2."]
    #[inline(always)]
    pub fn fclk_sel2(&self) -> FCLK_SEL2_R {
        FCLK_SEL2_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:25 - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 3."]
    #[inline(always)]
    pub fn cap_edge3(&self) -> CAP_EDGE3_R {
        CAP_EDGE3_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - This bit enables bypassing the input noise filter for Capture Register 3, so that the input signal goes directly into the timer."]
    #[inline(always)]
    pub fn filter_byp3(&self) -> FILTER_BYP3_R {
        FILTER_BYP3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 29:31 - This 3-bit field sets the clock source for the input filter for Capture Register 3."]
    #[inline(always)]
    pub fn fclk_sel3(&self) -> FCLK_SEL3_R {
        FCLK_SEL3_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 0."]
    #[inline(always)]
    pub fn cap_edge0(&mut self) -> CAP_EDGE0_W<0> {
        CAP_EDGE0_W::new(self)
    }
    #[doc = "Bit 2 - This bit enables bypassing the input noise filter for Capture Register 0, so that the input signal goes directly into the timer."]
    #[inline(always)]
    pub fn filter_byp0(&mut self) -> FILTER_BYP0_W<2> {
        FILTER_BYP0_W::new(self)
    }
    #[doc = "Bits 5:7 - This 3-bit field sets the clock source for the input filter for Capture Register 0."]
    #[inline(always)]
    pub fn fclk_sel0(&mut self) -> FCLK_SEL0_W<5> {
        FCLK_SEL0_W::new(self)
    }
    #[doc = "Bits 8:9 - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 1."]
    #[inline(always)]
    pub fn cap_edge1(&mut self) -> CAP_EDGE1_W<8> {
        CAP_EDGE1_W::new(self)
    }
    #[doc = "Bit 10 - This bit enables bypassing the input noise filter for Capture Register 1, so that the input signal goes directly into the timer."]
    #[inline(always)]
    pub fn filter_byp1(&mut self) -> FILTER_BYP1_W<10> {
        FILTER_BYP1_W::new(self)
    }
    #[doc = "Bits 13:15 - This 3-bit field sets the clock source for the input filter for Capture Register 1."]
    #[inline(always)]
    pub fn fclk_sel1(&mut self) -> FCLK_SEL1_W<13> {
        FCLK_SEL1_W::new(self)
    }
    #[doc = "Bits 16:17 - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 2."]
    #[inline(always)]
    pub fn cap_edge2(&mut self) -> CAP_EDGE2_W<16> {
        CAP_EDGE2_W::new(self)
    }
    #[doc = "Bit 18 - This bit enables bypassing the input noise filter for Capture Register 2, so that the input signal goes directly into the timer."]
    #[inline(always)]
    pub fn filter_byp2(&mut self) -> FILTER_BYP2_W<18> {
        FILTER_BYP2_W::new(self)
    }
    #[doc = "Bits 21:23 - This 3-bit field sets the clock source for the input filter for Capture Register 2."]
    #[inline(always)]
    pub fn fclk_sel2(&mut self) -> FCLK_SEL2_W<21> {
        FCLK_SEL2_W::new(self)
    }
    #[doc = "Bits 24:25 - This field selects the edge type that triggers the capture of the Free Running Counter into Capture Register 3."]
    #[inline(always)]
    pub fn cap_edge3(&mut self) -> CAP_EDGE3_W<24> {
        CAP_EDGE3_W::new(self)
    }
    #[doc = "Bit 26 - This bit enables bypassing the input noise filter for Capture Register 3, so that the input signal goes directly into the timer."]
    #[inline(always)]
    pub fn filter_byp3(&mut self) -> FILTER_BYP3_W<26> {
        FILTER_BYP3_W::new(self)
    }
    #[doc = "Bits 29:31 - This 3-bit field sets the clock source for the input filter for Capture Register 3."]
    #[inline(always)]
    pub fn fclk_sel3(&mut self) -> FCLK_SEL3_W<29> {
        FCLK_SEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure capture and compare timers 0-3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap0_ctrl](index.html) module"]
pub struct CAP0_CTRL_SPEC;
impl crate::RegisterSpec for CAP0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap0_ctrl::R](R) reader structure"]
impl crate::Readable for CAP0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap0_ctrl::W](W) writer structure"]
impl crate::Writable for CAP0_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP0_CTRL to value 0"]
impl crate::Resettable for CAP0_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
