#[doc = "Register `LM_CTRLSTS` reader"]
pub struct R(crate::R<LM_CTRLSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LM_CTRLSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LM_CTRLSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LM_CTRLSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LM_CTRLSTS` writer"]
pub struct W(crate::W<LM_CTRLSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LM_CTRLSTS_SPEC>;
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
impl From<crate::W<LM_CTRLSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LM_CTRLSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `W` reader - Waiting"]
pub type W_R = crate::BitReader<bool>;
#[doc = "Field `W` writer - Waiting"]
pub type W_W<'a, const O: u8> = crate::BitWriter<'a, u32, LM_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `B` reader - Begin Byte seen"]
pub type B_R = crate::BitReader<bool>;
#[doc = "Field `B` writer - Begin Byte seen"]
pub type B_W<'a, const O: u8> = crate::BitWriter<'a, u32, LM_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `E` reader - End Byte seen"]
pub type E_R = crate::BitReader<bool>;
#[doc = "Field `E` writer - End Byte seen"]
pub type E_W<'a, const O: u8> = crate::BitWriter<'a, u32, LM_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `F` reader - Finalized and result ready"]
pub type F_R = crate::BitReader<bool>;
#[doc = "Field `F` writer - Finalized and result ready"]
pub type F_W<'a, const O: u8> = crate::BitWriter<'a, u32, LM_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `W_INTEN` reader - Enable Waiting Interrupt"]
pub type W_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `W_INTEN` writer - Enable Waiting Interrupt"]
pub type W_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LM_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `B_INTEN` reader - Enable Begin Byte seen Interrupt"]
pub type B_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `B_INTEN` writer - Enable Begin Byte seen Interrupt"]
pub type B_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LM_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `E_INTEN` reader - Enable End Byte seen Interrupt"]
pub type E_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `E_INTEN` writer - Enable End Byte seen Interrupt"]
pub type E_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LM_CTRLSTS_SPEC, bool, O>;
#[doc = "Field `F_INTEN` reader - Enable Finalized and result ready Interrupt"]
pub type F_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `F_INTEN` writer - Enable Finalized and result ready Interrupt"]
pub type F_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LM_CTRLSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Waiting"]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Begin Byte seen"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End Byte seen"]
    #[inline(always)]
    pub fn e(&self) -> E_R {
        E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Finalized and result ready"]
    #[inline(always)]
    pub fn f(&self) -> F_R {
        F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Waiting Interrupt"]
    #[inline(always)]
    pub fn w_inten(&self) -> W_INTEN_R {
        W_INTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Begin Byte seen Interrupt"]
    #[inline(always)]
    pub fn b_inten(&self) -> B_INTEN_R {
        B_INTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable End Byte seen Interrupt"]
    #[inline(always)]
    pub fn e_inten(&self) -> E_INTEN_R {
        E_INTEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Finalized and result ready Interrupt"]
    #[inline(always)]
    pub fn f_inten(&self) -> F_INTEN_R {
        F_INTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Waiting"]
    #[inline(always)]
    pub fn w(&mut self) -> W_W<0> {
        W_W::new(self)
    }
    #[doc = "Bit 1 - Begin Byte seen"]
    #[inline(always)]
    pub fn b(&mut self) -> B_W<1> {
        B_W::new(self)
    }
    #[doc = "Bit 2 - End Byte seen"]
    #[inline(always)]
    pub fn e(&mut self) -> E_W<2> {
        E_W::new(self)
    }
    #[doc = "Bit 3 - Finalized and result ready"]
    #[inline(always)]
    pub fn f(&mut self) -> F_W<3> {
        F_W::new(self)
    }
    #[doc = "Bit 8 - Enable Waiting Interrupt"]
    #[inline(always)]
    pub fn w_inten(&mut self) -> W_INTEN_W<8> {
        W_INTEN_W::new(self)
    }
    #[doc = "Bit 9 - Enable Begin Byte seen Interrupt"]
    #[inline(always)]
    pub fn b_inten(&mut self) -> B_INTEN_W<9> {
        B_INTEN_W::new(self)
    }
    #[doc = "Bit 10 - Enable End Byte seen Interrupt"]
    #[inline(always)]
    pub fn e_inten(&mut self) -> E_INTEN_W<10> {
        E_INTEN_W::new(self)
    }
    #[doc = "Bit 11 - Enable Finalized and result ready Interrupt"]
    #[inline(always)]
    pub fn f_inten(&mut self) -> F_INTEN_W<11> {
        F_INTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Loadtime Monitor Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lm_ctrlsts](index.html) module"]
pub struct LM_CTRLSTS_SPEC;
impl crate::RegisterSpec for LM_CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lm_ctrlsts::R](R) reader structure"]
impl crate::Readable for LM_CTRLSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lm_ctrlsts::W](W) writer structure"]
impl crate::Writable for LM_CTRLSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LM_CTRLSTS to value 0"]
impl crate::Resettable for LM_CTRLSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
