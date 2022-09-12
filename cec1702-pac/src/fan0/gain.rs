#[doc = "Register `GAIN` reader"]
pub struct R(crate::R<GAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAIN` writer"]
pub struct W(crate::W<GAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAIN_SPEC>;
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
impl From<crate::W<GAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAINP` reader - The proportional gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x"]
pub type GAINP_R = crate::FieldReader<u8, GAINPSELECT_A>;
#[doc = "The proportional gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAINPSELECT_A {
    #[doc = "3: Gain Factor: 3=8x"]
    GAINP_8X = 3,
    #[doc = "2: Gain Factor: 2=4x"]
    GAINP_4X = 2,
    #[doc = "1: Gain Factor: 1=2x"]
    GAINP_2X = 1,
    #[doc = "0: Gain Factor: 0=1x"]
    GAINP_1X = 0,
}
impl From<GAINPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINPSELECT_A) -> Self {
        variant as _
    }
}
impl GAINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAINPSELECT_A {
        match self.bits {
            3 => GAINPSELECT_A::GAINP_8X,
            2 => GAINPSELECT_A::GAINP_4X,
            1 => GAINPSELECT_A::GAINP_2X,
            0 => GAINPSELECT_A::GAINP_1X,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GAINP_8X`"]
    #[inline(always)]
    pub fn is_gainp_8x(&self) -> bool {
        *self == GAINPSELECT_A::GAINP_8X
    }
    #[doc = "Checks if the value of the field is `GAINP_4X`"]
    #[inline(always)]
    pub fn is_gainp_4x(&self) -> bool {
        *self == GAINPSELECT_A::GAINP_4X
    }
    #[doc = "Checks if the value of the field is `GAINP_2X`"]
    #[inline(always)]
    pub fn is_gainp_2x(&self) -> bool {
        *self == GAINPSELECT_A::GAINP_2X
    }
    #[doc = "Checks if the value of the field is `GAINP_1X`"]
    #[inline(always)]
    pub fn is_gainp_1x(&self) -> bool {
        *self == GAINPSELECT_A::GAINP_1X
    }
}
#[doc = "Field `GAINP` writer - The proportional gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x"]
pub type GAINP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, GAIN_SPEC, u8, GAINPSELECT_A, 2, O>;
impl<'a, const O: u8> GAINP_W<'a, O> {
    #[doc = "Gain Factor: 3=8x"]
    #[inline(always)]
    pub fn gainp_8x(self) -> &'a mut W {
        self.variant(GAINPSELECT_A::GAINP_8X)
    }
    #[doc = "Gain Factor: 2=4x"]
    #[inline(always)]
    pub fn gainp_4x(self) -> &'a mut W {
        self.variant(GAINPSELECT_A::GAINP_4X)
    }
    #[doc = "Gain Factor: 1=2x"]
    #[inline(always)]
    pub fn gainp_2x(self) -> &'a mut W {
        self.variant(GAINPSELECT_A::GAINP_2X)
    }
    #[doc = "Gain Factor: 0=1x"]
    #[inline(always)]
    pub fn gainp_1x(self) -> &'a mut W {
        self.variant(GAINPSELECT_A::GAINP_1X)
    }
}
#[doc = "Field `GAINI` reader - The integral gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x"]
pub type GAINI_R = crate::FieldReader<u8, GAINISELECT_A>;
#[doc = "The integral gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAINISELECT_A {
    #[doc = "3: Gain Factor: 3=8x"]
    GAINI_8X = 3,
    #[doc = "2: Gain Factor: 2=4x"]
    GAINI_4X = 2,
    #[doc = "1: Gain Factor: 1=2x"]
    GAINI_2X = 1,
    #[doc = "0: Gain Factor: 0=1x"]
    GAINI_1X = 0,
}
impl From<GAINISELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINISELECT_A) -> Self {
        variant as _
    }
}
impl GAINI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAINISELECT_A {
        match self.bits {
            3 => GAINISELECT_A::GAINI_8X,
            2 => GAINISELECT_A::GAINI_4X,
            1 => GAINISELECT_A::GAINI_2X,
            0 => GAINISELECT_A::GAINI_1X,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GAINI_8X`"]
    #[inline(always)]
    pub fn is_gaini_8x(&self) -> bool {
        *self == GAINISELECT_A::GAINI_8X
    }
    #[doc = "Checks if the value of the field is `GAINI_4X`"]
    #[inline(always)]
    pub fn is_gaini_4x(&self) -> bool {
        *self == GAINISELECT_A::GAINI_4X
    }
    #[doc = "Checks if the value of the field is `GAINI_2X`"]
    #[inline(always)]
    pub fn is_gaini_2x(&self) -> bool {
        *self == GAINISELECT_A::GAINI_2X
    }
    #[doc = "Checks if the value of the field is `GAINI_1X`"]
    #[inline(always)]
    pub fn is_gaini_1x(&self) -> bool {
        *self == GAINISELECT_A::GAINI_1X
    }
}
#[doc = "Field `GAINI` writer - The integral gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x"]
pub type GAINI_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, GAIN_SPEC, u8, GAINISELECT_A, 2, O>;
impl<'a, const O: u8> GAINI_W<'a, O> {
    #[doc = "Gain Factor: 3=8x"]
    #[inline(always)]
    pub fn gaini_8x(self) -> &'a mut W {
        self.variant(GAINISELECT_A::GAINI_8X)
    }
    #[doc = "Gain Factor: 2=4x"]
    #[inline(always)]
    pub fn gaini_4x(self) -> &'a mut W {
        self.variant(GAINISELECT_A::GAINI_4X)
    }
    #[doc = "Gain Factor: 1=2x"]
    #[inline(always)]
    pub fn gaini_2x(self) -> &'a mut W {
        self.variant(GAINISELECT_A::GAINI_2X)
    }
    #[doc = "Gain Factor: 0=1x"]
    #[inline(always)]
    pub fn gaini_1x(self) -> &'a mut W {
        self.variant(GAINISELECT_A::GAINI_1X)
    }
}
#[doc = "Field `GAIND` reader - The derivative gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x"]
pub type GAIND_R = crate::FieldReader<u8, GAINDSELECT_A>;
#[doc = "The derivative gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAINDSELECT_A {
    #[doc = "3: Gain Factor: 3=8x"]
    GAIND_8X = 3,
    #[doc = "2: Gain Factor: 2=4x"]
    GAIND_4X = 2,
    #[doc = "1: Gain Factor: 1=2x"]
    GAIND_2X = 1,
    #[doc = "0: Gain Factor: 0=1x"]
    GAIND_1X = 0,
}
impl From<GAINDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINDSELECT_A) -> Self {
        variant as _
    }
}
impl GAIND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAINDSELECT_A {
        match self.bits {
            3 => GAINDSELECT_A::GAIND_8X,
            2 => GAINDSELECT_A::GAIND_4X,
            1 => GAINDSELECT_A::GAIND_2X,
            0 => GAINDSELECT_A::GAIND_1X,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GAIND_8X`"]
    #[inline(always)]
    pub fn is_gaind_8x(&self) -> bool {
        *self == GAINDSELECT_A::GAIND_8X
    }
    #[doc = "Checks if the value of the field is `GAIND_4X`"]
    #[inline(always)]
    pub fn is_gaind_4x(&self) -> bool {
        *self == GAINDSELECT_A::GAIND_4X
    }
    #[doc = "Checks if the value of the field is `GAIND_2X`"]
    #[inline(always)]
    pub fn is_gaind_2x(&self) -> bool {
        *self == GAINDSELECT_A::GAIND_2X
    }
    #[doc = "Checks if the value of the field is `GAIND_1X`"]
    #[inline(always)]
    pub fn is_gaind_1x(&self) -> bool {
        *self == GAINDSELECT_A::GAIND_1X
    }
}
#[doc = "Field `GAIND` writer - The derivative gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x"]
pub type GAIND_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, GAIN_SPEC, u8, GAINDSELECT_A, 2, O>;
impl<'a, const O: u8> GAIND_W<'a, O> {
    #[doc = "Gain Factor: 3=8x"]
    #[inline(always)]
    pub fn gaind_8x(self) -> &'a mut W {
        self.variant(GAINDSELECT_A::GAIND_8X)
    }
    #[doc = "Gain Factor: 2=4x"]
    #[inline(always)]
    pub fn gaind_4x(self) -> &'a mut W {
        self.variant(GAINDSELECT_A::GAIND_4X)
    }
    #[doc = "Gain Factor: 1=2x"]
    #[inline(always)]
    pub fn gaind_2x(self) -> &'a mut W {
        self.variant(GAINDSELECT_A::GAIND_2X)
    }
    #[doc = "Gain Factor: 0=1x"]
    #[inline(always)]
    pub fn gaind_1x(self) -> &'a mut W {
        self.variant(GAINDSELECT_A::GAIND_1X)
    }
}
impl R {
    #[doc = "Bits 0:1 - The proportional gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x"]
    #[inline(always)]
    pub fn gainp(&self) -> GAINP_R {
        GAINP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The integral gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x"]
    #[inline(always)]
    pub fn gaini(&self) -> GAINI_R {
        GAINI_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - The derivative gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x"]
    #[inline(always)]
    pub fn gaind(&self) -> GAIND_R {
        GAIND_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - The proportional gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x"]
    #[inline(always)]
    pub fn gainp(&mut self) -> GAINP_W<0> {
        GAINP_W::new(self)
    }
    #[doc = "Bits 2:3 - The integral gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x"]
    #[inline(always)]
    pub fn gaini(&mut self) -> GAINI_W<2> {
        GAINI_W::new(self)
    }
    #[doc = "Bits 4:5 - The derivative gain term.\n Gain Factor:\n 3=8x\n 2=4x\n 1=2x\n 0=1x"]
    #[inline(always)]
    pub fn gaind(&mut self) -> GAIND_W<4> {
        GAIND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gain Register stores the gain terms used by the proportional and integral portions of the RPM based Fan Control Algorithm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gain](index.html) module"]
pub struct GAIN_SPEC;
impl crate::RegisterSpec for GAIN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gain::R](R) reader structure"]
impl crate::Readable for GAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gain::W](W) writer structure"]
impl crate::Writable for GAIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GAIN to value 0x2a"]
impl crate::Resettable for GAIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2a
    }
}
