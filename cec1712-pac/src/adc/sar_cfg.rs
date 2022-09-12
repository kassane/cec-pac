#[doc = "Register `SAR_CFG` reader"]
pub struct R(crate::R<SAR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_CFG` writer"]
pub struct W(crate::W<SAR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_CFG_SPEC>;
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
impl From<crate::W<SAR_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_CMBF` reader - Enable Common Mode Buffer Amplifier.\n 0= Common Mode Buffer Amplifier is high all the time.\n 1= Controls Common Mode Buffer Amplifier during power cycling.\n"]
pub type EN_CMBF_R = crate::BitReader<bool>;
#[doc = "Field `EN_CMBF` writer - Enable Common Mode Buffer Amplifier.\n 0= Common Mode Buffer Amplifier is high all the time.\n 1= Controls Common Mode Buffer Amplifier during power cycling.\n"]
pub type EN_CMBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_CFG_SPEC, bool, O>;
#[doc = "Field `DIS_DOUT` reader - Disable Parallel Output.\n 0= Enable Parallel Output.\n 1= Disable Parallel Output.\n"]
pub type DIS_DOUT_R = crate::BitReader<bool>;
#[doc = "Field `DIS_DOUT` writer - Disable Parallel Output.\n 0= Enable Parallel Output.\n 1= Disable Parallel Output.\n"]
pub type DIS_DOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_CFG_SPEC, bool, O>;
#[doc = "Field `EN_DITHER` reader - Enable Dithering.\n 0= Disable Dither.\n 1= Enable Dither.\n"]
pub type EN_DITHER_R = crate::BitReader<bool>;
#[doc = "Field `EN_DITHER` writer - Enable Dithering.\n 0= Disable Dither.\n 1= Enable Dither.\n"]
pub type EN_DITHER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_CFG_SPEC, bool, O>;
#[doc = "Field `FAZ_AU_ZERO` reader - Enable F_AZ AUTOZEROING.\n 1= Disable f_az autozeroing.\n 0= Enable f_az autozeroing.\n"]
pub type FAZ_AU_ZERO_R = crate::BitReader<bool>;
#[doc = "Field `FAZ_AU_ZERO` writer - Enable F_AZ AUTOZEROING.\n 1= Disable f_az autozeroing.\n 0= Enable f_az autozeroing.\n"]
pub type FAZ_AU_ZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_CFG_SPEC, bool, O>;
#[doc = "Field `SAZ_AU_ZERO` reader - Enable S_AZ AUTOZEROING.\n 1= Disable S_AZ autozeroing.\n 0= Enable S_AZ autozeroing.\n"]
pub type SAZ_AU_ZERO_R = crate::BitReader<bool>;
#[doc = "Field `SAZ_AU_ZERO` writer - Enable S_AZ AUTOZEROING.\n 1= Disable S_AZ autozeroing.\n 0= Enable S_AZ autozeroing.\n"]
pub type SAZ_AU_ZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_CFG_SPEC, bool, O>;
#[doc = "Field `LAZ_AU_ZERO` reader - Enable L_AZ AUTOZEROING.\n 1= Disable L_AZ autozeroing.\n 0= Enable L_AZ autozeroing.\n"]
pub type LAZ_AU_ZERO_R = crate::BitReader<bool>;
#[doc = "Field `LAZ_AU_ZERO` writer - Enable L_AZ AUTOZEROING.\n 1= Disable L_AZ autozeroing.\n 0= Enable L_AZ autozeroing.\n"]
pub type LAZ_AU_ZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_CFG_SPEC, bool, O>;
#[doc = "Field `EN_RADC` reader - Enable RADC.\n 1 = RDAC remains high during power cycling.\n 0 = Controls RDAC during power cycling.\n"]
pub type EN_RADC_R = crate::BitReader<bool>;
#[doc = "Field `EN_RADC` writer - Enable RADC.\n 1 = RDAC remains high during power cycling.\n 0 = Controls RDAC during power cycling.\n"]
pub type EN_RADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_CFG_SPEC, bool, O>;
#[doc = "Field `REGEN_DLY` reader - This register defines the delay between regen and latch.\n"]
pub type REGEN_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGEN_DLY` writer - This register defines the delay between regen and latch.\n"]
pub type REGEN_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLK_DIV` reader - This register defines the programmable ADC Clock divider value. Divider ratios of 256,128,64,32,16 are supported.\n"]
pub type CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK_DIV` writer - This register defines the programmable ADC Clock divider value. Divider ratios of 256,128,64,32,16 are supported.\n"]
pub type CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_CFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `IADC_RANGE2` reader - This register controls the current consumption for the whole ADC.\n"]
pub type IADC_RANGE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IADC_RANGE2` writer - This register controls the current consumption for the whole ADC.\n"]
pub type IADC_RANGE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `IADC_RANGE1` reader - This register controls the current consumption for the whole ADC.\n"]
pub type IADC_RANGE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IADC_RANGE1` writer - This register controls the current consumption for the whole ADC.\n"]
pub type IADC_RANGE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `ICMBF_STG1` reader - This register controls the bias current for the 1st stage of the comparator.\n"]
pub type ICMBF_STG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICMBF_STG1` writer - This register controls the bias current for the 1st stage of the comparator.\n"]
pub type ICMBF_STG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `ICMBF_STG2` reader - This register controls the bias current for the 2nd stage of the comparator.\n"]
pub type ICMBF_STG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICMBF_STG2` writer - This register controls the bias current for the 2nd stage of the comparator.\n"]
pub type ICMBF_STG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `ICMBF` reader - This register controls the bias current for common mode buffer amplifier.\n"]
pub type ICMBF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICMBF` writer - This register controls the bias current for common mode buffer amplifier.\n"]
pub type ICMBF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `EN_EXT_BIAS` reader - EN external bias.\n 1 = Disables internal switched cap bias circuit.\n 0 = Enables internal switched cap bias circuit.\n"]
pub type EN_EXT_BIAS_R = crate::BitReader<bool>;
#[doc = "Field `EN_EXT_BIAS` writer - EN external bias.\n 1 = Disables internal switched cap bias circuit.\n 0 = Enables internal switched cap bias circuit.\n"]
pub type EN_EXT_BIAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable Common Mode Buffer Amplifier.\n 0= Common Mode Buffer Amplifier is high all the time.\n 1= Controls Common Mode Buffer Amplifier during power cycling.\n"]
    #[inline(always)]
    pub fn en_cmbf(&self) -> EN_CMBF_R {
        EN_CMBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Parallel Output.\n 0= Enable Parallel Output.\n 1= Disable Parallel Output.\n"]
    #[inline(always)]
    pub fn dis_dout(&self) -> DIS_DOUT_R {
        DIS_DOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Dithering.\n 0= Disable Dither.\n 1= Enable Dither.\n"]
    #[inline(always)]
    pub fn en_dither(&self) -> EN_DITHER_R {
        EN_DITHER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable F_AZ AUTOZEROING.\n 1= Disable f_az autozeroing.\n 0= Enable f_az autozeroing.\n"]
    #[inline(always)]
    pub fn faz_au_zero(&self) -> FAZ_AU_ZERO_R {
        FAZ_AU_ZERO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable S_AZ AUTOZEROING.\n 1= Disable S_AZ autozeroing.\n 0= Enable S_AZ autozeroing.\n"]
    #[inline(always)]
    pub fn saz_au_zero(&self) -> SAZ_AU_ZERO_R {
        SAZ_AU_ZERO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable L_AZ AUTOZEROING.\n 1= Disable L_AZ autozeroing.\n 0= Enable L_AZ autozeroing.\n"]
    #[inline(always)]
    pub fn laz_au_zero(&self) -> LAZ_AU_ZERO_R {
        LAZ_AU_ZERO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable RADC.\n 1 = RDAC remains high during power cycling.\n 0 = Controls RDAC during power cycling.\n"]
    #[inline(always)]
    pub fn en_radc(&self) -> EN_RADC_R {
        EN_RADC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 9:10 - This register defines the delay between regen and latch.\n"]
    #[inline(always)]
    pub fn regen_dly(&self) -> REGEN_DLY_R {
        REGEN_DLY_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:15 - This register defines the programmable ADC Clock divider value. Divider ratios of 256,128,64,32,16 are supported.\n"]
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - This register controls the current consumption for the whole ADC.\n"]
    #[inline(always)]
    pub fn iadc_range2(&self) -> IADC_RANGE2_R {
        IADC_RANGE2_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - This register controls the current consumption for the whole ADC.\n"]
    #[inline(always)]
    pub fn iadc_range1(&self) -> IADC_RANGE1_R {
        IADC_RANGE1_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - This register controls the bias current for the 1st stage of the comparator.\n"]
    #[inline(always)]
    pub fn icmbf_stg1(&self) -> ICMBF_STG1_R {
        ICMBF_STG1_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - This register controls the bias current for the 2nd stage of the comparator.\n"]
    #[inline(always)]
    pub fn icmbf_stg2(&self) -> ICMBF_STG2_R {
        ICMBF_STG2_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - This register controls the bias current for common mode buffer amplifier.\n"]
    #[inline(always)]
    pub fn icmbf(&self) -> ICMBF_R {
        ICMBF_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - EN external bias.\n 1 = Disables internal switched cap bias circuit.\n 0 = Enables internal switched cap bias circuit.\n"]
    #[inline(always)]
    pub fn en_ext_bias(&self) -> EN_EXT_BIAS_R {
        EN_EXT_BIAS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Common Mode Buffer Amplifier.\n 0= Common Mode Buffer Amplifier is high all the time.\n 1= Controls Common Mode Buffer Amplifier during power cycling.\n"]
    #[inline(always)]
    pub fn en_cmbf(&mut self) -> EN_CMBF_W<0> {
        EN_CMBF_W::new(self)
    }
    #[doc = "Bit 1 - Disable Parallel Output.\n 0= Enable Parallel Output.\n 1= Disable Parallel Output.\n"]
    #[inline(always)]
    pub fn dis_dout(&mut self) -> DIS_DOUT_W<1> {
        DIS_DOUT_W::new(self)
    }
    #[doc = "Bit 2 - Enable Dithering.\n 0= Disable Dither.\n 1= Enable Dither.\n"]
    #[inline(always)]
    pub fn en_dither(&mut self) -> EN_DITHER_W<2> {
        EN_DITHER_W::new(self)
    }
    #[doc = "Bit 3 - Enable F_AZ AUTOZEROING.\n 1= Disable f_az autozeroing.\n 0= Enable f_az autozeroing.\n"]
    #[inline(always)]
    pub fn faz_au_zero(&mut self) -> FAZ_AU_ZERO_W<3> {
        FAZ_AU_ZERO_W::new(self)
    }
    #[doc = "Bit 4 - Enable S_AZ AUTOZEROING.\n 1= Disable S_AZ autozeroing.\n 0= Enable S_AZ autozeroing.\n"]
    #[inline(always)]
    pub fn saz_au_zero(&mut self) -> SAZ_AU_ZERO_W<4> {
        SAZ_AU_ZERO_W::new(self)
    }
    #[doc = "Bit 5 - Enable L_AZ AUTOZEROING.\n 1= Disable L_AZ autozeroing.\n 0= Enable L_AZ autozeroing.\n"]
    #[inline(always)]
    pub fn laz_au_zero(&mut self) -> LAZ_AU_ZERO_W<5> {
        LAZ_AU_ZERO_W::new(self)
    }
    #[doc = "Bit 6 - Enable RADC.\n 1 = RDAC remains high during power cycling.\n 0 = Controls RDAC during power cycling.\n"]
    #[inline(always)]
    pub fn en_radc(&mut self) -> EN_RADC_W<6> {
        EN_RADC_W::new(self)
    }
    #[doc = "Bits 9:10 - This register defines the delay between regen and latch.\n"]
    #[inline(always)]
    pub fn regen_dly(&mut self) -> REGEN_DLY_W<9> {
        REGEN_DLY_W::new(self)
    }
    #[doc = "Bits 11:15 - This register defines the programmable ADC Clock divider value. Divider ratios of 256,128,64,32,16 are supported.\n"]
    #[inline(always)]
    pub fn clk_div(&mut self) -> CLK_DIV_W<11> {
        CLK_DIV_W::new(self)
    }
    #[doc = "Bits 20:21 - This register controls the current consumption for the whole ADC.\n"]
    #[inline(always)]
    pub fn iadc_range2(&mut self) -> IADC_RANGE2_W<20> {
        IADC_RANGE2_W::new(self)
    }
    #[doc = "Bits 22:23 - This register controls the current consumption for the whole ADC.\n"]
    #[inline(always)]
    pub fn iadc_range1(&mut self) -> IADC_RANGE1_W<22> {
        IADC_RANGE1_W::new(self)
    }
    #[doc = "Bits 24:25 - This register controls the bias current for the 1st stage of the comparator.\n"]
    #[inline(always)]
    pub fn icmbf_stg1(&mut self) -> ICMBF_STG1_W<24> {
        ICMBF_STG1_W::new(self)
    }
    #[doc = "Bits 26:27 - This register controls the bias current for the 2nd stage of the comparator.\n"]
    #[inline(always)]
    pub fn icmbf_stg2(&mut self) -> ICMBF_STG2_W<26> {
        ICMBF_STG2_W::new(self)
    }
    #[doc = "Bits 28:29 - This register controls the bias current for common mode buffer amplifier.\n"]
    #[inline(always)]
    pub fn icmbf(&mut self) -> ICMBF_W<28> {
        ICMBF_W::new(self)
    }
    #[doc = "Bit 31 - EN external bias.\n 1 = Disables internal switched cap bias circuit.\n 0 = Enables internal switched cap bias circuit.\n"]
    #[inline(always)]
    pub fn en_ext_bias(&mut self) -> EN_EXT_BIAS_W<31> {
        EN_EXT_BIAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the SAR ADC Configuration Register.\n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_cfg](index.html) module"]
pub struct SAR_CFG_SPEC;
impl crate::RegisterSpec for SAR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_cfg::R](R) reader structure"]
impl crate::Readable for SAR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_cfg::W](W) writer structure"]
impl crate::Writable for SAR_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_CFG to value 0"]
impl crate::Resettable for SAR_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
