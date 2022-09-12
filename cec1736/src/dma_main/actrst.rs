#[doc = "Register `ACTRST` reader"]
pub struct R(crate::R<ACTRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTRST` writer"]
pub struct W(crate::W<ACTRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTRST_SPEC>;
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
impl From<crate::W<ACTRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT` reader - Enable the blocks operation. (R/WS) 1=Enable block. Each individual channel must be enabled separately. 0=Disable all channels."]
pub type ACT_R = crate::BitReader<bool>;
#[doc = "Field `ACT` writer - Enable the blocks operation. (R/WS) 1=Enable block. Each individual channel must be enabled separately. 0=Disable all channels."]
pub type ACT_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACTRST_SPEC, bool, O>;
#[doc = "Field `SOFT_RST` reader - Soft reset the entire module. This bit is self-clearing."]
pub type SOFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `SOFT_RST` writer - Soft reset the entire module. This bit is self-clearing."]
pub type SOFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACTRST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable the blocks operation. (R/WS) 1=Enable block. Each individual channel must be enabled separately. 0=Disable all channels."]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft reset the entire module. This bit is self-clearing."]
    #[inline(always)]
    pub fn soft_rst(&self) -> SOFT_RST_R {
        SOFT_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the blocks operation. (R/WS) 1=Enable block. Each individual channel must be enabled separately. 0=Disable all channels."]
    #[inline(always)]
    pub fn act(&mut self) -> ACT_W<0> {
        ACT_W::new(self)
    }
    #[doc = "Bit 1 - Soft reset the entire module. This bit is self-clearing."]
    #[inline(always)]
    pub fn soft_rst(&mut self) -> SOFT_RST_W<1> {
        SOFT_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Soft reset the entire module. Enable the blocks operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actrst](index.html) module"]
pub struct ACTRST_SPEC;
impl crate::RegisterSpec for ACTRST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [actrst::R](R) reader structure"]
impl crate::Readable for ACTRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actrst::W](W) writer structure"]
impl crate::Writable for ACTRST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACTRST to value 0"]
impl crate::Resettable for ACTRST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
