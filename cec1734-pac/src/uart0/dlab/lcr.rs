#[doc = "Register `LCR` reader"]
pub struct R(crate::R<LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCR` writer"]
pub struct W(crate::W<LCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCR_SPEC>;
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
impl From<crate::W<LCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WORD_LEN` reader - WORD_LENGTH These two bits specify the number of bits in each transmitted or received serial character."]
pub type WORD_LEN_R = crate::FieldReader<u8, WORD_LENSELECT_A>;
#[doc = "WORD_LENGTH These two bits specify the number of bits in each transmitted or received serial character.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WORD_LENSELECT_A {
    #[doc = "0: 5 Bits"]
    _5_BIT = 0,
    #[doc = "1: 6 Bits"]
    _6_BIT = 1,
    #[doc = "2: 7 Bits"]
    _7_BIT = 2,
    #[doc = "3: 8 Bits"]
    _8_BIT = 3,
}
impl From<WORD_LENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WORD_LENSELECT_A) -> Self {
        variant as _
    }
}
impl WORD_LEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WORD_LENSELECT_A {
        match self.bits {
            0 => WORD_LENSELECT_A::_5_BIT,
            1 => WORD_LENSELECT_A::_6_BIT,
            2 => WORD_LENSELECT_A::_7_BIT,
            3 => WORD_LENSELECT_A::_8_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5_BIT`"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == WORD_LENSELECT_A::_5_BIT
    }
    #[doc = "Checks if the value of the field is `_6_BIT`"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == WORD_LENSELECT_A::_6_BIT
    }
    #[doc = "Checks if the value of the field is `_7_BIT`"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == WORD_LENSELECT_A::_7_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == WORD_LENSELECT_A::_8_BIT
    }
}
#[doc = "Field `WORD_LEN` writer - WORD_LENGTH These two bits specify the number of bits in each transmitted or received serial character."]
pub type WORD_LEN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, LCR_SPEC, u8, WORD_LENSELECT_A, 2, O>;
impl<'a, const O: u8> WORD_LEN_W<'a, O> {
    #[doc = "5 Bits"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut W {
        self.variant(WORD_LENSELECT_A::_5_BIT)
    }
    #[doc = "6 Bits"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut W {
        self.variant(WORD_LENSELECT_A::_6_BIT)
    }
    #[doc = "7 Bits"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut W {
        self.variant(WORD_LENSELECT_A::_7_BIT)
    }
    #[doc = "8 Bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(WORD_LENSELECT_A::_8_BIT)
    }
}
#[doc = "Field `STOP_BITS` reader - STOP_BITS This bit specifies the number of stop bits in each transmitted or received serial character."]
pub type STOP_BITS_R = crate::BitReader<STOP_BITSSELECT_A>;
#[doc = "STOP_BITS This bit specifies the number of stop bits in each transmitted or received serial character.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_BITSSELECT_A {
    #[doc = "0: 1 Stop bit"]
    ONE_STOP_BIT = 0,
    #[doc = "1: 1.5 or 2 Stop bits"]
    ONE_FIVE_TWO_STOP_BITS = 1,
}
impl From<STOP_BITSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_BITSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_BITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_BITSSELECT_A {
        match self.bits {
            false => STOP_BITSSELECT_A::ONE_STOP_BIT,
            true => STOP_BITSSELECT_A::ONE_FIVE_TWO_STOP_BITS,
        }
    }
    #[doc = "Checks if the value of the field is `ONE_STOP_BIT`"]
    #[inline(always)]
    pub fn is_one_stop_bit(&self) -> bool {
        *self == STOP_BITSSELECT_A::ONE_STOP_BIT
    }
    #[doc = "Checks if the value of the field is `ONE_FIVE_TWO_STOP_BITS`"]
    #[inline(always)]
    pub fn is_one_five_two_stop_bits(&self) -> bool {
        *self == STOP_BITSSELECT_A::ONE_FIVE_TWO_STOP_BITS
    }
}
#[doc = "Field `STOP_BITS` writer - STOP_BITS This bit specifies the number of stop bits in each transmitted or received serial character."]
pub type STOP_BITS_W<'a, const O: u8> = crate::BitWriter<'a, u8, LCR_SPEC, STOP_BITSSELECT_A, O>;
impl<'a, const O: u8> STOP_BITS_W<'a, O> {
    #[doc = "1 Stop bit"]
    #[inline(always)]
    pub fn one_stop_bit(self) -> &'a mut W {
        self.variant(STOP_BITSSELECT_A::ONE_STOP_BIT)
    }
    #[doc = "1.5 or 2 Stop bits"]
    #[inline(always)]
    pub fn one_five_two_stop_bits(self) -> &'a mut W {
        self.variant(STOP_BITSSELECT_A::ONE_FIVE_TWO_STOP_BITS)
    }
}
#[doc = "Field `EN_PAR` reader - ENABLE_PARITY Parity Enable bit."]
pub type EN_PAR_R = crate::BitReader<bool>;
#[doc = "Field `EN_PAR` writer - ENABLE_PARITY Parity Enable bit."]
pub type EN_PAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, LCR_SPEC, bool, O>;
#[doc = "Field `PAR_SEL` reader - PARITY_SELECT Even Parity Select bit."]
pub type PAR_SEL_R = crate::BitReader<PAR_SELSELECT_A>;
#[doc = "PARITY_SELECT Even Parity Select bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAR_SELSELECT_A {
    #[doc = "0: Odd Parity"]
    ODD = 0,
    #[doc = "1: Even Parity"]
    EVEN = 1,
}
impl From<PAR_SELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PAR_SELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PAR_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAR_SELSELECT_A {
        match self.bits {
            false => PAR_SELSELECT_A::ODD,
            true => PAR_SELSELECT_A::EVEN,
        }
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PAR_SELSELECT_A::ODD
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PAR_SELSELECT_A::EVEN
    }
}
#[doc = "Field `PAR_SEL` writer - PARITY_SELECT Even Parity Select bit."]
pub type PAR_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, LCR_SPEC, PAR_SELSELECT_A, O>;
impl<'a, const O: u8> PAR_SEL_W<'a, O> {
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PAR_SELSELECT_A::ODD)
    }
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PAR_SELSELECT_A::EVEN)
    }
}
#[doc = "Field `STICK_PAR` reader - STICK_PARITY Stick Parity bit."]
pub type STICK_PAR_R = crate::BitReader<bool>;
#[doc = "Field `STICK_PAR` writer - STICK_PARITY Stick Parity bit."]
pub type STICK_PAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, LCR_SPEC, bool, O>;
#[doc = "Field `BRK_CTRL` reader - BREAK_CONTROL Set Break Control bit"]
pub type BRK_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `BRK_CTRL` writer - BREAK_CONTROL Set Break Control bit"]
pub type BRK_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u8, LCR_SPEC, bool, O>;
#[doc = "Field `DLAB` reader - DLAB Divisor Latch Access Bit (DLAB)."]
pub type DLAB_R = crate::BitReader<bool>;
#[doc = "Field `DLAB` writer - DLAB Divisor Latch Access Bit (DLAB)."]
pub type DLAB_W<'a, const O: u8> = crate::BitWriter<'a, u8, LCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - WORD_LENGTH These two bits specify the number of bits in each transmitted or received serial character."]
    #[inline(always)]
    pub fn word_len(&self) -> WORD_LEN_R {
        WORD_LEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - STOP_BITS This bit specifies the number of stop bits in each transmitted or received serial character."]
    #[inline(always)]
    pub fn stop_bits(&self) -> STOP_BITS_R {
        STOP_BITS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ENABLE_PARITY Parity Enable bit."]
    #[inline(always)]
    pub fn en_par(&self) -> EN_PAR_R {
        EN_PAR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PARITY_SELECT Even Parity Select bit."]
    #[inline(always)]
    pub fn par_sel(&self) -> PAR_SEL_R {
        PAR_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - STICK_PARITY Stick Parity bit."]
    #[inline(always)]
    pub fn stick_par(&self) -> STICK_PAR_R {
        STICK_PAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BREAK_CONTROL Set Break Control bit"]
    #[inline(always)]
    pub fn brk_ctrl(&self) -> BRK_CTRL_R {
        BRK_CTRL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DLAB Divisor Latch Access Bit (DLAB)."]
    #[inline(always)]
    pub fn dlab(&self) -> DLAB_R {
        DLAB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - WORD_LENGTH These two bits specify the number of bits in each transmitted or received serial character."]
    #[inline(always)]
    pub fn word_len(&mut self) -> WORD_LEN_W<0> {
        WORD_LEN_W::new(self)
    }
    #[doc = "Bit 2 - STOP_BITS This bit specifies the number of stop bits in each transmitted or received serial character."]
    #[inline(always)]
    pub fn stop_bits(&mut self) -> STOP_BITS_W<2> {
        STOP_BITS_W::new(self)
    }
    #[doc = "Bit 3 - ENABLE_PARITY Parity Enable bit."]
    #[inline(always)]
    pub fn en_par(&mut self) -> EN_PAR_W<3> {
        EN_PAR_W::new(self)
    }
    #[doc = "Bit 4 - PARITY_SELECT Even Parity Select bit."]
    #[inline(always)]
    pub fn par_sel(&mut self) -> PAR_SEL_W<4> {
        PAR_SEL_W::new(self)
    }
    #[doc = "Bit 5 - STICK_PARITY Stick Parity bit."]
    #[inline(always)]
    pub fn stick_par(&mut self) -> STICK_PAR_W<5> {
        STICK_PAR_W::new(self)
    }
    #[doc = "Bit 6 - BREAK_CONTROL Set Break Control bit"]
    #[inline(always)]
    pub fn brk_ctrl(&mut self) -> BRK_CTRL_W<6> {
        BRK_CTRL_W::new(self)
    }
    #[doc = "Bit 7 - DLAB Divisor Latch Access Bit (DLAB)."]
    #[inline(always)]
    pub fn dlab(&mut self) -> DLAB_W<7> {
        DLAB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Line Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcr](index.html) module"]
pub struct LCR_SPEC;
impl crate::RegisterSpec for LCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lcr::R](R) reader structure"]
impl crate::Readable for LCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcr::W](W) writer structure"]
impl crate::Writable for LCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
