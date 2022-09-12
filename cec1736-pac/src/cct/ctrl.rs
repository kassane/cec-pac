#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT` reader - This bit is used to start the capture and compare timer running and power it down."]
pub type ACT_R = crate::BitReader<bool>;
#[doc = "Field `ACT` writer - This bit is used to start the capture and compare timer running and power it down."]
pub type ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FREE_EN` reader - Free-Running Timer Enable. This bit is used to start and stop the free running timer."]
pub type FREE_EN_R = crate::BitReader<bool>;
#[doc = "Field `FREE_EN` writer - Free-Running Timer Enable. This bit is used to start and stop the free running timer."]
pub type FREE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FREE_RST` reader - Free Running Timer Reset. This bit stops the timer and resets the internal counter to 0000_0000h."]
pub type FREE_RST_R = crate::BitReader<bool>;
#[doc = "Field `FREE_RST` writer - Free Running Timer Reset. This bit stops the timer and resets the internal counter to 0000_0000h."]
pub type FREE_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TCLK` reader - This 3-bit field sets the clock source for the Free-Running Counter."]
pub type TCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCLK` writer - This 3-bit field sets the clock source for the Free-Running Counter."]
pub type TCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CMP_EN0` reader - Compare Enable for Compare 0 Register."]
pub type CMP_EN0_R = crate::BitReader<bool>;
#[doc = "Field `CMP_EN0` writer - Compare Enable for Compare 0 Register."]
pub type CMP_EN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CMP_EN1` reader - Compare Enable for Compare 1 Register."]
pub type CMP_EN1_R = crate::BitReader<bool>;
#[doc = "Field `CMP_EN1` writer - Compare Enable for Compare 1 Register."]
pub type CMP_EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CMP_SET1` reader - When read, returns the current value off the Compare Timer Output 1 state."]
pub type CMP_SET1_R = crate::BitReader<bool>;
#[doc = "Field `CMP_SET1` writer - When read, returns the current value off the Compare Timer Output 1 state."]
pub type CMP_SET1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CMP_SET0` reader - When read, returns the current value off the Compare Timer Output 0 state."]
pub type CMP_SET0_R = crate::BitReader<bool>;
#[doc = "Field `CMP_SET0` writer - When read, returns the current value off the Compare Timer Output 0 state."]
pub type CMP_SET0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CMP_CLR1` reader - When read, returns the current value off the Compare Timer Output 1 state."]
pub type CMP_CLR1_R = crate::BitReader<bool>;
#[doc = "Field `CMP_CLR1` writer - When read, returns the current value off the Compare Timer Output 1 state."]
pub type CMP_CLR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CMP_CLR0` reader - When read, returns the current value off the Compare Timer Output 0 state."]
pub type CMP_CLR0_R = crate::BitReader<bool>;
#[doc = "Field `CMP_CLR0` writer - When read, returns the current value off the Compare Timer Output 0 state."]
pub type CMP_CLR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to start the capture and compare timer running and power it down."]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Free-Running Timer Enable. This bit is used to start and stop the free running timer."]
    #[inline(always)]
    pub fn free_en(&self) -> FREE_EN_R {
        FREE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Free Running Timer Reset. This bit stops the timer and resets the internal counter to 0000_0000h."]
    #[inline(always)]
    pub fn free_rst(&self) -> FREE_RST_R {
        FREE_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - This 3-bit field sets the clock source for the Free-Running Counter."]
    #[inline(always)]
    pub fn tclk(&self) -> TCLK_R {
        TCLK_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Compare Enable for Compare 0 Register."]
    #[inline(always)]
    pub fn cmp_en0(&self) -> CMP_EN0_R {
        CMP_EN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare Enable for Compare 1 Register."]
    #[inline(always)]
    pub fn cmp_en1(&self) -> CMP_EN1_R {
        CMP_EN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - When read, returns the current value off the Compare Timer Output 1 state."]
    #[inline(always)]
    pub fn cmp_set1(&self) -> CMP_SET1_R {
        CMP_SET1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When read, returns the current value off the Compare Timer Output 0 state."]
    #[inline(always)]
    pub fn cmp_set0(&self) -> CMP_SET0_R {
        CMP_SET0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - When read, returns the current value off the Compare Timer Output 1 state."]
    #[inline(always)]
    pub fn cmp_clr1(&self) -> CMP_CLR1_R {
        CMP_CLR1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When read, returns the current value off the Compare Timer Output 0 state."]
    #[inline(always)]
    pub fn cmp_clr0(&self) -> CMP_CLR0_R {
        CMP_CLR0_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to start the capture and compare timer running and power it down."]
    #[inline(always)]
    pub fn act(&mut self) -> ACT_W<0> {
        ACT_W::new(self)
    }
    #[doc = "Bit 1 - Free-Running Timer Enable. This bit is used to start and stop the free running timer."]
    #[inline(always)]
    pub fn free_en(&mut self) -> FREE_EN_W<1> {
        FREE_EN_W::new(self)
    }
    #[doc = "Bit 2 - Free Running Timer Reset. This bit stops the timer and resets the internal counter to 0000_0000h."]
    #[inline(always)]
    pub fn free_rst(&mut self) -> FREE_RST_W<2> {
        FREE_RST_W::new(self)
    }
    #[doc = "Bits 4:6 - This 3-bit field sets the clock source for the Free-Running Counter."]
    #[inline(always)]
    pub fn tclk(&mut self) -> TCLK_W<4> {
        TCLK_W::new(self)
    }
    #[doc = "Bit 8 - Compare Enable for Compare 0 Register."]
    #[inline(always)]
    pub fn cmp_en0(&mut self) -> CMP_EN0_W<8> {
        CMP_EN0_W::new(self)
    }
    #[doc = "Bit 9 - Compare Enable for Compare 1 Register."]
    #[inline(always)]
    pub fn cmp_en1(&mut self) -> CMP_EN1_W<9> {
        CMP_EN1_W::new(self)
    }
    #[doc = "Bit 16 - When read, returns the current value off the Compare Timer Output 1 state."]
    #[inline(always)]
    pub fn cmp_set1(&mut self) -> CMP_SET1_W<16> {
        CMP_SET1_W::new(self)
    }
    #[doc = "Bit 17 - When read, returns the current value off the Compare Timer Output 0 state."]
    #[inline(always)]
    pub fn cmp_set0(&mut self) -> CMP_SET0_W<17> {
        CMP_SET0_W::new(self)
    }
    #[doc = "Bit 24 - When read, returns the current value off the Compare Timer Output 1 state."]
    #[inline(always)]
    pub fn cmp_clr1(&mut self) -> CMP_CLR1_W<24> {
        CMP_CLR1_W::new(self)
    }
    #[doc = "Bit 25 - When read, returns the current value off the Compare Timer Output 0 state."]
    #[inline(always)]
    pub fn cmp_clr0(&mut self) -> CMP_CLR0_W<25> {
        CMP_CLR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the capture and compare timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
