#[doc = "Register `SAR_CTRL` reader"]
pub struct R(crate::R<SAR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_CTRL` writer"]
pub struct W(crate::W<SAR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_CTRL_SPEC>;
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
impl From<crate::W<SAR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL_DIFF` reader - This field select between Single ended / Differential input.\n 0= ADC core is enabled for single ended input operation.\n 1= ADC core is enabled for differential input operation.\n"]
pub type SEL_DIFF_R = crate::BitReader<bool>;
#[doc = "Field `SEL_DIFF` writer - This field select between Single ended / Differential input.\n 0= ADC core is enabled for single ended input operation.\n 1= ADC core is enabled for differential input operation.\n"]
pub type SEL_DIFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_CTRL_SPEC, bool, O>;
#[doc = "Field `SEL_RES` reader - This field select the ADC Resolution (10/12 bits).\n 0x0= Reserved.\n 0x1= Reserved.\n 0x2= 10 bit ADC resolution.\n 0x3= 12 bit ADC resolution.\n"]
pub type SEL_RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL_RES` writer - This field select the ADC Resolution (10/12 bits).\n 0x0= Reserved.\n 0x1= Reserved.\n 0x2= 10 bit ADC resolution.\n 0x3= 12 bit ADC resolution.\n"]
pub type SEL_RES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SHIFT_DAT` reader - This field defined if the ADC output is Right or Left Justified.\n 1= adc_dout is not shifted and lower bits are set to 0.\n 0= adc_dout is shifted right following resolution selected.\n"]
pub type SHIFT_DAT_R = crate::BitReader<bool>;
#[doc = "Field `SHIFT_DAT` writer - This field defined if the ADC output is Right or Left Justified.\n 1= adc_dout is not shifted and lower bits are set to 0.\n 0= adc_dout is shifted right following resolution selected.\n"]
pub type SHIFT_DAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_CTRL_SPEC, bool, O>;
#[doc = "Field `EN_ASYN_SMPL` reader - This field enables asynchronous sampling.\n 0= Async Sampling Disabled.\n 1= Async Sampling Enabled.\n"]
pub type EN_ASYN_SMPL_R = crate::BitReader<bool>;
#[doc = "Field `EN_ASYN_SMPL` writer - This field enables asynchronous sampling.\n 0= Async Sampling Disabled.\n 1= Async Sampling Enabled.\n"]
pub type EN_ASYN_SMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_CTRL_SPEC, bool, O>;
#[doc = "Field `EN_SERIAL` reader - This field enables serial output (dout) from ADC.\n 0= Parallel dout.\n 1= Serial dout.\n"]
pub type EN_SERIAL_R = crate::BitReader<bool>;
#[doc = "Field `EN_SERIAL` writer - This field enables serial output (dout) from ADC.\n 0= Parallel dout.\n 1= Serial dout.\n"]
pub type EN_SERIAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_CTRL_SPEC, bool, O>;
#[doc = "Field `WARM_UP_DLY` reader - This field represents the warmup delay number in microseconds.\n"]
pub type WARM_UP_DLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WARM_UP_DLY` writer - This field represents the warmup delay number in microseconds.\n"]
pub type WARM_UP_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_CTRL_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bit 0 - This field select between Single ended / Differential input.\n 0= ADC core is enabled for single ended input operation.\n 1= ADC core is enabled for differential input operation.\n"]
    #[inline(always)]
    pub fn sel_diff(&self) -> SEL_DIFF_R {
        SEL_DIFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - This field select the ADC Resolution (10/12 bits).\n 0x0= Reserved.\n 0x1= Reserved.\n 0x2= 10 bit ADC resolution.\n 0x3= 12 bit ADC resolution.\n"]
    #[inline(always)]
    pub fn sel_res(&self) -> SEL_RES_R {
        SEL_RES_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - This field defined if the ADC output is Right or Left Justified.\n 1= adc_dout is not shifted and lower bits are set to 0.\n 0= adc_dout is shifted right following resolution selected.\n"]
    #[inline(always)]
    pub fn shift_dat(&self) -> SHIFT_DAT_R {
        SHIFT_DAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This field enables asynchronous sampling.\n 0= Async Sampling Disabled.\n 1= Async Sampling Enabled.\n"]
    #[inline(always)]
    pub fn en_asyn_smpl(&self) -> EN_ASYN_SMPL_R {
        EN_ASYN_SMPL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This field enables serial output (dout) from ADC.\n 0= Parallel dout.\n 1= Serial dout.\n"]
    #[inline(always)]
    pub fn en_serial(&self) -> EN_SERIAL_R {
        EN_SERIAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:15 - This field represents the warmup delay number in microseconds.\n"]
    #[inline(always)]
    pub fn warm_up_dly(&self) -> WARM_UP_DLY_R {
        WARM_UP_DLY_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - This field select between Single ended / Differential input.\n 0= ADC core is enabled for single ended input operation.\n 1= ADC core is enabled for differential input operation.\n"]
    #[inline(always)]
    pub fn sel_diff(&mut self) -> SEL_DIFF_W<0> {
        SEL_DIFF_W::new(self)
    }
    #[doc = "Bits 1:2 - This field select the ADC Resolution (10/12 bits).\n 0x0= Reserved.\n 0x1= Reserved.\n 0x2= 10 bit ADC resolution.\n 0x3= 12 bit ADC resolution.\n"]
    #[inline(always)]
    pub fn sel_res(&mut self) -> SEL_RES_W<1> {
        SEL_RES_W::new(self)
    }
    #[doc = "Bit 3 - This field defined if the ADC output is Right or Left Justified.\n 1= adc_dout is not shifted and lower bits are set to 0.\n 0= adc_dout is shifted right following resolution selected.\n"]
    #[inline(always)]
    pub fn shift_dat(&mut self) -> SHIFT_DAT_W<3> {
        SHIFT_DAT_W::new(self)
    }
    #[doc = "Bit 4 - This field enables asynchronous sampling.\n 0= Async Sampling Disabled.\n 1= Async Sampling Enabled.\n"]
    #[inline(always)]
    pub fn en_asyn_smpl(&mut self) -> EN_ASYN_SMPL_W<4> {
        EN_ASYN_SMPL_W::new(self)
    }
    #[doc = "Bit 5 - This field enables serial output (dout) from ADC.\n 0= Parallel dout.\n 1= Serial dout.\n"]
    #[inline(always)]
    pub fn en_serial(&mut self) -> EN_SERIAL_W<5> {
        EN_SERIAL_W::new(self)
    }
    #[doc = "Bits 7:15 - This field represents the warmup delay number in microseconds.\n"]
    #[inline(always)]
    pub fn warm_up_dly(&mut self) -> WARM_UP_DLY_W<7> {
        WARM_UP_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the SAR ADC Control Register.\n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_ctrl](index.html) module"]
pub struct SAR_CTRL_SPEC;
impl crate::RegisterSpec for SAR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_ctrl::R](R) reader structure"]
impl crate::Readable for SAR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_ctrl::W](W) writer structure"]
impl crate::Writable for SAR_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_CTRL to value 0x06"]
impl crate::Resettable for SAR_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
