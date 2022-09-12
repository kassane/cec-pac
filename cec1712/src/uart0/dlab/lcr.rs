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
pub type WORD_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WORD_LEN` writer - WORD_LENGTH These two bits specify the number of bits in each transmitted or received serial character."]
pub type WORD_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `STOP_BITS` reader - STOP_BITS This bit specifies the number of stop bits in each transmitted or received serial character."]
pub type STOP_BITS_R = crate::BitReader<bool>;
#[doc = "Field `STOP_BITS` writer - STOP_BITS This bit specifies the number of stop bits in each transmitted or received serial character."]
pub type STOP_BITS_W<'a, const O: u8> = crate::BitWriter<'a, u8, LCR_SPEC, bool, O>;
#[doc = "Field `EN_PAR` reader - ENABLE_PARITY Parity Enable bit."]
pub type EN_PAR_R = crate::BitReader<bool>;
#[doc = "Field `EN_PAR` writer - ENABLE_PARITY Parity Enable bit."]
pub type EN_PAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, LCR_SPEC, bool, O>;
#[doc = "Field `PAR_SEL` reader - PARITY_SELECT Even Parity Select bit."]
pub type PAR_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PAR_SEL` writer - PARITY_SELECT Even Parity Select bit."]
pub type PAR_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, LCR_SPEC, bool, O>;
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
