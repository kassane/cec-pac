#[doc = "Register `BAUDRT_MSB` reader"]
pub struct R(crate::R<BAUDRT_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUDRT_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUDRT_MSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUDRT_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUDRT_MSB` writer"]
pub struct W(crate::W<BAUDRT_MSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUDRT_MSB_SPEC>;
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
impl From<crate::W<BAUDRT_MSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUDRT_MSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUDRT_DIV_MSB` reader - Baud Rate divisor (MSB)."]
pub type BAUDRT_DIV_MSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BAUDRT_DIV_MSB` writer - Baud Rate divisor (MSB)."]
pub type BAUDRT_DIV_MSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, BAUDRT_MSB_SPEC, u8, u8, 7, O>;
#[doc = "Field `BAUD_CLK_SEL` reader - Baud Clock Selection"]
pub type BAUD_CLK_SEL_R = crate::BitReader<BAUD_CLK_SELSELECT_A>;
#[doc = "Baud Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BAUD_CLK_SELSELECT_A {
    #[doc = "0: Baud clock is derived from the 1.8432MHz Clk"]
    _1843200_HZ = 0,
    #[doc = "1: baud clock is derived from the 48MHz Clk"]
    _48000000_HZ = 1,
}
impl From<BAUD_CLK_SELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BAUD_CLK_SELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BAUD_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BAUD_CLK_SELSELECT_A {
        match self.bits {
            false => BAUD_CLK_SELSELECT_A::_1843200_HZ,
            true => BAUD_CLK_SELSELECT_A::_48000000_HZ,
        }
    }
    #[doc = "Checks if the value of the field is `_1843200_HZ`"]
    #[inline(always)]
    pub fn is_1843200_hz(&self) -> bool {
        *self == BAUD_CLK_SELSELECT_A::_1843200_HZ
    }
    #[doc = "Checks if the value of the field is `_48000000_HZ`"]
    #[inline(always)]
    pub fn is_48000000_hz(&self) -> bool {
        *self == BAUD_CLK_SELSELECT_A::_48000000_HZ
    }
}
#[doc = "Field `BAUD_CLK_SEL` writer - Baud Clock Selection"]
pub type BAUD_CLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, BAUDRT_MSB_SPEC, BAUD_CLK_SELSELECT_A, O>;
impl<'a, const O: u8> BAUD_CLK_SEL_W<'a, O> {
    #[doc = "Baud clock is derived from the 1.8432MHz Clk"]
    #[inline(always)]
    pub fn _1843200_hz(self) -> &'a mut W {
        self.variant(BAUD_CLK_SELSELECT_A::_1843200_HZ)
    }
    #[doc = "baud clock is derived from the 48MHz Clk"]
    #[inline(always)]
    pub fn _48000000_hz(self) -> &'a mut W {
        self.variant(BAUD_CLK_SELSELECT_A::_48000000_HZ)
    }
}
impl R {
    #[doc = "Bits 0:6 - Baud Rate divisor (MSB)."]
    #[inline(always)]
    pub fn baudrt_div_msb(&self) -> BAUDRT_DIV_MSB_R {
        BAUDRT_DIV_MSB_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Baud Clock Selection"]
    #[inline(always)]
    pub fn baud_clk_sel(&self) -> BAUD_CLK_SEL_R {
        BAUD_CLK_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Baud Rate divisor (MSB)."]
    #[inline(always)]
    pub fn baudrt_div_msb(&mut self) -> BAUDRT_DIV_MSB_W<0> {
        BAUDRT_DIV_MSB_W::new(self)
    }
    #[doc = "Bit 7 - Baud Clock Selection"]
    #[inline(always)]
    pub fn baud_clk_sel(&mut self) -> BAUD_CLK_SEL_W<7> {
        BAUD_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Programmable BAUD Rate Generator (MSB) Register (DLAB=1). \\[6:0\\]
BAUD_RATE_DIVISOR_MSB, \\[7:7\\]
BAUD_CLK_SEL 1=If CLK_SRC is '0', the baud clock is derived from the 1.8432MHz_Clk. If CLK_SRC is '1', this bit has no effect 0=If CLK_SRC is '0', the baud clock is derived from the 24MHz_Clk. If CLK_SRC is '1', this bit has no effect\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baudrt_msb](index.html) module"]
pub struct BAUDRT_MSB_SPEC;
impl crate::RegisterSpec for BAUDRT_MSB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [baudrt_msb::R](R) reader structure"]
impl crate::Readable for BAUDRT_MSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baudrt_msb::W](W) writer structure"]
impl crate::Writable for BAUDRT_MSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUDRT_MSB to value 0"]
impl crate::Resettable for BAUDRT_MSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
