#[doc = "Register `EN_SET20` reader"]
pub struct R(crate::R<EN_SET20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_SET20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_SET20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_SET20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN_SET20` writer"]
pub struct W(crate::W<EN_SET20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_SET20_SPEC>;
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
impl From<crate::W<EN_SET20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_SET20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMSPI` reader - IMSPI"]
pub type IMSPI_R = crate::BitReader<bool>;
#[doc = "Field `IMSPI` writer - IMSPI"]
pub type IMSPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET20_SPEC, bool, O>;
#[doc = "Field `CLK_MON` reader - CLK_MON"]
pub type CLK_MON_R = crate::BitReader<bool>;
#[doc = "Field `CLK_MON` writer - CLK_MON"]
pub type CLK_MON_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET20_SPEC, bool, O>;
#[doc = "Field `VTR1_PAD_MON` reader - VTR1_PAD_MON"]
pub type VTR1_PAD_MON_R = crate::BitReader<bool>;
#[doc = "Field `VTR1_PAD_MON` writer - VTR1_PAD_MON"]
pub type VTR1_PAD_MON_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET20_SPEC, bool, O>;
#[doc = "Field `VTR2_PAD_MON` reader - VTR2_PAD_MON"]
pub type VTR2_PAD_MON_R = crate::BitReader<bool>;
#[doc = "Field `VTR2_PAD_MON` writer - VTR2_PAD_MON"]
pub type VTR2_PAD_MON_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SET20_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - IMSPI"]
    #[inline(always)]
    pub fn imspi(&self) -> IMSPI_R {
        IMSPI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CLK_MON"]
    #[inline(always)]
    pub fn clk_mon(&self) -> CLK_MON_R {
        CLK_MON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VTR1_PAD_MON"]
    #[inline(always)]
    pub fn vtr1_pad_mon(&self) -> VTR1_PAD_MON_R {
        VTR1_PAD_MON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - VTR2_PAD_MON"]
    #[inline(always)]
    pub fn vtr2_pad_mon(&self) -> VTR2_PAD_MON_R {
        VTR2_PAD_MON_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - IMSPI"]
    #[inline(always)]
    pub fn imspi(&mut self) -> IMSPI_W<8> {
        IMSPI_W::new(self)
    }
    #[doc = "Bit 9 - CLK_MON"]
    #[inline(always)]
    pub fn clk_mon(&mut self) -> CLK_MON_W<9> {
        CLK_MON_W::new(self)
    }
    #[doc = "Bit 10 - VTR1_PAD_MON"]
    #[inline(always)]
    pub fn vtr1_pad_mon(&mut self) -> VTR1_PAD_MON_W<10> {
        VTR1_PAD_MON_W::new(self)
    }
    #[doc = "Bit 11 - VTR2_PAD_MON"]
    #[inline(always)]
    pub fn vtr2_pad_mon(&mut self) -> VTR2_PAD_MON_W<11> {
        VTR2_PAD_MON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ20 ENABLE SET\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_set20](index.html) module"]
pub struct EN_SET20_SPEC;
impl crate::RegisterSpec for EN_SET20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en_set20::R](R) reader structure"]
impl crate::Readable for EN_SET20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en_set20::W](W) writer structure"]
impl crate::Writable for EN_SET20_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN_SET20 to value 0"]
impl crate::Resettable for EN_SET20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
