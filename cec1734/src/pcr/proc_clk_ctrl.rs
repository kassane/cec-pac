#[doc = "Register `PROC_CLK_CTRL` reader"]
pub struct R(crate::R<PROC_CLK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC_CLK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC_CLK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC_CLK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROC_CLK_CTRL` writer"]
pub struct W(crate::W<PROC_CLK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROC_CLK_CTRL_SPEC>;
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
impl From<crate::W<PROC_CLK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROC_CLK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Selects the EC clock rate"]
pub type DIV_R = crate::FieldReader<u8, DIVSELECT_A>;
#[doc = "Selects the EC clock rate\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVSELECT_A {
    #[doc = "1: Divide 96 MHz clock by 1 (96 MHz Processor Clock)"]
    DIVIDE_BY_1 = 1,
    #[doc = "2: Divide 96 MHz clock by 2 (48 MHz Processor Clock)"]
    DIVIDE_BY_2 = 2,
    #[doc = "4: Divide 96 MHz clock by 4 (24 MHz Processor Clock)"]
    DIVIDE_BY_4 = 4,
    #[doc = "16: Divide 96 MHz clock by 16 (6 MHz Processor Clock)"]
    DIVIDE_BY_16 = 16,
    #[doc = "48: Divide 96 MHz clock by 48 (2 MHz Processor Clock)"]
    DIVIDE_BY_48 = 48,
}
impl From<DIVSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVSELECT_A) -> Self {
        variant as _
    }
}
impl DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVSELECT_A> {
        match self.bits {
            1 => Some(DIVSELECT_A::DIVIDE_BY_1),
            2 => Some(DIVSELECT_A::DIVIDE_BY_2),
            4 => Some(DIVSELECT_A::DIVIDE_BY_4),
            16 => Some(DIVSELECT_A::DIVIDE_BY_16),
            48 => Some(DIVSELECT_A::DIVIDE_BY_48),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_1`"]
    #[inline(always)]
    pub fn is_divide_by_1(&self) -> bool {
        *self == DIVSELECT_A::DIVIDE_BY_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_2`"]
    #[inline(always)]
    pub fn is_divide_by_2(&self) -> bool {
        *self == DIVSELECT_A::DIVIDE_BY_2
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_4`"]
    #[inline(always)]
    pub fn is_divide_by_4(&self) -> bool {
        *self == DIVSELECT_A::DIVIDE_BY_4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_16`"]
    #[inline(always)]
    pub fn is_divide_by_16(&self) -> bool {
        *self == DIVSELECT_A::DIVIDE_BY_16
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_48`"]
    #[inline(always)]
    pub fn is_divide_by_48(&self) -> bool {
        *self == DIVSELECT_A::DIVIDE_BY_48
    }
}
#[doc = "Field `DIV` writer - Selects the EC clock rate"]
pub type DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PROC_CLK_CTRL_SPEC, u8, DIVSELECT_A, 8, O>;
impl<'a, const O: u8> DIV_W<'a, O> {
    #[doc = "Divide 96 MHz clock by 1 (96 MHz Processor Clock)"]
    #[inline(always)]
    pub fn divide_by_1(self) -> &'a mut W {
        self.variant(DIVSELECT_A::DIVIDE_BY_1)
    }
    #[doc = "Divide 96 MHz clock by 2 (48 MHz Processor Clock)"]
    #[inline(always)]
    pub fn divide_by_2(self) -> &'a mut W {
        self.variant(DIVSELECT_A::DIVIDE_BY_2)
    }
    #[doc = "Divide 96 MHz clock by 4 (24 MHz Processor Clock)"]
    #[inline(always)]
    pub fn divide_by_4(self) -> &'a mut W {
        self.variant(DIVSELECT_A::DIVIDE_BY_4)
    }
    #[doc = "Divide 96 MHz clock by 16 (6 MHz Processor Clock)"]
    #[inline(always)]
    pub fn divide_by_16(self) -> &'a mut W {
        self.variant(DIVSELECT_A::DIVIDE_BY_16)
    }
    #[doc = "Divide 96 MHz clock by 48 (2 MHz Processor Clock)"]
    #[inline(always)]
    pub fn divide_by_48(self) -> &'a mut W {
        self.variant(DIVSELECT_A::DIVIDE_BY_48)
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects the EC clock rate"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects the EC clock rate"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Processor Clock Control Register \\[7:0\\]
Processor Clock Divide Value (PROC_DIV)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc_clk_ctrl](index.html) module"]
pub struct PROC_CLK_CTRL_SPEC;
impl crate::RegisterSpec for PROC_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proc_clk_ctrl::R](R) reader structure"]
impl crate::Readable for PROC_CLK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [proc_clk_ctrl::W](W) writer structure"]
impl crate::Writable for PROC_CLK_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROC_CLK_CTRL to value 0x04"]
impl crate::Resettable for PROC_CLK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
