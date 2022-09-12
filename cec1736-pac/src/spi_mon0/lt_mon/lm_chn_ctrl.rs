#[doc = "Register `LM_CHN_CTRL` reader"]
pub struct R(crate::R<LM_CHN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LM_CHN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LM_CHN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LM_CHN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LM_CHN_CTRL` writer"]
pub struct W(crate::W<LM_CHN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LM_CHN_CTRL_SPEC>;
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
impl From<crate::W<LM_CHN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LM_CHN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GO` reader - GO: Run the Load Monitor for this channel"]
pub type GO_R = crate::BitReader<bool>;
#[doc = "Field `GO` writer - GO: Run the Load Monitor for this channel"]
pub type GO_W<'a, const O: u8> = crate::BitWriter<'a, u32, LM_CHN_CTRL_SPEC, bool, O>;
#[doc = "Field `RST` reader - RST: Stop and Reset calculation for this channel when set to 1"]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - RST: Stop and Reset calculation for this channel when set to 1"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LM_CHN_CTRL_SPEC, bool, O>;
#[doc = "Field `RSF` reader - RSF: Reset just Result FIFO pointer when this bit is set to 1"]
pub type RSF_R = crate::BitReader<bool>;
#[doc = "Field `RSF` writer - RSF: Reset just Result FIFO pointer when this bit is set to 1"]
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LM_CHN_CTRL_SPEC, bool, O>;
#[doc = "Field `FPTR` reader - FPTR: Result FIFO pointer. In 32Bit. Ranges: 0 to 11 for SHA-384 result 0 to 7 for SHA-256 result"]
pub type FPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPTR` writer - FPTR: Result FIFO pointer. In 32Bit. Ranges: 0 to 11 for SHA-384 result 0 to 7 for SHA-256 result"]
pub type FPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LM_CHN_CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - GO: Run the Load Monitor for this channel"]
    #[inline(always)]
    pub fn go(&self) -> GO_R {
        GO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RST: Stop and Reset calculation for this channel when set to 1"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RSF: Reset just Result FIFO pointer when this bit is set to 1"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - FPTR: Result FIFO pointer. In 32Bit. Ranges: 0 to 11 for SHA-384 result 0 to 7 for SHA-256 result"]
    #[inline(always)]
    pub fn fptr(&self) -> FPTR_R {
        FPTR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - GO: Run the Load Monitor for this channel"]
    #[inline(always)]
    pub fn go(&mut self) -> GO_W<0> {
        GO_W::new(self)
    }
    #[doc = "Bit 1 - RST: Stop and Reset calculation for this channel when set to 1"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<1> {
        RST_W::new(self)
    }
    #[doc = "Bit 2 - RSF: Reset just Result FIFO pointer when this bit is set to 1"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<2> {
        RSF_W::new(self)
    }
    #[doc = "Bits 8:11 - FPTR: Result FIFO pointer. In 32Bit. Ranges: 0 to 11 for SHA-384 result 0 to 7 for SHA-256 result"]
    #[inline(always)]
    pub fn fptr(&mut self) -> FPTR_W<8> {
        FPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Loadtime Monitor Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lm_chn_ctrl](index.html) module"]
pub struct LM_CHN_CTRL_SPEC;
impl crate::RegisterSpec for LM_CHN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lm_chn_ctrl::R](R) reader structure"]
impl crate::Readable for LM_CHN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lm_chn_ctrl::W](W) writer structure"]
impl crate::Writable for LM_CHN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LM_CHN_CTRL to value 0"]
impl crate::Resettable for LM_CHN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
